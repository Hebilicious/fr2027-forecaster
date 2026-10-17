<script setup lang="ts">
import { computed, ref } from "vue"
import { useIntlayer, useLocale } from "vue-intlayer"
import type { Forecast, HistoryRun } from "../../api/schemas"
import { scale } from "../../charts"
import { chanceLines, type ChanceLine } from "../../explorer"
import { count, dateTime, list, longDate, percent, probability, shortDate } from "../../format"

const props = defineProps<{ forecast: Forecast; history: readonly HistoryRun[] }>()
const content = useIntlayer("chances-over-time")
const common = useIntlayer("common")
const { locale } = useLocale()

const width = 760
const height = 300
const margin = { top: 14, right: 168, bottom: 30, left: 44 }
const xRange: [number, number] = [margin.left, width - margin.right]
const yRange: [number, number] = [height - margin.bottom, margin.top]

const lines = computed(() => chanceLines(props.history, props.forecast))
// Zoomed, the leader leaves the chart so the others' movements show at their own scale.
const zoomed = ref(false)
const leader = computed(() => lines.value[0])
const shown = computed(() => (zoomed.value ? lines.value.slice(1) : lines.value))
const times = computed(() =>
  [...new Set(lines.value.flatMap((l) => l.points.map((p) => p.time)))].toSorted((a, b) => a - b),
)
const enough = computed(() => times.value.length >= 2)

const xDomain = computed<[number, number]>(() => [times.value[0] ?? 0, times.value.at(-1) ?? 1])
// The axis starts at 0 and stops at the first round step above the highest line shown, in at most five steps.
const STEPS = [0.01, 0.02, 0.05, 0.1, 0.2, 0.25]
const yStep = computed(() => {
  const highest = Math.max(0.01, ...shown.value.flatMap((l) => l.points.map((p) => p.p)))
  return STEPS.find((step) => Math.ceil(highest / step) <= 5) ?? 0.25
})
const yTop = computed(() => {
  const highest = Math.max(0.01, ...shown.value.flatMap((l) => l.points.map((p) => p.p)))
  return Math.min(1, Math.ceil(highest / yStep.value) * yStep.value)
})
const yTicks = computed(() =>
  Array.from({ length: Math.round(yTop.value / yStep.value) + 1 }, (_, i) => i * yStep.value),
)
const x = (time: number) => scale(time, { domain: xDomain.value, range: xRange })
const y = (p: number) => scale(p, { domain: [0, yTop.value], range: yRange })

const path = (line: ChanceLine) =>
  line.points.map((p, i) => `${i === 0 ? "M" : "L"}${x(p.time).toFixed(1)},${y(p.p).toFixed(1)}`).join("")

const DAY = 86_400_000
const dateTicks = computed(() => {
  const [start, end] = xDomain.value
  const days = Math.max(1, Math.round((end - start) / DAY))
  const step = Math.max(1, Math.ceil(days / 6)) * DAY
  const first = Math.ceil(start / DAY) * DAY
  const ticks: number[] = []
  for (let t = first; t <= end; t += step) ticks.push(t)
  return ticks
})
const isoDate = (time: number) => new Date(time).toISOString().slice(0, 10)

// Names at the end of each line, pushed apart so they never overlap: down from the top, then back up from the
// axis if the last ones fell below it.
const endLabels = computed(() => {
  const labels = shown.value
    .map((line) => ({ line, last: line.points.at(-1) }))
    .filter((l): l is { line: ChanceLine; last: { time: number; p: number } } => l.last !== undefined)
    .map((l) => ({ ...l, at: y(l.last.p) }))
    .toSorted((a, b) => a.at - b.at)
  const gap = 17
  for (let i = 1; i < labels.length; i++) labels[i]!.at = Math.max(labels[i]!.at, labels[i - 1]!.at + gap)
  const last = labels.at(-1)
  if (last && last.at > yRange[0]) {
    last.at = yRange[0]
    for (let i = labels.length - 2; i >= 0; i--) labels[i]!.at = Math.min(labels[i]!.at, labels[i + 1]!.at - gap)
  }
  return labels
})

