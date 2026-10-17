import { describe, expect, it } from "vitest"
import type { Forecast, HistoryRun } from "./api/schemas"
import {
  DIVERGING,
  LINE_COLOURS,
  chanceLines,
  divergingColour,
  pathsToVictory,
  pollsterLeans,
  trackRecordView,
} from "./explorer"

const candidate = (id: string, pQualify: number, pWin: number, simulated = true) => ({
  candidate_id: id,
  name: id.toUpperCase(),
  party: "P",
  bloc: "b",
  status: "declared",
  p_run: 1,
  simulated,
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
  candidates: [
    candidate("a", 0.9, 0.7),
    candidate("b", 0.7, 0.05),
    candidate("c", 0.4, 0.25),
    candidate("d", 0, 0, false),
  ],
  pairs: [
    { a: "a", b: "b", p_matchup: 0.6, p_win_given_matchup: { a: 0.92, b: 0.08 }, polled: true },
    { a: "a", b: "c", p_matchup: 0.3, p_win_given_matchup: { a: 0.3, c: 0.7 }, polled: false },
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
    house_effects: [
      { firm: "ifop", bloc: "far_right", mean: 0.04, sd: 0.02 },
      { firm: "elabe", bloc: "left", mean: -0.01, sd: 0.03 },
      { firm: "elabe", bloc: "far_right", mean: -0.08, sd: 0.02 },
      { firm: "elabe", bloc: "other", mean: 0.01, sd: 0.02 },
    ],
  },
  changes: { previous: null, new_polls: [], p_win_delta: [] },
  warnings: [],
}

const run = (generatedAt: string, wins: Record<string, number | null>): HistoryRun => ({
  file: `${generatedAt.slice(0, 13)}.json`,
  as_of: generatedAt.slice(0, 10),
  generated_at: generatedAt,
  candidates: Object.entries(wins).map(([candidate_id, p_win]) => ({ candidate_id, p_win, p_qualify_r1: null })),
})

describe("chanceLines", () => {
  it("follows the latest leaders through every run, oldest first, with fixed colours", () => {
    const runs = [
      run("2026-10-09T14:00:00Z", { a: 0.7, b: 0.05, c: 0.25 }),
      run("2026-10-02T14:00:00Z", { a: 0.5, b: 0.2, c: null }),
    ]
    const lines = chanceLines(runs, forecast, 2)
    expect(lines.map((l) => l.candidateId)).toEqual(["a", "c"])
    expect(lines.map((l) => l.colour)).toEqual([LINE_COLOURS[0], LINE_COLOURS[1]])
    expect(lines[0]!.points.map((p) => p.p)).toEqual([0.5, 0.7])
    // A run that gives no value for a candidate leaves a gap rather than a zero.
    expect(lines[1]!.points.map((p) => p.p)).toEqual([0.25])
  })
})

describe("pathsToVictory", () => {
  it("ranks by the chance of reaching round 2 and splits wins by opponent", () => {
    const paths = pathsToVictory(forecast)
    expect(paths.map((p) => p.candidate.candidate_id)).toEqual(["a", "b", "c"])
    const b = paths[1]!
    expect(b.conversion).toBeCloseTo(0.05 / 0.7)
    expect(b.opponents.map((o) => [o.candidateId, o.pMatchup, o.pWins])).toEqual([
      ["a", 0.6, 0.08],
      ["c", 0.1, 0.5],
    ])
  })
})

describe("pollsterLeans", () => {
  it("lists firms alphabetically and blocs left to right, with relative differences", () => {
    const leans = pollsterLeans(forecast)
    expect(leans.firms).toEqual(["elabe", "ifop"])
    expect(leans.spectrum).toEqual(["left", "far_right"])
    expect(leans.outside).toEqual(["other"])
    const ifop = leans.cells.find((c) => c.firm === "ifop")!
    expect(ifop.relative).toBeCloseTo(Math.exp(0.04) - 1)
    expect(ifop.clear).toBe(true)
    expect(leans.cells.find((c) => c.firm === "elabe" && c.bloc === "left")!.clear).toBe(false)
    expect(leans.max).toBeCloseTo(1 - Math.exp(-0.08))
  })
})

describe("divergingColour", () => {
  it("is the neutral grey at zero and a pole at either end", () => {
    expect(divergingColour(0, 1)).toBe(DIVERGING.mid)
    expect(divergingColour(-1, 1)).toBe(DIVERGING.low)
    expect(divergingColour(2, 1)).toBe(DIVERGING.high)
  })
})

describe("trackRecordView", () => {
  it("orders horizons furthest first and drops empty calibration bins", () => {
    const view = trackRecordView({
      model_version: "0.3.0",
      elections: [2017, 2022],
      by_horizon: [
        { days: 7, replays: 2, coverage80: 0.8, share_mae_points: 1 },
        { days: 182, replays: 2, coverage80: 0.7, share_mae_points: 3 },
      ],
      reliability: [
        { lo: 0, hi: 0.1, events: 10, mean_forecast: 0.02, observed: 0 },
        { lo: 0.1, hi: 0.2, events: 0, mean_forecast: null, observed: null },
      ],
    })
    expect(view.horizons.map((h) => h.days)).toEqual([182, 7])
    expect(view.bins).toHaveLength(1)
  })
})
