<script setup lang="ts">
import { computed, ref } from "vue"
import { useIntlayer, useLocale } from "vue-intlayer"
import type { XRow } from "../../api/schemas"
import { count, dateTime, percent, points } from "../../format"
import { THIN_POSTS, windowsInRange, xPooled, xWindows, type Window, type XPooledRow } from "../../signals"
import { useSignalRange } from "../../useSignalRange"
import CandidateAvatar from "../CandidateAvatar.vue"
import Sparkline from "../Sparkline.vue"
import RangeControl from "./RangeControl.vue"

// Grok Bot's X drops over the chosen period, one line per candidate: a full-width bar stacking the favourable,
// neutral and unfavourable shares of the posts about them, pooled over the period's drops with the 95% interval of
// the favourable share; the share drop by drop; then the volume. Or one drop at a time, with its topics.
// The most mentioned first; the rest behind a toggle.
const props = defineProps<{ rows: readonly XRow[]; names: ReadonlyMap<string, string> }>()
const content = useIntlayer("x-section")
const common = useIntlayer("common")
const { locale } = useLocale()
const range = useSignalRange()

const SHOWN = 12
const showAll = ref(false)
const windows = computed(() => xWindows(props.rows))
// One drop at a time: the index of the drop shown, the newest when the reader switches to it.
const byDrop = ref(false)
const dropIndex = ref(0)
function toggleByDrop() {
  byDrop.value = !byDrop.value
  dropIndex.value = windows.value.length - 1
}
const selected = computed(() => {
  if (!byDrop.value) return windowsInRange(windows.value, range.value)
  const drop = windows.value[Math.min(dropIndex.value, windows.value.length - 1)]
  return drop ? [drop] : []
})
const span = computed(() => {
  const first = selected.value[0]
  const last = selected.value.at(-1)
  return first && last ? { start: first.start, end: last.end } : null
})
const all = computed(() =>
  xPooled(
    props.rows,
    selected.value.map((w) => w.end),
    props.names,
  ),
)
const rows = computed(() => (showAll.value ? all.value : all.value.slice(0, SHOWN)))
// The share drop by drop shows once the period holds two stance drops.
const trendWindow = computed<Window | null>(() => {
  const stance = selected.value.filter((w) => w.stance)
  return stance.length >= 2 ? [stance[0]!.end, stance.at(-1)!.end] : null
})
const hasLegacy = computed(() => all.value.some((r) => r.legacy))
const hasThin = computed(() => all.value.some((r) => r.thin && r.mood))

const share = (x: number) => percent(x, 0, locale.value)
const pct = (x: number) => `${(x * 100).toFixed(1)}%`
// A segment shows its share inside when it is wide enough to hold the label.
const labelled = (x: number) => x >= 0.12
function moodLabel(row: XPooledRow): string {
  const mood = row.mood!
  const shares = { pos: share(mood.pos), neu: share(mood.neu), neg: share(mood.neg) }
  const support = row.stance?.support
  if (!support) return content.sentimentLabel(shares).value
  return content.sentimentWithInterval({ ...shares, low: share(support.low), high: share(support.high) }).value
}
function trendLabel(row: XPooledRow): string {
  const first = row.trend[0]
  const last = row.trend.at(-1)
  if (!first || !last) return ""
  return content.trendLabel({ first: share(first.value), last: share(last.value) }).value
}
</script>

