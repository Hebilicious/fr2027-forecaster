<script setup lang="ts">
import { computed, toRef } from "vue"
import { useIntlayer, useLocale } from "vue-intlayer"
import type { Forecast } from "../api/schemas"
import { round2Matrix, type MatrixCell } from "../charts"
import CandidateAvatar from "../components/CandidateAvatar.vue"
import FieldFilter from "../components/FieldFilter.vue"
import { probability } from "../format"
import { useFieldFilter } from "../useFieldFilter"

const props = defineProps<{ forecast: Forecast }>()
const content = useIntlayer("round2-view")
const common = useIntlayer("common")
const { locale } = useLocale()

const { query, showAll, field } = useFieldFilter(toRef(() => props.forecast.candidates))
// The matrix drops whoever the toggle sets aside; the search narrows the list of pairings to those it names.
const matrix = computed(() => round2Matrix(props.forecast, 8, field.value.setAside))
const names = computed(() => new Map(props.forecast.candidates.map((c) => [c.candidate_id, c.name])))
const rows = computed(() =>
  matrix.value.candidates.map((row) => ({
    candidate: row,
    cells: matrix.value.candidates.map(
      (column): MatrixCell | null =>
        matrix.value.cells.find((c) => c.row === row.candidate_id && c.column === column.candidate_id) ?? null,
    ),
  })),
)

// From rouge to bleu by how likely the pairing is (--seq-* in style.css); every step carries white text.
const steps = ["--seq-0", "--seq-1", "--seq-2", "--seq-3", "--seq-4", "--seq-5", "--seq-6", "--seq-7"]
const maxMatchup = computed(() => Math.max(0.01, ...props.forecast.pairs.map((p) => p.p_matchup)))
function stepFor(p: number): number {
  if (p <= 0) return 0
  return Math.min(steps.length - 1, 1 + Math.floor((p / maxMatchup.value) * (steps.length - 1.0001)))
}
const topPairs = computed(() =>
  props.forecast.pairs
    .filter((p) => !field.value.setAside.has(p.a) && !field.value.setAside.has(p.b))
    .filter((p) => !field.value.hidden.has(p.a) || !field.value.hidden.has(p.b))
    .slice(0, 10),
)
const share = (pair: Forecast["pairs"][number], id: string) => pair.p_win_given_matchup[id] ?? 0
</script>

