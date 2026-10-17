<script setup lang="ts">
import { computed, toRef } from "vue"
import { useIntlayer, useLocale } from "vue-intlayer"
import type { Blend, Forecast, HistoryRun, TrackRecord } from "../api/schemas"
import { blendAt } from "../blend"
import CandidateAvatar from "../components/CandidateAvatar.vue"
import ModelExplorer from "../components/explorer/ModelExplorer.vue"
import MethodSection from "../components/MethodSection.vue"
import FieldFilter from "../components/FieldFilter.vue"
import MarketWeight from "../components/MarketWeight.vue"
import SignalWeight from "../components/SignalWeight.vue"
import OutcomeDots from "../components/OutcomeDots.vue"
import { count, daysUntil, list, percent, points, probability, shareWithInterval } from "../format"
import { useFieldFilter } from "../useFieldFilter"
import { useMarketWeight } from "../useMarketWeight"
import { useSignalScale } from "../useSignalScale"

const props = defineProps<{
  forecast: Forecast
  blend: Blend | null
  history: readonly HistoryRun[]
  trackRecord: TrackRecord | null
}>()
const content = useIntlayer("overview-view")
const common = useIntlayer("common")
const { locale } = useLocale()

// The markets' weight: the reader's slider, or the site's published one.
const chosenWeight = useMarketWeight()
const publishedWeight = computed(() => (props.blend ? 1 - props.blend.model_weight : 0))
const marketWeight = computed({
  get: () => chosenWeight.value ?? publishedWeight.value,
  set: (w: number) => {
    chosenWeight.value = w
  },
})
// The news and social signals' strength: the forecast was run at each of these.
const signalScale = useSignalScale()
const scales = computed(() => (props.forecast.signal_levels ?? []).map((l) => l.scale))
const levelModel = computed(() => {
  const level = props.forecast.signal_levels?.find((l) => l.scale === signalScale.value)
  return level
    ? new Map(level.candidates.map((c) => [c.candidate_id, { win: c.p_win, qualify: c.p_qualify_r1 }]))
    : undefined
})
const blended = computed(() =>
  props.blend ? blendAt(props.blend, 1 - marketWeight.value, levelModel.value) : (levelModel.value ?? null),
)
// The forecast as shown: chances of winning and of reaching round 2 blended at the slider's
// weight, candidates in that order.
const shown = computed<Forecast>(() => {
  const b = blended.value
  if (!b) return props.forecast
  const candidates = props.forecast.candidates
    .map((c) => {
      const x = b.get(c.candidate_id)
      return x ? { ...c, p_win: x.win, p_qualify_r1: x.qualify } : c
    })
    .toSorted((a, c) => c.p_win - a.p_win || c.p_qualify_r1 - a.p_qualify_r1)
  return { ...props.forecast, candidates }
})
const { query, showAll, field } = useFieldFilter(toRef(() => shown.value.candidates))

const maxShare = computed(() => Math.max(0.05, ...props.forecast.candidates.map((c) => c.r1_share?.hi80 ?? 0)))
const names = computed(() => new Map(props.forecast.candidates.map((c) => [c.candidate_id, c.name])))
const movers = computed(() =>
  props.forecast.changes.p_win_delta
    .filter((d) => Math.abs(d.delta) >= 0.005 && !field.value.setAside.has(d.candidate_id))
    .slice(0, 6),
)
const daysLeft = computed(() => daysUntil(props.forecast.election.round1, new Date()))
const horizon = computed(() => content.horizon({ days: count(daysLeft.value, locale.value) }))
const newSpikes = computed(() => [
  ...new Set((props.forecast.changes.new_spikes ?? []).map((s) => names.value.get(s.candidate_id) ?? s.candidate_id)),
])
const pct = (x: number) => `${(x * 100).toFixed(1)}%`
const at = (share: number) => (share / maxShare.value) * 100
// Everyone the model or the markets give at least half a point to win, in the forecast's order,
// leaving out those the field filter sets aside.
const blendRows = computed(() => {
  const rows = new Map((props.blend?.candidates ?? []).map((c) => [c.candidate_id, c]))
  return shown.value.candidates.flatMap((c) => {
    const row = field.value.setAside.has(c.candidate_id) ? undefined : rows.get(c.candidate_id)
    const blendedRow = blended.value?.get(c.candidate_id)
    const model = levelModel.value?.get(c.candidate_id)
    const shownRow = row && model ? { ...row, model: { ...row.model, win: model.win, qualify: model.qualify } } : row
    return shownRow && blendedRow && Math.max(shownRow.market.win ?? 0, shownRow.model.win) >= 0.005
      ? [{ ...shownRow, blended: blendedRow, name: c.name }]
      : []
  })
})
const maybe = (p: number | null) => (p === null ? "—" : probability(p, locale.value))
</script>