<template>
  <section aria-labelledby="x-title" class="card">
    <header class="card-head head">
      <div class="titles">
        <h2 id="x-title">{{ content.title }}</h2>
        <p v-if="span" class="muted small">
          {{ content.window({ start: dateTime(span.start, locale), end: dateTime(span.end, locale) }) }}
          · {{ content.drops({ n: count(selected.length, locale) }) }}
        </p>
      </div>
      <div v-if="windows.length" class="controls">
        <RangeControl :disabled="byDrop" />
        <button class="chip" type="button" :aria-pressed="byDrop" @click="toggleByDrop">{{ content.byDrop }}</button>
        <span v-if="byDrop" class="stepper">
          <button
            class="chip step"
            type="button"
            :aria-label="content.previousDrop.value"
            :title="content.previousDrop.value"
            :disabled="dropIndex <= 0"
            @click="dropIndex--"
          >
            ‹
          </button>
          <button
            class="chip step"
            type="button"
            :aria-label="content.nextDrop.value"
            :title="content.nextDrop.value"
            :disabled="dropIndex >= windows.length - 1"
            @click="dropIndex++"
          >
            ›
          </button>
        </span>
      </div>
    </header>
    <p v-if="!windows.length" class="secondary small">{{ content.empty }}</p>
    <template v-else>
      <p aria-hidden="true" class="legend small">
        <span class="key"><span class="swatch pos" />{{ content.positive }}</span>
        <span class="key"><span class="swatch neu" />{{ content.neutral }}</span>
        <span class="key"><span class="swatch neg" />{{ content.negative }}</span>
        <span class="key"><span class="whisker-key" />{{ content.interval }}</span>
      </p>
      <div class="table-wrap">
        <table class="x">
          <thead>
            <tr>
              <th scope="col">{{ common.candidate }}</th>
              <th class="bar-col" scope="col">{{ content.sentiment }}</th>
              <th v-if="trendWindow" scope="col">{{ content.trend }}</th>
              <th v-if="trendWindow" class="num" scope="col">{{ content.change }}</th>
              <th class="num" scope="col">{{ content.mentions }}</th>
              <th class="num" scope="col">{{ content.posts }}</th>
              <th class="num" scope="col">{{ content.botShare }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="row in rows" :key="row.candidateId" :class="{ thin: row.thin }">
              <th scope="row">
                <span class="who">
                  <CandidateAvatar :id="row.candidateId" :name="row.name" :size="22" />
                  {{ row.name }}
                </span>
              </th>
              <td class="bar-col">
                <div v-if="row.mood" class="mood" role="img" :aria-label="moodLabel(row)" :title="moodLabel(row)">
                  <span v-if="row.mood.pos > 0" class="pos" :style="{ width: pct(row.mood.pos) }">
                    <span v-if="labelled(row.mood.pos)" class="share">{{ share(row.mood.pos) }}</span>
                  </span>
                  <span v-if="row.mood.neu > 0" class="neu" :style="{ width: pct(row.mood.neu) }">
                    <span v-if="labelled(row.mood.neu)" class="share">{{ share(row.mood.neu) }}</span>
                  </span>
                  <span v-if="row.mood.neg > 0" class="neg" :style="{ width: pct(row.mood.neg) }">
                    <span v-if="labelled(row.mood.neg)" class="share">{{ share(row.mood.neg) }}</span>
                  </span>
                  <i
                    v-if="row.stance"
                    aria-hidden="true"
                    class="whisker"
                    :style="{
                      left: pct(row.stance.support.low),
                      width: pct(row.stance.support.high - row.stance.support.low),
                    }"
                  />
                </div>
                <span v-else class="muted">—</span>
                <p v-if="byDrop && row.topics.length" class="topics muted small">
                  {{ content.topics }}
                  <template v-for="(topic, i) in row.topics" :key="topic">
                    <span v-if="i > 0" aria-hidden="true" class="dot">·</span>{{ topic }}
                  </template>
                </p>
              </td>
              <td v-if="trendWindow">
                <Sparkline
                  v-if="row.trend.length >= 2"
                  :points="row.trend"
                  :window="trendWindow"
                  :label="trendLabel(row)"
                  :min-span="0.2"
                />
                <span v-else class="muted">—</span>
              </td>
              <td v-if="trendWindow" class="num secondary">
                {{ row.change === null ? "—" : points(row.change, locale) }}
              </td>
              <td class="num strong" :title="row.mentionsMethod ? content.methods[row.mentionsMethod].value : undefined">
                {{ row.mentions === null ? "—" : count(row.mentions, locale) }}
              </td>
              <td class="num secondary">{{ row.postsRead === null ? "—" : count(row.postsRead, locale) }}</td>
              <td class="num secondary">{{ row.botShare === null ? "—" : share(row.botShare) }}</td>
            </tr>
          </tbody>
        </table>
      </div>
      <button v-if="all.length > SHOWN" class="more small" type="button" @click="showAll = !showAll">
        {{ showAll ? content.showFewer : content.showAll({ n: count(all.length, locale) }) }}
      </button>
      <div class="notes muted small">
        <p v-if="selected.length > 1">{{ content.poolNote }}</p>
        <p v-if="hasThin">{{ content.thinNote({ n: count(THIN_POSTS, locale) }) }}</p>
        <p v-if="hasLegacy">{{ content.legacyNote }}</p>
      </div>
    </template>
  </section>
