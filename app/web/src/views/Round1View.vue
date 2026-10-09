<script setup lang="ts">
import { computed, ref } from "vue"
import type { Forecast, PollRow, SeriesPoint } from "../api/schemas"
import { round1Panels } from "../charts"
import SharePanel from "../components/SharePanel.vue"
import { useI18n } from "../i18n"

const props = defineProps<{
  forecast: Forecast
  series: readonly SeriesPoint[]
  polls: readonly PollRow[]
}>()
const { t } = useI18n()

const panels = computed(() => round1Panels(props.forecast, props.series, props.polls))
const firms = computed(() => [...new Set(panels.value.flatMap((p) => p.polls.map((d) => d.firm)))].toSorted())
const highlight = ref<string | null>(null)

const xDomain = computed<readonly [number, number]>(() => {
  const dates = panels.value.flatMap((p) => [...p.points.map((q) => q.date), ...p.polls.map((d) => d.date)])
  const times = dates.map((d) => Date.parse(`${d}T00:00:00Z`))
  const end = Date.parse(`${props.forecast.as_of}T00:00:00Z`)
  return [Math.min(end, ...times), end]
})
const yMax = computed(() =>
  Math.max(0.1, ...panels.value.flatMap((p) => [...p.points.map((q) => q.hi80), ...p.polls.map((d) => d.share)])),
)
</script>

<template>
  <section class="stack">
    <div class="stack">
      <h2>{{ t("round1Title") }}</h2>
      <p class="secondary small">{{ t("round1Help") }}</p>
    </div>
    <div v-if="firms.length" class="chips" role="group" :aria-label="t('highlightFirm')">
      <span class="small secondary">{{ t("highlightFirm") }}</span>
      <button class="chip" type="button" :aria-pressed="highlight === null" @click="highlight = null">
        {{ t("allFirms") }}
      </button>
      <button
        v-for="firm in firms"
        :key="firm"
        class="chip"
        type="button"
        :aria-pressed="highlight === firm"
        @click="highlight = highlight === firm ? null : firm"
      >
        {{ firm }}
      </button>
    </div>
    <div class="panels">
      <SharePanel
        v-for="panel in panels"
        :key="panel.candidate.candidate_id"
        :panel="panel"
        :x-domain="xDomain"
        :y-max="yMax"
        :highlight-firm="highlight"
      />
    </div>
  </section>
</template>

<style scoped>
.panels {
  display: grid;
  gap: 12px;
  grid-template-columns: repeat(auto-fill, minmax(min(100%, 300px), 1fr));
}
</style>
