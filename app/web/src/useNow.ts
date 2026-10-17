import { onBeforeUnmount, onMounted, ref } from "vue"

// The reader's clock, in milliseconds, ticking every minute so "3 hours ago" stays true while the page is open.
const now = ref(Date.now())
let users = 0
let timer: ReturnType<typeof setInterval> | undefined

export function useNow() {
  onMounted(() => {
    users += 1
    now.value = Date.now()
    timer ??= setInterval(() => {
      now.value = Date.now()
    }, 60_000)
  })
  onBeforeUnmount(() => {
    users -= 1
    if (users === 0 && timer !== undefined) {
      clearInterval(timer)
      timer = undefined
    }
  })
  return now
}
