import { afterEach, beforeAll, describe, expect, it, vi } from "vitest"
import { dispatchLiveRun, signAppJwt, type DispatchContext } from "./dispatch"

const NOW = new Date("2026-10-16T12:17:00Z")
const SECONDS = NOW.getTime() / 1000
const CLIENT_ID = "Iv23lierXRciGQXCgOuJ"

let keys: CryptoKeyPair
let pem: string

beforeAll(async () => {
  keys = (await crypto.subtle.generateKey(
    {
      name: "RSASSA-PKCS1-v1_5",
      modulusLength: 2048,
      publicExponent: new Uint8Array([1, 0, 1]),
      hash: "SHA-256",
    },
    true,
    ["sign", "verify"],
  )) as CryptoKeyPair
  const pkcs8 = (await crypto.subtle.exportKey("pkcs8", keys.privateKey)) as ArrayBuffer
  const der = Buffer.from(pkcs8).toString("base64")
  pem = `-----BEGIN PRIVATE KEY-----\n${der.match(/.{1,64}/g)!.join("\n")}\n-----END PRIVATE KEY-----\n`
})

afterEach(() => vi.restoreAllMocks())

const decode = (part: string) => JSON.parse(Buffer.from(part, "base64url").toString("utf8"))

interface Call {
  readonly method: string
  readonly url: string
  readonly headers: Headers
  readonly body: unknown
}

/** A fake GitHub API: records each request and answers from `answers`, in order. */
function github(answers: Response[]) {
  const calls: Call[] = []
  const fetch = async (request: Request) => {
    const text = await request.text()
    calls.push({
      method: request.method,
      url: request.url,
      headers: request.headers,
      body: text ? JSON.parse(text) : undefined,
    })
    const answer = answers.shift()
    if (!answer) throw new Error(`unexpected request ${request.method} ${request.url}`)
    return answer
  }
  return { calls, fetch }
}

const json = (body: unknown, status = 200) => new Response(JSON.stringify(body), { status })

const context = (fetch: DispatchContext["fetch"], overrides: Partial<DispatchContext> = {}): DispatchContext => ({
  repository: "Hebilicious/fr2027-forecaster",
  clientId: CLIENT_ID,
  privateKey: pem,
  fetch,
  now: () => NOW,
  ...overrides,
})

describe("signAppJwt", () => {
  it("signs an RS256 JWT with the App's claims", async () => {
    const jwt = await signAppJwt(pem, CLIENT_ID, NOW)
    const [header, claims, signature] = jwt.split(".")
    expect(decode(header!)).toEqual({ alg: "RS256", typ: "JWT" })
    expect(decode(claims!)).toEqual({ iat: SECONDS - 60, exp: SECONDS + 540, iss: CLIENT_ID })
    const valid = await crypto.subtle.verify(
      "RSASSA-PKCS1-v1_5",
      keys.publicKey,
      Buffer.from(signature!, "base64url"),
      new TextEncoder().encode(`${header}.${claims}`),
    )
    expect(valid).toBe(true)
  })

  it("refuses a PKCS#1 key with a message that says so", async () => {
    const pkcs1 = "-----BEGIN RSA PRIVATE KEY-----\nAAAA\n-----END RSA PRIVATE KEY-----\n"
    await expect(signAppJwt(pkcs1, CLIENT_ID, NOW)).rejects.toThrow(/PKCS#1/)
  })
})

describe("dispatchLiveRun", () => {
  it("finds the installation, mints a narrow token, then dispatches live-hourly", async () => {
    const api = github([
      json({ id: 4242 }),
      json({ token: "ghs_installation" }, 201),
      new Response(null, { status: 204 }),
    ])
    expect(await dispatchLiveRun(context(api.fetch))).toBe(true)

    expect(api.calls.map((call) => `${call.method} ${call.url}`)).toEqual([
      "GET https://api.github.com/repos/Hebilicious/fr2027-forecaster/installation",
      "POST https://api.github.com/app/installations/4242/access_tokens",
      "POST https://api.github.com/repos/Hebilicious/fr2027-forecaster/dispatches",
    ])
    const [installation, token, dispatch] = api.calls
    expect(installation!.body).toBeUndefined()
    expect(token!.body).toEqual({ repositories: ["fr2027-forecaster"], permissions: { contents: "write" } })
    expect(dispatch!.body).toEqual({ event_type: "live-hourly" })

    // The App's JWT for the first two calls, the installation token for the dispatch.
    const jwt = installation!.headers.get("authorization")!.replace(/^Bearer /, "")
    expect(decode(jwt.split(".")[1]!).iss).toBe(CLIENT_ID)
    expect(token!.headers.get("authorization")).toBe(`Bearer ${jwt}`)
    expect(dispatch!.headers.get("authorization")).toBe("Bearer ghs_installation")
    for (const call of api.calls) {
      expect(call.headers.get("accept")).toBe("application/vnd.github+json")
      expect(call.headers.get("x-github-api-version")).toBe("2022-11-28")
      expect(call.headers.get("user-agent")).toBe("fr2027-worker")
    }
  })

  it("warns and does nothing without a private key", async () => {
    const warn = vi.spyOn(console, "warn").mockImplementation(() => {})
    const api = github([])
    for (const key of [undefined, "", "  \n"]) {
      expect(await dispatchLiveRun(context(api.fetch, { privateKey: key }))).toBe(false)
    }
    expect(api.calls).toEqual([])
    expect(warn).toHaveBeenCalledWith(expect.stringContaining("GITHUB_APP_PRIVATE_KEY"))
  })

  it("throws with GitHub's status and body, and stops there", async () => {
    const api = github([json({ id: 4242 }), json({ message: "Bad credentials" }, 401)])
    const failure = dispatchLiveRun(context(api.fetch))
    await expect(failure).rejects.toThrow(
      'GitHub POST /app/installations/4242/access_tokens: 401 {"message":"Bad credentials"}',
    )
    expect(api.calls).toHaveLength(2)
  })

  it("does not put the installation token in the error when the dispatch fails", async () => {
    const api = github([
      json({ id: 4242 }),
      json({ token: "ghs_installation" }, 201),
      json({ message: "Not Found" }, 404),
    ])
    const error = await dispatchLiveRun(context(api.fetch)).catch((thrown: unknown) => thrown)
    expect(String(error)).toContain("dispatches: 404")
    expect(String(error)).not.toContain("ghs_installation")
  })
})
