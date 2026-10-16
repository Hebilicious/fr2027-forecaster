// The inbox: items the Grok bot posts and the GitHub Actions pipeline drains and resolves.
// Everything here is pure: the handler takes a Store, the tokens and a clock, so the Durable
// Object (src/index.ts) and the tests run the same code. The HTTP contract is in README.md.

export const KINDS = ["x_drop", "poll", "event"] as const
export type Kind = (typeof KINDS)[number]

export const RESOLUTIONS = ["accepted", "rejected", "proposed"] as const
export type Resolution = (typeof RESOLUTIONS)[number]
export type Status = "pending" | Resolution

export const MAX_BODY_BYTES = 256 * 1024
export const MAX_PENDING = 200
export const PENDING_PAGE = 100
export const MAX_NOTE_CHARS = 2000
export const RETENTION_DAYS = 30

/** An item as stored. `payload` is JSON text; accepted and proposed items drop it (null). */
export interface StoredItem {
  readonly id: string
  readonly kind: Kind
  /** UTC, `YYYY-MM-DDTHH:MM:SSZ`, so string order is time order. */
  readonly received_at: string
  readonly status: Status
  readonly note: string | null
  readonly ref: string | null
  readonly resolved_at: string | null
  readonly payload: string | null
}

/** An item without its payload: what the bot sees, and what resolving returns. */
export type ItemView = Omit<StoredItem, "payload">

/**
 * Where items live. The methods are synchronous on purpose: each request's check-then-write
 * (count pending then insert, read status then resolve) runs without yielding, which keeps the
 * Durable Object consistent when requests arrive concurrently.
 */
export interface Store {
  insert(item: StoredItem): void
  get(id: string): StoredItem | undefined
  /** Replaces the item that has the same id. */
  update(item: StoredItem): void
  countPending(): number
  /** Items received at or after `since`, newest first. */
  listSince(since: string): ItemView[]
  /** Pending items, oldest first, at most `limit`. */
  listPending(limit: number): StoredItem[]
  /** Deletes every item received before `before`. */
  purgeBefore(before: string): void
}

export interface InboxContext {
  readonly store: Store
  /** The configured secrets; a missing or blank one makes its routes answer 503. */
  readonly tokens: { readonly bot: string | undefined; readonly pipeline: string | undefined }
  readonly now: () => Date
}

type Role = keyof InboxContext["tokens"]

interface Endpoint {
  readonly role: Role
  readonly run: (request: Request, context: InboxContext, id: string) => Promise<Response> | Response
}

interface Route {
  readonly id: string
  readonly methods: Readonly<Partial<Record<string, Endpoint>>>
}

/** An error that becomes a JSON response with this status. */
class HttpError extends Error {
  constructor(
    readonly status: number,
    message: string,
  ) {
    super(message)
  }
}

export async function handleInbox(request: Request, context: InboxContext): Promise<Response> {
  try {
    const { pathname } = new URL(request.url)
    const route = matchRoute(pathname)
    if (route === undefined) return failure(404, `no inbox route ${pathname}`)
    const endpoint = Object.hasOwn(route.methods, request.method) ? route.methods[request.method] : undefined
    if (endpoint === undefined) {
      const allow = Object.keys(route.methods).join(", ")
      return failure(405, `${request.method} is not allowed here; use ${allow}`, { allow })
    }
    // Trimmed: a secret piped into `wrangler secret put` can keep its trailing newline.
    const secret = context.tokens[endpoint.role]?.trim()
    if (secret === undefined || secret === "") return failure(503, "inbox not configured")
    if (!(await hasToken(request, secret))) {
      return failure(401, "missing or wrong bearer token", { "www-authenticate": 'Bearer realm="inbox"' })
    }
    return await endpoint.run(request, context, route.id)
  } catch (error) {
    if (error instanceof HttpError) return failure(error.status, error.message)
    console.error("inbox: unexpected error", error)
    return failure(500, "internal error")
  }
}

