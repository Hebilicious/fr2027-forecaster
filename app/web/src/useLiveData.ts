import { onBeforeUnmount, onMounted, ref, shallowRef } from "vue"
import { api, type Loaded } from "./api/client"
import type { Forecast, Health, PollRow, SeriesPoint } from "./api/schemas"

const REFRESH_MS = 60_000

function unwrap<A>(result: Loaded<A>, errors: string[]): A | null {
  if (result._tag === "Loaded") return result.value
  errors.push(result.message)
  return null
}

/**
 * Loads the latest forecast and everything shown beside it, then checks for a newer forecast
 * every minute. A failed refresh keeps the last good data on screen and reports the error.
 */
export function useLiveData() {
  const forecast = shallowRef<Forecast | null>(null)
  const series = shallowRef<readonly SeriesPoint[]>([])
  const polls = shallowRef<readonly PollRow[]>([])
  const health = shallowRef<Health | null>(null)
  const loading = ref(true)
  const error = ref<string | null>(null)
  const missing = ref(false)
  let timer: ReturnType<typeof setInterval> | undefined

  async function refresh(): Promise<void> {
    const errors: string[] = []
    const latest = await api.latestForecast()
    if (latest._tag === "Failed" && latest.message.includes("no forecast yet")) {
      missing.value = forecast.value === null
    }
    const next = unwrap(latest, errors)
    const changed = next !== null && next.generated_at !== forecast.value?.generated_at
    if (changed || forecast.value === null) {
      const [s, p, h] = await Promise.all([api.series(), api.polls(), api.health()])
      const nextSeries = unwrap(s, errors)
      const nextPolls = unwrap(p, errors)
      const nextHealth = unwrap(h, errors)
      if (nextSeries !== null) series.value = nextSeries.points
      if (nextPolls !== null) polls.value = nextPolls.rows
      if (nextHealth !== null) health.value = nextHealth
      if (next !== null) {
        forecast.value = next
        missing.value = false
      }
    }
    error.value = errors.length > 0 ? errors.join("; ") : null
    loading.value = false
  }

  onMounted(() => {
    void refresh()
    timer = setInterval(() => void refresh(), REFRESH_MS)
  })
  onBeforeUnmount(() => clearInterval(timer))

  return { forecast, series, polls, health, loading, error, missing, refresh }
}
