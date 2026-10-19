import { describe, expect, it } from "vitest"
import type { CampaignEvent, Forecast, Headline, Markets, NewsDay, PageViews, Signals, XRow } from "./api/schemas"
import {
  addDays,
  attentionRows,
  candidateNames,
  eventSummary,
  eventsNewestFirst,
  filterSignals,
  headlinesFor,
  headlinesInRange,
  marketRows,
  newsCounts,
  pressVolume,
  sentiment,
  stanceCounts,
  sparkline,
  sparkRange,
  unmatchedRows,
  wilson,
  windowsInRange,
  xPooled,
  xWindows,
} from "./signals"

const candidate = (id: string, pRun: number, pQualify: number, pWin: number) => ({
  candidate_id: id,
  name: id.toUpperCase(),
  party: "P",
  bloc: "b",
  status: "declared",
  p_run: pRun,
  simulated: true,
  p_qualify_r1: pQualify,
  p_win: pWin,
  r1_share: null,
})

const forecast: Forecast = {
  schema_version: "1.0",
  model_version: "test",
  generated_at: "2026-10-09T14:00:00Z",
  as_of: "2026-10-09",
  inputs_hash: "sha256:0",
  seed: 1,
  simulations: 100,
  election: { round1: "2027-04-18", round2: "2027-05-02" },
  candidates: [candidate("a", 0.9, 0.8, 0.6), candidate("b", 0.8, 0.5, 0.3), candidate("c", 0.2, 0.001, 0.001)],
  pairs: [],
  aggregation: {
    random_walk_sd: 0.01,
    design_effect: 1.5,
    scenario_noise_share: 0.2,
    log_likelihood: 0,
    polls_used: 1,
    scenarios_used: 1,
    round2_pairs_polled: 1,
    last_field_end: "2026-10-01",
    poll_ids: [],
    house_effects: [],
  },
  changes: { previous: null, new_polls: [], p_win_delta: [] },
  warnings: [],
}

const quote = (candidate_id: string, venue: "polymarket" | "kalshi", price: number | null, question = "win") => ({
  candidate_id,
  venue,
  question: question as "win" | "qualify" | "on_ballot",
  price,
  bid: null,
  ask: null,
  volume: null,
})

const close = (date: string, candidate_id: string, venue: "polymarket" | "kalshi", price: number | null) => ({
  date,
  candidate_id,
  venue,
  question: "win" as const,
  price,
})

const markets: Markets = {
  fetched_at: "2026-10-09T20:00:00Z",
  sources: [],
  latest: [
    quote("a", "polymarket", 0.41),
    quote("a", "kalshi", 0.45),
    quote("b", "polymarket", 0.2),
    quote("c", "kalshi", 0.004),
    // Matched to our config, absent from the forecast.
    quote("d", "polymarket", 0.07),
    quote("a", "polymarket", 0.9, "on_ballot"),
  ],
  unmatched: [
    { label: "X", venue: "polymarket", question: "win", price: 0.02 },
    { label: "X", venue: "kalshi", question: "win", price: 0.03 },
    { label: "Y", venue: "polymarket", question: "win", price: 0.009 },
    { label: "Z", venue: "polymarket", question: "qualify", price: 0.2 },
  ],
  history: [
    close("2026-08-01", "a", "polymarket", 0.3), // outside the 30-day window
    close("2026-09-20", "a", "polymarket", 0.35),
    close("2026-10-09", "a", "polymarket", 0.41),
    close("2026-10-08", "a", "polymarket", 0.4),
    close("2026-10-09", "a", "kalshi", 0.45), // a single day draws nothing
    close("2026-10-08", "b", "kalshi", null),
  ],
}