<template>
  <div class="overview">
    <section aria-labelledby="outcomes-title" class="card over-painting outcomes-card rise">
      <header class="card-head">
        <p class="eyebrow"><span class="tricolore-rule" />{{ content.outcomesEyebrow }}</p>
        <h2 id="outcomes-title">{{ content.outcomesTitle }}</h2>
        <p class="secondary">{{ content.outcomesHelp({ sims: count(forecast.simulations, locale) }) }}</p>
      </header>
      <SignalWeight v-if="scales.length > 1" v-model="signalScale" :scales="scales" />
      <MarketWeight v-if="blend" v-model="marketWeight" :blend="blend" :published="publishedWeight" />
      <!-- The dots ignore the search, which is for finding a row, but pool whoever the toggle sets aside. -->
      <OutcomeDots :forecast="shown" :pool="field.setAside" />
      <p class="notice note" role="note">
        <span aria-hidden="true" class="icon" translate="no">i</span>
        <span>{{ content.uncertainty }}</span>
      </p>
      <p v-if="daysLeft > 0" class="notice note" role="note">
        <span aria-hidden="true" class="icon" translate="no">i</span>
        <span>{{ horizon.value }}</span>
      </p>
    </section>

    <ModelExplorer :blend="blend" :forecast="forecast" :history="history" :track-record="trackRecord" />

    <section aria-labelledby="field-title" class="card rise" :style="{ '--i': 2 }">
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

    <section v-if="blend && blendRows.length" aria-labelledby="blend-title" class="card rise" :style="{ '--i': 2 }">
      <header class="card-head">
        <p class="eyebrow">{{ content.blendEyebrow }}</p>
        <h2 id="blend-title">{{ content.blendTitle }}</h2>
      </header>
      <div class="table-wrap">
        <table class="blend">
          <thead>
            <tr>
              <th rowspan="2" scope="col">{{ common.candidate }}</th>
              <th class="group" colspan="3" scope="colgroup">{{ common.wins }}</th>
              <th class="group" colspan="3" scope="colgroup">{{ common.qualifies }}</th>
            </tr>
            <tr>
              <template v-for="q in ['win', 'qualify']" :key="q">
                <th class="num" scope="col">{{ content.model }}</th>
                <th class="num" scope="col">{{ content.markets }}</th>
                <th class="num" scope="col">{{ content.blend }}</th>
              </template>
            </tr>
          </thead>
          <tbody>
            <tr v-for="row in blendRows" :key="row.candidate_id">
              <th scope="row">
                <span class="who">
                  <CandidateAvatar :id="row.candidate_id" :name="row.name" :size="26" />
                  {{ row.name }}
                </span>
              </th>
              <td class="num">{{ probability(row.model.win, locale) }}</td>
              <td class="num secondary">{{ maybe(row.market.win) }}</td>
              <td class="num strong">{{ probability(row.blended.win, locale) }}</td>
              <td class="num">{{ probability(row.model.qualify, locale) }}</td>
              <td class="num secondary">{{ maybe(row.market.qualify) }}</td>
              <td class="num strong">{{ probability(row.blended.qualify, locale) }}</td>
            </tr>
          </tbody>
        </table>
      </div>
      <p class="secondary small blend-note">
        {{ content.blendNote({ weight: percent(1 - marketWeight, 0, locale) }) }}
      </p>
    </section>

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
        <p v-if="newSpikes.length" class="small muted polls">
          {{ content.newSpikes({ list: list(newSpikes, locale) }) }}
        </p>
      </template>
      <p v-else class="secondary">{{ content.noPrevious }}</p>
    </section>

    <MethodSection :blend="blend" :forecast="forecast" />
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

.note + .note {
  margin-top: 10px;
}

.blend th.group {
  text-align: center;
}

.blend-note {
  margin-top: 12px;
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
