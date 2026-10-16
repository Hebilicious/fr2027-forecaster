import { describe, expect, it } from "vitest"
import { handleApi, type Assets } from "./site"

const INDEX = "<!doctype html><title>fr2027</title>"

/** The assets as configured: known files, and index.html with 200 for anything else. */
const assets = (files: Record<string, string>): Assets => ({
  fetch: async (request) => {
    const body = files[new URL(request.url).pathname]
    return body === undefined
      ? new Response(INDEX, { headers: { "content-type": "text/html; charset=utf-8" } })
      : new Response(body, { headers: { "content-type": "application/json" } })
  },
})

const site = assets({ "/api/health.json": '{"checked_at":"2026-10-16T12:00:00Z"}' })
const get = (path: string) => handleApi(new Request(`https://fr2027.example${path}`), site)

describe("handleApi", () => {
  it("serves a document that exists", async () => {
    const response = await get("/api/health.json")
    expect(response.status).toBe(200)
    expect(await response.json()).toEqual({ checked_at: "2026-10-16T12:00:00Z" })
  })

  it("answers 404 JSON for an unknown document instead of the single-page fallback", async () => {
    const response = await get("/api/nope.json")
    expect(response.status).toBe(404)
    expect(response.headers.get("content-type")).toBe("application/json; charset=utf-8")
    expect(response.headers.get("cache-control")).toBe("no-store")
    expect(await response.json()).toEqual({ error: "/api/nope.json: no such document" })
  })

  it("passes other answers from the assets through unchanged", async () => {
    const failing: Assets = { fetch: async () => new Response("down", { status: 503 }) }
    const response = await handleApi(new Request("https://fr2027.example/api/health.json"), failing)
    expect(response.status).toBe(503)
    expect(await response.text()).toBe("down")
  })
})
