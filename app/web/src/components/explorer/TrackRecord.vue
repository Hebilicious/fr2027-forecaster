<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from "vue"
import { useIntlayer, useLocale } from "vue-intlayer"
import type { TrackRecord } from "../../api/schemas"
import { scale } from "../../charts"
import { trackRecordView } from "../../explorer"
import { count, list, percent, probability } from "../../format"

const props = defineProps<{ record: TrackRecord }>()
const content = useIntlayer("track-record")
const common = useIntlayer("common")
const { locale } = useLocale()

const view = computed(() => trackRecordView(props.record))
const TARGET = 0.8

const help = computed(() => content.help({ elections: list(props.record.elections.map(String), locale.value) }))

// Both charts are drawn at their column's width up to 360 units, so their text keeps its size on a phone.
const chart = ref<HTMLElement | null>(null)
const chartWidth = ref(360)
const resize = new ResizeObserver(([entry]) => {
  if (entry && entry.contentRect.width > 0) chartWidth.value = entry.contentRect.width
})
onMounted(() => chart.value && resize.observe(chart.value))
onBeforeUnmount(() => resize.disconnect())
const chartUnits = computed(() => Math.round(Math.min(360, chartWidth.value)))

// Coverage: one bar per horizon, furthest from the vote on the left, against the 80% target.
const cw = chartUnits
const ch = 250
const cm = { top: 24, right: 12, bottom: 34, left: 40 }
const coverageY = (v: number) => scale(v, { domain: [0, 1], range: [ch - cm.bottom, cm.top] })
const slot = computed(() => (cw.value - cm.left - cm.right) / Math.max(1, view.value.horizons.length))
const barX = (i: number) => cm.left + i * slot.value + slot.value * 0.22
const barW = computed(() => slot.value * 0.56)
const coverageTicks = [0, 0.2, 0.4, 0.6, 0.8, 1]
// Replays monthly, then weekly, give some twenty bars, too narrow for a value and a label each: then
// only the furthest, the middle and the nearest are labelled, and a bar's value shows when pointed at.
const crowded = computed(() => slot.value < 30)
const labelAt = (i: number) => {
  const last = view.value.horizons.length - 1
  if (!crowded.value) return { x: barX(i) + barW.value / 2, anchor: "middle" }
  if (i === 0) return { x: cm.left, anchor: "start" }
  if (i === last) return { x: cw.value - cm.right, anchor: "end" }
  // A phone-wide chart has no room for the middle label between the two ends.
  if (i === Math.floor(last / 2) && cw.value >= 320) return { x: barX(i) + barW.value / 2, anchor: "middle" }
  return null
}

// Calibration: announced probability against how often it happened, dot area by number of forecasts.
const sw = chartUnits
const sh = 250
const sm = { top: 14, right: 14, bottom: 40, left: 60 }
const sx = (v: number) => scale(v, { domain: [0, 1], range: [sm.left, sw.value - sm.right] })
const sy = (v: number) => scale(v, { domain: [0, 1], range: [sh - sm.bottom, sm.top] })
const maxEvents = computed(() => Math.max(1, ...view.value.bins.map((b) => b.events)))
const radius = (events: number) => 4 + 9 * Math.sqrt(events / maxEvents.value)
const axisTicks = [0, 0.25, 0.5, 0.75, 1]

const pointedBin = ref<number | null>(null)
const pointedBar = ref<number | null>(null)
const band = (lo: number, hi: number) => `${percent(lo, 0, locale.value)}–${percent(hi, 0, locale.value)}`
const binText = (i: number) => {
  const bin = view.value.bins[i]!
  return content.binLabel({
    band: band(bin.lo, bin.hi),
    events: count(bin.events, locale.value),
    forecast: percent(bin.mean_forecast, 1, locale.value),
    observed: percent(bin.observed, 1, locale.value),
  }).value
}
const barText = (i: number) => {
  const h = view.value.horizons[i]!
  return content.coverageLabel({ days: count(h.days, locale.value), value: percent(h.coverage80, 0, locale.value) })
    .value
}
const miss = (value: number) =>
  content.pointsValue({
    value: value.toLocaleString(locale.value, { maximumFractionDigits: 1, minimumFractionDigits: 1 }),
  }).value
</script>

