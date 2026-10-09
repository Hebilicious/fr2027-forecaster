import { DatabaseSync } from "node:sqlite"
import { describe, expect, it, vi } from "vitest"
import {
  constantTimeEqual,
  handleInbox,
  MAX_BODY_BYTES,
  type InboxContext,
  type ItemView,
  type Store,
  type StoredItem,
} from "./inbox"
import { SqlStore, type Sql } from "./sql-store"

const BOT = "bot-token-0123456789"
const PIPELINE = "pipeline-token-9876543210"
const START = new Date("2026-10-09T12:00:00Z")
const SECOND = 1000
const DAY = 24 * 60 * 60 * SECOND

/** Keeps items in insertion order, which breaks ties the way the SQL store's `seq` does. */
class MemoryStore implements Store {
  items: StoredItem[] = []

  insert(item: StoredItem): void {
    if (this.items.some(({ id }) => id === item.id)) throw new Error(`duplicate id ${item.id}`)
    this.items.push(item)
  }

  get(id: string): StoredItem | undefined {
    return this.items.find((item) => item.id === id)
  }

  update(item: StoredItem): void {
    this.items = this.items.map((existing) => (existing.id === item.id ? item : existing))
  }

  countPending(): number {
    return this.items.filter(({ status }) => status === "pending").length
  }

  listSince(since: string): ItemView[] {
    return this.items
      .filter(({ received_at }) => received_at >= since)
      .toReversed()
      .toSorted((a, b) => b.received_at.localeCompare(a.received_at))
      .map(({ payload: _payload, ...view }) => view)
  }

  listPending(limit: number): StoredItem[] {
    return this.items
      .filter(({ status }) => status === "pending")
      .toSorted((a, b) => a.received_at.localeCompare(b.received_at))
      .slice(0, limit)
  }

  purgeBefore(before: string): void {
    this.items = this.items.filter(({ received_at }) => received_at >= before)
  }
}

/** The production SqlStore on Node's SQLite, through the same `exec` shape as `ctx.storage.sql`. */
function sqliteStore(): Store {
  const database = new DatabaseSync(":memory:")
  const sql: Sql = {
    exec(query, ...bindings) {
      const rows = database.prepare(query).all(...bindings) as ReturnType<ReturnType<Sql["exec"]>["toArray"]>
      return { toArray: () => rows }
    },
  }
  return new SqlStore(sql)
}

interface Reply {
  readonly status: number
  readonly headers: Headers
  readonly body: Record<string, unknown>
}

interface CallOptions {
  /** The bearer token; null sends no authorization header. */
  readonly token?: string | null
  /** Serialized to JSON. */
  readonly json?: unknown
  /** Sent as is, instead of `json`. */
  readonly raw?: string | Uint8Array<ArrayBuffer>
  readonly headers?: Record<string, string>
}

function inbox(makeStore: () => Store, tokens: InboxContext["tokens"] = { bot: BOT, pipeline: PIPELINE }) {
  const store = makeStore()
  let now = START
  const context: InboxContext = { store, tokens, now: () => now }

  async function call(method: string, path: string, options: CallOptions = {}): Promise<Reply> {
    const { token = null, json, raw, headers = {} } = options
    const body = raw ?? (json === undefined ? null : JSON.stringify(json))
    const request = new Request(`https://fr2027.example.workers.dev${path}`, {
      method,
      headers: { ...headers, ...(token === null ? {} : { authorization: `Bearer ${token}` }) },
      body,
    })
    const response = await handleInbox(request, context)
    return {
      status: response.status,
      headers: response.headers,
      body: (await response.json()) as Record<string, unknown>,
    }
  }

  const bot = {
    submit: (json: unknown, options: CallOptions = {}) =>
      call("POST", "/inbox/items", { token: BOT, json, ...options }),
    list: async () => {
      const reply = await call("GET", "/inbox/items", { token: BOT })
      expect(reply.status).toBe(200)
      return reply.body["items"] as Record<string, unknown>[]
    },
  }
  const pipeline = {
    pending: async () => {
      const reply = await call("GET", "/inbox/pending", { token: PIPELINE })
      expect(reply.status).toBe(200)
      return reply.body["items"] as Record<string, unknown>[]
    },
    resolve: (id: string, json: unknown) => call("POST", `/inbox/items/${id}/resolve`, { token: PIPELINE, json }),
  }

  /** Submits an item and returns its id. */
  async function submitted(payload: Record<string, unknown> = { n: 1 }, kind = "poll"): Promise<string> {
    const reply = await bot.submit({ kind, payload })
    expect(reply.status).toBe(201)
    return reply.body["id"] as string
  }

  return {
    store,
    call,
    bot,
    pipeline,
    submitted,
    advance: (ms: number) => {
      now = new Date(now.getTime() + ms)
    },
  }
}

