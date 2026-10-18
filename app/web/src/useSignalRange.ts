import { ref, watch } from "vue"
import { isSignalRange, type SignalRange } from "./signals"

// The period the Signals view's X and press sections cover, one choice for both. It lives in the address
// (`?range=3d`) beside the view's hash, so a link opens the same period.
const PARAM = "range"
const DEFAULT: SignalRange = "7d"

function fromAddress(): SignalRange {
  if (typeof location === "undefined") return DEFAULT
  const value = new URLSearchParams(location.search).get(PARAM)
  return isSignalRange(value) ? value : DEFAULT
}

const range = ref<SignalRange>(fromAddress())

watch(range, (value) => {
  const url = new URL(location.href)
  if (value === DEFAULT) url.searchParams.delete(PARAM)
  else url.searchParams.set(PARAM, value)
  history.replaceState(history.state, "", url)
})

export function useSignalRange() {
  return range
}
