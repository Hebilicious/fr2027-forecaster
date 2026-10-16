import { computed, ref, type Ref } from "vue"
import type { CandidateForecast } from "./api/schemas"
import { filterField, type FieldView } from "./field"

// One search and one toggle for the whole site, so a reader's choice follows them from view to view.
const query = ref("")
const showAll = ref(false)

export function useFieldFilter(candidates: Ref<readonly CandidateForecast[]>) {
  const field = computed<FieldView>(() => filterField(candidates.value, { query: query.value, showAll: showAll.value }))
  return { query, showAll, field }
}
