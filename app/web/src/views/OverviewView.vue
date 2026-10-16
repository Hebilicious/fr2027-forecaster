<script setup lang="ts">
import { computed, toRef } from "vue"
import { useIntlayer, useLocale } from "vue-intlayer"
import type { Forecast } from "../api/schemas"
import CandidateAvatar from "../components/CandidateAvatar.vue"
import FieldFilter from "../components/FieldFilter.vue"
import OutcomeDots from "../components/OutcomeDots.vue"
import { count, list, points, probability, shareWithInterval } from "../format"
import { useFieldFilter } from "../useFieldFilter"

const props = defineProps<{ forecast: Forecast }>()
const content = useIntlayer("overview-view")
const common = useIntlayer("common")
const { locale } = useLocale()
const { query, showAll, field } = useFieldFilter(toRef(() => props.forecast.candidates))

const maxShare = computed(() => Math.max(0.05, ...props.forecast.candidates.map((c) => c.r1_share?.hi80 ?? 0)))
const names = computed(() => new Map(props.forecast.candidates.map((c) => [c.candidate_id, c.name])))
const movers = computed(() =>
  props.forecast.changes.p_win_delta
    .filter((d) => Math.abs(d.delta) >= 0.005 && !field.value.setAside.has(d.candidate_id))
    .slice(0, 6),
)
const pct = (x: number) => `${(x * 100).toFixed(1)}%`
const at = (share: number) => (share / maxShare.value) * 100
</script>

