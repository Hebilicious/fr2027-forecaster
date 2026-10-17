import { describe, expect, it } from "vitest"
import type { Blend } from "./api/schemas"
import { blendAt, poolBinary, poolWin } from "./blend"

// The same cases as model/src/blend.rs's tests, so the slider pools as the build does.
describe("blend", () => {
  it("is the model at weight 1", () => {
    expect(poolWin([0.8, 0.15, 0.05], [0.4, 0.3, null], 1, 0.001)).toEqual([0.8, 0.15, 0.05])
    expect(poolBinary(0.9, 0.5, 1, 0.001)).toBe(0.9)
  })

  it("is the market where it prices at weight 0, the rest split as the model splits it", () => {
    // Prices already scaled with the others: 0.4/1.1, 0.5/1.1, everyone else 0.2/1.1.
    const blend = poolWin([0.7, 0.2, 0.1], [0.4 / 1.1, 0.5 / 1.1, null], 0, 0.001)
    expect(blend[0]).toBeCloseTo(0.4 / 1.1, 9)
    expect(blend[1]).toBeCloseTo(0.5 / 1.1, 9)
    expect(blend[2]).toBeCloseTo(0.2 / 1.1, 9)
  })

  it("is the normalised geometric mean halfway", () => {
    const blend = poolWin([0.9, 0.1], [0.5, 0.5], 0.5, 0.001)
    const [a, b] = [Math.sqrt(0.45), Math.sqrt(0.05)]
    expect(blend[0]).toBeCloseTo(a / (a + b), 3)
    expect((blend[0] ?? 0) + (blend[1] ?? 0)).toBeCloseTo(1, 9)
    expect(poolBinary(0.9, 0.5, 0.5, 0.001)).toBeCloseTo(0.75, 9)
  })

  it("keeps the model when the market snapshot is too old", () => {
    const blend: Blend = {
      forecast: "2026-10-16T19.json",
      model_weight: 0.75,
      probability_floor: 0.001,
      market_fetched_at: "2026-10-16T08:00:00Z",
      market_used: false,
      market_others_win: 0,
      candidates: [
        {
          candidate_id: "a",
          model: { win: 0.7, qualify: 0.9, run: 1 },
          market: { win: 0.4, qualify: 0.8, run: 1 },
          blend: { win: 0.7, qualify: 0.9, run: 1 },
        },
      ],
    }
    expect(blendAt(blend, 0)).toEqual(new Map([["a", { win: 0.7, qualify: 0.9 }]]))
  })

  it("pools another strength of the signals with the markets", () => {
    const blend: Blend = {
      forecast: "2026-10-17T08.json",
      model_weight: 0.75,
      probability_floor: 0.001,
      market_fetched_at: "2026-10-17T08:00:00Z",
      market_used: true,
      market_others_win: 0,
      candidates: [
        {
          candidate_id: "a",
          model: { win: 0.8, qualify: 0.9, run: 1 },
          market: { win: 0.5, qualify: null, run: null },
          blend: { win: 0.73, qualify: 0.9, run: 1 },
        },
        {
          candidate_id: "b",
          model: { win: 0.2, qualify: 0.6, run: 1 },
          market: { win: 0.5, qualify: null, run: null },
          blend: { win: 0.27, qualify: 0.6, run: 1 },
        },
      ],
    }
    const level = new Map([
      ["a", { win: 0.6, qualify: 0.8 }],
      ["b", { win: 0.4, qualify: 0.7 }],
    ])
    // At weight 1 the chances are the level's own; the market changes nothing.
    expect(blendAt(blend, 1, level).get("a")).toEqual({ win: 0.6, qualify: 0.8 })
    // Pooled with the markets, the level's chances are the model side.
    const halfway = blendAt(blend, 0.5, level).get("a")?.win ?? 0
    const [x, y] = [Math.sqrt(0.6 * 0.5), Math.sqrt(0.4 * 0.5)]
    expect(halfway).toBeCloseTo(x / (x + y), 3)
  })
})
