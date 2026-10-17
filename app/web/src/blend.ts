// The market blend, as model/src/blend.rs computes it, so the Overview's slider can pool the
// model with the markets at any weight without a new build. api/blend.json carries both sides'
// probabilities; `blend` in it is the published weight's result, which `blendAt` reproduces.
import type { Blend } from "./api/schemas"

const clip = (p: number, floor: number) => Math.min(Math.max(p, floor), 1 - floor)

/** `logit p = w logit p_model + (1 − w) logit p_market`; the model's own value without a price. */
export function poolBinary(model: number, market: number | null, w: number, floor: number): number {
  if (market === null || w >= 1) return model
  const logit = (p: number) => {
    const c = clip(p, floor)
    return Math.log(c / (1 - c))
  }
  const x = w * logit(model) + (1 - w) * logit(market)
  return 1 / (1 + Math.exp(-x))
}

/**
 * Who wins, pooled over the candidates both sides price plus everyone else: market prices are
 * already scaled with the markets' share on people outside the field (`others`); the pooled
 * "everyone else" goes to the remaining candidates as the model splits it.
 */
export function poolWin(
  model: readonly number[],
  market: readonly (number | null)[],
  w: number,
  floor: number,
): number[] {
  const both = model.flatMap((_, c) => (market[c] !== null ? [c] : []))
  if (w >= 1 || both.length === 0) return [...model]
  const modelRest = Math.max(0, 1 - both.reduce((s, c) => s + (model[c] ?? 0), 0))
  const marketRest = Math.max(0, 1 - both.reduce((s, c) => s + (market[c] ?? 0), 0))
  const pooled = (a: number, b: number) => Math.exp(w * Math.log(clip(a, floor)) + (1 - w) * Math.log(clip(b, floor)))
  const weights = both.map((c) => pooled(model[c] ?? 0, market[c] ?? 0))
  if (modelRest > 0) weights.push(pooled(modelRest, marketRest))
  const total = weights.reduce((s, x) => s + x, 0)
  const rest = both.length < weights.length ? (weights[both.length] ?? 0) / total : 0
  const out = model.map((p) => (modelRest > 0 ? (rest * p) / modelRest : 0))
  both.forEach((c, i) => {
    out[c] = (weights[i] ?? 0) / total
  })
  return out
}

export interface Blended {
  readonly win: number
  readonly qualify: number
}

/**
 * Each candidate's chance of winning and of reaching round 2 with the model weighted at `modelWeight`.
 * `model` replaces the published model's chances (another strength of the news and social signals).
 */
export function blendAt(
  blend: Blend,
  modelWeight: number,
  model?: ReadonlyMap<string, Blended>,
): ReadonlyMap<string, Blended> {
  const rows = blend.candidates.map((r) => {
    const own = model?.get(r.candidate_id)
    return own ? { ...r, model: { ...r.model, win: own.win, qualify: own.qualify } } : r
  })
  if (!blend.market_used)
    return new Map(rows.map((r) => [r.candidate_id, { win: r.model.win, qualify: r.model.qualify }]))
  const win = poolWin(
    rows.map((r) => r.model.win),
    rows.map((r) => r.market.win),
    modelWeight,
    blend.probability_floor,
  )
  return new Map(
    rows.map((r, i) => [
      r.candidate_id,
      {
        win: win[i] ?? r.model.win,
        qualify: poolBinary(r.model.qualify, r.market.qualify, modelWeight, blend.probability_floor),
      },
    ]),
  )
}
