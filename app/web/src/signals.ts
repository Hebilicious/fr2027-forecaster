// Pure preparation of the Signals view's data, kept out of the components so it can be tested headlessly.
// None of this feeds the model: it only lines up what markets, Wikipedia, the press and X say.
import type {
  CampaignEvent,
  CandidateForecast,
  Forecast,
  Headline,
  Markets,
  NewsDay,
  PageViews,
  Question,
  Venue,
  XRow,
} from "./api/schemas"
import { scale } from "./charts"

export const VENUES: readonly Venue[] = ["polymarket", "kalshi"]

const DAY_MS = 86_400_000

/** The ISO calendar date `days` after `isoDate` (negative to go back). */
export function addDays(isoDate: string, days: number): string {
  return new Date(Date.parse(`${isoDate}T00:00:00Z`) + days * DAY_MS).toISOString().slice(0, 10)
}

const latestDate = (dates: Iterable<string>): string | null => {
  let latest: string | null = null
  for (const date of dates) if (latest === null || date > latest) latest = date
  return latest
}

/** Display names by candidate id, from the forecast; `nameOf` falls back to the id. */
export function candidateNames(forecast: Forecast): ReadonlyMap<string, string> {
  return new Map(forecast.candidates.map((c) => [c.candidate_id, c.name]))
}

export const nameOf = (names: ReadonlyMap<string, string>, id: string): string => names.get(id) ?? id

export interface Point {
  readonly date: string
  readonly value: number
}

/** First and last day of a chart's x axis. */
export type Window = readonly [string, string]

// Markets

/** The model's probability for the question a market asks. */
export function modelValue(candidate: CandidateForecast, question: Question): number {
  if (question === "win") return candidate.p_win
  if (question === "qualify") return candidate.p_qualify_r1
  return candidate.p_run
}

export interface MarketRow {
  readonly candidateId: string
  readonly name: string
  /** `null` for someone a market matched to our config but the forecast doesn't list. */
  readonly model: number | null
  readonly prices: Readonly<Record<Venue, number | null>>
  /** Daily closing prices inside the window, oldest first; fewer than 2 days draws nothing. */
  readonly history: Readonly<Record<Venue, readonly Point[]>>
}

const MIN_SHOWN = 0.005

/**
 * Model vs markets for one question: everyone at 0.5% or more in the model or either venue, highest
 * first, with each venue's daily prices over the last `days` days of history.
 */
export function marketRows(
  forecast: Forecast,
  markets: Markets,
  question: Question,
  days = 30,
): { rows: MarketRow[]; window: Window | null } {
  const names = candidateNames(forecast)
  const prices = new Map<string, Record<Venue, number | null>>()
  for (const quote of markets.latest) {
    if (quote.question !== question) continue
    const entry = prices.get(quote.candidate_id) ?? { polymarket: null, kalshi: null }
    entry[quote.venue] ??= quote.price
    prices.set(quote.candidate_id, entry)
  }

  const end = latestDate(markets.history.map((h) => h.date))
  const window: Window | null = end === null ? null : [addDays(end, -(days - 1)), end]
  const history = new Map<string, Record<Venue, Point[]>>()
  for (const close of markets.history) {
    if (window === null || close.question !== question || close.price === null || close.date < window[0]) continue
    const entry = history.get(close.candidate_id) ?? { polymarket: [], kalshi: [] }
    entry[close.venue].push({ date: close.date, value: close.price })
    history.set(close.candidate_id, entry)
  }
  const series = (id: string, venue: Venue): readonly Point[] => {
    const points = (history.get(id)?.[venue] ?? []).toSorted((a, b) => a.date.localeCompare(b.date))
    return points.length >= 2 ? points : []
  }

  const model = new Map(forecast.candidates.map((c) => [c.candidate_id, modelValue(c, question)]))
  const ids = new Set([...model.keys(), ...prices.keys()])
  const ranked: { row: MarketRow; top: number }[] = []
  for (const id of ids) {
    const venuePrices = prices.get(id) ?? { polymarket: null, kalshi: null }
    const row: MarketRow = {
      candidateId: id,
      name: nameOf(names, id),
      model: model.get(id) ?? null,
      prices: venuePrices,
      history: { polymarket: series(id, "polymarket"), kalshi: series(id, "kalshi") },
    }
    const top = Math.max(row.model ?? 0, topPrice(venuePrices))
    if (top >= MIN_SHOWN) ranked.push({ row, top })
  }
  return {
    rows: ranked.toSorted((a, b) => b.top - a.top || a.row.name.localeCompare(b.row.name)).map(({ row }) => row),
    window,
  }
}

