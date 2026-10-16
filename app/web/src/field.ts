// Which candidates the views show. The model keeps everyone in its simulated fields; these filters change only
// what a reader sees, kept out of the components so they can be tested headlessly.
import type { CandidateForecast } from "./api/schemas"

/**
 * - `running`: may well be on the ballot.
 * - `standIn`: has not declared, and shares a slot with someone likelier to run, who they could only replace
 *   (Bardella, should Le Pen be barred).
 * - `out`: no chance of running left, such as a primary's losers.
 */
export type Standing = "running" | "standIn" | "out"

export function standing(candidate: CandidateForecast, field: readonly CandidateForecast[]): Standing {
  if (candidate.p_run === 0) return "out"
  if (candidate.status === "declared" || candidate.slot === undefined) return "running"
  const replaces = field.some(
    (other) =>
      other.slot === candidate.slot && other.candidate_id !== candidate.candidate_id && other.p_run > candidate.p_run,
  )
  return replaces ? "standIn" : "running"
}

/** Lower case, without accents, so "melenchon" finds Mélenchon. */
const fold = (text: string) => text.normalize("NFD").replace(/\p{M}/gu, "").toLocaleLowerCase("fr")

/** Whether the candidate's name or party contains every word of the query. */
export function matches(candidate: CandidateForecast, query: string): boolean {
  const haystack = fold(`${candidate.name} ${candidate.party}`)
  return fold(query)
    .split(/\s+/)
    .every((word) => haystack.includes(word))
}

export interface FieldView {
  /** The candidates to show, in the order given. */
  readonly visible: readonly CandidateForecast[]
  /** Ids of the candidates left out, by either filter. */
  readonly hidden: ReadonlySet<string>
  /** Ids of those left out because they will not run, whatever the search; empty when all are shown. */
  readonly setAside: ReadonlySet<string>
  /** How many will not run, shown or not. */
  readonly notRunning: number
}

export function filterField(
  candidates: readonly CandidateForecast[],
  { query, showAll }: { query: string; showAll: boolean },
): FieldView {
  const notRunning = candidates.filter((c) => standing(c, candidates) !== "running").map((c) => c.candidate_id)
  const setAside = new Set(showAll ? [] : notRunning)
  const kept = (c: CandidateForecast) => !setAside.has(c.candidate_id) && matches(c, query)
  return {
    visible: candidates.filter(kept),
    hidden: new Set(candidates.filter((c) => !kept(c)).map((c) => c.candidate_id)),
    setAside,
    notRunning: notRunning.length,
  }
}