describe("marketRows", () => {
  it("lines up the model and both venues, highest first, dropping everyone under 0.5%", () => {
    const { rows, window } = marketRows(forecast, markets, "win")
    expect(window).toEqual(["2026-09-10", "2026-10-09"])
    expect(rows.map((r) => r.candidateId)).toEqual(["a", "b", "d"])
    expect(rows[0]?.model).toBe(0.6)
    expect(rows[0]?.prices).toEqual({ polymarket: 0.41, kalshi: 0.45 })
    expect(rows[2]).toMatchObject({ name: "d", model: null, prices: { polymarket: 0.07, kalshi: null } })
  })

  it("keeps 30 days of daily prices per venue, only from 2 days up", () => {
    const [a] = marketRows(forecast, markets, "win").rows
    expect(a?.history.polymarket.map((p) => p.date)).toEqual(["2026-09-20", "2026-10-08", "2026-10-09"])
    expect(a?.history.kalshi).toEqual([])
  })

  it("compares each question with the matching model probability", () => {
    const { rows } = marketRows(forecast, markets, "on_ballot")
    expect(rows.map((r) => [r.candidateId, r.model, r.prices.polymarket])).toEqual([
      ["a", 0.9, 0.9],
      ["b", 0.8, null],
      ["c", 0.2, null],
    ])
  })

  it("has no window without history", () => {
    expect(marketRows(forecast, { ...markets, history: [] }, "win").window).toBeNull()
  })
})

describe("unmatchedRows", () => {
  it("groups people outside our field by name, at 1% or more on the question", () => {
    expect(unmatchedRows(markets, "win")).toEqual([{ label: "X", prices: { polymarket: 0.02, kalshi: 0.03 } }])
    expect(unmatchedRows(markets, "qualify").map((u) => u.label)).toEqual(["Z"])
    expect(unmatchedRows(markets, "on_ballot")).toEqual([])
  })
})

const views = (candidate_id: string, end: string, perDay: (daysAgo: number) => number, days: number): PageViews[] =>
  Array.from({ length: days }, (_, i) => ({ date: addDays(end, -i), candidate_id, views: perDay(i) }))

describe("attentionRows", () => {
  const names = candidateNames(forecast)

  it("totals the last 7 days, compares with the 7 before and keeps 90 days", () => {
    const rows = [
      ...views("a", "2026-10-09", (d) => (d < 7 ? 300 : 200), 100),
      ...views("b", "2026-10-09", () => 1000, 5),
      ...views("c", "2026-10-02", () => 50, 3), // nothing in the last week
    ]
    const table = attentionRows(rows, names)
    expect(table.window).toEqual(["2026-07-12", "2026-10-09"])
    expect(table.rows.map((r) => [r.candidateId, r.total, r.change])).toEqual([
      ["b", 5000, null],
      ["a", 2100, 0.5],
    ])
    expect(table.rows[1]?.daily).toHaveLength(90)
    expect(table.rows[1]?.daily[0]?.date).toBe("2026-07-12")
  })

  it("keeps the top 12 and is empty without data", () => {
    const many = Array.from({ length: 15 }, (_, i) => views(`k${i}`, "2026-10-09", () => i + 1, 7)).flat()
    expect(attentionRows(many, names).rows.map((r) => r.candidateId)).toHaveLength(12)
    expect(attentionRows(many, names).rows[0]?.candidateId).toBe("k14")
    expect(attentionRows([], names)).toEqual({ rows: [], window: null })
  })
})

const day = (date: string, candidate_id: string, items: number): NewsDay => ({ date, candidate_id, items })
const headline = (published_at: string, ids: string[]): Headline => ({
  published_at,
  outlet: "Le Monde",
  title: `t-${published_at}`,
  url: `https://example.org/${published_at}`,
  candidate_ids: ids,
})

const volumeDay = (date: string, candidate_id: string, articles: number) => ({
  date,
  candidate_id,
  articles,
  total_articles: 1000,
})