function matchRoute(pathname: string): Route | undefined {
  if (pathname === "/inbox/items") return { id: "", methods: { GET: listForBot, POST: submit } }
  if (pathname === "/inbox/pending") return { id: "", methods: { GET: listPending } }
  const resolve = /^\/inbox\/items\/([^/]+)\/resolve$/.exec(pathname)
  if (resolve?.[1] !== undefined) return { id: resolve[1], methods: { POST: resolveItem } }
  return undefined
}

const submit: Endpoint = {
  role: "bot",
  async run(request, { store, now }) {
    const { kind, payload } = parseSubmission(await readJson(request))
    const time = now()
    store.purgeBefore(retentionStart(time))
    if (store.countPending() >= MAX_PENDING) {
      throw new HttpError(429, `${MAX_PENDING} items are already pending; try again after the pipeline's next run`)
    }
    const receivedAt = utcSeconds(time)
    const id = `${receivedAt.replaceAll(/[-:]/g, "")}-${randomHex(8)}`
    store.insert({
      id,
      kind,
      received_at: receivedAt,
      status: "pending",
      note: null,
      ref: null,
      resolved_at: null,
      payload,
    })
    return respond(201, { id, status: "pending" })
  },
}

const listForBot: Endpoint = {
  role: "bot",
  run(_request, { store, now }) {
    return respond(200, { items: store.listSince(retentionStart(now())) })
  },
}

const listPending: Endpoint = {
  role: "pipeline",
  run(_request, { store }) {
    const items = store.listPending(PENDING_PAGE).map(({ id, kind, received_at, payload }) => ({
      id,
      kind,
      received_at,
      payload: JSON.parse(payload ?? "{}") as unknown,
    }))
    return respond(200, { items })
  },
}

const resolveItem: Endpoint = {
  role: "pipeline",
  async run(request, { store, now }, id) {
    const { status, note, ref } = parseResolution(await readJson(request))
    const time = now()
    store.purgeBefore(retentionStart(time))
    const item = store.get(id)
    if (item === undefined) throw new HttpError(404, `no item ${id}`)
    if (item.status !== "pending") throw new HttpError(409, `item ${id} is already ${item.status}`)
    const resolved: StoredItem = {
      ...item,
      status,
      note,
      ref,
      resolved_at: utcSeconds(time),
      payload: status === "rejected" ? item.payload : null,
    }
    store.update(resolved)
    return respond(200, withoutPayload(resolved))
  },
}

function parseSubmission(body: unknown): { kind: Kind; payload: string } {
  const object = expectObject(body, `the body must be a JSON object: {"kind": "poll", "payload": {...}}`, [
    "kind",
    "payload",
  ])
  const { kind, payload } = object
  if (!isOneOf(KINDS, kind)) throw new HttpError(400, `"kind" must be one of ${quoteAll(KINDS)}`)
  if (!isObject(payload)) throw new HttpError(400, `"payload" must be a JSON object`)
  return { kind, payload: JSON.stringify(payload) }
}

function parseResolution(body: unknown): { status: Resolution; note: string; ref: string | null } {
  const object = expectObject(body, `the body must be a JSON object: {"status": "accepted", "note": "..."}`, [
    "status",
    "note",
    "ref",
  ])
  const { status, note, ref } = object
  if (!isOneOf(RESOLUTIONS, status)) throw new HttpError(400, `"status" must be one of ${quoteAll(RESOLUTIONS)}`)
  if (typeof note !== "string") throw new HttpError(400, `"note" must be a string`)
  if ([...note].length > MAX_NOTE_CHARS) {
    throw new HttpError(400, `"note" must be at most ${MAX_NOTE_CHARS} characters`)
  }
  if (ref !== undefined && ref !== null && typeof ref !== "string") {
    throw new HttpError(400, `"ref" must be a string or null`)
  }
  return { status, note, ref: ref ?? null }
}

