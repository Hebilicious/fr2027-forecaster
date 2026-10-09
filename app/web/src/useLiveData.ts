import { onBeforeUnmount, onMounted, ref, shallowRef } from "vue"
import { api, type Loaded } from "./api/client"
import type { Forecast, Health, PollRow, SeriesPoint, Signals } from "./api/schemas"

const POLL_MS = 120_000

function unwrap<A>(result: Loaded<A>, errors: string[]): A | null {
  if (result._tag === "Loaded") return result.value
  errors.push(result.message)
  return null
}

/**
 * Loads the latest forecast and everything shown beside it, then checks `/api/health.json` every
 * two minutes and reloads everything when the site has been rebuilt (`checked_at` changed) or the
 * last load failed somewhere. A failed refresh keeps the last good data on screen and reports the
 * error.
 */
export function useLiveData() {
  const forecast = shallowRef<Forecast | null>(null)
  const series = shallowRef<readonly SeriesPoint[]>([])
  const polls = shallowRef<readonly PollRow[]>([])
  const health = shallowRef<Health | null>(null)
  const signals = shallowRef<Signals | null>(null)
  const loading = ref(true)
  const error = ref<string | null>(null)
  const missing = ref(false)
  let timer: ReturnType<typeof setInterval> | undefined
  // Errors from the last full load, kept so a successful health check doesn't hide them.
  let loadErrors: string[] = []
  // One request at a time: a tick that lands while a load or check is running is skipped.
  let busy = false

  async function reload(fresh: boolean): Promise<void> {
    const errors: string[] = []
    const [f, s, p, h, g] = await Promise.all([
      api.forecast(fresh),
      api.series(fresh),
      api.polls(fresh),
      api.health(fresh),
      api.signals(fresh),
    ])
    if (f._tag === "Missing") missing.value = forecast.value === null
    const next = unwrap(f, errors)
    const nextSeries = unwrap(s, errors)
    const nextPolls = unwrap(p, errors)
    const nextHealth = unwrap(h, errors)
    const nextSignals = unwrap(g, errors)
    if (nextSeries !== null) series.value = nextSeries.points
    if (nextPolls !== null) polls.value = nextPolls.rows
    if (nextHealth !== null) health.value = nextHealth
    if (nextSignals !== null) signals.value = nextSignals
    if (next !== null) {
      forecast.value = next
      missing.value = false
    }
    loadErrors = errors
    error.value = loadErrors.length > 0 ? loadErrors.join("; ") : null
    loading.value = false
  }

  async function check(): Promise<void> {
    const latest = await api.health(true)
    if (latest._tag !== "Loaded") {
      error.value = [...loadErrors, latest.message].join("; ")
      return
    }
    if (latest.value.checked_at !== health.value?.checked_at || loadErrors.length > 0) {
      await reload(true)
    } else {
      error.value = null
    }
  }

  async function once(task: () => Promise<void>): Promise<void> {
    if (busy) return
    busy = true
    try {
      await task()
    } finally {
      busy = false
    }
  }

  onMounted(() => {
    void once(() => reload(false))
    timer = setInterval(() => void once(check), POLL_MS)
  })
  onBeforeUnmount(() => clearInterval(timer))

  return { forecast, series, polls, health, signals, loading, error, missing, refresh: () => once(() => reload(true)) }
}