describe("news", () => {
  const names = candidateNames(forecast)
  const headlines = [
    headline("2026-10-09T10:00:00Z", ["b"]),
    headline("2026-10-09T08:00:00Z", ["a", "b"]),
    headline("2026-10-08T11:00:00Z", ["a"]), // 23 hours before the newest
    headline("2026-10-08T09:00:00Z", ["a"]), // 25 hours before
    headline("2026-10-01T09:00:00Z", ["c"]),
  ]

  it("keeps the headlines of the range before the newest one, newest first", () => {
    expect(headlinesInRange(headlines, "1d").map((h) => h.published_at)).toEqual([
      "2026-10-09T10:00:00Z",
      "2026-10-09T08:00:00Z",
      "2026-10-08T11:00:00Z",
    ])
    expect(headlinesInRange(headlines, "7d")).toHaveLength(4)
    expect(headlinesInRange(headlines.toReversed(), "all")).toEqual(headlines)
    expect(headlinesInRange([], "7d")).toEqual([])
  })

  it("filters headlines by candidate", () => {
    expect(headlinesFor(headlines, "b").map((h) => h.published_at)).toEqual([
      "2026-10-09T10:00:00Z",
      "2026-10-09T08:00:00Z",
    ])
    expect(headlinesFor(headlines, null)).toEqual(headlines)
    expect(headlinesFor(headlines, "z")).toEqual([])
  })

  it("counts headlines per candidate in the range, with their days", () => {
    const daily = [day("2026-10-09", "a", 1), day("2026-10-08", "a", 2), day("2026-10-01", "c", 1)]
    const day1 = newsCounts(headlines, daily, names, "1d")
    expect(day1.rows.map((r) => [r.candidateId, r.items])).toEqual([
      ["a", 2],
      ["b", 2],
    ])
    // A single day draws no line.
    expect(day1.window).toBeNull()
    // The line ends on 8 October, the last day before the newest headline's.
    const week = newsCounts(headlines, daily, names, "7d")
    expect(week.window).toEqual(["2026-10-02", "2026-10-08"])
    expect(week.rows[0]).toEqual({
      candidateId: "a",
      name: "A",
      items: 3,
      daily: ["02", "03", "04", "05", "06", "07", "08"].map((d) => ({
        date: `2026-10-${d}`,
        value: d === "08" ? 2 : 0,
      })),
    })
    expect(day1.rows[0]!.daily).toEqual([])
    expect(newsCounts(headlines, daily, names, "all").window).toEqual(["2026-10-01", "2026-10-08"])
    expect(newsCounts([], [], names, "7d")).toEqual({ rows: [], window: null })
  })

  it("sums GDELT's articles in the range, out of the day's sampled total counted once", () => {
    const volume = [
      volumeDay("2026-10-09", "a", 30),
      volumeDay("2026-10-09", "b", 10),
      volumeDay("2026-10-08", "a", 20),
      volumeDay("2026-08-01", "b", 5),
    ]
    const day1 = pressVolume(volume, names, "1d")
    expect(day1.range).toEqual(["2026-10-09", "2026-10-09"])
    expect(day1.window).toEqual(["2026-08-01", "2026-10-09"])
    expect(day1.rows.map((r) => [r.candidateId, r.articles, r.share])).toEqual([
      ["a", 30, 0.03],
      ["b", 10, 0.01],
    ])
    const all = pressVolume(volume, names, "all")
    expect(all.rows[0]).toMatchObject({ candidateId: "a", articles: 50, share: 50 / 3000 })
    expect(all.rows[1]!.daily).toEqual([
      { date: "2026-08-01", value: 5 },
      { date: "2026-10-09", value: 10 },
    ])
    expect(pressVolume([], names, "7d")).toEqual({ rows: [], window: null, range: null })
  })
})

const xRow = (window_end: string, candidate_id: string, mentions: number | null, mood?: [number, number, number]) =>
  ({
    window_start: addDays(window_end.slice(0, 10), -1) + window_end.slice(10),
    window_end,
    candidate_id,
    mentions,
    mentions_method: mentions === null ? null : "sample_estimate",
    sample_size: 200,
    unique_authors: null,
    engagement: null,
    sentiment_pos: mood?.[0] ?? null,
    sentiment_neu: mood?.[1] ?? null,
    sentiment_neg: mood?.[2] ?? null,
    bot_share_estimate: 0.1,
    top_topics: null,
    stance_posts_drawn: null,
    stance_off_topic: null,
    support_share: null,
    support_low: null,
    support_high: null,
    oppose_share: null,
    oppose_low: null,
    oppose_high: null,
    neutral_share: null,
    neutral_low: null,
    neutral_high: null,
    drop_kappa: null,
  }) satisfies XRow

