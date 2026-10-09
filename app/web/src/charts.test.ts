import { describe, expect, it } from "vitest"
import type { Forecast, PollRow, SeriesPoint } from "./api/schemas"
import { midpoint, round1Panels, round2Matrix, scale, shareTicks } from "./charts"

const candidate = (id: string, pQualify: number, pWin: number) => ({
  candidate_id: id,
  name: id.toUpperCase(),
  party: "P",
  bloc: "b",
  status: "declared",
  p_run: 1,
  simulated: true,
  p_qualify_r1: pQualify,
  p_win: pWin,
  r1_share: { mean: 0.2, lo80: 0.15, hi80: 0.25 },
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
  candidates: [candidate("a", 0.9, 0.6), candidate("b", 0.7, 0.3), candidate("c", 0.4, 0.1)],
  pairs: [
    { a: "a", b: "b", p_matchup: 0.6, p_win_given_matchup: { a: 0.55, b: 0.45 }, polled: true },
    { a: "a", b: "c", p_matchup: 0.3, p_win_given_matchup: { a: 0.7, c: 0.3 }, polled: false },
    { a: "b", b: "c", p_matchup: 0.1, p_win_given_matchup: { b: 0.5, c: 0.5 }, polled: false },
  ],
  aggregation: {
    random_walk_sd: 0.01,
    design_effect: 1.5,
    scenario_noise_share: 0.2,
    log_likelihood: 0,
    polls_used: 1,
    scenarios_used: 1,
    round2_pairs_polled: 1,
    last_field_end: "2026-10-01",
    poll_ids: ["p"],
    house_effects: [],
  },
  changes: { previous: null, new_polls: [], p_win_delta: [] },
  warnings: [],
}

const point = (candidate_id: string, date: string, mean: number): SeriesPoint => ({
  candidate_id,
  date,
  mean,
  lo80: mean - 0.02,
  hi80: mean + 0.02,
})

const row = (candidate_id: string, share: number, round = 1): PollRow => ({
  poll_id: "p",
  firm: "ifop",
  sponsor: null,
  field_start: "2026-09-01",
  field_end: "2026-09-04",
  published_at: "2026-09-05",
  sample_size: 1000,
  method: "online",
  population: null,
  round,
  scenario_id: "A",
  candidate_id,
  share,
  source_url: "https://example.org",
  notice_url: null,
  index_url: null,
  entry: "primary",
  retrieved_at: "2026-10-09T00:00:00Z",
  content_hash: "sha256:0",
})

describe("round1Panels", () => {
  it("orders panels by latest mean and attaches round 1 polls only", () => {
    const panels = round1Panels(
      forecast,
      [point("b", "2026-09-02", 0.3), point("a", "2026-09-02", 0.2), point("a", "2026-09-01", 0.25)],
      [row("a", 21), row("a", 55, 2), row("b", 30)],
    )
    expect(panels.map((p) => p.candidate.candidate_id)).toEqual(["b", "a"])
    const a = panels[1]
    expect(a?.points.map((p) => p.date)).toEqual(["2026-09-01", "2026-09-02"])
    expect(a?.polls).toHaveLength(1)
    expect(a?.polls[0]?.share).toBeCloseTo(0.21)
    expect(a?.polls[0]?.date).toBe("2026-09-02")
  })
})

describe("round2Matrix", () => {
  it("fills both orientations of every pair", () => {
    const { candidates, cells } = round2Matrix(forecast, 2)
    expect(candidates.map((c) => c.candidate_id)).toEqual(["a", "b"])
    expect(cells).toEqual([
      { row: "a", column: "b", pMatchup: 0.6, rowWins: 0.55, polled: true },
      { row: "b", column: "a", pMatchup: 0.6, rowWins: 0.45, polled: true },
    ])
  })
})

describe("helpers", () => {
  it("computes fieldwork midpoints like the model", () => {
    expect(midpoint("2026-09-01", "2026-09-04")).toBe("2026-09-02")
    expect(midpoint("2026-09-30", "2026-10-02")).toBe("2026-10-01")
  })

  it("scales linearly", () => {
    expect(scale(5, { domain: [0, 10], range: [100, 0] })).toBe(50)
  })

  it("picks clean share ticks", () => {
    expect(shareTicks(0.27)).toEqual([0, 0.05, 0.1, 0.15, 0.2, 0.25, 0.3])
    expect(shareTicks(0.36)).toEqual([0, 0.1, 0.2, 0.3, 0.4])
  })
})
