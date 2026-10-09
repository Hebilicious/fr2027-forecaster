import { describe, expect, it } from "vitest"
import type { CampaignEvent, Forecast, Headline, Markets, NewsDay, PageViews, XRow } from "./api/schemas"
import {
  addDays,
  attentionRows,
  candidateNames,
  eventsNewestFirst,
  latestHeadlines,
  latestXWindow,
  marketRows,
  newsCounts,
  sentiment,
  sparkline,
  sparkRange,
  unmatchedRows,
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

describe("news", () => {
  const names = candidateNames(forecast)

  it("counts headlines per candidate over the last 7 days in the data", () => {
    const daily = [
      day("2026-10-09", "a", 3),
      day("2026-10-03", "a", 2),
      day("2026-10-02", "a", 50), // eight days before the latest
      day("2026-10-08", "b", 6),
    ]
    expect(newsCounts(daily, names)).toEqual([
      { candidateId: "b", name: "B", items: 6 },
      { candidateId: "a", name: "A", items: 5 },
    ])
    expect(newsCounts([], names)).toEqual([])
  })

  it("lists the newest headlines, filtered by candidate", () => {
    const headlines = [
      headline("2026-10-08T10:00:00Z", ["a"]),
      headline("2026-10-09T10:00:00Z", ["b"]),
      headline("2026-10-09T08:00:00Z", ["a", "b"]),
    ]
    expect(latestHeadlines(headlines, null).map((h) => h.published_at)).toEqual([
      "2026-10-09T10:00:00Z",
      "2026-10-09T08:00:00Z",
      "2026-10-08T10:00:00Z",
    ])
    expect(latestHeadlines(headlines, "a", 1).map((h) => h.published_at)).toEqual(["2026-10-09T08:00:00Z"])
    expect(latestHeadlines(headlines, "c")).toEqual([])
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
  }) satisfies XRow

describe("X", () => {
  it("keeps the latest window, most mentioned first", () => {
    const window = latestXWindow([
      xRow("2026-10-09T06:00:00Z", "a", 9000),
      xRow("2026-10-09T12:00:00Z", "a", null),
      xRow("2026-10-09T12:00:00Z", "b", 1200),
      xRow("2026-10-09T12:00:00Z", "c", 4000),
    ])
    expect(window?.end).toBe("2026-10-09T12:00:00Z")
    expect(window?.start).toBe("2026-10-08T12:00:00Z")
    expect(window?.rows.map((r) => r.candidate_id)).toEqual(["c", "b", "a"])
    expect(latestXWindow([])).toBeNull()
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
  summary: "Something happened in the campaign.",
  source_url: "https://example.org",
  source_title: null,
  proposed_change: null,
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