const stanceRow = (window_end: string, candidate_id: string, counts: [number, number, number], mentions = 100) => {
  const [support, oppose, neutral] = counts
  const n = support + oppose + neutral
  return {
    ...xRow(window_end, candidate_id, mentions),
    window_start: new Date(Date.parse(window_end) - 12 * 3_600_000).toISOString().replace(".000", ""),
    rubric_version: "stance-1",
    sample_size: n + 5,
    stance_posts_drawn: n + 5,
    stance_off_topic: 5,
    stance_support: support,
    stance_oppose: oppose,
    stance_neutral: neutral,
    support_share: n ? support / n : null,
    oppose_share: n ? oppose / n : null,
    neutral_share: n ? neutral / n : null,
  } satisfies XRow
}

// 12-hour window ends on a day of October 2026.
const T = (date: number, hour: number) =>
  `2026-10-${String(date).padStart(2, "0")}T${String(hour).padStart(2, "0")}:00:00Z`

describe("X", () => {
  const names = new Map([["a", "A"]])

  it("lists the windows oldest first and keeps those starting inside a range", () => {
    const rows = [stanceRow(T(10, 0), "a", [1, 1, 1]), xRow(T(9, 12), "a", 5), stanceRow(T(10, 12), "b", [1, 0, 0])]
    const windows = xWindows(rows)
    expect(windows.map((w) => [w.end, w.stance])).toEqual([
      [T(9, 12), false],
      [T(10, 0), true],
      [T(10, 12), true],
    ])
    // 24 hours back from the newest end (10 Oct 12:00) is 9 Oct 12:00: the window that starts there stays.
    expect(windowsInRange(windows, "1d").map((w) => w.end)).toEqual([T(10, 0), T(10, 12)])
    expect(windowsInRange(windows, "all")).toHaveLength(3)
    expect(windowsInRange([], "7d")).toEqual([])
  })

  it("matches the collector's Wilson interval", () => {
    // 30 of 100: 0.2189 to 0.3958, as `wilson` in collectors/src/grok.rs gives it.
    const i = wilson(30, 100)!
    expect(i.share).toBe(0.3)
    expect(i.low).toBeCloseTo(0.2189, 4)
    expect(i.high).toBeCloseTo(0.3958, 4)
    expect(wilson(0, 0)).toBeNull()
  })

  it("rebuilds counts from the shares when an older API sends only those", () => {
    const { stance_support: _support, stance_oppose: _oppose, ...row } = stanceRow(T(10, 0), "a", [20, 10, 10])
    expect(stanceCounts(row)).toEqual({ support: 20, oppose: 10, neutral: 10 })
    expect(stanceCounts(xRow(T(10, 0), "a", 1, [0.3, 0.5, 0.2]))).toBeNull()
  })

  it("pools windows by summing posts, so a bigger drop weighs more", () => {
    const rows = [
      stanceRow(T(10, 0), "a", [10, 0, 0], 500),
      stanceRow(T(10, 12), "a", [30, 50, 20], 700),
      stanceRow(T(10, 12), "b", [1, 1, 1], 900),
    ]
    const pooled = xPooled(rows, [T(10, 0), T(10, 12)], names)
    expect(pooled.map((r) => r.candidateId)).toEqual(["a", "b"])
    const a = pooled[0]!
    expect(a.name).toBe("A")
    expect(a.mentions).toBe(1200)
    expect(a.relevant).toBe(110)
    expect(a.stance?.support.share).toBeCloseTo(40 / 110)
    expect(a.stance?.support.low).toBeCloseTo(wilson(40, 110)!.low)
    expect(a.mood).toEqual({ pos: 40 / 110, neu: 20 / 110, neg: 50 / 110 })
    expect(a.trend).toEqual([
      { date: T(10, 0), value: 1 },
      { date: T(10, 12), value: 0.3 },
    ])
    // The first window rests on 10 posts: too few for a change.
    expect(a.change).toBeNull()
    expect(a.thin).toBe(false)
    expect(pooled[1]!.thin).toBe(true)
  })

  it("gives the change between the range's first and last windows when both hold enough posts", () => {
    const rows = [stanceRow(T(10, 0), "a", [20, 10, 10]), stanceRow(T(10, 12), "a", [10, 20, 10])]
    expect(xPooled(rows, [T(10, 0), T(10, 12)], names)[0]!.change).toBeCloseTo(-0.25)
  })

  it("shows the sentiment drops read before the stance rubric only when no stance drop is kept", () => {
    const rows = [
      { ...xRow(T(9, 12), "a", 50, [0.6, 0.4, 0]), sample_size: 25 },
      stanceRow(T(10, 0), "a", [10, 20, 10]),
    ]
    const alone = xPooled(rows, [T(9, 12)], names)[0]!
    expect(alone.legacy).toBe(true)
    expect(alone.stance).toBeNull()
    expect(alone.mood?.pos).toBeCloseTo(0.6)
    expect(alone.thin).toBe(true)
    const both = xPooled(rows, [T(9, 12), T(10, 0)], names)[0]!
    expect(both.legacy).toBe(false)
    expect(both.mood?.pos).toBeCloseTo(0.25)
    expect(both.mentions).toBe(150)
  })

  it("rescales sentiment to a whole and needs all three shares", () => {
    const shares = sentiment(xRow("2026-10-09T12:00:00Z", "a", 1, [0.3, 0.5, 0.2]))
    expect(shares?.pos).toBeCloseTo(0.3)
    expect(shares?.neu).toBeCloseTo(0.5)
    expect(shares?.neg).toBeCloseTo(0.2)
    const rescaled = sentiment(xRow("2026-10-09T12:00:00Z", "a", 1, [2, 1, 1]))
    expect(rescaled).toEqual({ pos: 0.5, neu: 0.25, neg: 0.25 })
    expect(sentiment(xRow("2026-10-09T12:00:00Z", "a", 1))).toBeNull()
    expect(sentiment(xRow("2026-10-09T12:00:00Z", "a", 1, [0, 0, 0]))).toBeNull()
  })
})

