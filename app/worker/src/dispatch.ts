// The hourly clock. GitHub does not reliably fire the Live workflow's `schedule` trigger, so the
// Worker's cron (wrangler.jsonc) starts it instead: it signs in as the GitHub App, mints an
// installation token limited to this repository and contents:write, and sends a
// repository_dispatch event that the workflow listens for. Like the inbox, the logic takes its
// fetch and clock as arguments, so the tests run it without the network.

export const EVENT_TYPE = "live-hourly"

const API = "https://api.github.com"
const HEADERS = {
  accept: "application/vnd.github+json",
  "x-github-api-version": "2022-11-28",
  "user-agent": "fr2027-worker",
}

export interface DispatchContext {
  /** `owner/name`. */
  readonly repository: string
  /** The GitHub App's client ID, which GitHub accepts as the JWT issuer. */
  readonly clientId: string
  /** A PKCS#8 PEM ("BEGIN PRIVATE KEY"); missing or blank turns the dispatch off. */
  readonly privateKey: string | undefined
  readonly fetch: (request: Request) => Promise<Response>
  readonly now: () => Date
}

/** Sends one repository_dispatch event; returns false, after a warning, when no key is set. */
export async function dispatchLiveRun(context: DispatchContext): Promise<boolean> {
  if (!context.privateKey?.trim()) {
    console.warn("GITHUB_APP_PRIVATE_KEY is not set; the Live workflow was not dispatched")
    return false
  }
  const jwt = await signAppJwt(context.privateKey, context.clientId, context.now())
  const call = (method: string, path: string, token: string, body?: unknown) =>
    callGitHub(context.fetch, method, path, token, body)

  const installation = await call("GET", `/repos/${context.repository}/installation`, `Bearer ${jwt}`)
  const name = context.repository.split("/")[1]
  const access = await call("POST", `/app/installations/${field(installation, "id")}/access_tokens`, `Bearer ${jwt}`, {
    repositories: [name],
    permissions: { contents: "write" },
  })
  await call("POST", `/repos/${context.repository}/dispatches`, `Bearer ${field(access, "token")}`, {
    event_type: EVENT_TYPE,
  })
  return true
}

/**
 * A GitHub App JWT, RS256. `iat` is a minute in the past to allow for clock drift, and `exp`
 * stays under GitHub's ten-minute limit.
 */
export async function signAppJwt(pem: string, issuer: string, now: Date): Promise<string> {
  const seconds = Math.floor(now.getTime() / 1000)
  const header = { alg: "RS256", typ: "JWT" }
  const claims = { iat: seconds - 60, exp: seconds + 540, iss: issuer }
  const input = `${base64url(JSON.stringify(header))}.${base64url(JSON.stringify(claims))}`
  const key = await crypto.subtle.importKey(
    "pkcs8",
    pemToDer(pem),
    { name: "RSASSA-PKCS1-v1_5", hash: "SHA-256" },
    false,
    ["sign"],
  )
  const signature = await crypto.subtle.sign("RSASSA-PKCS1-v1_5", key, new TextEncoder().encode(input))
  return `${input}.${base64url(new Uint8Array(signature))}`
}

function pemToDer(pem: string): Uint8Array<ArrayBuffer> {
  // WebCrypto reads PKCS#8 only. GitHub hands out PKCS#1 keys, which the workflow converts.
  if (pem.includes("BEGIN RSA PRIVATE KEY")) {
    throw new Error("GITHUB_APP_PRIVATE_KEY is a PKCS#1 key; convert it to PKCS#8 (BEGIN PRIVATE KEY)")
  }
  const body = pem.replace(/-----(BEGIN|END) PRIVATE KEY-----/g, "").replace(/\s+/g, "")
  return Uint8Array.from(atob(body), (char) => char.charCodeAt(0))
}

function base64url(data: string | Uint8Array): string {
  const bytes = typeof data === "string" ? new TextEncoder().encode(data) : data
  const binary = Array.from(bytes, (byte) => String.fromCharCode(byte)).join("")
  return btoa(binary).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "")
}

/** One API call; a non-2xx answer throws with its status and body (never the credential). */
async function callGitHub(
  fetch: DispatchContext["fetch"],
  method: string,
  path: string,
  authorization: string,
  body?: unknown,
): Promise<unknown> {
  const headers: Record<string, string> = { ...HEADERS, authorization }
  if (body !== undefined) headers["content-type"] = "application/json"
  const request = new Request(`${API}${path}`, {
    method,
    headers,
    ...(body === undefined ? {} : { body: JSON.stringify(body) }),
  })
  const response = await fetch(request)
  const text = await response.text()
  if (!response.ok) throw new Error(`GitHub ${method} ${path}: ${response.status} ${text}`)
  // The dispatch answers 204 with no body.
  return text ? JSON.parse(text) : null
}

function field(json: unknown, name: string): string {
  const value = typeof json === "object" && json !== null ? (json as Record<string, unknown>)[name] : undefined
  if (typeof value !== "string" && typeof value !== "number") {
    throw new Error(`GitHub answered without a "${name}"`)
  }
  return String(value)
}
