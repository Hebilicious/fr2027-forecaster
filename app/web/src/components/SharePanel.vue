<script setup lang="ts">
import { computed, ref } from "vue"
import type { Panel, PollDot } from "../charts"
import { scale, shareTicks } from "../charts"
import type { SeriesPoint } from "../api/schemas"
import { percent, safeUrl, shareWithInterval, shortDate } from "../format"
import { useI18n, type Locale } from "../i18n"

const props = defineProps<{
  panel: Panel
  /** Shared x domain (epoch ms) so panels line up. */
  xDomain: readonly [number, number]
  /** Shared y maximum so panels compare. */
  yMax: number
  highlightFirm: string | null
}>()

const { t, locale } = useI18n()
const shareDate = (point: SeriesPoint, l: Locale) =>
  `${shareWithInterval(point.mean, point.lo80, point.hi80, l)} · ${shortDate(point.date, l)}`

const width = 320
const height = 180
const margin = { top: 10, right: 12, bottom: 22, left: 34 }
const x = (date: string) =>
  scale(Date.parse(`${date}T00:00:00Z`), { domain: props.xDomain, range: [margin.left, width - margin.right] })
const y = (share: number) => scale(share, { domain: [0, props.yMax], range: [height - margin.bottom, margin.top] })

const ticks = computed(() => shareTicks(props.yMax))
const band = computed(() => {
  const pts = props.panel.points
  if (pts.length === 0) return ""
  const upper = pts.map((p) => `${x(p.date).toFixed(1)},${y(p.hi80).toFixed(1)}`)
  const lower = pts.toReversed().map((p) => `${x(p.date).toFixed(1)},${y(p.lo80).toFixed(1)}`)
  return `M${upper.join("L")}L${lower.join("L")}Z`
})
const line = computed(() =>
  props.panel.points.map((p, i) => `${i === 0 ? "M" : "L"}${x(p.date).toFixed(1)},${y(p.mean).toFixed(1)}`).join(""),
)
const latest = computed(() => props.panel.points.at(-1))
const monthTicks = computed(() => {
  const [start, end] = props.xDomain
  const result: { at: number; label: string }[] = []
  const d = new Date(start)
  d.setUTCDate(1)
  d.setUTCMonth(d.getUTCMonth() + 1)
  const step = end - start > 200 * 86_400_000 ? 3 : end - start > 90 * 86_400_000 ? 2 : 1
  while (d.getTime() <= end) {
    result.push({
      at: scale(d.getTime(), { domain: props.xDomain, range: [margin.left, width - margin.right] }),
      label: d.toLocaleDateString(locale.value === "fr" ? "fr-FR" : "en-GB", { month: "short", timeZone: "UTC" }),
    })
    d.setUTCMonth(d.getUTCMonth() + step)
  }
  return result
})

// Crosshair: snaps to the nearest history point.
const hover = ref<{ index: number } | null>(null)
function onMove(event: PointerEvent) {
  const svg = event.currentTarget as SVGSVGElement
  const box = svg.getBoundingClientRect()
  const px = ((event.clientX - box.left) / box.width) * width
  let best = 0
  let bestDistance = Infinity
  props.panel.points.forEach((p, i) => {
    const distance = Math.abs(x(p.date) - px)
    if (distance < bestDistance) {
      best = i
      bestDistance = distance
    }
  })
  hover.value = { index: best }
}
const hovered = computed(() => (hover.value ? props.panel.points[hover.value.index] : undefined))

const focusedDot = ref<PollDot | null>(null)
const dotLabel = (dot: PollDot) =>
  t("pollTooltip", {
    firm: dot.firm,
    date: shortDate(dot.date, locale.value),
    share: percent(dot.share, 1, locale.value),
    scenario: dot.scenario,
  })
const sortedDots = computed(() =>
  props.panel.polls.toSorted((a, b) => Number(a.firm === props.highlightFirm) - Number(b.firm === props.highlightFirm)),
)
</script>

