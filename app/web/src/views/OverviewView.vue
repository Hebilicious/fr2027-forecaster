<script setup lang="ts">
import { computed } from "vue"
import type { Forecast } from "../api/schemas"
import { points, probability, shareWithInterval } from "../format"
import { useI18n } from "../i18n"

const props = defineProps<{ forecast: Forecast }>()
const { t, locale } = useI18n()

const maxShare = computed(() => Math.max(0.05, ...props.forecast.candidates.map((c) => c.r1_share?.hi80 ?? 0)))
const names = computed(() => new Map(props.forecast.candidates.map((c) => [c.candidate_id, c.name])))
const movers = computed(() => props.forecast.changes.p_win_delta.filter((d) => Math.abs(d.delta) >= 0.005).slice(0, 6))
const pct = (x: number) => `${(x * 100).toFixed(1)}%`
</script>

<template>
  <section class="stack">
    <p class="notice" role="note">
      <span aria-hidden="true" class="icon">i</span>
      <span>{{ t("uncertainty") }}</span>
    </p>

    <div class="card table-wrap">
      <table>
        <thead>
          <tr>
            <th scope="col">{{ t("candidate") }}</th>
            <th class="num" scope="col">{{ t("runs") }}</th>
            <th scope="col">{{ t("qualifies") }}</th>
            <th scope="col">{{ t("wins") }}</th>
            <th scope="col">{{ t("r1Share") }}</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="c in forecast.candidates" :key="c.candidate_id">
            <th scope="row">
              <span class="name">{{ c.name }}</span>
              <span class="muted small party">{{ c.party }}</span>
            </th>
            <td class="num">{{ probability(c.p_run, locale) }}</td>
            <td>
              <div class="bar-cell">
                <span class="bar-track"><span class="bar" :style="{ width: pct(c.p_qualify_r1) }" /></span>
                <span class="num value">{{ probability(c.p_qualify_r1, locale) }}</span>
              </div>
            </td>
            <td>
              <div class="bar-cell">
                <span class="bar-track"><span class="bar" :style="{ width: pct(c.p_win) }" /></span>
                <span class="num value strong">{{ probability(c.p_win, locale) }}</span>
              </div>
            </td>
            <td>
              <div v-if="c.r1_share" class="range-cell">
                <svg aria-hidden="true" class="range" preserveAspectRatio="none" viewBox="0 0 100 12">
                  <rect
                    :x="(c.r1_share.lo80 / maxShare) * 100"
                    y="3"
                    :width="((c.r1_share.hi80 - c.r1_share.lo80) / maxShare) * 100"
                    class="range-band"
                    height="6"
                    rx="3"
                  />
                  <line
                    :x1="(c.r1_share.mean / maxShare) * 100"
                    :x2="(c.r1_share.mean / maxShare) * 100"
                    class="range-mean"
                    y1="1"
                    y2="11"
                  />
                </svg>
                <span class="num value">{{
                    shareWithInterval(c.r1_share.mean, c.r1_share.lo80, c.r1_share.hi80, locale)
                  }}</span>
              </div>
              <span v-else class="muted small">{{ c.simulated ? "—" : t("notPolled") }}</span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div class="grid-2">
      <div class="card stack">
        <h2>{{ t("whatMoved") }}</h2>
        <template v-if="forecast.changes.previous">
          <ul v-if="movers.length" class="movers">
            <li v-for="d in movers" :key="d.candidate_id">
              <span>{{ names.get(d.candidate_id) ?? d.candidate_id }}</span>
              <span class="num strong">{{ points(d.delta, locale) }}</span>
            </li>
          </ul>
          <p v-else class="secondary">{{ t("noChange") }}</p>
          <p class="small secondary">
            {{
              forecast.changes.new_polls.length
                ? t("newPolls", { list: forecast.changes.new_polls.join(", ") })
                : t("noNewPolls")
            }}
          </p>
        </template>
        <p v-else class="secondary">{{ t("noPrevious") }}</p>
      </div>
      <div class="card stack">
        <p class="small secondary">
          {{
            t("modelNote", {
              version: forecast.model_version,
              polls: forecast.aggregation.polls_used,
              scenarios: forecast.aggregation.scenarios_used,
              sims: forecast.simulations.toLocaleString(locale === "fr" ? "fr-FR" : "en-GB"),
            })
          }}
        </p>
        <template v-if="forecast.warnings.length">
          <h3>{{ t("warnings") }}</h3>
          <ul class="small secondary warnings">
            <li v-for="w in forecast.warnings" :key="w">{{ w }}</li>
          </ul>
        </template>
      </div>
    </div>
  </section>
</template>

<style scoped>
th[scope="row"] {
  font-weight: 600;
  color: var(--ink);
  font-size: 0.95rem;
}

.party {
  display: block;
  font-weight: 400;
}

.bar-cell,
.range-cell {
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 150px;
}

.bar-track {
  flex: 1;
  height: 10px;
  min-width: 60px;
  background: var(--grid);
  border-radius: 0 4px 4px 0;
  overflow: hidden;
}

.bar {
  display: block;
  height: 100%;
  background: var(--series);
  border-radius: 0 4px 4px 0;
}

.value {
  min-width: 3.2em;
  text-align: right;
  font-size: 0.9rem;
}

.strong {
  font-weight: 600;
}

.range {
  flex: 1;
  min-width: 60px;
  height: 12px;
}

.range-band {
  fill: var(--series-wash);
  stroke: var(--series);
  stroke-width: 0;
}

.range-mean {
  stroke: var(--series);
  stroke-width: 2;
  vector-effect: non-scaling-stroke;
}

.range-cell .value {
  min-width: 7.5em;
}

@media (max-width: 720px) {
  .bar-track,
  .range {
    display: none;
  }

  .bar-cell,
  .range-cell {
    min-width: 0;
  }

  .value,
  .range-cell .value {
    min-width: 0;
  }

  th,
  td {
    padding: 8px 6px;
  }
}

.grid-2 {
  display: grid;
  gap: 16px;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  align-items: start;
}

.movers {
  list-style: none;
  padding: 0;
  margin: 0;
  display: grid;
  gap: 4px;
}

.movers li {
  display: flex;
  justify-content: space-between;
  gap: 12px;
}

.warnings {
  margin: 0;
  padding-left: 18px;
}
</style>
