// Pure preparation of chart data, kept out of the components so it can be tested headlessly.
import type { CandidateForecast, Forecast, PairForecast, Poll, SeriesPoint } from "./api/schemas"

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
export function round1Panels(forecast: Forecast, series: readonly SeriesPoint[], polls: readonly Poll[]): Panel[] {
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
    const dots = polls.flatMap((poll) =>
      poll.scenarios.flatMap((scenario) => {
        const share = scenario.shares[candidate.candidate_id]
        if (scenario.round !== 1 || share === undefined) return []
        return [
          {
            pollId: poll.poll_id,
            firm: poll.firm,
            date: midpoint(poll.field_start, poll.field_end),
            scenario: scenario.scenario_id,
            share: share / 100,
            sourceUrl: poll.source_url,
          },
        ]
      }),
    )
    panels.push({ candidate, points, polls: dots })
  }
  return panels.toSorted((a, b) => (b.points.at(-1)?.mean ?? 0) - (a.points.at(-1)?.mean ?? 0))
}

/**
 * The item whose position is closest to (x, y), if it lies within `radius`. On a tie the later item
 * wins, so the dot drawn on top is the one picked.
 */
export function nearest<T>(
  items: readonly T[],
  position: (item: T) => readonly [number, number],
  x: number,
  y: number,
  radius: number,
): T | null {
  let best: T | null = null
  let bestDistance = radius
  for (const item of items) {
    const [ix, iy] = position(item)
    const distance = Math.hypot(ix - x, iy - y)
    if (distance <= bestDistance) {
      best = item
      bestDistance = distance
    }
  }
  return best
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

/** The inverse of `scale`: the domain value drawn at `position`. */
export function invert(position: number, { domain, range }: Scale): number {
  const [d0, d1] = domain
  const [r0, r1] = range
  if (r1 === r0) return (d0 + d1) / 2
  return d0 + ((position - r0) / (r1 - r0)) * (d1 - d0)
}

export type Domain = readonly [number, number]

/**
 * `domain` zoomed by `factor` (below 1 zooms in) so that `anchor` stays where it is, no narrower
 * than `minSpan` and kept inside `limits`.
 */
export function zoomDomain(domain: Domain, anchor: number, factor: number, limits: Domain, minSpan: number): Domain {
  const span = domain[1] - domain[0]
  const next = Math.min(limits[1] - limits[0], Math.max(minSpan, span * factor))
  const at = span === 0 ? 0.5 : (anchor - domain[0]) / span
  return clampDomain(anchor - at * next, next, limits)
}

/** `domain` moved by `delta`, kept inside `limits`. */
export function panDomain(domain: Domain, delta: number, limits: Domain): Domain {
  return clampDomain(domain[0] + delta, domain[1] - domain[0], limits)
}

function clampDomain(start: number, span: number, limits: Domain): Domain {
  const from = Math.min(Math.max(start, limits[0]), limits[1] - span)
  return [from, from + span]
}

/**
 * A share axis that fits `values` with a little room around them, rounded out to whole points, at
 * least `minSpan` wide and inside `limits`. Falls back to `limits` when there is nothing to fit.
 */
export function fitShares(values: readonly number[], limits: Domain, minSpan: number): Domain {
  if (values.length === 0) return limits
  const lo = Math.min(...values)
  const hi = Math.max(...values)
  const pad = Math.max((hi - lo) * 0.1, 0.005)
  let from = Math.floor((lo - pad) * 100) / 100
  let to = Math.ceil((hi + pad) * 100) / 100
  if (to - from < minSpan) {
    const middle = (from + to) / 2
    from = middle - minSpan / 2
    to = middle + minSpan / 2
  }
  return clampDomain(from, Math.min(to - from, limits[1] - limits[0]), limits)
}

/** Y-axis ticks for a zoomed share axis: whole points (1, 2, 5 or 10 apart) inside `domain`. */
export function shareTicksWithin(domain: Domain): number[] {
  const step = [0.01, 0.02, 0.05].find((s) => (domain[1] - domain[0]) / s <= 6) ?? 0.1
  const ticks: number[] = []
  for (let t = Math.ceil(domain[0] / step - 1e-9) * step; t <= domain[1] + 1e-9; t += step) {
    ticks.push(Math.round(t * 100) / 100)
  }
  return ticks
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

/** The round 2 matrix over the `size` candidates most likely to qualify, leaving out those in `exclude`. */
export function round2Matrix(
  forecast: Forecast,
  size: number,
  exclude: ReadonlySet<string> = new Set(),
): { candidates: CandidateForecast[]; cells: MatrixCell[] } {
  const candidates = forecast.candidates
    .filter((c) => c.p_qualify_r1 > 0 && !exclude.has(c.candidate_id))
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

/** One dot per percentage point; a full hundred would read as certainty, which no chance short of 1 is. */
const dotsFor = (p: number) => Math.min(Math.round(p * 100), p < 1 ? 99 : 100)

export interface OutcomeGroup {
  /** The candidate, or `null` for everyone pooled together. */
  readonly candidateId: string | null
  readonly name: string | null
  /** The chance of winning, summed over the pool. */
  readonly probability: number
  /** One dot per percentage point, rounded as the tables round it. */
  readonly dots: number
  /** 1, or how many candidates the pool holds. */
  readonly members: number
}

/**
 * Each candidate's chance of winning as dots, one per percentage point, largest first. Everyone a
 * table would show as "<1%" (below half a point), and everyone in `pool`, is pooled into one group,
 * last; a group that rounds to no dot is left out, and only a certainty fills all 100. Rounding can
 * make the dots add up to 99 or 101.
 */
export function outcomeGroups(
  candidates: readonly CandidateForecast[],
  pool: ReadonlySet<string> = new Set(),
  min = 0.005,
): OutcomeGroup[] {
  const ranked = candidates.filter((c) => c.p_win > 0).toSorted((a, b) => b.p_win - a.p_win)
  const pooledHere = (c: CandidateForecast) => c.p_win < min || pool.has(c.candidate_id)
  const pooled = ranked.filter(pooledHere)
  const pooledProbability = pooled.reduce((sum, c) => sum + c.p_win, 0)
  const groups = [
    ...ranked
      .filter((c) => !pooledHere(c))
      .map((c) => ({ candidateId: c.candidate_id, name: c.name, probability: c.p_win, members: 1 })),
    ...(pooled.length > 0
      ? [{ candidateId: null, name: null, probability: pooledProbability, members: pooled.length }]
      : []),
  ]
  return groups.map((g) => ({ ...g, dots: dotsFor(g.probability) })).filter((g) => g.dots > 0)
}

/**
 * The groups the dots chart keeps while the reader moves the forecast's settings, so nothing jumps:
 * everyone who gets their own group in any of `states` (each a candidate list at one setting), in the
 * order of the first state, and the most dots each group needs in any state.
 */
export interface OutcomeFrame {
  readonly ids: readonly string[]
  /** Most dots per group across the states, by candidate id; `null` for the pool. */
  readonly capacity: ReadonlyMap<string | null, number>
}

export function outcomeFrame(
  states: readonly (readonly CandidateForecast[])[],
  pool: ReadonlySet<string> = new Set(),
  min = 0.005,
): OutcomeFrame {
  const ids: string[] = []
  for (const state of states) {
    for (const g of outcomeGroups(state, pool, min)) {
      if (g.candidateId !== null && !ids.includes(g.candidateId)) ids.push(g.candidateId)
    }
  }
  const capacity = new Map<string | null, number>()
  for (const state of states) {
    for (const g of framedGroups(state, { ids, capacity: new Map() }, pool)) {
      capacity.set(g.candidateId, Math.max(capacity.get(g.candidateId) ?? 0, g.dots))
    }
  }
  return { ids, capacity }
}

/** The dots of `candidates` in a frame's groups: its candidates in its order, everyone else pooled last. */
export function framedGroups(
  candidates: readonly CandidateForecast[],
  frame: OutcomeFrame,
  pool: ReadonlySet<string> = new Set(),
): (OutcomeGroup & { readonly capacity: number })[] {
  const byId = new Map(candidates.map((c) => [c.candidate_id, c]))
  const own = frame.ids.filter((id) => !pool.has(id) && byId.has(id))
  const pooled = candidates.filter((c) => c.p_win > 0 && !own.includes(c.candidate_id))
  const pooledProbability = pooled.reduce((sum, c) => sum + c.p_win, 0)
  const groups: OutcomeGroup[] = [
    ...own.map((id) => {
      const c = byId.get(id)!
      return { candidateId: id, name: c.name, probability: c.p_win, members: 1, dots: dotsFor(c.p_win) }
    }),
    ...(pooled.length > 0
      ? [
          {
            candidateId: null,
            name: null,
            probability: pooledProbability,
            members: pooled.length,
            dots: dotsFor(pooledProbability),
          },
        ]
      : []),
  ]
  return groups.map((g) => ({ ...g, capacity: Math.max(frame.capacity.get(g.candidateId) ?? 0, g.dots) }))
}
