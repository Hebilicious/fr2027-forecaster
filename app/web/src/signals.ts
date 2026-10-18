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
  Signals,
  Venue,
  XRow,
} from "./api/schemas"
import { scale } from "./charts"
import type { Locale } from "./locales"

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

/**
 * The signals without the candidates in `hidden`. A headline or event naming several people stays while one of
 * them is shown, and one naming nobody always stays.
 */
export function filterSignals(signals: Signals, hidden: ReadonlySet<string>): Signals {
  const shown = (row: { candidate_id: string }) => !hidden.has(row.candidate_id)
  const anyShown = (ids: readonly string[]) => ids.length === 0 || ids.some((id) => !hidden.has(id))
  return {
    ...signals,
    markets: {
      ...signals.markets,
      latest: signals.markets.latest.filter(shown),
      history: signals.markets.history.filter(shown),
    },
    attention: { ...signals.attention, rows: signals.attention.rows.filter(shown) },
    x: { rows: signals.x.rows.filter(shown) },
    news: {
      ...signals.news,
      daily: signals.news.daily.filter(shown),
      headlines: signals.news.headlines.filter((h) => anyShown(h.candidate_ids)),
    },
    events: signals.events.filter((e) => anyShown(e.candidate_ids)),
    forward: { ...signals.forward, flags: signals.forward.flags.filter(shown) },
  }
}

