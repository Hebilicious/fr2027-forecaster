import { Result, Schema } from "effect"
import { Forecast, Health, Polls, Series, Signals } from "./schemas"

// `Missing`: the document does not exist yet. `fr2027 serve` answers 404; the public site has no
// such file and its single-page fallback answers index.html with 200, so non-JSON counts too.
export type Loaded<A> =
  | { readonly _tag: "Loaded"; readonly value: A }
  | { readonly _tag: "Missing"; readonly message: string }
  | { readonly _tag: "Failed"; readonly message: string }

/** `fresh` revalidates with the server instead of trusting the browser cache (the API allows 2 minutes). */
async function load<A>(
  path: string,
  decode: (input: unknown) => Result.Result<A, Schema.SchemaError>,
  fresh: boolean,
): Promise<Loaded<A>> {
  let response: Response
  try {
    response = await fetch(path, { headers: { accept: "application/json" }, cache: fresh ? "no-cache" : "default" })
  } catch (error) {
    return { _tag: "Failed", message: `${path}: ${String(error)}` }
  }
  let body: unknown
  try {
    body = await response.json()
  } catch {
    return response.ok || response.status === 404
      ? { _tag: "Missing", message: `${path}: not found` }
      : { _tag: "Failed", message: `${path}: HTTP ${response.status}, not JSON` }
  }
  if (!response.ok) {
    const error =
      typeof body === "object" && body !== null && "error" in body ? String(body.error) : `HTTP ${response.status}`
    return { _tag: response.status === 404 ? "Missing" : "Failed", message: `${path}: ${error}` }
  }
  const decoded = decode(body)
  return Result.isSuccess(decoded)
    ? { _tag: "Loaded", value: decoded.success }
    : { _tag: "Failed", message: `${path}: unexpected payload: ${decoded.failure.message}` }
}

export const api = {
  forecast: (fresh = false) => load("/api/forecast.json", Schema.decodeUnknownResult(Forecast), fresh),
  series: (fresh = false) => load("/api/series.json", Schema.decodeUnknownResult(Series), fresh),
  polls: (fresh = false) => load("/api/polls.json", Schema.decodeUnknownResult(Polls), fresh),
  health: (fresh = false) => load("/api/health.json", Schema.decodeUnknownResult(Health), fresh),
  signals: (fresh = false) => load("/api/signals.json", Schema.decodeUnknownResult(Signals), fresh),
}
