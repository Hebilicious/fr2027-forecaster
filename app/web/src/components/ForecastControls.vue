<script setup lang="ts">
import { computed } from "vue"
import { useIntlayer, useLocale } from "vue-intlayer"
import type { Blend } from "../api/schemas"
import { percent, relativeTime } from "../format"
import { useNow } from "../useNow"

// The forecast's two settings, side by side: how strongly the news and social signals act (a few
// strengths the forecast was run at, so a pill switch) and how much the prediction markets weigh
// (any share, so a slider). Each says what it does in one line; the details are on the Signals page.
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
const changed = computed(() => scale.value !== 1 || Math.abs(marketWeight.value - props.published) > 1e-9)
function reset() {
  scale.value = 1
  marketWeight.value = props.published
}
</script>

<template>
  <section class="controls" :aria-label="content.label.value">
    <div v-if="scales.length > 1" class="control">
      <div class="head">
        <h3 class="title">{{ content.signalsTitle }}</h3>
      </div>
      <div class="pills" role="radiogroup" :aria-label="content.signalsTitle.value">
        <button
          v-for="s in scales"
          :key="s"
          role="radio"
          type="button"
          :aria-checked="scale === s"
          :class="{ published: s === 1 }"
          @click="scale = s"
        >
          <span v-if="s === 0">{{ content.off }}</span>
          <span v-else translate="no">{{ multiple(s) }}</span>
        </button>
      </div>
      <p class="caption">{{ content.signalsCaption }}</p>
    </div>

    <div v-if="blend" class="control">
      <div class="head">
        <h3 class="title">{{ content.marketsTitle }}</h3>
        <span class="value num">{{ percent(marketWeight, 0, locale) }}</span>
      </div>
      <div class="slider" :style="{ '--fill': `${marketPercent}%`, '--mark': `${published * 100}%` }">
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
        <span class="mark-label small">{{ content.published }}</span>
        <div class="ends small">
          <span>{{ content.modelOnly }}</span>
          <span>{{ content.marketsOnly }}</span>
        </div>
      </div>
      <p class="caption">{{ content.marketsCaption }}</p>
      <p v-if="!blend.market_used && blend.market_fetched_at" class="caption stale" role="status">
        {{ content.stale({ when: relativeTime(blend.market_fetched_at, now, locale) }) }}
      </p>
    </div>

    <button v-if="changed" class="reset" type="button" @click="reset">{{ content.reset }}</button>
  </section>
</template>

<style scoped>
/* An inset panel inside the card: two controls in columns, stacked on a phone. */
.controls {
  position: relative;
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 18px clamp(20px, 3vw, 40px);
  margin: 4px 0 26px;
  padding: 18px 20px 16px;
  border-radius: var(--radius-sm);
  background: rgba(0, 0, 0, 0.22);
  box-shadow: inset 0 0 0 1px var(--border);
}

.control {
  display: grid;
  gap: 10px;
  align-content: start;
}

/* Both columns' headings share one height, whether or not they show a value. */
.head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  min-height: 32px;
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

.caption {
  margin: 0;
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

/* A thin track, filled in bleu up to the thumb, with a tick at the published weight. */
/* Room above the track for the published weight's label. */
.slider {
  position: relative;
  padding-top: 18px;
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

/* Under the input, so the thumb covers it when it sits on the published weight. */
.mark {
  position: absolute;
  z-index: 0;
  top: 23px;
  left: calc(9px + (100% - 18px) * var(--mark) / 100%);
  width: 2px;
  height: 16px;
  margin-left: -1px;
  border-radius: 1px;
  background: var(--ink-2);
  pointer-events: none;
}

.ends {
  position: relative;
  display: flex;
  justify-content: space-between;
  margin-top: 4px;
  color: var(--muted);
}

.mark-label {
  position: absolute;
  top: 0;
  left: calc(9px + (100% - 18px) * var(--mark) / 100%);
  transform: translateX(-50%);
  line-height: 1;
  color: var(--muted);
}

/* Reset closes the panel, on the right, only once something moved. */
.reset {
  grid-column: 1 / -1;
  justify-self: end;
  margin-top: -8px;
  font: inherit;
  font-size: 0.75rem;
  font-weight: 500;
  padding: 4px 12px;
  border: none;
  border-radius: 999px;
  background: rgba(243, 239, 230, 0.08);
  color: var(--ink-2);
  cursor: pointer;
}

.reset:hover {
  background: rgba(243, 239, 230, 0.14);
  color: var(--ink);
}

@media (max-width: 720px) {
  .controls {
    grid-template-columns: 1fr;
    padding: 16px 14px 14px;
  }

  .reset {
    justify-self: start;
    margin-top: 0;
  }
}
</style>