<template>
  <div class="record">
    <div class="intro">
      <h3>{{ content.title }}</h3>
      <p class="secondary small">{{ help }}</p>
    </div>

    <div class="charts">
      <figure ref="chart" class="chart">
        <figcaption>
          <h4>{{ content.coverageTitle }}</h4>
          <p class="secondary small">{{ content.coverageHelp }}</p>
        </figcaption>
        <svg :viewBox="`0 0 ${cw} ${ch}`" role="img" :aria-label="content.coverageTitle.value">
          <g class="grid">
            <g v-for="tick in coverageTicks" :key="tick">
              <line :x1="cm.left" :x2="cw - cm.right" :y1="coverageY(tick)" :y2="coverageY(tick)" />
              <text :x="cm.left - 8" :y="coverageY(tick) + 4" text-anchor="end">{{ percent(tick, 0, locale) }}</text>
            </g>
          </g>
          <g
            v-for="(h, i) in view.horizons"
            :key="h.days"
            class="bar"
            :class="{ pointed: pointedBar === i }"
            tabindex="0"
            :aria-label="barText(i)"
            @pointerenter="pointedBar = i"
            @pointerleave="pointedBar = null"
            @focus="pointedBar = i"
            @blur="pointedBar = null"
          >
            <rect :x="cm.left + i * slot" :y="cm.top" :width="slot" :height="ch - cm.top - cm.bottom" class="hit" />
            <path
              :d="`M${barX(i)},${coverageY(0)}V${coverageY(h.coverage80) + 4}q0,-4 4,-4h${barW - 8}q4,0 4,4V${coverageY(0)}Z`"
              class="fill"
            />
            <text
              v-if="!crowded"
              :x="barX(i) + barW / 2"
              :y="h.coverage80 > 0.12 ? coverageY(h.coverage80) + 20 : coverageY(h.coverage80) - 7"
              text-anchor="middle"
              :class="h.coverage80 > 0.12 ? 'bar-value' : 'bar-value outside'"
            >
              {{ percent(h.coverage80, 0, locale) }}
            </text>
            <text v-if="labelAt(i)" :x="labelAt(i)!.x" :y="ch - 12" class="bar-label" :text-anchor="labelAt(i)!.anchor">
              {{ content.daysOut({ days: count(h.days, locale) }) }}
            </text>
          </g>
          <g class="target" pointer-events="none">
            <line :x1="cm.left" :x2="cw - cm.right" :y1="coverageY(TARGET)" :y2="coverageY(TARGET)" />
            <text :x="cw - cm.right" :y="coverageY(TARGET) - 6" text-anchor="end">
              {{ content.target({ value: percent(TARGET, 0, locale) }) }}
            </text>
          </g>
        </svg>
        <p aria-live="polite" class="readout small secondary">{{ pointedBar === null ? "" : barText(pointedBar) }}</p>
      </figure>

      <figure class="chart">
        <figcaption>
          <h4>{{ content.calibrationTitle }}</h4>
          <p class="secondary small">{{ content.calibrationHelp }}</p>
        </figcaption>
        <svg :viewBox="`0 0 ${sw} ${sh}`" role="img" :aria-label="content.calibrationTitle.value">
          <g class="grid">
            <g v-for="tick in axisTicks" :key="tick">
              <line :x1="sm.left" :x2="sw - sm.right" :y1="sy(tick)" :y2="sy(tick)" />
              <text :x="sm.left - 8" :y="sy(tick) + 4" text-anchor="end">{{ percent(tick, 0, locale) }}</text>
              <text :x="sx(tick)" :y="sh - sm.bottom + 16" text-anchor="middle">{{ percent(tick, 0, locale) }}</text>
            </g>
            <text :x="(sm.left + sw - sm.right) / 2" :y="sh - 4" class="axis-title" text-anchor="middle">
              {{ content.axisForecast }}
            </text>
            <text
              :x="10"
              :y="(sm.top + sh - sm.bottom) / 2"
              class="axis-title"
              text-anchor="middle"
              :transform="`rotate(-90 10 ${(sm.top + sh - sm.bottom) / 2})`"
            >
              {{ content.axisObserved }}
            </text>
          </g>
          <line :x1="sx(0)" :y1="sy(0)" :x2="sx(1)" :y2="sy(1)" class="diagonal" />
          <g
            v-for="(bin, i) in view.bins"
            :key="bin.lo"
            class="bin"
            :class="{ pointed: pointedBin === i }"
            tabindex="0"
            :aria-label="binText(i)"
            @pointerenter="pointedBin = i"
            @pointerleave="pointedBin = null"
            @focus="pointedBin = i"
            @blur="pointedBin = null"
          >
            <circle :cx="sx(bin.mean_forecast)" :cy="sy(bin.observed)" :r="Math.max(12, radius(bin.events))" class="hit" />
            <circle :cx="sx(bin.mean_forecast)" :cy="sy(bin.observed)" :r="radius(bin.events)" class="dot" />
          </g>
        </svg>
        <ul class="legend small" :aria-label="common.legend.value">
          <li><span aria-hidden="true" class="dash" />{{ content.perfect }}</li>
          <li><span aria-hidden="true" class="ring" />{{ content.dotSize }}</li>
        </ul>
        <p aria-live="polite" class="readout small secondary">{{ pointedBin === null ? "" : binText(pointedBin) }}</p>
      </figure>
    </div>

    <details class="data">
      <summary class="small">{{ common.showData }}</summary>
      <div class="tables">
        <div class="table-wrap">
          <table class="small">
            <thead>
              <tr>
                <th scope="col">{{ content.headHorizon }}</th>
                <th scope="col">{{ content.headCoverage }}</th>
                <th scope="col">{{ content.headMiss }}</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="h in view.horizons" :key="h.days">
                <th scope="row">{{ content.daysOut({ days: count(h.days, locale) }) }}</th>
                <td class="num">{{ percent(h.coverage80, 0, locale) }}</td>
                <td class="num">{{ miss(h.share_mae_points) }}</td>
              </tr>
            </tbody>
          </table>
        </div>
        <div class="table-wrap">
          <table class="small">
            <thead>
              <tr>
                <th scope="col">{{ content.headBand }}</th>
                <th scope="col">{{ content.headEvents }}</th>
                <th scope="col">{{ content.axisForecast }}</th>
                <th scope="col">{{ content.axisObserved }}</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="bin in view.bins" :key="bin.lo">
                <th scope="row">{{ band(bin.lo, bin.hi) }}</th>
                <td class="num">{{ count(bin.events, locale) }}</td>
                <td class="num">{{ probability(bin.mean_forecast, locale) }}</td>
                <td class="num">{{ percent(bin.observed, 0, locale) }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </details>
  </div>
</template>

<style scoped>
.record {
  display: grid;
  /* One column as wide as the panel: the data table scrolls inside it instead of widening the page. */
  grid-template-columns: minmax(0, 1fr);
  gap: 18px;
}

h3 {
  font-family: var(--font-sans);
  font-size: 1.05rem;
  font-weight: 600;
  margin: 0 0 6px;
}

h4 {
  font-family: var(--font-sans);
  font-size: 0.95rem;
  font-weight: 600;
  margin: 0 0 4px;
}

.charts {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: clamp(18px, 3vw, 36px);
}

.chart {
  margin: 0;
  display: grid;
  gap: 10px;
  align-content: start;
}

figcaption p {
  max-width: 52ch;
}

svg {
  display: block;
  width: 100%;
  height: auto;
  overflow: visible;
}

.grid line {
  stroke: var(--grid);
  vector-effect: non-scaling-stroke;
}

.grid text {
  fill: var(--muted);
  font-size: 11px;
  font-variant-numeric: tabular-nums;
}

.axis-title {
  fill: var(--ink-2);
  font-size: 11px;
}

.bar .hit,
.bin .hit {
  fill: transparent;
}

.bar .fill {
  fill: var(--series);
  transition: filter 0.15s;
}

.bar.pointed .fill,
.bar:focus-visible .fill {
  filter: brightness(1.3);
}

.bar:focus-visible,
.bin:focus-visible {
  outline: none;
}

.bar-value {
  fill: var(--page);
  font-size: 12px;
  font-weight: 650;
  font-variant-numeric: tabular-nums;
}

.bar-value.outside {
  fill: var(--ink);
}

.bar-label {
  fill: var(--ink-2);
  font-size: 11px;
}

.target line {
  stroke: var(--ink);
  stroke-width: 1.5;
  stroke-dasharray: 5 4;
  vector-effect: non-scaling-stroke;
}

.target text {
  fill: var(--ink);
  font-size: 11px;
  font-weight: 600;
}

.diagonal {
  stroke: var(--axis);
  stroke-width: 1.5;
  stroke-dasharray: 5 4;
  vector-effect: non-scaling-stroke;
}

.legend {
  display: flex;
  flex-wrap: wrap;
  gap: 8px 18px;
  margin: 0;
  padding: 0;
  list-style: none;
  color: var(--ink-2);
}

.legend li {
  display: inline-flex;
  align-items: center;
  gap: 8px;
}

.dash {
  width: 18px;
  border-top: 1.5px dashed var(--axis);
}

.ring {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: var(--series-wash);
  box-shadow: inset 0 0 0 2px var(--series);
}

.bin .dot {
  fill: var(--series-wash);
  stroke: var(--series);
  stroke-width: 2;
  vector-effect: non-scaling-stroke;
  transition: fill 0.15s;
}

.bin.pointed .dot,
.bin:focus-visible .dot {
  fill: var(--series);
  stroke: var(--ink);
}

.readout {
  min-height: 1.3em;
  margin: 0;
}

.data summary {
  cursor: pointer;
  color: var(--ink-2);
}

.tables {
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
  margin-top: 12px;
  align-items: flex-start;
}

@media (max-width: 860px) {
  .charts {
    grid-template-columns: 1fr;
  }
}
</style>