// The crosshair snaps to the nearest update.
const hovered = ref<number | null>(null)
function onMove(event: PointerEvent) {
  const svg = event.currentTarget as SVGSVGElement
  const box = svg.getBoundingClientRect()
  const px = ((event.clientX - box.left) / box.width) * width
  let best: number | null = null
  for (const time of times.value) if (best === null || Math.abs(x(time) - px) < Math.abs(x(best) - px)) best = time
  hovered.value = best
}
const readout = computed(() => {
  const time = hovered.value
  if (time === null) return null
  return {
    time,
    left: (x(time) / width) * 100,
    rows: shown.value
      .map((line) => ({ line, point: line.points.find((p) => p.time === time) }))
      .filter((r) => r.point !== undefined)
      .toSorted((a, b) => b.point!.p - a.point!.p),
  }
})

function valueAt(line: ChanceLine, time: number): string {
  const point = line.points.find((p) => p.time === time)
  return point ? probability(point.p, locale.value) : "—"
}

const help = computed(() =>
  content.help({
    count: count(lines.value.length, locale.value),
    runs: count(times.value.length, locale.value),
    since: times.value.length ? longDate(isoDate(times.value[0]!), locale.value) : "",
  }),
)
const chartLabel = computed(
  () =>
    content.chartLabel({
      names: list(
        lines.value.map((l) => l.name),
        locale.value,
      ),
    }).value,
)
</script>

<template>
  <div class="over-time">
    <div class="intro">
      <h3>{{ content.title }}</h3>
      <p class="secondary small">{{ help }}</p>
    </div>

    <p v-if="!enough" class="secondary">{{ content.empty }}</p>
    <template v-else>
      <div class="controls">
        <ul class="legend small" :aria-label="common.legend.value">
          <li v-for="line in lines" :key="line.candidateId" :class="{ off: zoomed && line === leader }">
            <span aria-hidden="true" class="key" :style="{ background: line.colour }" />{{ line.name }}
          </li>
        </ul>
        <div v-if="lines.length > 1" class="chips" role="group" :aria-label="content.scaleLabel.value">
          <button class="chip" type="button" :aria-pressed="!zoomed" @click="zoomed = false">{{ content.fullScale }}</button>
          <button class="chip" type="button" :aria-pressed="zoomed" @click="zoomed = true">{{ content.zoomChasers }}</button>
        </div>
      </div>
      <p v-if="zoomed && leader?.points.length" class="small muted off-scale">
        {{ content.offScale({ name: leader.name, value: probability(leader.points.at(-1)!.p, locale) }) }}
      </p>
      <div class="plot">
        <svg
          :viewBox="`0 0 ${width} ${height}`"
          role="img"
          :aria-label="chartLabel"
          @pointermove="onMove"
          @pointerleave="hovered = null"
        >
          <g class="grid">
            <g v-for="tick in yTicks" :key="tick">
              <line :x1="margin.left" :x2="xRange[1]" :y1="y(tick)" :y2="y(tick)" />
              <text :x="margin.left - 10" :y="y(tick) + 4" text-anchor="end">
                {{ percent(tick, 0, locale) }}
              </text>
            </g>
            <text v-for="tick in dateTicks" :key="tick" :x="x(tick)" :y="height - 8" text-anchor="middle">
              {{ shortDate(isoDate(tick), locale) }}
            </text>
          </g>
          <path
            v-for="line in shown"
            :key="line.candidateId"
            :d="path(line)"
            class="line"
            :style="{ stroke: line.colour }"
          />
          <g v-for="label in endLabels" :key="label.line.candidateId" class="end">
            <circle :cx="x(label.last.time)" :cy="y(label.last.p)" r="4" :style="{ fill: label.line.colour }" />
            <text :x="xRange[1] + 12" :y="label.at + 4">
              <tspan class="end-value">{{ probability(label.last.p, locale) }}</tspan>
              <tspan class="end-name" dx="6">{{ label.line.name }}</tspan>
            </text>
          </g>
          <g v-if="readout" class="crosshair" pointer-events="none">
            <line :x1="x(readout.time)" :x2="x(readout.time)" :y1="margin.top" :y2="yRange[0]" />
            <circle
              v-for="row in readout.rows"
              :key="row.line.candidateId"
              :cx="x(readout.time)"
              :cy="y(row.point!.p)"
              r="4.5"
              :style="{ fill: row.line.colour }"
            />
          </g>
        </svg>
        <div
          v-if="readout"
          class="tooltip small"
          role="status"
          :class="{ flip: readout.left > 55 }"
          :style="{ left: `${readout.left}%` }"
        >
          <p class="tooltip-date muted">{{ dateTime(new Date(readout.time).toISOString(), locale) }}</p>
          <p v-for="row in readout.rows" :key="row.line.candidateId" class="tooltip-row">
            <span aria-hidden="true" class="line-key" :style="{ background: row.line.colour }" />
            <strong class="num">{{ probability(row.point!.p, locale) }}</strong>
            <span class="secondary">{{ row.line.name }}</span>
          </p>
        </div>
      </div>

      <details class="data">
        <summary class="small">{{ common.showData }}</summary>
        <div class="table-wrap">
          <table class="small">
            <thead>
              <tr>
                <th scope="col">{{ content.headUpdate }}</th>
                <th v-for="line in lines" :key="line.candidateId" scope="col">{{ line.name }}</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="time in times.toReversed()" :key="time">
                <th scope="row">{{ dateTime(new Date(time).toISOString(), locale) }}</th>
                <td v-for="line in lines" :key="line.candidateId" class="num">
                  {{ valueAt(line, time) }}
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </details>
    </template>
  </div>