export interface UnmatchedRow {
  readonly label: string
  readonly prices: Readonly<Record<Venue, number | null>>
}

const topPrice = (p: Readonly<Record<Venue, number | null>>) => Math.max(p.polymarket ?? 0, p.kalshi ?? 0)

/** People a market prices at `min` or more on `question` who are not in our field, highest first. */
export function unmatchedRows(markets: Markets, question: Question, min = 0.01): UnmatchedRow[] {
  const byLabel = new Map<string, Record<Venue, number | null>>()
  for (const quote of markets.unmatched) {
    if (quote.question !== question || quote.price === null || quote.price < min) continue
    const entry = byLabel.get(quote.label) ?? { polymarket: null, kalshi: null }
    entry[quote.venue] = Math.max(entry[quote.venue] ?? 0, quote.price)
    byLabel.set(quote.label, entry)
  }
  return [...byLabel]
    .map(([label, prices]) => ({ label, prices }))
    .toSorted((a, b) => topPrice(b.prices) - topPrice(a.prices) || a.label.localeCompare(b.label))
}

// Attention

export interface AttentionRow {
  readonly candidateId: string
  readonly name: string
  /** Views over the last 7 days in the data. */
  readonly total: number
  /** Relative change against the 7 days before; `null` without views in that week. */
  readonly change: number | null
  /** Daily views inside the window, oldest first. */
  readonly daily: readonly Point[]
}

/** The `top` candidates by page views over the last 7 days in the data, with their last `days` days. */
export function attentionRows(
  rows: readonly PageViews[],
  names: ReadonlyMap<string, string>,
  top = 12,
  days = 90,
): { rows: AttentionRow[]; window: Window | null } {
  const end = latestDate(rows.map((r) => r.date))
  if (end === null) return { rows: [], window: null }
  const window: Window = [addDays(end, -(days - 1)), end]
  const weekStart = addDays(end, -6)
  const previousStart = addDays(end, -13)
  const byCandidate = new Map<string, { total: number; previous: number; daily: Point[] }>()
  for (const row of rows) {
    const entry = byCandidate.get(row.candidate_id) ?? { total: 0, previous: 0, daily: [] }
    if (row.date >= weekStart) entry.total += row.views
    else if (row.date >= previousStart) entry.previous += row.views
    if (row.date >= window[0]) entry.daily.push({ date: row.date, value: row.views })
    byCandidate.set(row.candidate_id, entry)
  }
  const result = [...byCandidate]
    .filter(([, entry]) => entry.total > 0)
    .map(([id, entry]) => ({
      candidateId: id,
      name: nameOf(names, id),
      total: entry.total,
      change: entry.previous > 0 ? entry.total / entry.previous - 1 : null,
      daily: entry.daily.toSorted((a, b) => a.date.localeCompare(b.date)),
    }))
    .toSorted((a, b) => b.total - a.total || a.name.localeCompare(b.name))
    .slice(0, top)
  return { rows: result, window }
}

// News

export interface NewsCount {
  readonly candidateId: string
  readonly name: string
  readonly items: number
}

/** Headlines per candidate over the last `days` days in the data, the `top` most covered first. */
export function newsCounts(
  daily: readonly NewsDay[],
  names: ReadonlyMap<string, string>,
  days = 7,
  top = 12,
): NewsCount[] {
  const end = latestDate(daily.map((d) => d.date))
  if (end === null) return []
  const start = addDays(end, -(days - 1))
  const counts = new Map<string, number>()
  for (const day of daily) {
    if (day.date >= start) counts.set(day.candidate_id, (counts.get(day.candidate_id) ?? 0) + day.items)
  }
  return [...counts]
    .filter(([, items]) => items > 0)
    .map(([id, items]) => ({ candidateId: id, name: nameOf(names, id), items }))
    .toSorted((a, b) => b.items - a.items || a.name.localeCompare(b.name))
    .slice(0, top)
}

