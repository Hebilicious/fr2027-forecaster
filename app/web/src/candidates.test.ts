import { describe, expect, it } from "vitest"
import type { CandidateForecast } from "./api/schemas"
import { byBloc } from "./candidates"

const candidate = (id: string, bloc: string, pRun: number): CandidateForecast => ({
  candidate_id: id,
  name: id.toUpperCase(),
  party: "P",
  bloc,
  status: "declared",
  p_run: pRun,
  simulated: true,
  p_qualify_r1: 0,
  p_win: 0,
  r1_share: null,
})

describe("byBloc", () => {
  it("orders families left to right, unknown ones last, and the likeliest to run first", () => {
    const groups = byBloc([
      candidate("e", "far_right", 0.9),
      candidate("b", "left", 0.2),
      candidate("x", "regional", 1),
      candidate("a", "left", 0.9),
      candidate("c", "left", 0.9),
      candidate("o", "other", 0.5),
    ])
    expect(groups.map((g) => g.bloc)).toEqual(["left", "far_right", "other", "regional"])
    expect(groups[0]?.candidates.map((c) => c.candidate_id)).toEqual(["a", "c", "b"])
  })
})