/** Reads the body as UTF-8 JSON, refusing anything over MAX_BODY_BYTES without buffering it. */
async function readJson(request: Request): Promise<unknown> {
  const tooLarge = new HttpError(413, `the body must be at most ${MAX_BODY_BYTES} bytes`)
  const declared = request.headers.get("content-length")
  if (declared !== null && Number(declared) > MAX_BODY_BYTES) throw tooLarge
  const chunks: Uint8Array[] = []
  let size = 0
  if (request.body !== null) {
    const reader = request.body.getReader()
    for (let chunk = await reader.read(); !chunk.done; chunk = await reader.read()) {
      size += chunk.value.byteLength
      if (size > MAX_BODY_BYTES) {
        await reader.cancel()
        throw tooLarge
      }
      chunks.push(chunk.value)
    }
  }
  const bytes = new Uint8Array(size)
  let offset = 0
  for (const chunk of chunks) {
    bytes.set(chunk, offset)
    offset += chunk.byteLength
  }
  let text: string
  try {
    text = new TextDecoder("utf-8", { fatal: true, ignoreBOM: false }).decode(bytes)
  } catch {
    throw new HttpError(400, "the body is not valid UTF-8")
  }
  try {
    return JSON.parse(text) as unknown
  } catch (error) {
    const reason = error instanceof Error ? `: ${error.message}` : ""
    throw new HttpError(400, `the body is not valid JSON${reason}`)
  }
}

function expectObject(value: unknown, message: string, fields: readonly string[]): Record<string, unknown> {
  if (!isObject(value)) throw new HttpError(400, message)
  const unexpected = Object.keys(value).find((key) => !fields.includes(key))
  if (unexpected !== undefined) {
    throw new HttpError(400, `unexpected field "${unexpected}"; the body takes only ${quoteAll(fields)}`)
  }
  return value
}

function isObject(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value)
}

function isOneOf<T extends string>(options: readonly T[], value: unknown): value is T {
  return typeof value === "string" && (options as readonly string[]).includes(value)
}

function quoteAll(values: readonly string[]): string {
  return values.map((value) => `"${value}"`).join(", ")
}

function withoutPayload(item: StoredItem): ItemView {
  const { id, kind, received_at, status, note, ref, resolved_at } = item
  return { id, kind, received_at, status, note, ref, resolved_at }
}

function respond(status: number, body: unknown, headers: Record<string, string> = {}): Response {
  return new Response(JSON.stringify(body), {
    status,
    headers: { "content-type": "application/json; charset=utf-8", "cache-control": "no-store", ...headers },
  })
}

function failure(status: number, message: string, headers: Record<string, string> = {}): Response {
  return respond(status, { error: message }, headers)
}

async function hasToken(request: Request, secret: string): Promise<boolean> {
  const match = /^Bearer\s+(\S+)\s*$/i.exec(request.headers.get("authorization") ?? "")
  return match?.[1] !== undefined && (await constantTimeEqual(match[1], secret))
}

/**
 * Compares two strings in time that depends on neither their content nor where they differ:
 * both are hashed to SHA-256 and every byte of the digests is compared.
 */
export async function constantTimeEqual(a: string, b: string): Promise<boolean> {
  const encoder = new TextEncoder()
  const [left, right] = await Promise.all([
    crypto.subtle.digest("SHA-256", encoder.encode(a)),
    crypto.subtle.digest("SHA-256", encoder.encode(b)),
  ])
  const rightBytes = new Uint8Array(right)
  const difference = new Uint8Array(left).reduce((acc, byte, index) => acc | (byte ^ (rightBytes[index] ?? 0)), 0)
  return difference === 0
}

/** `YYYY-MM-DDTHH:MM:SSZ` in UTC. */
export function utcSeconds(time: Date): string {
  return `${time.toISOString().slice(0, 19)}Z`
}

/** The oldest `received_at` an item can have and still be kept. */
function retentionStart(now: Date): string {
  return utcSeconds(new Date(now.getTime() - RETENTION_DAYS * 24 * 60 * 60 * 1000))
}

function randomHex(bytes: number): string {
  return Array.from(crypto.getRandomValues(new Uint8Array(bytes)), (byte) => byte.toString(16).padStart(2, "0")).join(
    "",
  )
}
