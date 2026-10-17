<script setup lang="ts">
import { computed } from "vue"
import { useIntlayer, useLocale } from "vue-intlayer"
import type { Blend } from "../api/schemas"
import { percent, relativeTime } from "../format"
import { useNow } from "../useNow"

// The forecast's two settings, side by side: how strongly the news and social signals act (a few
// strengths the forecast was run at, so a pill switch) and how much the prediction markets weigh
// (any share, so a slider). Both columns share one grid, so their rows line up; each marks its
// published setting, which is also the way back.
const props = defineProps<{ scales: readonly number[]; blend: Blend | null; published: number }>()
const scale = defineModel<number>("scale", { required: true })
const marketWeight = defineModel<number>("marketWeight", { required: true })
const content = useIntlayer("forecast-controls")
const { locale } = useLocale()
const now = useNow()

// Multiples of the published weights, as notation: "½×", "1×", "1½×".
const FRACTIONS: Readonly<Record<number, string>> = { 0.5: "½×", 1: "1×", 1.5: "1½×", 2: "2×" }
const multiple = (s: number) => FRACTIONS[s] ?? `${s}×`
const marketPercent = computed({
  get: () => Math.round(marketWeight.value * 100),
  set: (value: number) => {
    marketWeight.value = value / 100
  },
})
const atPublished = computed(() => Math.abs(marketWeight.value - props.published) < 1e-9)
</script>

<template>
  <section class="controls" :class="{ single: !blend || scales.length < 2 }" :aria-label="content.label.value">
    <template v-if="scales.length > 1">
      <div class="head s-head">
        <h3 class="title">{{ content.signalsTitle }}</h3>
      </div>
      <div class="pills s-ctrl" role="radiogroup" :aria-label="content.signalsTitle.value">
        <button
          v-for="s in scales"
          :key="s"
          role="radio"
          type="button"
          :aria-checked="scale === s"
          :class="{ published: s === 1 }"
          :title="s === 1 ? content.backToPublished.value : undefined"
          @click="scale = s"
        >
          <span v-if="s === 0">{{ content.off }}</span>
          <span v-else translate="no">{{ multiple(s) }}</span>
        </button>
      </div>
      <div class="ends s-ends small">
        <span>{{ content.pollsOnly }}</span>
        <span>{{ content.twice }}</span>
      </div>
      <p class="caption s-cap">{{ content.signalsCaption }}</p>
    </template>

    <template v-if="blend">
      <div class="head m-head">
        <h3 class="title">{{ content.marketsTitle }}</h3>
        <span class="value num">{{ percent(marketWeight, 0, locale) }}</span>
      </div>
      <div class="slider m-ctrl" :style="{ '--fill': `${marketPercent}%`, '--mark': `${published * 100}%` }">
        <button
          class="mark-label small"
          :class="{ away: !atPublished }"
          type="button"
          :title="content.backToPublished.value"
          @click="marketWeight = published"
        >
          {{ content.published }}
        </button>
        <span aria-hidden="true" class="mark" />
        <input
          v-model.number="marketPercent"
          max="100"
          min="0"
          step="5"
          type="range"
          :aria-label="content.marketsTitle.value"
          :aria-valuetext="percent(marketWeight, 0, locale)"
        />
      </div>
      <div class="ends m-ends small">
        <span>{{ content.modelOnly }}</span>
        <span>{{ content.marketsOnly }}</span>
      </div>
      <p class="caption m-cap">{{ content.marketsCaption }}</p>
      <p v-if="!blend.market_used && blend.market_fetched_at" class="caption stale m-stale" role="status">
        {{ content.stale({ when: relativeTime(blend.market_fetched_at, now, locale) }) }}
      </p>
    </template>
  </section>
</template>

<style scoped>
/* An inset panel inside the card. One grid for both controls: head, control, end labels, caption,
   each row shared by the two columns so they line up whatever each holds. */
.controls {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  grid-template-areas: "s-head m-head"
                       "s-ctrl m-ctrl"
                       "s-ends m-ends"
                       "s-cap m-cap"
                       ". m-stale";
  align-items: center;
  column-gap: clamp(24px, 4vw, 56px);
  row-gap: 8px;
  margin: 4px 0 26px;
  padding: 16px 20px 18px;
  border-radius: var(--radius-sm);
  background: rgba(0, 0, 0, 0.22);
  box-shadow: inset 0 0 0 1px var(--border);
}

.controls.single {
  grid-template-columns: minmax(0, 1fr);
  grid-template-areas: "s-head"
                       "s-ctrl"
                       "s-ends"
                       "s-cap"
                       "m-head"
                       "m-ctrl"
                       "m-ends"
                       "m-cap"
                       "m-stale";
}

.s-head {
  grid-area: s-head;
}

.s-ctrl {
  grid-area: s-ctrl;
}

.s-ends {
  grid-area: s-ends;
}

.s-cap {
  grid-area: s-cap;
}

.m-head {
  grid-area: m-head;
}

.m-ctrl {
  grid-area: m-ctrl;
}

.m-ends {
  grid-area: m-ends;
}

.m-cap {
  grid-area: m-cap;
}

.m-stale {
  grid-area: m-stale;
}

.head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  min-height: 30px;
}

