<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from "vue"
import { useIntlayer, useLocale } from "vue-intlayer"
import type { OutcomeGroup } from "../charts"
import CandidateAvatar from "./CandidateAvatar.vue"
import { probability } from "../format"

// The groups at the reader's settings, largest first. When the settings change, rows glide to their
// new place, fade in or out, and the chart's height eases to fit them.
defineProps<{ groups: readonly OutcomeGroup[] }>()
const content = useIntlayer("outcome-dots")
const { locale } = useLocale()

const label = (g: OutcomeGroup) =>
  g.name === null
    ? content.othersGroup({ m: g.members, p: probability(g.probability, locale.value) })
    : content.group({ name: g.name, p: probability(g.probability, locale.value) })

// The list's height, followed by the frame around it with a transition, so the page below slides
// when a row comes or goes instead of jumping.
const list = ref<HTMLElement>()
const height = ref<number | null>(null)
let observer: ResizeObserver | undefined
onMounted(() => {
  observer = new ResizeObserver(() => {
    height.value = list.value?.offsetHeight ?? null
  })
  if (list.value) observer.observe(list.value)
})
onBeforeUnmount(() => observer?.disconnect())
</script>

<template>
  <div class="frame" :style="height === null ? undefined : { height: `${height}px` }">
    <div ref="list" class="list">
      <TransitionGroup class="outcomes" name="row" tag="ol">
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
          <!-- A hundred places, one per percentage point, filled column by column: the filled run is as
               long as the chance. -->
          <span aria-hidden="true" class="dots">
            <span v-for="n in 100" :key="n" class="dot" :class="{ on: n <= g.dots }" />
          </span>
        </li>
      </TransitionGroup>
    </div>
  </div>
</template>

<style scoped>
/* Clipped top and bottom only, so a row fading out never overlaps what follows while the height
   eases, and the dots' glow still shows at the sides. */
.frame {
  margin: 4px 0 24px;
  overflow-x: visible;
  overflow-y: clip;
  transition: height 0.45s var(--ease-out);
}

.list {
  display: flow-root;
}

.outcomes {
  position: relative;
  list-style: none;
  margin: 0;
  padding: 0;
  display: grid;
}

/* Every row has the same columns, wide enough for the longest name, so names, figures and dots line
   up down the chart whoever is in it. */
.row {
  display: grid;
  grid-template-columns: 15rem 5.4rem minmax(0, 1fr);
  grid-template-areas: "who value dots";
  column-gap: clamp(16px, 2.2vw, 28px);
  align-items: center;
  padding: 12px 0;
  border-top: 1px solid var(--grid);
}

.row:first-child {
  border-top-color: transparent;
}

.row-move {
  transition: transform 0.45s var(--ease-out);
}

.row-enter-active {
  transition: opacity 0.35s var(--ease-out) 0.1s;
}

.row-leave-active {
  position: absolute;
  left: 0;
  right: 0;
  transition: opacity 0.2s;
}

.row-enter-from,
.row-leave-to {
  opacity: 0;
}

.who {
  grid-area: who;
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 0;
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

.value {
  grid-area: value;
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

/* Once the reader has moved a setting, the dots appear as they are: a row moved in the list would
   otherwise sweep in again. */
.settled .dots {
  animation: none;
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
  .row {
    grid-template-columns: minmax(0, 1fr) auto;
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
