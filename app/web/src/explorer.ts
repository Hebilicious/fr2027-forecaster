// Pure preparation of the model explorer's data (the tabs under the overview's header), kept out of the
// components so it can be tested headlessly.
import type { CandidateForecast, Forecast, HistoryRun, TrackRecord } from "./api/schemas"
import { FAMILIES } from "./candidates"

/** The categorical hues for lines, in fixed order, stepped for the dark surface and validated together. */
export const LINE_COLOURS = ["#3987e5", "#d95926", "#199e70", "#c98500", "#d55181"] as const

export interface ChancePoint {
  readonly time: number
  readonly p: number
}

export interface ChanceLine {
  readonly candidateId: string
  readonly name: string
  readonly colour: string
  readonly points: readonly ChancePoint[]
}

/**
 * Each forecast run's chance of winning for the `count` candidates the latest forecast rates highest, oldest
 * run first. A candidate keeps their colour whatever the others do: colours follow the latest ranking.
 */
export function chanceLines(
  runs: readonly HistoryRun[],
  forecast: Forecast,
  count: number = LINE_COLOURS.length,
): ChanceLine[] {
  const leaders = forecast.candidates
    .filter((c) => c.p_win > 0)
    .toSorted((a, b) => b.p_win - a.p_win)
    .slice(0, Math.min(count, LINE_COLOURS.length))
  const ordered = runs.toSorted((a, b) => a.generated_at.localeCompare(b.generated_at))
  return leaders.map((candidate, i) => ({
    candidateId: candidate.candidate_id,
    name: candidate.name,
    colour: LINE_COLOURS[i]!,
    points: ordered.flatMap((run) => {
      const p = run.candidates.find((c) => c.candidate_id === candidate.candidate_id)?.p_win
      return p === null || p === undefined ? [] : [{ time: Date.parse(run.generated_at), p }]
    }),
  }))
}

export interface Opponent {
  readonly candidateId: string
  readonly name: string
  /** Probability that this runoff happens. */
  readonly pMatchup: number
  /** Probability the candidate wins it, when it happens. */
  readonly pWins: number
}

export interface Path {
  readonly candidate: CandidateForecast
  readonly pQualify: number
  readonly pWin: number
  /** Share of the runoffs they reach that they win; null when they never reach one. */
  readonly conversion: number | null
  /** Every runoff opponent the simulations gave them, likeliest first. */
  readonly opponents: readonly Opponent[]
}

/** The `count` candidates likeliest to reach round 2: how often they get there, how often they then win, and against whom. */
export function pathsToVictory(forecast: Forecast, count = 8): Path[] {
  const names = new Map(forecast.candidates.map((c) => [c.candidate_id, c.name]))
  return forecast.candidates
    .filter((c) => c.simulated && c.p_qualify_r1 > 0)
    .toSorted((a, b) => b.p_qualify_r1 - a.p_qualify_r1)
    .slice(0, count)
    .map((candidate) => {
      const id = candidate.candidate_id
      const faced = forecast.pairs
        .filter((p) => p.a === id || p.b === id)
        .map((p) => {
          const other = p.a === id ? p.b : p.a
          return {
            candidateId: other,
            name: names.get(other) ?? other,
            pMatchup: p.p_matchup,
            pWins: p.p_win_given_matchup[id] ?? 0,
          }
        })
        .toSorted((a, b) => b.pMatchup - a.pMatchup)
      return {
        candidate,
        pQualify: candidate.p_qualify_r1,
        pWin: candidate.p_win,
        conversion: candidate.p_qualify_r1 > 0 ? Math.min(1, candidate.p_win / candidate.p_qualify_r1) : null,
        opponents: faced,
      }
    })
}

export interface Lean {
  readonly firm: string
  readonly bloc: string
  /** How much higher (positive) or lower than the average pollster the firm puts the bloc, as a ratio minus 1. */
  readonly relative: number
  /** The same, one standard deviation either side. */
  readonly lo: number
  readonly hi: number
  /** Whether zero lies outside one standard deviation: a lean the polls show, not noise. */
  readonly clear: boolean
}

export interface Leans {
  readonly firms: readonly string[]
  readonly blocs: readonly string[]
  readonly cells: readonly Lean[]
  /** The largest |relative| shown, for the colour scale. */
  readonly max: number
}

/**
 * Each pollster's house effect on each bloc, as estimated by the poll aggregation (on the log scale, shown
 * as a relative difference): pollsters in alphabetical order, blocs left to right.
 */
export function pollsterLeans(forecast: Forecast): Leans {
  const effects = forecast.aggregation.house_effects
  const firms = [...new Set(effects.map((e) => e.firm))].toSorted()
  const present = new Set(effects.map((e) => e.bloc))
  const blocs = [
    ...FAMILIES.filter((b) => present.has(b)),
    ...[...present].filter((b) => !FAMILIES.includes(b)).toSorted(),
  ]
  const cells = effects.map((e) => ({
    firm: e.firm,
    bloc: e.bloc,
    relative: Math.expm1(e.mean),
    lo: Math.expm1(e.mean - e.sd),
    hi: Math.expm1(e.mean + e.sd),
    clear: Math.abs(e.mean) > e.sd,
  }))
  return { firms, blocs, cells, max: Math.max(0.01, ...cells.map((c) => Math.abs(c.relative))) }
}

/** The diverging scale's poles and neutral midpoint, for the dark surface. */
export const DIVERGING = { low: "#3987e5", mid: "#383835", high: "#d95926" } as const

/** A colour on the diverging scale for `value` in [-max, max]: the neutral grey at zero, a pole at either end. */
export function divergingColour(value: number, max: number): string {
  const t = Math.max(-1, Math.min(1, max > 0 ? value / max : 0))
  const pole = t < 0 ? DIVERGING.low : DIVERGING.high
  return mix(DIVERGING.mid, pole, Math.abs(t))
}

/** Channel `i` (0 red, 1 green, 2 blue) of a #rrggbb colour. */
const channel = (hex: string, i: number) => Number.parseInt(hex.slice(1 + 2 * i, 3 + 2 * i), 16)

function mix(from: string, to: string, t: number): string {
  const parts = [0, 1, 2].map((i) => Math.round(channel(from, i) + (channel(to, i) - channel(from, i)) * t))
  return `#${parts.map((v) => v.toString(16).padStart(2, "0")).join("")}`
}

/** The backtest's horizons, furthest from the vote first, and its calibration bins that hold any event. */
export function trackRecordView(record: TrackRecord) {
  return {
    horizons: record.by_horizon.toSorted((a, b) => b.days - a.days),
    bins: record.reliability.flatMap((bin) =>
      bin.events > 0 && bin.mean_forecast !== null && bin.observed !== null
        ? [{ ...bin, mean_forecast: bin.mean_forecast, observed: bin.observed }]
        : [],
    ),
  }
}