.title {
  margin: 0;
  font-family: var(--font-sans);
  font-size: 0.72rem;
  font-weight: 600;
  letter-spacing: 0.16em;
  text-transform: uppercase;
  color: var(--ink-2);
}

.value {
  font-size: 1.35rem;
  font-weight: 600;
  font-variant-numeric: tabular-nums;
  color: var(--ink);
}

.ends {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  color: var(--muted);
}

.caption {
  align-self: start;
  margin: 4px 0 0;
  font-size: 0.82rem;
  line-height: 1.45;
  color: var(--muted);
}

.stale {
  color: var(--warning);
}

/* The strengths as one pill switch, the site's segmented control. */
.pills {
  display: grid;
  grid-auto-flow: column;
  grid-auto-columns: 1fr;
  padding: 4px;
  gap: 2px;
  border-radius: 999px;
  background: rgba(0, 0, 0, 0.3);
  box-shadow: inset 0 0 0 1px var(--border);
}

.pills button {
  position: relative;
  font: inherit;
  font-size: 0.85rem;
  font-weight: 500;
  font-variant-numeric: tabular-nums;
  padding: 7px 6px;
  border: none;
  border-radius: 999px;
  background: transparent;
  color: var(--ink-2);
  cursor: pointer;
  transition: background-color 0.2s, color 0.2s;
}

.pills button:hover {
  color: var(--ink);
}

/* The published strength keeps a small dot, so the way back is visible. */
.pills button.published::after {
  content: "";
  position: absolute;
  left: 50%;
  bottom: 3px;
  width: 4px;
  height: 4px;
  margin-left: -2px;
  border-radius: 50%;
  background: var(--series);
}

.pills button[aria-checked="true"] {
  background: var(--ink);
  color: var(--page);
  box-shadow: 0 4px 14px -4px rgba(243, 239, 230, 0.35);
}

/* A thin track, filled in bleu up to the thumb, with a tick at the published weight and its label
   above; the label takes the slider back there. The block is as tall as the pill switch beside it. */
.slider {
  position: relative;
  height: 42px;
  display: flex;
  align-items: flex-end;
}

.slider input[type="range"] {
  position: relative;
  z-index: 1;
  -webkit-appearance: none;
  appearance: none;
  display: block;
  width: 100%;
  height: 22px;
  margin: 0;
  background: transparent;
  cursor: pointer;
}

.slider input[type="range"]::-webkit-slider-runnable-track {
  height: 4px;
  border-radius: 999px;
  background: linear-gradient(90deg, var(--series) 0 var(--fill), rgba(243, 239, 230, 0.14) var(--fill) 100%);
}

.slider input[type="range"]::-moz-range-track {
  height: 4px;
  border-radius: 999px;
  background: rgba(243, 239, 230, 0.14);
}

.slider input[type="range"]::-moz-range-progress {
  height: 4px;
  border-radius: 999px;
  background: var(--series);
}

.slider input[type="range"]::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 18px;
  height: 18px;
  margin-top: -7px;
  border-radius: 50%;
  background: var(--ink);
  box-shadow: 0 0 0 4px var(--series-wash),
    0 2px 8px rgba(0, 0, 0, 0.5);
  transition: box-shadow 0.2s;
}

.slider input[type="range"]::-moz-range-thumb {
  width: 18px;
  height: 18px;
  border: none;
  border-radius: 50%;
  background: var(--ink);
  box-shadow: 0 0 0 4px var(--series-wash),
    0 2px 8px rgba(0, 0, 0, 0.5);
}

.slider input[type="range"]:hover::-webkit-slider-thumb, .slider input[type="range"]:focus-visible::-webkit-slider-thumb {
  box-shadow: 0 0 0 6px var(--series-glow),
    0 2px 8px rgba(0, 0, 0, 0.5);
}

.slider input[type="range"]:focus-visible {
  outline: none;
}

/* The tick sits under the input, so the thumb covers it when it rests on the published weight. */
.mark {
  position: absolute;
  z-index: 0;
  bottom: 3px;
  left: calc(9px + (100% - 18px) * var(--mark) / 100%);
  width: 2px;
  height: 16px;
  margin-left: -1px;
  border-radius: 1px;
  background: var(--ink-2);
  pointer-events: none;
}

.mark-label {
  position: absolute;
  top: 0;
  left: calc(9px + (100% - 18px) * var(--mark) / 100%);
  transform: translateX(-50%);
  padding: 0 6px;
  border: none;
  border-radius: 999px;
  background: transparent;
  font: inherit;
  line-height: 1.4;
  color: var(--muted);
  cursor: pointer;
  transition: background-color 0.2s, color 0.2s;
}

/* Away from the published weight, the label reads as the way back. */
.mark-label.away {
  color: var(--ink-2);
  background: rgba(243, 239, 230, 0.08);
}

.mark-label:hover {
  color: var(--ink);
  background: rgba(243, 239, 230, 0.14);
}

@media (max-width: 720px) {
  .controls {
    grid-template-columns: minmax(0, 1fr);
    grid-template-areas: "s-head" "s-ctrl" "s-ends" "s-cap" "m-head" "m-ctrl" "m-ends" "m-cap" "m-stale";
    padding: 14px 14px 16px;
  }

  .m-head {
    margin-top: 14px;
  }
}
</style>
