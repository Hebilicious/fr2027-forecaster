<script setup lang="ts">
import { computed } from "vue"
import type { Forecast, Signals } from "../api/schemas"
import AttentionSection from "../components/signals/AttentionSection.vue"
import EventsSection from "../components/signals/EventsSection.vue"
import MarketsSection from "../components/signals/MarketsSection.vue"
import NewsSection from "../components/signals/NewsSection.vue"
import XSection from "../components/signals/XSection.vue"
import { useI18n } from "../i18n"
import { candidateNames } from "../signals"

const props = defineProps<{ forecast: Forecast; signals: Signals | null }>()
const { t } = useI18n()

const names = computed(() => candidateNames(props.forecast))
</script>

<template>
  <div class="signals">
    <p v-if="!signals" class="notice" role="status">
      <span aria-hidden="true" class="icon">!</span>
      <span>{{ t("signalsMissing") }}</span>
    </p>
    <template v-else>
      <MarketsSection class="rise" :forecast="forecast" :markets="signals.markets" />
      <div class="pair">
        <AttentionSection class="rise" :style="{ '--i': 1 }" :attention="signals.attention" :names="names" />
        <XSection class="rise" :style="{ '--i': 2 }" :rows="signals.x.rows" :names="names" />
      </div>
      <NewsSection class="rise" :style="{ '--i': 3 }" :news="signals.news" :names="names" />
      <EventsSection class="rise" :style="{ '--i': 4 }" :events="signals.events" :names="names" />
    </template>
  </div>
</template>

<style scoped>
.signals {
  display: grid;
  gap: clamp(20px, 2.4vw, 28px);
}

.pair {
  display: grid;
  gap: clamp(20px, 2.4vw, 28px);
  grid-template-columns: repeat(auto-fit, minmax(min(100%, 480px), 1fr));
  align-items: start;
}
</style>
