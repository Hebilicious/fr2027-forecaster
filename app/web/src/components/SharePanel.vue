<script setup lang="ts">
import { computed, ref, shallowRef, useId } from "vue"
import { useIntlayer, useLocale } from "vue-intlayer"
import type { Domain, Panel, PollDot } from "../charts"
import { fitShares, invert, nearest, panDomain, scale, shareTicks, shareTicksWithin, zoomDomain } from "../charts"
import type { SeriesPoint } from "../api/schemas"
import { percent, safeUrl, shareWithInterval, shortDate } from "../format"
import type { Locale } from "../locales"
import CandidateAvatar from "./CandidateAvatar.vue"

const props = defineProps<{
  panel: Panel
  /** Shared x domain (epoch ms) so panels line up. */
  xDomain: readonly [number, number]
  /** Shared y maximum so panels compare. */
  yMax: number
  highlightFirm: string | null
}>()

const content = useIntlayer("share-panel")
const clipId = `plot-${useId()}`
const { locale } = useLocale()
const shareDate = (point: SeriesPoint, l: Locale) =>
  `${shareWithInterval(point.mean, point.lo80, point.hi80, l)} · ${shortDate(point.date, l)}`

const DAY = 86_400_000
const width = 320
const height = 180
const margin = { top: 10, right: 12, bottom: 22, left: 34 }
const xRange: Domain = [margin.left, width - margin.right]
const yRange: Domain = [height - margin.bottom, margin.top]

// Zoom: the wheel zooms the time axis around the pointer, a drag pans it, and a double click goes
// back to the shared scales. The share axis then fits what is in view. null is the shared scales,
// so panels line up by default.
const zoom = shallowRef<Domain | null>(null)
const fullTicks = computed(() => shareTicks(props.yMax))
const fullY = computed<Domain>(() => [0, fullTicks.value.at(-1) ?? props.yMax])
const xView = computed(() => zoom.value ?? props.xDomain)
const yView = computed<Domain>(() => {
  const view = zoom.value
  if (!view) return fullY.value
  const within = (date: string) => {
    const time = Date.parse(`${date}T00:00:00Z`)
    return time >= view[0] && time <= view[1]
  }
  const shares = [
    ...props.panel.points.filter((p) => within(p.date)).flatMap((p) => [p.lo80, p.hi80]),
    ...props.panel.polls.filter((d) => within(d.date)).map((d) => d.share),
  ]
  return fitShares(shares, fullY.value, 0.04)
})
const ticks = computed(() => (zoom.value ? shareTicksWithin(yView.value) : fullTicks.value))

const x = (date: string) => scale(Date.parse(`${date}T00:00:00Z`), { domain: xView.value, range: xRange })
const y = (share: number) => scale(share, { domain: yView.value, range: yRange })

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
const dateTicks = computed(() => {
  const [start, end] = xView.value
  const result: { at: number; label: string }[] = []
  const at = (time: number) => scale(time, { domain: xView.value, range: xRange })
  if (end - start <= 75 * DAY) {
    const days = (end - start) / DAY
    const step = ([2, 3, 7, 14].find((d) => days / d <= 4) ?? 21) * DAY
    for (let t = Math.ceil(start / DAY) * DAY; t <= end; t += step) {
      result.push({ at: at(t), label: shortDate(new Date(t).toISOString().slice(0, 10), locale.value) })
    }
    return result
  }
  const d = new Date(start)
  d.setUTCDate(1)
  d.setUTCMonth(d.getUTCMonth() + 1)
  const step = end - start > 200 * DAY ? 3 : end - start > 90 * DAY ? 2 : 1
  while (d.getTime() <= end) {
    result.push({ at: at(d.getTime()), label: d.toLocaleDateString(locale.value, { month: "short", timeZone: "UTC" }) })
    d.setUTCMonth(d.getUTCMonth() + step)
  }
  return result
})
const inView = (date: string, share: number) => {
  const px = x(date)
  const py = y(share)
  return px >= xRange[0] && px <= xRange[1] && py <= yRange[0] && py >= yRange[1]
}

/** The pointer's position in the SVG's own units, and how many units one screen pixel is. */
function plotPosition(event: MouseEvent) {
  const svg = event.currentTarget as SVGSVGElement
  const box = svg.getBoundingClientRect()
  const units = width / box.width
  return { px: (event.clientX - box.left) * units, py: (event.clientY - box.top) * units, units }
}

/** Whether two time domains are the same to the millisecond. */
const same = (a: Domain, b: Domain) => Math.abs(a[0] - b[0]) + Math.abs(a[1] - b[1]) < 1

