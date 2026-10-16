<script setup lang="ts">
import { computed, ref, toRef } from "vue"
import { useIntlayer } from "vue-intlayer"
import type { Forecast, Poll, SeriesPoint } from "../api/schemas"
import { round1Panels } from "../charts"
import FieldFilter from "../components/FieldFilter.vue"
import SharePanel from "../components/SharePanel.vue"
import { useFieldFilter } from "../useFieldFilter"

const props = defineProps<{
  forecast: Forecast
  series: readonly SeriesPoint[]
  polls: readonly Poll[]
}>()
const content = useIntlayer("round1-view")

const { query, showAll, field } = useFieldFilter(toRef(() => props.forecast.candidates))
const allPanels = computed(() => round1Panels(props.forecast, props.series, props.polls))
const panels = computed(() => allPanels.value.filter((p) => !field.value.hidden.has(p.candidate.candidate_id)))
const firms = computed(() => [...new Set(allPanels.value.flatMap((p) => p.polls.map((d) => d.firm)))].toSorted())
const highlight = ref<string | null>(null)

const xDomain = computed<readonly [number, number]>(() => {
  const dates = allPanels.value.flatMap((p) => [...p.points.map((q) => q.date), ...p.polls.map((d) => d.date)])
  const times = dates.map((d) => Date.parse(`${d}T00:00:00Z`))
  const end = Date.parse(`${props.forecast.as_of}T00:00:00Z`)
  return [Math.min(end, ...times), end]
})
const yMax = computed(() =>
  Math.max(0.1, ...allPanels.value.flatMap((p) => [...p.points.map((q) => q.hi80), ...p.polls.map((d) => d.share)])),
)
</script>

<template>
  <section class="round1">
    <div class="card toolbar rise">
      <FieldFilter
        v-model:query="query"
        v-model:show-all="showAll"
        :field="field"
        :shown="panels.length"
        :total="allPanels.length"
      />
      <div v-if="firms.length" class="chips" role="group" :aria-label="content.highlightFirm.value">
        <span class="eyebrow">{{ content.highlightFirm }}</span>
        <button class="chip" type="button" :aria-pressed="highlight === null" @click="highlight = null">
          {{ content.allFirms }}
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
      <p aria-hidden="true" class="legend small">
        <span class="key"><span class="line-key" />{{ content.legend.average }}</span>
        <span class="key"><span class="band-key" />{{ content.legend.band }}</span>
        <span class="key"><span class="dot-key" />{{ content.legend.polls }}</span>
        <span v-if="highlight" class="key"><span class="dot-key picked" />{{ highlight }}</span>
      </p>
    </div>
    <div class="panels">
      <SharePanel
        v-for="(panel, i) in panels"
        :key="panel.candidate.candidate_id"
        class="rise"
        :style="{ '--i': Math.min(i, 8) + 1 }"
        :panel="panel"
        :x-domain="xDomain"
        :y-max="yMax"
        :highlight-firm="highlight"
      />
    </div>
  </section>
</template>

<style scoped>
.round1 {
  display: grid;
  gap: clamp(20px, 2.4vw, 28px);
}

.toolbar {
  display: grid;
  gap: 16px;
  padding: 18px 22px;
}

.toolbar .eyebrow {
  margin-right: 6px;
}

.legend {
  display: flex;
  flex-wrap: wrap;
  gap: 6px 22px;
  color: var(--muted);
}

.key {
  display: inline-flex;
  align-items: center;
  gap: 8px;
}

.line-key {
  width: 18px;
  height: 2px;
  border-radius: 2px;
  background: var(--series);
}

.band-key {
  width: 18px;
  height: 10px;
  border-radius: 3px;
  background: var(--series-wash);
  box-shadow: inset 0 0 0 1px rgba(88, 124, 241, 0.35);
}

.dot-key {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--dot);
}

.dot-key.picked {
  background: var(--accent);
}

.panels {
  display: grid;
  gap: clamp(14px, 1.6vw, 20px);
  grid-template-columns: repeat(auto-fill, minmax(min(100%, 340px), 1fr));
}
</style>
