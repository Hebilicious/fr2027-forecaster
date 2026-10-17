<script setup lang="ts">
import { computed } from "vue"
import { useIntlayer, useLocale } from "vue-intlayer"
import type { Forecast } from "../api/schemas"
import { regroup, type OutcomeGroup } from "../charts"
import CandidateAvatar from "./CandidateAvatar.vue"
import { probability } from "../format"

// `rows` are the groups at the published setting. Each keeps its row whatever the reader's settings,
// so moving them only fills and empties dots.
const props = defineProps<{ forecast: Forecast; rows: readonly OutcomeGroup[] }>()
const content = useIntlayer("outcome-dots")
const { locale } = useLocale()

const groups = computed(() => regroup(props.rows, props.forecast.candidates))
const label = (g: OutcomeGroup) =>
  g.name === null
    ? content.othersGroup({ m: g.members, p: probability(g.probability, locale.value) })
    : content.group({ name: g.name, p: probability(g.probability, locale.value) })
</script>

<template>
  <ol class="outcomes">
    <li
      v-for="(g, i) in groups"
      :key="g.candidateId ?? 'others'"
      class="row"
      :class="{ others: g.candidateId === null }"
      :style="{ '--i': i }"
    >
      <span class="sr-only">{{ label(g) }}</span>
      <span aria-hidden="true" class="who">
        <CandidateAvatar v-if="g.candidateId && g.name" :id="g.candidateId" :name="g.name" :size="36" />
        <span v-else class="pool-mark" />
        <span class="text">
          <span class="name">{{ g.name ?? content.others }}</span>
          <span v-if="g.candidateId === null" class="members">{{ content.othersCount({ n: g.members }) }}</span>
        </span>
      </span>
      <span aria-hidden="true" class="value">{{ probability(g.probability, locale) }}</span>
      <!-- A hundred places, one per percentage point, filled column by column: the filled run is as long
           as the chance. -->
      <span aria-hidden="true" class="dots">
        <span v-for="n in 100" :key="n" class="dot" :class="{ on: n <= g.dots }" />
      </span>
    </li>
  </ol>
</template>

<style scoped>
/* One grid for every row, so names, figures and dots line up down the chart. */
.outcomes {
  list-style: none;
  margin: 4px 0 24px;
  padding: 0;
  display: grid;
  grid-template-columns: max-content max-content minmax(0, 1fr);
  column-gap: clamp(16px, 2.2vw, 28px);
}

.row {
  display: grid;
  grid-column: 1 / -1;
  grid-template-columns: subgrid;
  grid-template-areas: "who value dots";
  align-items: center;
  padding: 12px 0;
  border-top: 1px solid var(--grid);
}

.row:first-child {
  border-top: none;
}

.who {
  grid-area: who;
  display: flex;
  align-items: center;
  gap: 12px;
}

.pool-mark {
  flex: none;
  width: 36px;
  height: 36px;
  border-radius: 50%;
  border: 1px dashed var(--axis);
}

.text {
  display: grid;
  gap: 1px;
}

.name {
  font-size: 0.95rem;
  font-weight: 600;
  color: var(--ink);
  line-height: 1.25;
}

.members {
  font-size: 0.78rem;
  color: var(--muted);
}

/* Wide enough for "100 %" in any locale, so a changing figure never moves the dots. */
.value {
  grid-area: value;
  min-width: 3.3em;
  text-align: right;
  font-variant-numeric: tabular-nums;
  font-size: clamp(1.2rem, 1rem + 0.6vw, 1.55rem);
  font-weight: 600;
  letter-spacing: -0.02em;
  line-height: 1;
  color: var(--ink);
}

.others .value {
  color: var(--ink-2);
}

.dots {
  --gap: 3px;
  grid-area: dots;
  display: grid;
  grid-template-columns: repeat(50, minmax(0, 1fr));
  grid-template-rows: repeat(2, auto);
  grid-auto-flow: column;
  gap: var(--gap);
  max-width: calc(50 * 16px);
  animation: sweep 1s var(--ease-out) both;
  animation-delay: calc(var(--i) * 70ms + 150ms);
}

.dot {
  aspect-ratio: 1;
  border-radius: 50%;
  background-color: rgba(243, 239, 230, 0.075);
  transition:
    background-color 0.3s var(--ease-out),
    box-shadow 0.3s var(--ease-out),
    filter 0.25s;
}

.dot.on {
  background-color: var(--series);
  background-image: radial-gradient(circle at 34% 30%, rgba(255, 255, 255, 0.55), rgba(255, 255, 255, 0) 46%);
  box-shadow: 0 0 7px var(--series-glow);
}

.others .dot.on {
  background-color: var(--dot);
  background-image: radial-gradient(circle at 34% 30%, rgba(255, 255, 255, 0.4), rgba(255, 255, 255, 0) 46%);
  box-shadow: none;
}

.row:hover .dot.on {
  filter: brightness(1.25);
}

@keyframes sweep {
  from {
    clip-path: inset(0 100% 0 0);
  }

  to {
    clip-path: inset(0 0 0 0);
  }
}

/* On a phone the dots go under the name, four rows of twenty-five. */
@media (max-width: 640px) {
  .outcomes {
    grid-template-columns: minmax(0, 1fr) max-content;
  }

  .row {
    grid-template-areas:
      "who value"
      "dots dots";
    row-gap: 10px;
  }

  .dots {
    grid-template-columns: repeat(25, minmax(0, 1fr));
    grid-template-rows: repeat(4, auto);
    max-width: none;
  }
}
</style>