<template>
  <div class="round2">
    <FieldFilter
      v-model:query="query"
      v-model:show-all="showAll"
      class="card filter rise"
      :field="field"
      :total="forecast.candidates.length"
    />
    <section aria-labelledby="matrix-title" class="card rise">
      <header class="card-head">
        <h2 id="matrix-title">{{ content.matrixTitle }}</h2>
      </header>
      <div class="table-wrap">
        <table class="matrix">
          <thead>
            <tr>
              <th scope="col"><span class="sr-only">{{ common.candidate }}</span></th>
              <th v-for="c in matrix.candidates" :key="c.candidate_id" class="column-head" scope="col">
                <span class="head-in">
                  <CandidateAvatar :id="c.candidate_id" :name="c.name" :size="30" />
                  <span>{{ c.name }}</span>
                </span>
              </th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="row in rows" :key="row.candidate.candidate_id">
              <th scope="row">
                <span class="row-in">
                  <CandidateAvatar :id="row.candidate.candidate_id" :name="row.candidate.name" :size="30" />
                  {{ row.candidate.name }}
                </span>
              </th>
              <td v-for="(c, i) in row.cells" :key="i" class="cell-wrap">
                <div
                  v-if="c"
                  class="cell"
                  :class="{ empty: c.pMatchup <= 0 }"
                  :style="{ background: `var(${steps[stepFor(c.pMatchup)]})` }"
                  :title="c.rowWins !== null
                    ? content.matchupCell({ p: probability(c.pMatchup, locale), w: probability(c.rowWins, locale) }).value
                    : undefined"
                >
                  <template v-if="c.pMatchup > 0">
                    <span class="p num">{{ probability(c.pMatchup, locale) }}</span>
                    <span v-if="c.rowWins !== null" class="w num">{{ probability(c.rowWins, locale) }}</span>
                  </template>
                  <span v-else class="p">—</span>
                </div>
                <div v-else aria-hidden="true" class="cell self" />
              </td>
            </tr>
          </tbody>
        </table>
      </div>
      <div class="legend small">
        <div aria-hidden="true" class="scale">
          <span class="scale-title">{{ content.matrixScale }}</span>
          <span class="ramp">
            <span v-for="step in steps.slice(1)" :key="step" class="ramp-step" :style="{ background: `var(${step})` }" />
          </span>
          <span class="scale-ends num">
            <span>{{ probability(0.005, locale) }}</span>
            <span>{{ probability(maxMatchup, locale) }}</span>
          </span>
        </div>
        <p class="figures">{{ content.matrixFigures }}</p>
      </div>
    </section>

    <section aria-labelledby="top-title" class="card rise" :style="{ '--i': 1 }">
      <header class="card-head">
        <h2 id="top-title">{{ content.topMatchups }}</h2>
      </header>
      <div class="table-wrap">
        <table class="duels">
          <thead>
            <tr>
              <th scope="col">{{ content.pairing }}</th>
              <th class="num" scope="col">{{ content.happens }}</th>
              <th scope="col">{{ content.winner }}</th>
              <th scope="col"><span class="sr-only">{{ common.source }}</span></th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="pair in topPairs" :key="`${pair.a}-${pair.b}`">
              <th scope="row">{{ names.get(pair.a) }} – {{ names.get(pair.b) }}</th>
              <td class="num happens">{{ probability(pair.p_matchup, locale) }}</td>
              <td>
                <div class="duel">
                  <span class="side">
                    <CandidateAvatar :id="pair.a" :name="names.get(pair.a) ?? pair.a" :size="34" />
                    <span class="side-text">
                      <span class="who">{{ names.get(pair.a) }}</span>
                      <strong class="num">{{ probability(share(pair, pair.a), locale) }}</strong>
                    </span>
                  </span>
                  <span aria-hidden="true" class="split">
                    <span class="part" :style="{ flexGrow: share(pair, pair.a) }" />
                    <span class="part" :style="{ flexGrow: share(pair, pair.b) }" />
                    <span class="midline" />
                  </span>
                  <span class="side end">
                    <span class="side-text">
                      <span class="who">{{ names.get(pair.b) }}</span>
                      <strong class="num">{{ probability(share(pair, pair.b), locale) }}</strong>
                    </span>
                    <CandidateAvatar :id="pair.b" :name="names.get(pair.b) ?? pair.b" :size="34" />
                  </span>
                </div>
              </td>
              <td>
                <span class="badge">
                  <template v-if="pair.polled">{{ content.polledBadge }}</template>
                  <template v-else-if="pair.anchor">{{
                      content.anchoredBadge({
                        a: names.get(pair.anchor.a) ?? pair.anchor.a,
                        b: names.get(pair.anchor.b) ?? pair.anchor.b,
                      })
                    }}</template>
                  <template v-else>{{ content.modelledBadge }}</template>
                </span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </section>
  </div>
</template>

<style scoped>
.round2 {
  display: grid;
  gap: clamp(20px, 2.4vw, 28px);
}

.filter {
  padding: 18px 22px;
}

.matrix {
  border-collapse: separate;
  border-spacing: 4px;
}

.matrix th,
.matrix td {
  border-bottom: none;
  padding: 0;
}

.matrix th[scope="row"] {
  white-space: nowrap;
  padding-right: 14px;
  font-weight: 500;
  color: var(--ink);
}

.column-head {
  font-size: 0.66rem;
  text-align: center;
  min-width: 84px;
  padding-bottom: 8px !important;
  vertical-align: bottom;
  white-space: normal;
  line-height: 1.35;
}

