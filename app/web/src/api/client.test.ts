import { afterEach, describe, expect, it, vi } from "vitest"
import { api } from "./client"

const respond = (body: string, status: number, type: string) =>
  vi.stubGlobal("fetch", async () => new Response(body, { status, headers: { "content-type": type } }))

afterEach(() => vi.unstubAllGlobals())

describe("api", () => {
  it("reports a missing document from `fr2027 serve` (404 with an error)", async () => {
    respond(JSON.stringify({ error: "no forecast yet: run `moon run cli:forecast`" }), 404, "application/json")
    expect(await api.forecast()).toEqual({
      _tag: "Missing",
      message: "/api/forecast.json: no forecast yet: run `moon run cli:forecast`",
    })
  })

  it("reports a missing document from the public site (index.html fallback, 200)", async () => {
    respond("<!doctype html><html></html>", 200, "text/html")
    expect((await api.forecast())._tag).toBe("Missing")
  })

  it("fails on server errors and unexpected payloads", async () => {
    respond("<html>Bad gateway</html>", 502, "text/html")
    expect(await api.health()).toEqual({ _tag: "Failed", message: "/api/health.json: HTTP 502, not JSON" })
    respond(JSON.stringify({ checked_at: 1 }), 200, "application/json")
    const result = await api.health()
    expect(result._tag).toBe("Failed")
  })

  it("decodes an empty signals document", async () => {
    const empty = {
      generated_at: "2026-10-09T20:48:33Z",
      markets: { fetched_at: null, sources: [], latest: [], unmatched: [], history: [] },
      attention: { fetched_at: null, rows: [] },
      news: { fetched_at: null, feeds: [], daily: [], headlines: [] },
      x: { rows: [] },
      events: [],
    }
    respond(JSON.stringify(empty), 200, "application/json")
    expect(await api.signals()).toEqual({ _tag: "Loaded", value: empty })
  })
})