<template>
  <figure class="card panel">
    <figcaption>
      <h3>{{ panel.candidate.name }}</h3>
      <p v-if="latest" class="secondary small num">
        {{ shareDate(latest, locale) }}
      </p>
    </figcaption>
    <div class="plot">
      <svg
        :viewBox="`0 0 ${width} ${height}`"
        role="img"
        :aria-label="`${panel.candidate.name}: ${latest ? shareDate(latest, locale) : ''}`"
        @pointermove="onMove"
        @pointerleave="hover = null"
      >
        <g class="grid">
          <g v-for="tick in ticks" :key="tick">
            <line :x1="margin.left" :x2="width - margin.right" :y1="y(tick)" :y2="y(tick)" />
            <text :x="margin.left - 6" :y="y(tick) + 3.5" text-anchor="end">{{ Math.round(tick * 100) }}</text>
          </g>
          <text v-for="m in monthTicks" :key="m.at" :x="m.at" :y="height - 6" text-anchor="middle">{{ m.label }}</text>
        </g>
        <path :d="band" class="band" />
        <path :d="line" class="line" />
        <g>
          <a
            v-for="(dot, i) in sortedDots"
            :key="`${dot.pollId}-${dot.scenario}-${i}`"
            :href="safeUrl(dot.sourceUrl)"
            rel="noopener noreferrer"
            target="_blank"
            :aria-label="dotLabel(dot)"
            @focus="focusedDot = dot"
            @blur="focusedDot = null"
            @pointerenter="focusedDot = dot"
            @pointerleave="focusedDot = null"
          >
            <circle :cx="x(dot.date)" :cy="y(dot.share)" class="hit" r="12" />
            <circle
              :cx="x(dot.date)"
              :cy="y(dot.share)"
              r="4"
              :class="['dot', { highlighted: dot.firm === highlightFirm, faded: highlightFirm && dot.firm !== highlightFirm }]"
            />
          </a>
        </g>
        <g v-if="hovered" class="crosshair" pointer-events="none">
          <line :x1="x(hovered.date)" :x2="x(hovered.date)" :y1="margin.top" :y2="height - margin.bottom" />
          <circle :cx="x(hovered.date)" :cy="y(hovered.mean)" r="4" />
        </g>
      </svg>
      <p v-if="focusedDot" class="tooltip small" role="status">{{ dotLabel(focusedDot) }}</p>
      <p v-else-if="hovered" class="tooltip small num" role="status">
        <strong>{{ percent(hovered.mean, 1, locale) }}</strong>
        <span class="secondary">
          ({{ Math.round(hovered.lo80 * 100) }}–{{ Math.round(hovered.hi80 * 100) }}) · {{ shortDate(hovered.date, locale) }}</span>
      </p>
    </div>
  </figure>
</template>

<style scoped>
.panel {
  margin: 0;
  display: grid;
  gap: 6px;
}

figcaption {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  gap: 8px;
}

.plot {
  position: relative;
}

svg {
  width: 100%;
  height: auto;
  display: block;
  touch-action: pan-y;
}

.grid line {
  stroke: var(--grid);
  stroke-width: 1;
}

.grid text {
  fill: var(--muted);
  font-size: 10px;
  font-variant-numeric: tabular-nums;
}

.band {
  fill: var(--series-wash);
}

.line {
  fill: none;
  stroke: var(--series);
  stroke-width: 2;
  stroke-linejoin: round;
  stroke-linecap: round;
}

.hit {
  fill: transparent;
}

.dot {
  fill: var(--dot);
  stroke: var(--surface);
  stroke-width: 2;
}

.dot.faded {
  opacity: 0.35;
}

.dot.highlighted {
  fill: var(--accent);
}

a:focus-visible .dot {
  stroke: var(--ink);
}

.crosshair line {
  stroke: var(--axis);
  stroke-width: 1;
}

.crosshair circle {
  fill: var(--series);
  stroke: var(--surface);
  stroke-width: 2;
}

.tooltip {
  position: absolute;
  top: 2px;
  right: 6px;
  margin: 0;
  padding: 2px 8px;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 6px;
  max-width: 80%;
}
</style>