.head-in {
  display: grid;
  justify-items: center;
  gap: 8px;
}

.row-in {
  display: flex;
  align-items: center;
  gap: 10px;
}

.cell {
  border-radius: 10px;
  min-height: 52px;
  display: grid;
  place-items: center;
  align-content: center;
  gap: 1px;
  color: var(--ink);
  padding: 6px 4px;
  box-shadow:
    inset 0 1px 0 rgba(255, 255, 255, 0.08),
    inset 0 0 0 1px rgba(255, 255, 255, 0.04);
  transition:
    transform 0.2s var(--ease-out),
    box-shadow 0.2s;
}

.cell:not(.self):hover {
  transform: translateY(-1px);
  box-shadow:
    inset 0 1px 0 rgba(255, 255, 255, 0.16),
    inset 0 0 0 1px rgba(255, 255, 255, 0.22),
    0 8px 20px -8px rgba(0, 0, 0, 0.8);
}

.cell.empty {
  color: var(--muted);
}

.cell.self {
  background: repeating-linear-gradient(135deg, rgba(255, 255, 255, 0.04) 0 1px, transparent 1px 7px);
  box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.04);
}

.p {
  font-weight: 650;
  font-size: 0.92rem;
}

.w {
  font-size: 0.72rem;
  font-weight: 500;
}

.legend {
  display: flex;
  flex-wrap: wrap;
  align-items: flex-end;
  justify-content: space-between;
  gap: 16px 32px;
  margin-top: 22px;
  color: var(--muted);
}

.figures {
  max-width: 40rem;
  color: var(--ink-2);
}

.scale {
  display: grid;
  gap: 6px;
  min-width: min(100%, 300px);
}

.scale-title {
  color: var(--ink-2);
}

.ramp {
  display: flex;
  gap: 2px;
  height: 10px;
}

.ramp-step {
  flex: 1;
}

.ramp-step:first-child {
  border-radius: 3px 0 0 3px;
}

.ramp-step:last-child {
  border-radius: 0 3px 3px 0;
}

.scale-ends {
  display: flex;
  justify-content: space-between;
}

.duels th[scope="row"] {
  font-weight: 500;
  color: var(--ink);
  white-space: nowrap;
}

.happens {
  font-weight: 650;
  font-size: 1rem;
}

.duel {
  display: grid;
  grid-template-columns: minmax(0, 12rem) minmax(120px, 1fr) minmax(0, 12rem);
  align-items: center;
  gap: 14px;
  min-width: 460px;
}

/* Face to face: each side's portrait on its outer edge. */
.side {
  display: flex;
  align-items: center;
  gap: 10px;
  min-width: 0;
  font-size: 0.8rem;
  color: var(--muted);
}

.side.end {
  justify-content: flex-end;
  text-align: right;
}

.side-text {
  display: grid;
  gap: 1px;
  min-width: 0;
}

.side strong {
  font-size: 0.95rem;
  color: var(--ink);
  font-weight: 650;
}

.who {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* One bleu for both sides: where the 2px gap falls against the midline says who is ahead. */
.split {
  position: relative;
  display: flex;
  gap: 2px;
  height: 8px;
}

.part {
  flex-basis: 0;
  min-width: 2px;
  background: var(--series);
  box-shadow: 0 0 10px var(--series-glow);
}

.part:first-child {
  border-radius: 4px 0 0 4px;
}

.part:nth-child(2) {
  border-radius: 0 4px 4px 0;
}

.midline {
  position: absolute;
  left: 50%;
  top: -5px;
  bottom: -5px;
  width: 1px;
  background: var(--axis);
}

@media (max-width: 760px) {
  .matrix th[scope="row"] {
    font-size: 0.8rem;
  }

  .column-head {
    min-width: 64px;
  }

  .cell {
    min-height: 46px;
  }
}
</style>
