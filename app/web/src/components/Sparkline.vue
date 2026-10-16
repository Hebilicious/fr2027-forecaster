<script setup lang="ts">
import { computed } from "vue"
import { sparkline, type Point, type Window } from "../signals"

const props = defineProps<{
  points: readonly Point[]
  /** Shared by every sparkline in a column so their days line up. */
  window: Window
  /** What the line shows, for screen readers and the hover title. */
  label: string
  /** Scale to the series' own range, at least this wide, instead of from 0 (then no zero baseline). */
  minSpan?: number
}>()

const width = 72
const height = 22
const pad = 3
const shape = computed(() => sparkline(props.points, props.window, { width, height, pad }, props.minSpan))
</script>

<template>
  <svg class="sparkline" :viewBox="`0 0 ${width} ${height}`" role="img" :aria-label="label">
    <title>{{ label }}</title>
    <line v-if="minSpan === undefined" :x1="pad" :x2="width - pad" :y1="height - pad" :y2="height - pad" class="baseline" />
    <path :d="shape.line" class="line" />
    <circle v-if="shape.end" :cx="shape.end.x" :cy="shape.end.y" class="end" r="2.5" />
  </svg>
</template>

<style scoped>
.sparkline {
  display: block;
  width: 72px;
  height: 22px;
  flex: none;
  overflow: visible;
}

.baseline {
  stroke: var(--grid);
  stroke-width: 1;
}

.line {
  fill: none;
  stroke: var(--series);
  stroke-width: 1.5;
  stroke-linejoin: round;
  stroke-linecap: round;
}

.end {
  fill: var(--series);
  stroke: var(--surface);
  stroke-width: 1.5;
}

@media (max-width: 720px) {
  .sparkline {
    width: 56px;
    height: 17px;
  }
}
</style>
