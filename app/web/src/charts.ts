// Pure preparation of chart data, kept out of the components so it can be tested headlessly.
import type { CandidateForecast, Forecast, PairForecast, PollRow, SeriesPoint } from "./api/schemas"

export interface PollDot {
  readonly pollId: string
  readonly firm: string
  readonly date: string
  readonly scenario: string
  readonly share: number
  readonly sourceUrl: string
}

export interface Panel {
  readonly candidate: CandidateForecast
  readonly points: readonly SeriesPoint[]
  readonly polls: readonly PollDot[]
}

/** One small-multiple panel per candidate with a latent-share history, latest mean first. */
export function round1Panels(forecast: Forecast, series: readonly SeriesPoint[], polls: readonly PollRow[]): Panel[] {
  const byCandidate = new Map<string, SeriesPoint[]>()
  for (const point of series) {
    const list = byCandidate.get(point.candidate_id) ?? []
    list.push(point)
    byCandidate.set(point.candidate_id, list)
  }
  const panels: Panel[] = []
  for (const candidate of forecast.candidates) {
    const points = (byCandidate.get(candidate.candidate_id) ?? []).toSorted((a, b) => a.date.localeCompare(b.date))
    if (points.length === 0) continue
    const dots = polls
      .filter((row) => row.round === 1 && row.candidate_id === candidate.candidate_id)
      .map((row) => ({
        pollId: row.poll_id,
        firm: row.firm,
        date: midpoint(row.field_start, row.field_end),
        scenario: row.scenario_id,
        share: row.share / 100,
        sourceUrl: row.source_url,
      }))
    panels.push({ candidate, points, polls: dots })
  }
  return panels.toSorted((a, b) => (b.points.at(-1)?.mean ?? 0) - (a.points.at(-1)?.mean ?? 0))
}

/** The fieldwork midpoint, as the model places a poll. */
export function midpoint(start: string, end: string): string {
  const a = Date.parse(`${start}T00:00:00Z`)
  const b = Date.parse(`${end}T00:00:00Z`)
  const days = Math.floor((b - a) / 86_400_000 / 2)
  return new Date(a + days * 86_400_000).toISOString().slice(0, 10)
}

export interface Scale {
  readonly domain: readonly [number, number]
  readonly range: readonly [number, number]
}

export function scale(value: number, { domain, range }: Scale): number {
  const [d0, d1] = domain
  const [r0, r1] = range
  if (d1 === d0) return (r0 + r1) / 2
  return r0 + ((value - d0) / (d1 - d0)) * (r1 - r0)
}

/** Clean y-axis ticks for shares: steps of 5 or 10 points from 0 to just above `max`. */
export function shareTicks(max: number): number[] {
  const step = max > 0.3 ? 0.1 : 0.05
  const top = Math.max(step, Math.ceil(max / step) * step)
  const ticks: number[] = []
  for (let t = 0; t <= top + 1e-9; t += step) ticks.push(Math.round(t * 100) / 100)
  return ticks
}

export interface MatrixCell {
  readonly row: string
  readonly column: string
  readonly pMatchup: number
  /** Probability the row candidate wins this matchup, when it ever happens. */
  readonly rowWins: number | null
  readonly polled: boolean
}

/** The round 2 matrix over the `size` candidates most likely to qualify. */
export function round2Matrix(
  forecast: Forecast,
  size: number,
): { candidates: CandidateForecast[]; cells: MatrixCell[] } {
  const candidates = forecast.candidates
    .filter((c) => c.p_qualify_r1 > 0)
    .toSorted((a, b) => b.p_qualify_r1 - a.p_qualify_r1)
    .slice(0, size)
  const cells: MatrixCell[] = []
  for (const row of candidates) {
    for (const column of candidates) {
      if (row.candidate_id === column.candidate_id) continue
      const pair = findPair(forecast.pairs, row.candidate_id, column.candidate_id)
      cells.push({
        row: row.candidate_id,
        column: column.candidate_id,
        pMatchup: pair?.p_matchup ?? 0,
        rowWins: pair?.p_win_given_matchup[row.candidate_id] ?? null,
        polled: pair?.polled ?? false,
      })
    }
  }
  return { candidates, cells }
}

export function findPair(pairs: readonly PairForecast[], a: string, b: string): PairForecast | undefined {
  return pairs.find((p) => (p.a === a && p.b === b) || (p.a === b && p.b === a))
}
