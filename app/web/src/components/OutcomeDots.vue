<script setup lang="ts">
import { computed } from "vue"
import type { Forecast } from "../api/schemas"
import { outcomeGroups, type OutcomeGroup } from "../charts"
import CandidateAvatar from "./CandidateAvatar.vue"
import { probability } from "../format"
import { useI18n } from "../i18n"

const props = defineProps<{ forecast: Forecast }>()
const { t, locale } = useI18n()

// One dot per percentage point. Every candidate's dots are the same bleu; the pooled tail is grey.
const groups = computed(() => outcomeGroups(props.forecast.candidates))
const label = (g: OutcomeGroup) =>
  g.name === null
    ? t("outcomesOthersGroup", { m: g.members, p: probability(g.probability, locale.value) })
    : t("outcomesGroup", { name: g.name, p: probability(g.probability, locale.value) })
</script>

<template>
  <ul class="outcomes">
    <li
      v-for="(g, i) in groups"
      :key="g.candidateId ?? 'others'"
      class="group"
      :class="{ others: g.candidateId === null, wide: g.dots >= 10 }"
      :style="{ '--i': i }"
    >
      <span class="sr-only">{{ label(g) }}</span>
      <div aria-hidden="true" class="label">
        <CandidateAvatar v-if="g.candidateId && g.name" :id="g.candidateId" :name="g.name" :size="40" />
        <p class="text">
          <span class="value">{{ probability(g.probability, locale) }}</span>
          <span class="name">{{ g.name ?? t("outcomesOthers") }}</span>
          <span v-if="g.candidateId === null" class="members">{{ t("outcomesOthersCount", { n: g.members }) }}</span>
        </p>
      </div>
      <span aria-hidden="true" class="dots">
        <span v-for="n in g.dots" :key="n" class="dot" :style="{ '--n': n }" />
      </span>
    </li>
  </ul>
</template>

<style scoped>
.outcomes {
  list-style: none;
  margin: 8px 0 28px;
  padding: 0;
  display: flex;
  flex-wrap: wrap;
  align-items: flex-end;
  gap: 28px clamp(20px, 3vw, 40px);
}

.group {
  display: grid;
  gap: 12px;
  justify-items: start;
  max-width: 14rem;
}

.group.wide {
  max-width: none;
}

.label {
  display: flex;
  align-items: center;
  gap: 12px;
}

.text {
  display: grid;
  gap: 2px;
}

.value {
  font-size: clamp(1.6rem, 1.3rem + 1vw, 2.15rem);
  font-weight: 600;
  letter-spacing: -0.02em;
  line-height: 1;
  color: var(--ink);
}

.name {
  font-size: 0.86rem;
  font-weight: 500;
  color: var(--ink-2);
  line-height: 1.3;
}

.members {
  font-size: 0.78rem;
  color: var(--muted);
}

/* Ten to a row, filled from the bottom up so each group stands on the same baseline. */
.dots {
  --size: 10px;
  --gap: 4px;
  display: flex;
  flex-wrap: wrap-reverse;
  gap: var(--gap);
  max-width: calc(10 * var(--size) + 9 * var(--gap));
}

.dot {
  width: var(--size);
  height: var(--size);
  border-radius: 50%;
  background: radial-gradient(circle at 34% 30%, rgba(255, 255, 255, 0.55), rgba(255, 255, 255, 0) 46%), var(--series);
  box-shadow: 0 0 8px var(--series-glow);
  animation: appear 0.6s var(--ease-out) both;
  animation-delay: calc(var(--i) * 90ms + var(--n) * 7ms + 150ms);
  transition:
    transform 0.25s var(--ease-out),
    filter 0.25s;
}

.others .dot {
  background: radial-gradient(circle at 34% 30%, rgba(255, 255, 255, 0.4), rgba(255, 255, 255, 0) 46%), var(--dot);
  box-shadow: none;
}

.group:hover .dot {
  filter: brightness(1.25);
}

@keyframes appear {
  from {
    opacity: 0;
    transform: scale(0.2);
  }
}

@media (max-width: 520px) {
  .dots {
    --size: 9px;
    --gap: 3px;
  }

  .outcomes {
    gap: 22px 22px;
  }
}
</style>
