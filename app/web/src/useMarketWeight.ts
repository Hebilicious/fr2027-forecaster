import { ref } from "vue"

// The prediction markets' weight in the forecast the reader sees (0 to 1), set with the Overview's
// slider. Null until the reader moves it: the published weight from api/blend.json applies.
const chosen = ref<number | null>(null)

export function useMarketWeight() {
  return chosen
}