export interface Point {
  /** An ISO date, or a full timestamp for data finer than a day. */
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

// Periods

/** How far back the X and press sections look, counted from the newest data each has. */
export const RANGES = ["1d", "3d", "7d", "14d", "all"] as const
export type SignalRange = (typeof RANGES)[number]

const RANGE_DAYS: Readonly<Record<SignalRange, number | null>> = { "1d": 1, "3d": 3, "7d": 7, "14d": 14, all: null }

/** The range's length in days, `null` for everything the API sends. */
export const rangeDays = (range: SignalRange): number | null => RANGE_DAYS[range]

export const isSignalRange = (value: unknown): value is SignalRange => RANGES.includes(value as SignalRange)

// X

/** One of Grok Bot's measurement windows; `stance` once drops follow the stance rubric. */
export interface XWindowInfo {
  readonly start: string
  readonly end: string
  readonly stance: boolean
}

const isStanceRow = (row: XRow) => row.support_share !== null

/** Every measurement window in the rows, oldest first. */
export function xWindows(rows: readonly XRow[]): XWindowInfo[] {
  const byEnd = new Map<string, { start: string; stance: boolean }>()
  for (const row of rows) {
    const entry = byEnd.get(row.window_end) ?? { start: row.window_start, stance: false }
    if (row.window_start < entry.start) entry.start = row.window_start
    entry.stance ||= isStanceRow(row)
    byEnd.set(row.window_end, entry)
  }
  return [...byEnd]
    .map(([end, { start, stance }]) => ({ start, end, stance }))
    .toSorted((a, b) => a.end.localeCompare(b.end))
}

/** The windows that start inside the range's last days before the newest window ends. */
export function windowsInRange(windows: readonly XWindowInfo[], range: SignalRange): XWindowInfo[] {
  const days = rangeDays(range)
  const last = windows.at(-1)
  if (days === null || last === undefined) return [...windows]
  const from = new Date(Date.parse(last.end) - days * DAY_MS).toISOString()
  return windows.filter((w) => Date.parse(w.start) >= Date.parse(from))
}

/** A share and its 95% Wilson score interval. */
export interface Interval {
  readonly share: number
  readonly low: number
  readonly high: number
}

const Z95 = 1.959963984540054

/** The 95% Wilson score interval for `k` successes out of `n`, as the collector computes it; `null` when `n` is 0. */
export function wilson(k: number, n: number): Interval | null {
  if (n <= 0) return null
  const p = k / n
  const z2 = Z95 * Z95
  const denominator = 1 + z2 / n
  const centre = (p + z2 / (2 * n)) / denominator
  const half = (Z95 / denominator) * Math.sqrt((p * (1 - p)) / n + z2 / (4 * n * n))
  return { share: p, low: Math.max(0, centre - half), high: Math.min(1, centre + half) }
}

/** Support, oppose and neutral posts in a stance row; older API builds send only the shares, so those are rescaled. */
export function stanceCounts(row: XRow): { support: number; oppose: number; neutral: number } | null {
  if (!isStanceRow(row)) return null
  if (row.stance_support != null && row.stance_oppose != null && row.stance_neutral != null) {
    return { support: row.stance_support, oppose: row.stance_oppose, neutral: row.stance_neutral }
  }
  const relevant = (row.stance_posts_drawn ?? 0) - (row.stance_off_topic ?? 0)
  const of = (share: number | null) => Math.round((share ?? 0) * relevant)
  return { support: of(row.support_share), oppose: of(row.oppose_share), neutral: of(row.neutral_share) }
}

/** Below this many posts about a candidate, a share is shown faded and its change is not given. */
export const THIN_POSTS = 30

export interface XPooledRow {
  readonly candidateId: string
  readonly name: string
  /** Summed over the windows that count them; `null` when none does. */
  readonly mentions: number | null
  /** How the newest counted window got its mentions. */
  readonly mentionsMethod: XRow["mentions_method"]
  /** Posts Grok Bot read, summed. */
  readonly postsRead: number | null
  /** Posts about the candidate behind the shares: stance posts, or the sentiment sample before the rubric. */
  readonly relevant: number
  /** Pooled stance shares with their intervals; `null` without a stance window. */
  readonly stance: { readonly support: Interval; readonly oppose: Interval; readonly neutral: Interval } | null
  /** The bar: the stance shares, or the sample-weighted sentiment of the first drops when no stance window is kept. */
  readonly mood: Sentiment | null
  /** The bar comes from the sentiment drops read before the stance rubric. */
  readonly legacy: boolean
  readonly thin: boolean
  /** Support share window by window (window ends), oldest first. */
  readonly trend: readonly Point[]
  /** Support share in the last window minus the first, when both rest on at least `THIN_POSTS` posts. */
  readonly change: number | null
  /** Bot share weighted by posts read. */
  readonly botShare: number | null
  /** The newest kept window's topics. */
  readonly topics: readonly string[]
}

/**
 * Grok Bot's measurements pooled over the windows ending at `ends`: counts summed, stance shares and their
 * intervals recomputed from the summed posts (so a window with more posts weighs more), most mentioned first.
 */
export function xPooled(
  rows: readonly XRow[],
  ends: readonly string[],
  names: ReadonlyMap<string, string>,
): XPooledRow[] {
  const kept = new Set(ends)
  const byCandidate = new Map<string, XRow[]>()
  for (const row of rows) {
    if (!kept.has(row.window_end)) continue
    byCandidate.set(row.candidate_id, [...(byCandidate.get(row.candidate_id) ?? []), row])
  }
  const pooled = [...byCandidate].map(([id, list]) =>
    pool(
      id,
      nameOf(names, id),
      list.toSorted((a, b) => a.window_end.localeCompare(b.window_end)),
    ),
  )
  return pooled.toSorted(
    (a, b) => (b.mentions ?? -1) - (a.mentions ?? -1) || a.candidateId.localeCompare(b.candidateId),
  )
}

const sumOf = (values: readonly (number | null)[]): number | null => {
  const present = values.filter((v): v is number => v !== null)
  return present.length ? present.reduce((a, b) => a + b, 0) : null
}

function pool(candidateId: string, name: string, rows: readonly XRow[]): XPooledRow {
  const counted = rows.filter((r) => r.mentions !== null)
  const tally = { support: 0, oppose: 0, neutral: 0 }
  const trend: Point[] = []
  const trendPosts: number[] = []
  for (const row of rows) {
    const counts = stanceCounts(row)
    if (counts === null) continue
    tally.support += counts.support
    tally.oppose += counts.oppose
    tally.neutral += counts.neutral
    const n = counts.support + counts.oppose + counts.neutral
    if (n > 0) {
      trend.push({ date: row.window_end, value: counts.support / n })
      trendPosts.push(n)
    }
  }
  const n = tally.support + tally.oppose + tally.neutral
  const support = wilson(tally.support, n)
  const oppose = wilson(tally.oppose, n)
  const neutral = wilson(tally.neutral, n)
  const stance = support && oppose && neutral ? { support, oppose, neutral } : null

  // Before the stance rubric, Grok Bot gave sentiment shares from a small sample: used only when nothing better is kept.
  let mood: Sentiment | null = stance && { pos: support!.share, neu: neutral!.share, neg: oppose!.share }
  let relevant = n
  const legacy = stance === null && rows.some((r) => sentiment(r) !== null)
  if (legacy) {
    const weighed = rows.flatMap((r) => {
      const s = sentiment(r)
      return s && r.sample_size ? [{ s, w: r.sample_size }] : []
    })
    relevant = weighed.reduce((sum, { w }) => sum + w, 0)
    const avg = (pick: (s: Sentiment) => number) => weighed.reduce((sum, { s, w }) => sum + pick(s) * w, 0) / relevant
    mood = relevant > 0 ? { pos: avg((s) => s.pos), neu: avg((s) => s.neu), neg: avg((s) => s.neg) } : null
  }

  const firstPosts = trendPosts[0] ?? 0
  const lastPosts = trendPosts.at(-1) ?? 0
  const change =
    trend.length >= 2 && firstPosts >= THIN_POSTS && lastPosts >= THIN_POSTS
      ? trend.at(-1)!.value - trend[0]!.value
      : null

  const botRows = rows.filter((r) => r.bot_share_estimate !== null && (r.sample_size ?? 0) > 0)
  const botWeight = botRows.reduce((sum, r) => sum + r.sample_size!, 0)
  return {
    candidateId,
    name,
    mentions: sumOf(rows.map((r) => r.mentions)),
    mentionsMethod: counted.at(-1)?.mentions_method ?? null,
    postsRead: sumOf(rows.map((r) => r.sample_size)),
    relevant,
    stance,
    mood,
    legacy,
    thin: relevant < THIN_POSTS,
    trend,
    change,
    botShare:
      botWeight > 0 ? botRows.reduce((sum, r) => sum + r.bot_share_estimate! * r.sample_size!, 0) / botWeight : null,
    topics: rows.findLast((r) => r.top_topics?.length)?.top_topics ?? [],
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

/** Milliseconds since the epoch for an ISO date (read as midnight UTC) or timestamp. */
const instant = (value: string): number => Date.parse(value.length === 10 ? `${value}T00:00:00Z` : value)

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
  const domain = [instant(window[0]), instant(window[1])] as const
  const [low, high] = sparkRange(
    points.map((p) => p.value),
    minSpan,
  )
  const at = (p: Point) => ({
    x: scale(instant(p.date), { domain, range: [pad, width - pad] }),
    y: high > low ? scale(p.value, { domain: [low, high], range: [height - pad, pad] }) : height - pad,
  })
  const placed = points.map(at)
  const line = placed.map((p, i) => `${i === 0 ? "M" : "L"}${p.x.toFixed(1)},${p.y.toFixed(1)}`).join("")
  const last = placed.at(-1) ?? null
  return { line, end: last && { x: Number(last.x.toFixed(1)), y: Number(last.y.toFixed(1)) } }
}

/** Which of an event's two summaries each locale shows: the French site reads French, every other one English. */
const SUMMARY_LANGUAGE: Readonly<Record<Locale, "fr" | "en">> = {
  fr: "fr",
  "en-GB": "en",
  de: "en",
  it: "en",
  "es-ES": "en",
  "pt-PT": "en",
  nl: "en",
  pl: "en",
  ru: "en",
  uk: "en",
  ja: "en",
  ko: "en",
  "zh-Hans": "en",
  th: "en",
}

/**
 * An event's summary for `locale`, the language it is written in, and whether that differs from the page's language
 * (so the page can say so). A legacy string from an older API build is English, as those summaries were.
 */
export function eventSummary(
  summary: CampaignEvent["summary"],
  locale: Locale,
): { text: string; lang: "fr" | "en"; foreign: boolean } {
  const lang = typeof summary === "string" ? "en" : SUMMARY_LANGUAGE[locale]
  const text = typeof summary === "string" ? summary : summary[lang]
  return { text, lang, foreign: new Intl.Locale(locale).language !== lang }
}