const event = (date: string, event_id: string): CampaignEvent => ({
  date,
  event_id,
  kind: "declaration",
  candidate_ids: ["a"],
  summary: { fr: "Il s'est passé quelque chose.", en: "Something happened in the campaign." },
  source_url: "https://example.org",
  source_title: null,
  reported_by: "grok",
})

describe("eventsNewestFirst", () => {
  it("orders by date, then id, newest first", () => {
    const sorted = eventsNewestFirst([
      event("2026-10-01", "2026-10-01-a"),
      event("2026-10-05", "2026-10-05-a"),
      event("2026-10-05", "2026-10-05-b"),
    ])
    expect(sorted.map((e) => e.event_id)).toEqual(["2026-10-05-b", "2026-10-05-a", "2026-10-01-a"])
  })
})

describe("filterSignals", () => {
  it("drops hidden candidates, keeping what also names someone shown or nobody", () => {
    const signals: Signals = {
      generated_at: "2026-10-16T16:00:00Z",
      markets,
      attention: {
        fetched_at: null,
        rows: views("a", "2026-10-09", () => 10, 2).concat(views("b", "2026-10-09", () => 5, 2)),
      },
      news: {
        fetched_at: null,
        feeds: [],
        daily: [day("2026-10-09", "a", 3), day("2026-10-09", "b", 2)],
        headlines: [
          headline("2026-10-09T08:00:00Z", ["a"]),
          headline("2026-10-09T09:00:00Z", ["a", "b"]),
          headline("2026-10-09T10:00:00Z", []),
        ],
        volume: [
          { date: "2026-10-09", candidate_id: "a", articles: 3, total_articles: 100 },
          { date: "2026-10-09", candidate_id: "b", articles: 1, total_articles: 100 },
        ],
      },
      x: { rows: [xRow("2026-10-09T12:00:00Z", "a", 100), xRow("2026-10-09T12:00:00Z", "b", 50)] },
      events: [event("2026-10-01", "2026-10-01-a"), { ...event("2026-10-02", "2026-10-02-b"), candidate_ids: ["b"] }],
      forward: {
        horizon_days: 14,
        flags: (["a", "b"] as const).map((id) => ({
          detected_at: "2026-10-10T07:17:00Z",
          kind: "spike" as const,
          candidate_id: id,
          date: "2026-10-09",
          score: 1.2,
          detail: "",
          change: null,
        })),
        summary: {
          logged: 2,
          spikes_scored: 0,
          spike_mean_move: null,
          typical_mean_move: null,
          x_shifts_scored: 0,
          x_shifts_agreeing: 0,
        },
      },
    }
    const shown = filterSignals(signals, new Set(["a"]))
    expect(shown.markets.latest.some((q) => q.candidate_id === "a")).toBe(false)
    expect(shown.markets.history.some((q) => q.candidate_id === "a")).toBe(false)
    expect(new Set(shown.attention.rows.map((r) => r.candidate_id))).toEqual(new Set(["b"]))
    expect(shown.news.daily.map((d) => d.candidate_id)).toEqual(["b"])
    expect(shown.news.volume?.map((d) => d.candidate_id)).toEqual(["b"])
    expect(shown.news.headlines.map((h) => h.candidate_ids)).toEqual([["a", "b"], []])
    expect(shown.x.rows.map((r) => r.candidate_id)).toEqual(["b"])
    expect(shown.events.map((e) => e.event_id)).toEqual(["2026-10-02-b"])
    expect(shown.forward.flags.map((f) => f.candidate_id)).toEqual(["b"])
  })
})

