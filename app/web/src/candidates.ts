// The Candidates view's grouping, kept out of the component so it can be tested headlessly.
import type { CandidateForecast } from "./api/schemas"
import { ARTICLES } from "./articles"
import type { Locale } from "./locales"

/** Political families, left to right, then the rest. */
export const FAMILIES = ["far_left", "left", "centre", "right", "far_right", "other"]

/**
 * Where the page lists a candidate when it differs from their bloc in config/candidates.yaml.
 * LFI and PCF are listed with the far left, as RN and Reconquête are with the far right. The
 * model's blocs, which drive vote transfers and correlated errors, are unchanged.
 */
const FAMILY_OVERRIDES: Readonly<Record<string, string>> = {
  melenchon: "far_left",
  roussel: "far_left",
}

export const familyOf = (candidate: CandidateForecast): string =>
  FAMILY_OVERRIDES[candidate.candidate_id] ?? candidate.bloc

/** A family's place in FAMILIES; one it doesn't list goes after all of them. */
const rank = (family: string) => {
  const i = FAMILIES.indexOf(family)
  return i === -1 ? FAMILIES.length : i
}

export interface FamilyGroup {
  readonly family: string
  readonly candidates: readonly CandidateForecast[]
}

/** Candidates by political family, left to right; within one, the likeliest to run first, then by name. */
export function byFamily(candidates: readonly CandidateForecast[]): FamilyGroup[] {
  const groups = new Map<string, CandidateForecast[]>()
  for (const candidate of candidates) {
    const family = familyOf(candidate)
    groups.set(family, [...(groups.get(family) ?? []), candidate])
  }
  return [...groups]
    .toSorted(([a], [b]) => rank(a) - rank(b) || a.localeCompare(b))
    .map(([family, list]) => ({
      family,
      candidates: list.toSorted((a, b) => b.p_run - a.p_run || a.name.localeCompare(b.name, "fr")),
    }))
}

/**
 * The candidate's Wikipedia article in the reader's language, else the French one, with the language it is in;
 * `null` when French Wikipedia has no article about them.
 */
export function articleFor(candidateId: string, locale: Locale): { url: string; lang: Locale } | null {
  const articles = ARTICLES[candidateId]
  if (!articles) return null
  const local = articles[locale]
  return local ? { url: local, lang: locale } : { url: articles.fr, lang: "fr" }
}