function onWheel(event: WheelEvent) {
  const { px } = plotPosition(event)
  const pixels = event.deltaY * (event.deltaMode === 1 ? 16 : event.deltaMode === 2 ? height : 1)
  const anchor = invert(Math.min(Math.max(px, xRange[0]), xRange[1]), { domain: xView.value, range: xRange })
  const next = zoomDomain(xView.value, anchor, Math.exp(pixels * 0.002), props.xDomain, 14 * DAY)
  // Fully zoomed out (or in), the wheel scrolls the page as usual.
  if (same(next, xView.value)) return
  event.preventDefault()
  zoom.value = same(next, props.xDomain) ? null : next
  onMove(event)
}

let drag: { clientX: number; from: Domain; moved: boolean } | null = null
let lastTap: { time: number; clientX: number; clientY: number } | null = null
function onDown(event: PointerEvent) {
  onMove(event)
  if (event.pointerType === "mouse" && event.button === 0 && zoom.value) {
    drag = { clientX: event.clientX, from: zoom.value, moved: false }
  }
}
function onCancel() {
  drag = null
}
/** Ends a drag; two quick clicks in one place go back to the shared scales. */
function onUp(event: PointerEvent) {
  const moved = drag?.moved ?? false
  drag = null
  if (moved) return
  const again =
    lastTap &&
    event.timeStamp - lastTap.time < 400 &&
    Math.hypot(event.clientX - lastTap.clientX, event.clientY - lastTap.clientY) < 6
  lastTap = again ? null : { time: event.timeStamp, clientX: event.clientX, clientY: event.clientY }
  if (again) zoom.value = null
}

// Pointer: within DOT_RADIUS screen pixels of a poll dot it picks the nearest dot, otherwise the
// crosshair snaps to the nearest history point. Picking by distance, rather than by overlapping hit
// circles, keeps dots a few pixels apart separately reachable.
const DOT_RADIUS = 14
const hover = ref<{ index: number } | null>(null)
const pointedDot = shallowRef<PollDot | null>(null)
function onMove(event: MouseEvent) {
  const { px, py, units } = plotPosition(event)
  if (drag && event instanceof PointerEvent) {
    const dx = (event.clientX - drag.clientX) * units
    if (!drag.moved && Math.abs(dx) < 3 * units) return
    if (!drag.moved) (event.currentTarget as SVGSVGElement).setPointerCapture(event.pointerId)
    drag.moved = true
    const from = drag.from
    zoom.value = panDomain(from, (-dx / (xRange[1] - xRange[0])) * (from[1] - from[0]), props.xDomain)
    pointedDot.value = null
    hover.value = null
    return
  }
  pointedDot.value = nearest(
    sortedDots.value.filter((dot) => inView(dot.date, dot.share)),
    (dot) => [x(dot.date), y(dot.share)],
    px,
    py,
    DOT_RADIUS * units,
  )
  let best: number | null = null
  let bestDistance = Infinity
  props.panel.points.forEach((p, i) => {
    const distance = Math.abs(x(p.date) - px)
    if (distance < bestDistance && x(p.date) >= xRange[0] && x(p.date) <= xRange[1]) {
      best = i
      bestDistance = distance
    }
  })
  hover.value = best === null ? null : { index: best }
}
function onLeave() {
  hover.value = null
  pointedDot.value = null
}
const hovered = computed(() => (hover.value ? props.panel.points[hover.value.index] : undefined))

const focusedDot = shallowRef<PollDot | null>(null)
const activeDot = computed(() => pointedDot.value ?? focusedDot.value)
const dotLabel = (dot: PollDot) =>
  content.poll({
    firm: dot.firm,
    date: shortDate(dot.date, locale.value),
    share: percent(dot.share, 1, locale.value),
    scenario: dot.scenario,
  }).value
const sortedDots = computed(() =>
  props.panel.polls.toSorted((a, b) => Number(a.firm === props.highlightFirm) - Number(b.firm === props.highlightFirm)),
)
</script>

