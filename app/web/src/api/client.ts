import { Result, Schema } from "effect"
import { Forecast, Health, Polls, Series } from "./schemas"

export type Loaded<A> =
  | { readonly _tag: "Loaded"; readonly value: A }
  | { readonly _tag: "Failed"; readonly message: string }

async function load<A>(
  path: string,
  decode: (input: unknown) => Result.Result<A, Schema.SchemaError>,
): Promise<Loaded<A>> {
  let response: Response
  try {
    response = await fetch(path, { headers: { accept: "application/json" } })
  } catch (error) {
    return { _tag: "Failed", message: `${path}: ${String(error)}` }
  }
  let body: unknown
  try {
    body = await response.json()
  } catch {
    return { _tag: "Failed", message: `${path}: HTTP ${response.status}, not JSON` }
  }
  if (!response.ok) {
    const error =
      typeof body === "object" && body !== null && "error" in body ? String(body.error) : `HTTP ${response.status}`
    return { _tag: "Failed", message: `${path}: ${error}` }
  }
  const decoded = decode(body)
  return Result.isSuccess(decoded)
    ? { _tag: "Loaded", value: decoded.success }
    : { _tag: "Failed", message: `${path}: unexpected payload: ${decoded.failure.message}` }
}

export const api = {
  latestForecast: () => load("/api/forecasts/latest", Schema.decodeUnknownResult(Forecast)),
  series: () => load("/api/series", Schema.decodeUnknownResult(Series)),
  polls: () => load("/api/polls", Schema.decodeUnknownResult(Polls)),
  health: () => load("/api/health", Schema.decodeUnknownResult(Health)),
}