<template>
  <div class="overview">
    <section aria-labelledby="outcomes-title" class="card over-painting outcomes-card rise">
      <header class="card-head">
        <p class="eyebrow"><span class="tricolore-rule" />{{ content.outcomesEyebrow }}</p>
        <h2 id="outcomes-title">{{ content.outcomesTitle }}</h2>
        <p class="secondary">{{ content.outcomesHelp({ sims: count(forecast.simulations, locale) }) }}</p>
      </header>
      <!-- The dots ignore the search, which is for finding a row, but pool whoever the toggle sets aside. -->
      <OutcomeDots :forecast="forecast" :pool="field.setAside" />
      <p class="notice note" role="note">
        <span aria-hidden="true" class="icon" translate="no">i</span>
        <span>{{ content.uncertainty }}</span>
      </p>
    </section>

    <section aria-labelledby="field-title" class="card rise" :style="{ '--i': 1 }">
      <header class="card-head">
        <p class="eyebrow">{{ content.fieldEyebrow }}</p>
        <h2 id="field-title">{{ content.fieldTitle }}</h2>
      </header>
      <FieldFilter
        v-model:query="query"
        v-model:show-all="showAll"
        class="filter"
        :field="field"
        :total="forecast.candidates.length"
      />
      <div v-if="field.visible.length" class="table-wrap">
        <table class="field">
          <thead>
            <tr>
              <th scope="col">{{ common.candidate }}</th>
              <th scope="col">{{ common.wins }}</th>
              <th scope="col">{{ common.qualifies }}</th>
              <th class="num" scope="col">{{ common.runs }}</th>
              <th scope="col">{{ common.r1Share }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(c, i) in field.visible" :key="c.candidate_id" :style="{ '--i': i }">
              <th scope="row">
                <span class="who">
                  <CandidateAvatar :id="c.candidate_id" :name="c.name" :size="36" />
                  <span>
                    <span class="name">{{ c.name }}</span>
                    <span class="party">{{ c.party }}</span>
                  </span>
                </span>
              </th>
              <td>
                <div class="bar-cell">
                  <span class="bar-track"><span class="bar" :style="{ width: pct(c.p_win) }" /></span>
                  <span class="num value strong">{{ probability(c.p_win, locale) }}</span>
                </div>
              </td>
              <td>
                <div class="bar-cell">
                  <span class="bar-track"><span class="bar" :style="{ width: pct(c.p_qualify_r1) }" /></span>
                  <span class="num value">{{ probability(c.p_qualify_r1, locale) }}</span>
                </div>
              </td>
              <td class="num runs">{{ probability(c.p_run, locale) }}</td>
              <td>
                <div v-if="c.r1_share" class="range-cell">
                  <svg aria-hidden="true" class="range" preserveAspectRatio="none" viewBox="0 0 100 12">
                    <line class="range-track" x1="0" x2="100" y1="6" y2="6" />
                    <rect
                      :x="at(c.r1_share.lo80)"
                      y="2.5"
                      :width="at(c.r1_share.hi80) - at(c.r1_share.lo80)"
                      class="range-band"
                      height="7"
                      rx="3.5"
                    />
                    <line :x1="at(c.r1_share.mean)" :x2="at(c.r1_share.mean)" class="range-mean" y1="0.5" y2="11.5" />
                  </svg>
                  <span class="num value">{{
                      shareWithInterval(c.r1_share.mean, c.r1_share.lo80, c.r1_share.hi80, locale)
                    }}</span>
                </div>
                <span v-else class="muted small">{{ c.simulated ? "—" : common.notPolled }}</span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </section>

    <div class="grid-2">
      <section aria-labelledby="moved-title" class="card rise" :style="{ '--i': 2 }">
        <header class="card-head">
          <h2 id="moved-title">{{ content.whatMoved }}</h2>
        </header>
        <template v-if="forecast.changes.previous">
          <ul v-if="movers.length" class="movers">
            <li v-for="d in movers" :key="d.candidate_id">
              <span class="who">
                <CandidateAvatar :id="d.candidate_id" :name="names.get(d.candidate_id) ?? d.candidate_id" :size="24" />
                {{ names.get(d.candidate_id) ?? d.candidate_id }}
              </span>
              <span class="delta num">
                <span aria-hidden="true" class="arrow">{{ d.delta > 0 ? "▲" : "▼" }}</span>
                {{ points(d.delta, locale) }}
              </span>
            </li>
          </ul>
          <p v-else class="secondary">{{ content.noChange }}</p>
          <p class="small muted polls">
            {{
              forecast.changes.new_polls.length
                ? content.newPolls({ list: list(forecast.changes.new_polls, locale) })
                : content.noNewPolls
            }}
          </p>
        </template>
        <p v-else class="secondary">{{ content.noPrevious }}</p>
      </section>

      <section aria-labelledby="model-title" class="card rise" :style="{ '--i': 3 }">
        <header class="card-head">
          <h2 id="model-title">{{ content.modelTitle }}</h2>
        </header>
        <p class="secondary model-note">
          {{
            content.modelNote({
              version: forecast.model_version,
              polls: forecast.aggregation.polls_used,
              scenarios: forecast.aggregation.scenarios_used,
              sims: count(forecast.simulations, locale),
            })
          }}
        </p>
        <template v-if="forecast.warnings.length">
          <h3 class="warnings-title">{{ content.warnings }}</h3>
          <ul class="warnings">
            <li v-for="w in forecast.warnings" :key="w" class="notice">
              <span aria-hidden="true" class="icon">!</span>
              <span>{{ w }}</span>
            </li>
          </ul>
        </template>
      </section>
    </div>
  </div>
</template>

<style scoped>
.overview {
  display: grid;
  gap: clamp(20px, 2.4vw, 28px);
}

/* The first card rises over the bottom of the painting. */
.outcomes-card {
  margin-top: -132px;
}

.filter {
  margin-bottom: 18px;
}

.field th[scope="row"] {
  font-weight: 400;
  min-width: 13rem;
}

.who {
  display: flex;
  align-items: center;
  gap: 12px;
}

.name {
  display: block;
  font-weight: 600;
  font-size: 0.95rem;
  color: var(--ink);
}

.party {
  display: block;
  font-size: 0.8rem;
  color: var(--muted);
}

.bar-cell,
.range-cell {
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 160px;
}

.bar-track {
  flex: 1;
  height: 6px;
  min-width: 64px;
  background: rgba(243, 239, 230, 0.07);
  border-radius: 0 3px 3px 0;
}

.bar {
  display: block;
  height: 100%;
  min-width: 2px;
  background: var(--series);
  border-radius: 0 3px 3px 0;
  box-shadow: 0 0 10px var(--series-glow);
  transform-origin: left;
  animation: grow 1.1s var(--ease-out) both;
  animation-delay: calc(var(--i, 0) * 22ms + 250ms);
}

.value {
  min-width: 3.2em;
  text-align: right;
  font-size: 0.9rem;
  color: var(--ink-2);
}

.value.strong {
  font-weight: 650;
  color: var(--ink);
}

.runs {
  color: var(--ink-2);
}

.range {
  flex: 1;
  min-width: 64px;
  height: 12px;
  overflow: visible;
}

.range-track {
  stroke: rgba(243, 239, 230, 0.07);
  stroke-width: 1;
  vector-effect: non-scaling-stroke;
}

.range-band {
  fill: var(--series-wash);
  stroke: rgba(88, 124, 241, 0.55);
  stroke-width: 1;
  vector-effect: non-scaling-stroke;
}

.range-mean {
  stroke: var(--ink);
  stroke-width: 2;
  vector-effect: non-scaling-stroke;
}

.range-cell .value {
  min-width: 7.5em;
}

.movers {
  list-style: none;
  padding: 0;
  margin: 0 0 16px;
  display: grid;
}

.movers li {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  padding: 10px 0;
  border-bottom: 1px solid var(--grid);
}

.movers li:last-child {
  border-bottom: none;
}

.delta {
  font-weight: 600;
  white-space: nowrap;
}

.arrow {
  font-size: 0.62rem;
  color: var(--muted);
  margin-right: 4px;
  vertical-align: 1px;
}

.polls {
  overflow-wrap: anywhere;
}

.model-note {
  line-height: 1.65;
}

.warnings-title {
  margin: 20px 0 10px;
  font-size: 0.72rem;
  letter-spacing: 0.14em;
  text-transform: uppercase;
  color: var(--muted);
}

.warnings {
  list-style: none;
  margin: 0;
  padding: 0;
  display: grid;
  gap: 8px;
}

@media (max-width: 760px) {
  .outcomes-card {
    margin-top: -64px;
  }

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

  .field th[scope="row"] {
    min-width: 11rem;
  }

  th,
  td {
    padding: 10px 6px;
  }
}
</style>
