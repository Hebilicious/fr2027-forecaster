<script setup lang="ts">
import { computed } from "vue"
import type { Forecast } from "../api/schemas"
import { round2Matrix, type MatrixCell } from "../charts"
import { probability } from "../format"
import { useI18n } from "../i18n"

const props = defineProps<{ forecast: Forecast }>()
const { t, locale } = useI18n()

const matrix = computed(() => round2Matrix(props.forecast, 8))
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

// Sequential blue by how likely the pairing is; the darkest steps carry white text.
const steps = ["--seq-0", "--seq-1", "--seq-2", "--seq-3", "--seq-4", "--seq-5", "--seq-6", "--seq-7"]
const maxMatchup = computed(() => Math.max(0.01, ...props.forecast.pairs.map((p) => p.p_matchup)))
function stepFor(p: number): number {
  if (p <= 0) return 0
  return Math.min(steps.length - 1, 1 + Math.floor((p / maxMatchup.value) * (steps.length - 1.0001)))
}
const topPairs = computed(() => props.forecast.pairs.slice(0, 10))
</script>

<template>
  <section class="stack">
    <div class="stack">
      <h2>{{ t("round2Title") }}</h2>
      <p class="secondary small">{{ t("round2Help") }}</p>
    </div>

    <div class="card table-wrap">
      <table class="matrix">
        <thead>
          <tr>
            <th scope="col"><span class="sr-only">{{ t("candidate") }}</span></th>
            <th v-for="c in matrix.candidates" :key="c.candidate_id" class="column-head" scope="col">{{ c.name }}</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="row in rows" :key="row.candidate.candidate_id">
            <th scope="row">{{ row.candidate.name }}</th>
            <td v-for="(c, i) in row.cells" :key="i" class="cell-wrap">
              <div
                v-if="c"
                class="cell"
                :class="{ dark: stepFor(c.pMatchup) >= 5 }"
                :style="{ background: `var(${steps[stepFor(c.pMatchup)]})` }"
                :title="c.rowWins !== null
                  ? t('matchupCell', { p: probability(c.pMatchup, locale), w: probability(c.rowWins, locale) })
                  : undefined"
              >
                <template v-if="c.pMatchup > 0">
                  <span class="p num">{{ probability(c.pMatchup, locale) }}</span>
                  <span v-if="c.rowWins !== null" class="w num">{{ probability(c.rowWins, locale) }}</span>
                </template>
                <span v-else class="p">—</span>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
      <p class="small muted legend">
        <span class="swatch" style="background: var(--seq-2)" /> {{ t("happens") }} ·
        <span class="num">→</span> {{ t("winner") }}
      </p>
    </div>

    <div class="card table-wrap">
      <h2>{{ t("topMatchups") }}</h2>
      <table>
        <thead>
          <tr>
            <th scope="col">{{ t("pairing") }}</th>
            <th class="num" scope="col">{{ t("happens") }}</th>
            <th scope="col">{{ t("winner") }}</th>
            <th scope="col"><span class="sr-only">{{ t("source") }}</span></th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="pair in topPairs" :key="`${pair.a}-${pair.b}`">
            <td>{{ names.get(pair.a) }} – {{ names.get(pair.b) }}</td>
            <td class="num">{{ probability(pair.p_matchup, locale) }}</td>
            <td class="num">
              {{ names.get(pair.a) }} {{ probability(pair.p_win_given_matchup[pair.a] ?? 0, locale) }} ·
              {{ names.get(pair.b) }} {{ probability(pair.p_win_given_matchup[pair.b] ?? 0, locale) }}
            </td>
            <td>
              <span class="badge">{{ pair.polled ? t("polledBadge") : t("modelledBadge") }}</span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </section>
</template>

<style scoped>
.matrix th[scope="row"] {
  white-space: nowrap;
  color: var(--ink);
}

.column-head {
  font-size: 0.78rem;
  text-align: center;
  min-width: 84px;
}

.cell-wrap {
  padding: 2px;
  border-bottom: none;
}

.cell {
  border-radius: 4px;
  min-height: 44px;
  display: grid;
  place-items: center;
  align-content: center;
  color: var(--ink);
  padding: 4px;
}

.cell.dark {
  color: #ffffff;
}

.cell .p {
  font-weight: 600;
  font-size: 0.9rem;
}

.cell .w {
  font-size: 0.75rem;
}

.legend {
  margin-top: 8px;
}

.swatch {
  display: inline-block;
  width: 10px;
  height: 10px;
  border-radius: 2px;
  vertical-align: middle;
}

h2 {
  margin-bottom: 8px;
}
</style>