</template>

<style scoped>
.over-time {
  display: grid;
  gap: 16px;
}

h3 {
  font-family: var(--font-sans);
  font-size: 1.05rem;
  font-weight: 600;
  margin: 0 0 6px;
}

.controls {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
  align-items: center;
  gap: 12px 24px;
}

.legend li.off {
  opacity: 0.4;
}

.off-scale {
  margin: -6px 0 0;
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

/* Lines are keyed with a short stroke, not a box. */
.key,
.line-key {
  width: 16px;
  height: 2px;
  border-radius: 2px;
}

.plot {
  position: relative;
}

svg {
  display: block;
  width: 100%;
  height: auto;
  overflow: visible;
  touch-action: pan-y;
}

.grid line {
  stroke: var(--grid);
  stroke-width: 1;
  vector-effect: non-scaling-stroke;
}

.grid text {
  fill: var(--muted);
  font-size: 11px;
  font-variant-numeric: tabular-nums;
}

.line {
  fill: none;
  stroke-width: 2;
  stroke-linejoin: round;
  stroke-linecap: round;
  vector-effect: non-scaling-stroke;
}

.end circle,
.crosshair circle {
  stroke: var(--surface);
  stroke-width: 2;
}

.end text {
  font-size: 12px;
}

.end-value {
  fill: var(--ink);
  font-weight: 650;
  font-variant-numeric: tabular-nums;
}

.end-name {
  fill: var(--ink-2);
}

.crosshair line {
  stroke: var(--axis);
  stroke-width: 1;
  vector-effect: non-scaling-stroke;
}

.tooltip {
  position: absolute;
  top: 4px;
  transform: translateX(14px);
  display: grid;
  gap: 4px;
  min-width: 11rem;
  padding: 8px 12px;
  border-radius: 10px;
  background: rgba(16, 19, 26, 0.94);
  box-shadow:
    inset 0 0 0 1px rgba(255, 255, 255, 0.1),
    0 8px 24px rgba(0, 0, 0, 0.5);
  pointer-events: none;
}

.tooltip.flip {
  transform: translateX(calc(-100% - 14px));
}

.tooltip p {
  margin: 0;
}

.tooltip-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.tooltip-row strong {
  min-width: 3em;
  color: var(--ink);
}

.data summary {
  cursor: pointer;
  color: var(--ink-2);
}

.data .table-wrap {
  margin-top: 12px;
}
</style>
