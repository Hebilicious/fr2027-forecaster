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
  <section class="stack">
    <div class="stack">
      <h2>{{ t("signalsTitle") }}</h2>
      <p class="secondary small">{{ t("signalsIntro") }}</p>
    </div>
    <p v-if="!signals" class="notice" role="status">
      <span aria-hidden="true" class="icon">!</span>
      <span>{{ t("signalsMissing") }}</span>
    </p>
    <template v-else>
      <MarketsSection :forecast="forecast" :markets="signals.markets" />
      <AttentionSection :attention="signals.attention" :names="names" />
      <NewsSection :news="signals.news" :names="names" />
      <XSection :rows="signals.x.rows" :names="names" />
      <EventsSection :events="signals.events" :names="names" />
    </template>
  </section>
</template>