/** A valid submission whose JSON is exactly `bytes` long. */
function bodyOf(bytes: number): string {
  const empty = JSON.stringify({ kind: "poll", payload: { padding: "" } })
  return JSON.stringify({ kind: "poll", payload: { padding: "a".repeat(bytes - empty.length) } })
}

const stores: [string, () => Store][] = [
  ["memory", () => new MemoryStore()],
  ["SQLite", sqliteStore],
]

describe.each(stores)("the inbox on the %s store", (_name, makeStore) => {
  describe("routing", () => {
    it.each([
      "/inbox/",
      "/inbox/item",
      "/inbox/items/",
      "/inbox/items/x",
      "/inbox/items/x/resolve/y",
      "/inbox/pending/x",
    ])("answers 404 for %s, before checking the token", async (path) => {
      const reply = await inbox(makeStore).call("GET", path)
      expect(reply.status).toBe(404)
      expect(reply.body).toEqual({ error: `no inbox route ${path}` })
    })

    it.each([
      ["PUT", "/inbox/items", "GET, POST"],
      ["DELETE", "/inbox/items", "GET, POST"],
      ["POST", "/inbox/pending", "GET"],
      ["GET", "/inbox/items/x/resolve", "POST"],
      ["constructor", "/inbox/items", "GET, POST"],
      ["toString", "/inbox/pending", "GET"],
    ])("answers 405 for %s %s and lists the allowed methods", async (method, path, allow) => {
      const reply = await inbox(makeStore).call(method, path)
      expect(reply.status).toBe(405)
      expect(reply.headers.get("allow")).toBe(allow)
      expect(reply.body["error"]).toContain(allow)
    })

    it("answers JSON that is never cached, errors included", async () => {
      const { call, bot, submitted } = inbox(makeStore)
      const id = await submitted()
      const replies = [
        await bot.submit({ kind: "poll", payload: {} }),
        await call("GET", "/inbox/items", { token: BOT }),
        await call("GET", "/inbox/pending", { token: PIPELINE }),
        await call("POST", `/inbox/items/${id}/resolve`, { token: PIPELINE, json: { status: "accepted", note: "" } }),
        await call("GET", "/inbox/nope"),
        await call("PATCH", "/inbox/items"),
        await call("GET", "/inbox/items"),
        await bot.submit({ kind: "nope" }),
      ]
      for (const reply of replies) {
        expect(reply.headers.get("content-type")).toBe("application/json; charset=utf-8")
        expect(reply.headers.get("cache-control")).toBe("no-store")
      }
    })
  })

  describe("authentication", () => {
    it.each([undefined, ""])("answers 503 on the bot's routes when its token is %j", async (bot) => {
      const { call } = inbox(makeStore, { bot, pipeline: PIPELINE })
      for (const [method, path] of [
        ["GET", "/inbox/items"],
        ["POST", "/inbox/items"],
      ] as const) {
        const reply = await call(method, path, { token: BOT })
        expect(reply.status).toBe(503)
        expect(reply.body).toEqual({ error: "inbox not configured" })
      }
      expect((await call("GET", "/inbox/pending", { token: PIPELINE })).status).toBe(200)
    })

    it.each([undefined, ""])("answers 503 on the pipeline's routes when its token is %j", async (pipeline) => {
      const { call } = inbox(makeStore, { bot: BOT, pipeline })
      expect((await call("GET", "/inbox/pending", { token: PIPELINE })).body).toEqual({ error: "inbox not configured" })
      expect((await call("POST", "/inbox/items/x/resolve", { token: PIPELINE })).status).toBe(503)
      expect((await call("GET", "/inbox/items", { token: BOT })).status).toBe(200)
    })

    it.each([
      ["no authorization header", {}],
      ["an empty bearer token", { headers: { authorization: "Bearer " } }],
      ["another scheme", { headers: { authorization: `Basic ${BOT}` } }],
      ["a wrong token", { token: "bot-token-0123456788" }],
      ["a prefix of the token", { token: BOT.slice(0, -1) }],
      ["the token with a suffix", { token: `${BOT}x` }],
      ["the pipeline's token", { token: PIPELINE }],
    ] satisfies [string, CallOptions][])("answers 401 to the bot's routes with %s", async (_case, options) => {
      const { call, store } = inbox(makeStore)
      for (const method of ["GET", "POST"]) {
        const json = method === "POST" ? { kind: "poll", payload: {} } : undefined
        const reply = await call(method, "/inbox/items", { json, ...options })
        expect(reply.status).toBe(401)
        expect(reply.body).toEqual({ error: "missing or wrong bearer token" })
        expect(reply.headers.get("www-authenticate")).toBe('Bearer realm="inbox"')
      }
      expect(store.countPending()).toBe(0)
    })

    it("answers 401 to the pipeline's routes with the bot's token", async () => {
      const { call, submitted } = inbox(makeStore)
      const id = await submitted()
      expect((await call("GET", "/inbox/pending", { token: BOT })).status).toBe(401)
      const resolve = await call("POST", `/inbox/items/${id}/resolve`, {
        token: BOT,
        json: { status: "accepted", note: "" },
      })
      expect(resolve.status).toBe(401)
    })

    it("ignores whitespace around the configured secrets", async () => {
      const { call } = inbox(makeStore, { bot: `${BOT}\n`, pipeline: `  ${PIPELINE} ` })
      expect((await call("GET", "/inbox/items", { token: BOT })).status).toBe(200)
      expect((await call("GET", "/inbox/pending", { token: PIPELINE })).status).toBe(200)
      expect((await call("GET", "/inbox/items", { token: `${BOT}\n` })).status).toBe(200)
    })

    it("answers 503 when a secret is only whitespace", async () => {
      const { call } = inbox(makeStore, { bot: " \n", pipeline: PIPELINE })
      expect((await call("GET", "/inbox/items", { token: BOT })).status).toBe(503)
    })

    it("accepts the scheme in any case", async () => {
      const { call } = inbox(makeStore)
      const reply = await call("GET", "/inbox/items", { headers: { authorization: `bearer ${BOT}` } })
      expect(reply.status).toBe(200)
    })
  })

  describe("POST /inbox/items", () => {
    it("stores a pending item and answers 201 with its id", async () => {
      const { bot, pipeline } = inbox(makeStore)
      const reply = await bot.submit({ kind: "x_drop", payload: { window: "2026-10-09T11", trends: ["a"] } })
      expect(reply.status).toBe(201)
      expect(reply.body).toEqual({ id: expect.stringMatching(/^20261009T120000Z-[0-9a-f]{16}$/), status: "pending" })
      expect(await pipeline.pending()).toEqual([
        {
          id: reply.body["id"],
          kind: "x_drop",
          received_at: "2026-10-09T12:00:00Z",
          payload: { window: "2026-10-09T11", trends: ["a"] },
        },
      ])
    })

    it("accepts every kind", async () => {
      const { submitted, pipeline } = inbox(makeStore)
      for (const kind of ["x_drop", "poll", "event"]) await submitted({}, kind)
      expect((await pipeline.pending()).map(({ kind }) => kind)).toEqual(["x_drop", "poll", "event"])
    })

    it("gives unique ids that sort in the order items arrive", async () => {
      const { submitted, advance } = inbox(makeStore)
      const ids = [await submitted(), await submitted()]
      advance(SECOND)
      ids.push(await submitted())
      advance(DAY)
      ids.push(await submitted())
      expect(new Set(ids).size).toBe(4)
      expect(ids[2]?.startsWith("20261009T120001Z-")).toBe(true)
      expect(ids.slice(2)).toEqual(ids.slice(2).toSorted())
      expect(ids[3]! > ids[1]! && ids[3]! > ids[0]!).toBe(true)
    })

    it.each([
      ["an empty body", "", "the body is not valid JSON"],
      ["truncated JSON", '{"kind": "poll"', "the body is not valid JSON"],
      ["an array", "[]", "the body must be a JSON object"],
      ["null", "null", "the body must be a JSON object"],
      ["a string", '"poll"', "the body must be a JSON object"],
      ["no kind", '{"payload": {}}', '"kind" must be one of "x_drop", "poll", "event"'],
      ["an unknown kind", '{"kind": "tweet", "payload": {}}', '"kind" must be one of'],
      ["a kind in the wrong case", '{"kind": "POLL", "payload": {}}', '"kind" must be one of'],
      ["no payload", '{"kind": "poll"}', '"payload" must be a JSON object'],
      ["an array payload", '{"kind": "poll", "payload": []}', '"payload" must be a JSON object'],
      ["a string payload", '{"kind": "poll", "payload": "{}"}', '"payload" must be a JSON object'],
      ["a null payload", '{"kind": "poll", "payload": null}', '"payload" must be a JSON object'],
      ["an extra field", '{"kind": "poll", "payload": {}, "source": "x"}', 'unexpected field "source"'],
    ])("answers 400 to %s", async (_case, raw, message) => {
      const { bot, store } = inbox(makeStore)
      const reply = await bot.submit(undefined, { raw })
      expect(reply.status).toBe(400)
      expect(reply.body["error"]).toContain(message)
      expect(store.countPending()).toBe(0)
    })

    it("answers 400 to a body that is not UTF-8", async () => {
      const reply = await inbox(makeStore).bot.submit(undefined, { raw: new Uint8Array([0x7b, 0xff, 0x7d]) })
      expect(reply.status).toBe(400)
      expect(reply.body).toEqual({ error: "the body is not valid UTF-8" })
    })

    it("accepts a body of exactly 256 KiB", async () => {
      const raw = bodyOf(MAX_BODY_BYTES)
      expect(new TextEncoder().encode(raw).byteLength).toBe(262_144)
      expect((await inbox(makeStore).bot.submit(undefined, { raw })).status).toBe(201)
    })

    it("answers 413 to a larger body, read as a stream", async () => {
      const { bot, store } = inbox(makeStore)
      const raw = bodyOf(MAX_BODY_BYTES + 1)
      expect(new Request("https://x/", { method: "POST", body: raw }).headers.get("content-length")).toBeNull()
      const reply = await bot.submit(undefined, { raw })
      expect(reply.status).toBe(413)
      expect(reply.body).toEqual({ error: "the body must be at most 262144 bytes" })
      expect(store.countPending()).toBe(0)
    })

    it("answers 413 when content-length announces a larger body", async () => {
      const reply = await inbox(makeStore).bot.submit(
        { kind: "poll", payload: {} },
        { headers: { "content-length": String(MAX_BODY_BYTES + 1) } },
      )
      expect(reply.status).toBe(413)
    })

    it("answers 429 while 200 items are pending, and takes items again once one is resolved", async () => {
      const { bot, pipeline, submitted, store } = inbox(makeStore)
      const ids: string[] = []
      for (let n = 0; n < 200; n++) ids.push(await submitted({ n }))
      const refused = await bot.submit({ kind: "poll", payload: { n: 200 } })
      expect(refused.status).toBe(429)
      expect(refused.body["error"]).toContain("200 items are already pending")
      expect(store.countPending()).toBe(200)
      expect((await pipeline.resolve(ids[0]!, { status: "rejected", note: "duplicate" })).status).toBe(200)
      expect((await bot.submit({ kind: "poll", payload: { n: 200 } })).status).toBe(201)
      expect((await bot.submit({ kind: "poll", payload: { n: 201 } })).status).toBe(429)
    })
  })

  describe("GET /inbox/items", () => {
    it("lists the bot's items newest first, with their outcome and without payloads", async () => {
      const { bot, pipeline, submitted, advance } = inbox(makeStore)
      const first = await submitted({ secret: "payload" }, "poll")
      advance(SECOND)
      const second = await submitted({}, "event")
      const third = await submitted({}, "x_drop")
      advance(5 * SECOND)
      await pipeline.resolve(first, { status: "accepted", note: "added", ref: "data/raw/polls/ifop-2026-10-08.yaml" })
      expect(await bot.list()).toEqual([
        {
          id: third,
          kind: "x_drop",
          received_at: "2026-10-09T12:00:01Z",
          status: "pending",
          note: null,
          ref: null,
          resolved_at: null,
        },
        {
          id: second,
          kind: "event",
          received_at: "2026-10-09T12:00:01Z",
          status: "pending",
          note: null,
          ref: null,
          resolved_at: null,
        },
        {
          id: first,
          kind: "poll",
          received_at: "2026-10-09T12:00:00Z",
          status: "accepted",
          note: "added",
          ref: "data/raw/polls/ifop-2026-10-08.yaml",
          resolved_at: "2026-10-09T12:00:06Z",
        },
      ])
    })

    it("answers an empty list when there is nothing", async () => {
      expect(await inbox(makeStore).bot.list()).toEqual([])
    })
  })

  describe("GET /inbox/pending", () => {
    it("lists only pending items, oldest first, with their payloads", async () => {
      const { pipeline, submitted, advance } = inbox(makeStore)
      const a = await submitted({ n: "a" })
      advance(SECOND)
      const b = await submitted({ n: "b", nested: { list: [1, 2.5, null, true] } }, "event")
      const c = await submitted({ n: "c" })
      await pipeline.resolve(a, { status: "proposed", note: "needs a look", ref: "https://github.com/o/r/pull/1" })
      expect(await pipeline.pending()).toEqual([
        {
          id: b,
          kind: "event",
          received_at: "2026-10-09T12:00:01Z",
          payload: { n: "b", nested: { list: [1, 2.5, null, true] } },
        },
        { id: c, kind: "poll", received_at: "2026-10-09T12:00:01Z", payload: { n: "c" } },
      ])
    })

    it("answers at most 100 items, the oldest", async () => {
      const { pipeline, submitted, advance } = inbox(makeStore)
      const ids: string[] = []
      for (let n = 0; n < 150; n++) {
        ids.push(await submitted({ n }))
        if (n % 7 === 0) advance(SECOND)
      }
      const pending = await pipeline.pending()
      expect(pending).toHaveLength(100)
      expect(pending.map(({ id }) => id)).toEqual(ids.slice(0, 100))
    })
  })

  describe("POST /inbox/items/{id}/resolve", () => {
    it.each(["accepted", "proposed"])("marks an item %s, drops its payload and answers the item", async (status) => {
      const { pipeline, submitted, store, advance } = inbox(makeStore)
      const id = await submitted({ big: "payload" })
      advance(90 * SECOND)
      const reply = await pipeline.resolve(id, { status, note: "ok", ref: "data/raw/polls/x.yaml" })
      expect(reply.status).toBe(200)
      const resolved = {
        id,
        kind: "poll",
        received_at: "2026-10-09T12:00:00Z",
        status,
        note: "ok",
        ref: "data/raw/polls/x.yaml",
        resolved_at: "2026-10-09T12:01:30Z",
      }
      expect(reply.body).toEqual(resolved)
      expect({ ...store.get(id) }).toEqual({ ...resolved, payload: null })
      expect(await pipeline.pending()).toEqual([])
    })

    it("marks an item rejected and keeps its payload", async () => {
      const { pipeline, submitted, store } = inbox(makeStore)
      const id = await submitted({ keep: "me" })
      const reply = await pipeline.resolve(id, { status: "rejected", note: "schema: missing field fieldwork_end" })
      expect(reply.status).toBe(200)
      expect(reply.body).toMatchObject({ status: "rejected", ref: null, note: "schema: missing field fieldwork_end" })
      expect(reply.body).not.toHaveProperty("payload")
      expect(store.get(id)?.payload).toBe('{"keep":"me"}')
    })

    it("takes a null ref", async () => {
      const { pipeline, submitted } = inbox(makeStore)
      const reply = await pipeline.resolve(await submitted(), { status: "accepted", note: "", ref: null })
      expect(reply.body).toMatchObject({ status: "accepted", note: "", ref: null })
    })

    it("answers 409 for an item that is already resolved, and leaves it as it was", async () => {
      const { pipeline, submitted, store } = inbox(makeStore)
      const id = await submitted()
      await pipeline.resolve(id, { status: "rejected", note: "first" })
      const again = await pipeline.resolve(id, { status: "accepted", note: "second" })
      expect(again.status).toBe(409)
      expect(again.body).toEqual({ error: `item ${id} is already rejected` })
      expect(store.get(id)).toMatchObject({ status: "rejected", note: "first" })
    })

    it("answers 404 for an unknown id", async () => {
      const reply = await inbox(makeStore).pipeline.resolve("20261009T120000Z-0000000000000000", {
        status: "accepted",
        note: "",
      })
      expect(reply.status).toBe(404)
      expect(reply.body).toEqual({ error: "no item 20261009T120000Z-0000000000000000" })
    })

    it.each([
      ["no status", { note: "" }, '"status" must be one of "accepted", "rejected", "proposed"'],
      ["status pending", { status: "pending", note: "" }, '"status" must be one of'],
      ["an unknown status", { status: "done", note: "" }, '"status" must be one of'],
      ["no note", { status: "accepted" }, '"note" must be a string'],
      ["a null note", { status: "accepted", note: null }, '"note" must be a string'],
      ["a note of 2001 characters", { status: "accepted", note: "x".repeat(2001) }, "at most 2000 characters"],
      ["a numeric ref", { status: "accepted", note: "", ref: 3 }, '"ref" must be a string or null'],
      ["an extra field", { status: "accepted", note: "", reason: "x" }, 'unexpected field "reason"'],
      ["an array", [], "the body must be a JSON object"],
    ])("answers 400 to %s and leaves the item pending", async (_case, json, message) => {
      const { pipeline, submitted, store } = inbox(makeStore)
      const id = await submitted()
      const reply = await pipeline.resolve(id, json)
      expect(reply.status).toBe(400)
      expect(reply.body["error"]).toContain(message)
      expect(store.get(id)?.status).toBe("pending")
    })

    it("counts the note in characters, not UTF-16 units", async () => {
      const { pipeline, submitted } = inbox(makeStore)
      const note = "🗳".repeat(2000)
      expect(note.length).toBe(4000)
      const reply = await pipeline.resolve(await submitted(), { status: "rejected", note })
      expect(reply.status).toBe(200)
      expect(reply.body["note"]).toBe(note)
    })
  })

  describe("retention", () => {
    it("hides items older than 30 days at once, and deletes them on the next write", async () => {
      const { bot, submitted, store, advance } = inbox(makeStore)
      const old = await submitted()
      advance(30 * DAY)
      expect((await bot.list()).map(({ id }) => id)).toEqual([old])
      advance(SECOND)
      expect(await bot.list()).toEqual([])
      expect(store.get(old)).toBeDefined()
      const fresh = await submitted()
      expect(store.get(old)).toBeUndefined()
      expect((await bot.list()).map(({ id }) => id)).toEqual([fresh])
    })

    it("deletes old items when an item is resolved", async () => {
      const { pipeline, submitted, store, advance } = inbox(makeStore)
      const old = await submitted()
      advance(20 * DAY)
      const recent = await submitted()
      advance(11 * DAY)
      expect((await pipeline.resolve(recent, { status: "accepted", note: "" })).status).toBe(200)
      expect(store.get(old)).toBeUndefined()
      expect((await pipeline.resolve(old, { status: "accepted", note: "" })).status).toBe(404)
    })

    it("frees the pending slots of items that expired unprocessed", async () => {
      const { bot, submitted, advance } = inbox(makeStore)
      for (let n = 0; n < 200; n++) await submitted({ n })
      expect((await bot.submit({ kind: "poll", payload: {} })).status).toBe(429)
      advance(31 * DAY)
      expect((await bot.submit({ kind: "poll", payload: {} })).status).toBe(201)
    })
  })

  it("answers 500 in JSON when the store fails", async () => {
    const failing = makeStore()
    failing.countPending = () => {
      throw new Error("disk on fire")
    }
    const logged = vi.spyOn(console, "error").mockImplementation(() => {})
    const reply = await inbox(() => failing).bot.submit({ kind: "poll", payload: {} })
    expect(reply.status).toBe(500)
    expect(reply.body).toEqual({ error: "internal error" })
    expect(reply.headers.get("content-type")).toBe("application/json; charset=utf-8")
    expect(logged).toHaveBeenCalledOnce()
    logged.mockRestore()
  })
})

describe("constantTimeEqual", () => {
  it.each([
    ["", "", true],
    ["token", "token", true],
    ["tökén-🗳", "tökén-🗳", true],
    ["token", "tokem", false],
    ["token", "Token", false],
    ["token", "token ", false],
    ["token", "toke", false],
    ["", "token", false],
    ["a".repeat(1000), "a".repeat(999), false],
  ])("compares %j with %j: %s", async (a, b, equal) => {
    expect(await constantTimeEqual(a, b)).toBe(equal)
    expect(await constantTimeEqual(b, a)).toBe(equal)
  })

  it("compares fixed-size digests, so every comparison does the same work", async () => {
    const digest = vi.spyOn(crypto.subtle, "digest")
    await constantTimeEqual("short", "a much longer candidate token")
    expect(digest).toHaveBeenCalledTimes(2)
    expect(digest.mock.calls.every(([algorithm]) => algorithm === "SHA-256")).toBe(true)
    digest.mockRestore()
  })
})