/** The `limit` newest headlines, only those naming `candidateId` when one is given. */
export function latestHeadlines(headlines: readonly Headline[], candidateId: string | null, limit = 30): Headline[] {
  return headlines
    .filter((h) => candidateId === null || h.candidate_ids.includes(candidateId))
    .toSorted((a, b) => b.published_at.localeCompare(a.published_at))
    .slice(0, limit)
}

// X

export interface XWindow {
  readonly start: string
  readonly end: string
  /** Most mentioned first; rows without a count last. */
  readonly rows: readonly XRow[]
}

/** Grok Bot's most recent measurement window, or `null` before the first one. */
export function latestXWindow(rows: readonly XRow[]): XWindow | null {
  const end = latestDate(rows.map((r) => r.window_end))
  if (end === null) return null
  const latest = rows.filter((r) => r.window_end === end)
  return {
    start: latestDate(latest.map((r) => r.window_start)) ?? end,
    end,
    rows: latest.toSorted(
      (a, b) => (b.mentions ?? -1) - (a.mentions ?? -1) || a.candidate_id.localeCompare(b.candidate_id),
    ),
  }
}

export interface Sentiment {
  readonly pos: number
  readonly neu: number
  readonly neg: number
}

/** The positive, neutral and negative shares rescaled to sum to 1, or `null` when any is missing. */
export function sentiment(row: XRow): Sentiment | null {
  const { sentiment_pos: pos, sentiment_neu: neu, sentiment_neg: neg } = row
  if (pos === null || neu === null || neg === null) return null
  const sum = pos + neu + neg
  return sum > 0 ? { pos: pos / sum, neu: neu / sum, neg: neg / sum } : null
}

// Events

export function eventsNewestFirst(events: readonly CampaignEvent[]): CampaignEvent[] {
  return events.toSorted((a, b) => b.date.localeCompare(a.date) || b.event_id.localeCompare(a.event_id))
}

// Sparklines

export interface Box {
  readonly width: number
  readonly height: number
  readonly pad: number
}

/** The y range of a sparkline: from 0, or the series' own range widened to at least `minSpan`. */
export function sparkRange(values: readonly number[], minSpan?: number): readonly [number, number] {
  const max = Math.max(...values)
  if (minSpan === undefined) return [0, max]
  const min = Math.min(...values)
  if (max - min >= minSpan) return [min, max]
  const low = Math.max(0, (min + max) / 2 - minSpan / 2)
  return [low, low + minSpan]
}

/**
 * A sparkline through daily values placed on a date window, plus where its last point lands. The y
 * axis runs from 0 to the series' maximum, or over its own range when `minSpan` is given (so a
 * small move shows without noise looking like a swing). Empty below 2 points.
 */
export function sparkline(
  points: readonly Point[],
  window: Window,
  { width, height, pad }: Box,
  minSpan?: number,
): { line: string; end: { x: number; y: number } | null } {
  if (points.length < 2) return { line: "", end: null }
  const domain = [Date.parse(`${window[0]}T00:00:00Z`), Date.parse(`${window[1]}T00:00:00Z`)] as const
  const [low, high] = sparkRange(
    points.map((p) => p.value),
    minSpan,
  )
  const at = (p: Point) => ({
    x: scale(Date.parse(`${p.date}T00:00:00Z`), { domain, range: [pad, width - pad] }),
    y: high > low ? scale(p.value, { domain: [low, high], range: [height - pad, pad] }) : height - pad,
  })
  const placed = points.map(at)
  const line = placed.map((p, i) => `${i === 0 ? "M" : "L"}${p.x.toFixed(1)},${p.y.toFixed(1)}`).join("")
  const last = placed.at(-1) ?? null
  return { line, end: last && { x: Number(last.x.toFixed(1)), y: Number(last.y.toFixed(1)) } }
}