describe("sparkline", () => {
  const box = { width: 100, height: 20, pad: 0 }

  it("places days on the window and values from 0 to the series maximum", () => {
    const shape = sparkline(
      [
        { date: "2026-10-01", value: 0 },
        { date: "2026-10-06", value: 5 },
        { date: "2026-10-11", value: 10 },
      ],
      ["2026-10-01", "2026-10-11"],
      box,
    )
    expect(shape.line).toBe("M0.0,20.0L50.0,10.0L100.0,0.0")
    expect(shape.end).toEqual({ x: 100, y: 0 })
  })

  it("lays a flat zero series on the baseline and draws nothing below 2 points", () => {
    const flat = sparkline(
      [
        { date: "2026-10-01", value: 0 },
        { date: "2026-10-11", value: 0 },
      ],
      ["2026-10-01", "2026-10-11"],
      box,
    )
    expect(flat.line).toBe("M0.0,20.0L100.0,20.0")
    expect(sparkline([{ date: "2026-10-01", value: 1 }], ["2026-10-01", "2026-10-11"], box)).toEqual({
      line: "",
      end: null,
    })
  })

  it("scales prices to their own range, never narrower than the minimum span", () => {
    expect(sparkRange([0.4, 0.5])).toEqual([0, 0.5])
    expect(sparkRange([0.3, 0.5], 0.05)).toEqual([0.3, 0.5])
    const [low, high] = sparkRange([0.41, 0.43], 0.05)
    expect(low).toBeCloseTo(0.395)
    expect(high).toBeCloseTo(0.445)
    expect(sparkRange([0.01, 0.02], 0.05)).toEqual([0, 0.05])
    const shape = sparkline(
      [
        { date: "2026-10-01", value: 0.3 },
        { date: "2026-10-11", value: 0.5 },
      ],
      ["2026-10-01", "2026-10-11"],
      box,
      0.05,
    )
    expect(shape.line).toBe("M0.0,20.0L100.0,0.0")
  })

  it("steps calendar days across month ends", () => {
    expect(addDays("2026-10-01", -1)).toBe("2026-09-30")
    expect(addDays("2026-12-31", 1)).toBe("2027-01-01")
  })
})

describe("eventSummary", () => {
  const both = { fr: "En français.", en: "In English." }
  it("shows French on the French site and English on every other", () => {
    expect(eventSummary(both, "fr")).toEqual({ text: "En français.", lang: "fr", foreign: false })
    expect(eventSummary(both, "en-GB")).toEqual({ text: "In English.", lang: "en", foreign: false })
    expect(eventSummary(both, "ja")).toEqual({ text: "In English.", lang: "en", foreign: true })
  })
  it("shows a legacy string as English", () => {
    expect(eventSummary("Old summary.", "fr")).toEqual({ text: "Old summary.", lang: "en", foreign: true })
  })
})
