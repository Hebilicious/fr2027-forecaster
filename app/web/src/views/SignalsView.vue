<script setup lang="ts">
import { computed, toRef } from "vue"
import { useIntlayer } from "vue-intlayer"
import type { Blend, Forecast, Signals } from "../api/schemas"
import FieldFilter from "../components/FieldFilter.vue"
import AttentionSection from "../components/signals/AttentionSection.vue"
import EventsSection from "../components/signals/EventsSection.vue"
import MarketsSection from "../components/signals/MarketsSection.vue"
import ModelUseSection from "../components/signals/ModelUseSection.vue"
import NewsSection from "../components/signals/NewsSection.vue"
import XSection from "../components/signals/XSection.vue"
import { candidateNames, filterSignals } from "../signals"
import { useFieldFilter } from "../useFieldFilter"

const props = defineProps<{ forecast: Forecast; signals: Signals | null; blend: Blend | null }>()
const content = useIntlayer("signals-view")

const names = computed(() => candidateNames(props.forecast))
const { query, showAll, field } = useFieldFilter(toRef(() => props.forecast.candidates))
// Every section reads the same filtered data; the markets table lists the model's side from the visible field.
const shown = computed(() => (props.signals ? filterSignals(props.signals, field.value.hidden) : null))
const visibleForecast = computed(() => ({ ...props.forecast, candidates: field.value.visible }))
</script>

<template>
  <div class="signals">
    <p v-if="!signals" class="notice" role="status">
      <span aria-hidden="true" class="icon">!</span>
      <span>{{ content.missing }}</span>
    </p>
    <template v-else-if="shown">
      <FieldFilter
        v-model:query="query"
        v-model:show-all="showAll"
        class="card filter rise"
        :field="field"
        :total="forecast.candidates.length"
      />
      <MarketsSection class="rise" :forecast="visibleForecast" :markets="shown.markets" />
      <XSection class="rise" :style="{ '--i': 1 }" :rows="shown.x.rows" :names="names" />
      <NewsSection class="rise" :style="{ '--i': 2 }" :news="shown.news" :names="names" />
      <div class="pair">
        <AttentionSection class="rise" :style="{ '--i': 3 }" :attention="shown.attention" :names="names" />
        <EventsSection class="rise" :style="{ '--i': 4 }" :events="shown.events" :names="names" />
      </div>
      <ModelUseSection
        class="rise"
        :style="{ '--i': 5 }"
        :forecast="forecast"
        :blend="blend"
        :forward="shown.forward"
        :names="names"
      />
    </template>
  </div>
</template>

<style scoped>
.signals {
  display: grid;
  gap: clamp(20px, 2.4vw, 28px);
}

.filter {
  padding: 18px 22px;
}

.pair {
  display: grid;
  gap: clamp(20px, 2.4vw, 28px);
  grid-template-columns: repeat(auto-fit, minmax(min(100%, 480px), 1fr));
  align-items: start;
}
</style>
