import { ref } from "vue"

// How strong the news and social signals are in the forecast the reader sees, as a multiple of the
// published weights (0: the polls alone, 1: published), set with the Overview's slider.
const scale = ref(1)

export function useSignalScale() {
  return scale
}
