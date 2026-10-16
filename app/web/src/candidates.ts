// The Candidates view's grouping, kept out of the component so it can be tested headlessly.
import type { CandidateForecast } from "./api/schemas"

/** The political families of config/candidates.yaml, in its order: left to right, then the rest. */
export const BLOCS = ["far_left", "left", "centre", "right", "far_right", "other"]

/** A family's place in BLOCS; one it doesn't list goes after all of them. */
const rank = (bloc: string) => {
  const i = BLOCS.indexOf(bloc)
  return i === -1 ? BLOCS.length : i
}

export interface BlocGroup {
  readonly bloc: string
  readonly candidates: readonly CandidateForecast[]
}

/**
 * Candidates by political family, in the order of BLOCS (a family it doesn't list comes after,
 * alphabetically); within a family, the likeliest to run first, then by name.
 */
export function byBloc(candidates: readonly CandidateForecast[]): BlocGroup[] {
  const groups = new Map<string, CandidateForecast[]>()
  for (const candidate of candidates) {
    const list = groups.get(candidate.bloc) ?? []
    list.push(candidate)
    groups.set(candidate.bloc, list)
  }
  return [...groups]
    .toSorted(([a], [b]) => rank(a) - rank(b) || a.localeCompare(b))
    .map(([bloc, list]) => ({
      bloc,
      candidates: list.toSorted((a, b) => b.p_run - a.p_run || a.name.localeCompare(b.name, "fr")),
    }))
}