</template>

<style scoped>
.head {
  display: flex;
  flex-wrap: wrap;
  align-items: flex-start;
  justify-content: space-between;
  gap: 12px 24px;
}

.titles {
  display: grid;
  gap: 4px;
}

.controls {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px 10px;
}

.stepper {
  display: inline-flex;
  gap: 4px;
}

.step {
  padding: 7px 12px;
}

.step:disabled {
  cursor: default;
  opacity: 0.4;
}

.x th, .x td {
  padding-top: 7px;
  padding-bottom: 7px;
}

.who {
  display: flex;
  align-items: center;
  gap: 10px;
  white-space: nowrap;
}

th[scope="row"] {
  font-weight: 500;
  color: var(--ink);
}

.strong {
  font-weight: 650;
  color: var(--ink);
}

.bar-col {
  width: 50%;
}

/* A line resting on fewer than THIN_POSTS posts about the candidate, faded. */
tr.thin .mood, tr.thin :deep(.sparkline) {
  opacity: 0.45;
}

/* A 100% bar: favourable in bleu, neutral in grey, unfavourable in rouge, with the shares written inside. */
.mood {
  position: relative;
  display: flex;
  gap: 2px;
  height: 20px;
  min-width: 160px;
}

.mood > span {
  container-type: inline-size;
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  overflow: hidden;
  font-size: 0.72rem;
  font-weight: 600;
  font-variant-numeric: tabular-nums;
  color: #ffffff;
  white-space: nowrap;
}

/* The segments are the bar's spans; the interval after them is an <i>, so they round by type. */
.mood > span:first-of-type {
  border-radius: 4px 0 0 4px;
}

.mood > span:last-of-type {
  border-radius: 0 4px 4px 0;
}

/* A share too wide for its segment is left out; the bar's label still reads it. */
@container (max-width: 28px) {
  .share {
    display: none;
  }
}

/* The 95% interval of the favourable share, drawn across the bar's bottom edge. */
.whisker {
  position: absolute;
  bottom: -4px;
  height: 3px;
  min-width: 2px;
  border-radius: 2px;
  background: var(--ink);
}

.topics {
  margin-top: 6px;
}

.topics .dot {
  margin: 0 0.4em;
}

.x td.num {
  white-space: nowrap;
}

.pos {
  background: var(--series);
}

.neu {
  background: var(--dot);
}

.neg {
  background: var(--accent);
}

.legend {
  display: flex;
  flex-wrap: wrap;
  gap: 4px 18px;
  margin-bottom: 10px;
  color: var(--ink-2);
}

.key {
  display: inline-flex;
  align-items: center;
  gap: 8px;
}

.swatch {
  display: inline-block;
  width: 12px;
  height: 8px;
  border-radius: 2px;
}

.whisker-key {
  display: inline-block;
  width: 16px;
  height: 3px;
  border-radius: 2px;
  background: var(--ink);
}

.more {
  margin-top: 10px;
  background: none;
  border: none;
  padding: 0;
  font: inherit;
  color: var(--link);
  text-decoration: underline;
  cursor: pointer;
}

.notes {
  display: grid;
  gap: 4px;
  margin-top: 14px;
}

@media (max-width: 760px) {
  .bar-col {
    width: auto;
  }

  /* The names wrap, so the sentiment bar keeps its room. */
  .who {
    white-space: normal;
  }

  .mood {
    min-width: 110px;
  }

  th,
  td {
    padding: 7px 6px;
  }
}
</style>