<template>
  <figure class="card panel">
    <figcaption>
      <div class="who">
        <CandidateAvatar :id="panel.candidate.candidate_id" :name="panel.candidate.name" :size="38" />
        <h2>{{ panel.candidate.name }}</h2>
      </div>
      <p v-if="latest" class="latest">
        <span class="latest-value">{{ percent(latest.mean, 0, locale) }}</span>
        <span class="latest-range num">
          {{ Math.round(latest.lo80 * 100) }}–{{ Math.round(latest.hi80 * 100) }} · {{ shortDate(latest.date, locale) }}
        </span>
      </p>
    </figcaption>
    <div class="plot">
      <svg
        :viewBox="`0 0 ${width} ${height}`"
        role="img"
        :aria-label="content.label({ name: panel.candidate.name, latest: latest ? shareDate(latest, locale) : '' }).value"
        :class="{ zoomed: zoom }"
        @pointermove="onMove"
        @pointerdown="onDown"
        @pointerup="onUp"
        @pointercancel="onCancel"
        @pointerleave="onLeave"
        @wheel="onWheel"
      >
        <defs>
          <clipPath :id="clipId">
            <rect :x="xRange[0]" :y="yRange[1] - 6" :width="xRange[1] - xRange[0]" :height="yRange[0] - yRange[1] + 12" />
          </clipPath>
        </defs>
        <g class="grid">
          <g v-for="tick in ticks" :key="tick">
            <line :x1="margin.left" :x2="width - margin.right" :y1="y(tick)" :y2="y(tick)" />
            <text :x="margin.left - 8" :y="y(tick) + 3.5" text-anchor="end">{{ Math.round(tick * 100) }}</text>
          </g>
          <text v-for="m in dateTicks" :key="m.at" :x="m.at" :y="height - 6" text-anchor="middle">{{ m.label }}</text>
        </g>
        <g :clip-path="`url(#${clipId})`">
          <path :d="band" class="band" />
          <path :d="line" class="line" />
          <!-- The links are for the keyboard; the pointer reaches a poll through the overlay below. -->
          <g class="dots">
            <a
              v-for="(dot, i) in sortedDots"
              :key="`${dot.pollId}-${dot.scenario}-${i}`"
              :href="safeUrl(dot.sourceUrl)"
              rel="noopener noreferrer"
              target="_blank"
              :aria-label="dotLabel(dot)"
              @focus="focusedDot = dot"
              @blur="focusedDot = null"
            >
              <circle
                :cx="x(dot.date)"
                :cy="y(dot.share)"
                :r="dot.firm === highlightFirm ? 4.5 : 3.5"
                :class="['dot', { highlighted: dot.firm === highlightFirm, faded: highlightFirm && dot.firm !== highlightFirm }]"
              />
            </a>
          </g>
          <!-- The pointed or focused dot, drawn again on top of its neighbours. -->
          <g v-if="activeDot" class="active" pointer-events="none">
            <circle :cx="x(activeDot.date)" :cy="y(activeDot.share)" class="dot" r="4.5" />
          </g>
          <g v-if="hovered && !activeDot" class="crosshair" pointer-events="none">
            <line :x1="x(hovered.date)" :x2="x(hovered.date)" :y1="margin.top" :y2="height - margin.bottom" />
            <circle :cx="x(hovered.date)" :cy="y(hovered.mean)" r="4" />
          </g>
        </g>
        <!-- While the pointer is near a dot, a click anywhere on the plot opens that dot's poll. -->
        <a
          v-if="pointedDot"
          :href="safeUrl(pointedDot.sourceUrl)"
          rel="noopener noreferrer"
          tabindex="-1"
          target="_blank"
          :aria-label="dotLabel(pointedDot)"
          class="pointed"
        >
          <rect :width="width" :height="height" />
        </a>
      </svg>
      <p v-if="activeDot" class="tooltip small" role="status">{{ dotLabel(activeDot) }}</p>
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
  gap: 12px;
  padding: 20px 20px 14px;
}

figcaption {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 12px;
}

.who {
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 0;
}

h2 {
  font-family: var(--font-sans);
  font-size: 0.98rem;
  font-weight: 600;
  letter-spacing: -0.005em;
}

.latest {
  display: grid;
  justify-items: end;
  gap: 2px;
  text-align: right;
}

.latest-value {
  font-size: 1.6rem;
  font-weight: 600;
  letter-spacing: -0.02em;
  line-height: 1;
}

.latest-range {
  font-size: 0.75rem;
  color: var(--muted);
  white-space: nowrap;
}

.plot {
  position: relative;
}

svg {
  width: 100%;
  height: auto;
  display: block;
  overflow: visible;
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
  filter: drop-shadow(0 0 4px rgba(88, 124, 241, 0.55));
}

.dots {
  pointer-events: none;
}

.pointed rect {
  fill: transparent;
  cursor: pointer;
}

.dot {
  fill: var(--dot);
  stroke: var(--surface);
  stroke-width: 1.5;
  transition: opacity 0.25s;
}

.dot.faded {
  opacity: 0.28;
}

.dot.highlighted {
  fill: var(--accent);
  stroke-width: 2;
}

.active .dot {
  fill: var(--ink);
  stroke: var(--surface);
  stroke-width: 1.5;
}

svg.zoomed {
  cursor: grab;
}

svg.zoomed:active {
  cursor: grabbing;
}

/* Keyboard focus rings the dot itself, and the tooltip names the poll. */
a:focus-visible {
  outline: none;
}

a:focus-visible .dot {
  stroke: var(--focus);
  stroke-width: 3;
  opacity: 1;
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
  top: 0;
  right: 4px;
  margin: 0;
  padding: 4px 10px;
  max-width: 82%;
  border-radius: 8px;
  background: rgba(16, 19, 26, 0.92);
  box-shadow:
    inset 0 0 0 1px rgba(255, 255, 255, 0.1),
    0 8px 24px rgba(0, 0, 0, 0.5);
  -webkit-backdrop-filter: blur(8px);
  backdrop-filter: blur(8px);
  pointer-events: none;
}
</style>
