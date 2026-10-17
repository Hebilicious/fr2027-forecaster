<script setup lang="ts">
import { computed } from "vue"
import { useIntlayer, useLocale } from "vue-intlayer"
import type { Blend, Forecast, Forward, ForwardFlag } from "../../api/schemas"
import { count, decimal, percent, points, relativeChange, shortDate } from "../../format"
import { labelOf } from "../../labels"
import { nameOf } from "../../signals"

const props = defineProps<{
  forecast: Forecast
  blend: Blend | null
  forward: Forward
  names: ReadonlyMap<string, string>
}>()
const content = useIntlayer("model-use-section")
const { locale } = useLocale()

const signals = computed(() => props.forecast.signals)
const marketWeight = computed(() => (props.blend ? 1 - props.blend.model_weight : 0))
const ballotWeight = computed(() => (props.forecast.ballot ? 1 - props.forecast.ballot.hand_weight : 0))
const summary = computed(() => props.forward.summary)

// A spike's size as how far the day stood above its baseline; an X shift in points of support.
function size(flag: ForwardFlag): string {
  return flag.kind === "spike"
    ? relativeChange(Math.exp(flag.score) - 1, locale.value)
    : points(flag.score, locale.value)
}
</script>

<template>
  <section aria-labelledby="model-use-title" class="card">
    <header class="card-head">
      <h2 id="model-use-title">{{ content.title }}</h2>
      <p class="secondary small">{{ content.intro }}</p>
    </header>
    <div class="table-wrap">
      <table class="weights">
        <thead>
          <tr>
            <th scope="col">{{ content.layer }}</th>
            <th class="num" scope="col">{{ content.weight }}</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <th scope="row">{{ content.spikes }}</th>
            <td class="num">{{ signals && signals.shock_sd > 0 ? decimal(signals.shock_sd, 2, locale) : content.off }}</td>
          </tr>
          <tr>
            <th scope="row">{{ content.momentum }}</th>
            <td class="num">
              {{ signals && signals.momentum_weight !== 0 ? decimal(signals.momentum_weight, 2, locale) : content.off }}
            </td>
          </tr>
          <tr>
            <th scope="row">{{ content.xMomentum }}</th>
            <td class="num">
              {{ signals?.x_momentum_weight ? decimal(signals.x_momentum_weight, 2, locale) : content.off }}
            </td>
          </tr>
          <tr>
            <th scope="row">{{ content.ballot }}</th>
            <td class="num">{{ ballotWeight > 0 ? percent(ballotWeight, 0, locale) : content.off }}</td>
          </tr>
          <tr>
            <th scope="row">{{ content.blend }}</th>
            <td class="num">{{ marketWeight > 0 ? percent(marketWeight, 0, locale) : content.off }}</td>
          </tr>
        </tbody>
      </table>
    </div>
    <p v-if="signals" class="secondary small">
      {{ content.recentSpikes({ count: count(signals.spikes.length, locale) }) }}
    </p>

    <h3 class="sub">{{ content.forwardTitle }}</h3>
    <p class="secondary small">{{ content.forwardHelp({ days: count(forward.horizon_days, locale) }) }}</p>
    <p v-if="summary.spikes_scored === 0 && summary.x_shifts_scored === 0" class="secondary small">
      {{ content.forwardNone }}
    </p>
    <template v-else>
      <p v-if="summary.spikes_scored > 0 && summary.spike_mean_move !== null && summary.typical_mean_move !== null">
        {{
          content.forwardSpikes({
            count: count(summary.spikes_scored, locale),
            after: percent(summary.spike_mean_move, 1, locale),
            typical: percent(summary.typical_mean_move, 1, locale),
          })
        }}
      </p>
      <p v-if="summary.x_shifts_scored > 0">
        {{
          content.forwardX({
            agree: count(summary.x_shifts_agreeing, locale),
            count: count(summary.x_shifts_scored, locale),
          })
        }}
      </p>
    </template>
    <div v-if="forward.flags.length" class="table-wrap">
      <table class="flags small">
        <thead>
          <tr>
            <th scope="col">{{ content.date }}</th>
            <th scope="col">{{ content.signal }}</th>
            <th class="num" scope="col">{{ content.strength }}</th>
            <th class="num" scope="col">{{ content.after({ days: count(forward.horizon_days, locale) }) }}</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="f in forward.flags" :key="`${f.kind}:${f.candidate_id}:${f.date}`">
            <td class="num">{{ shortDate(f.date, locale) }}</td>
            <th scope="row">
              {{ nameOf(names, f.candidate_id) }}
              <span class="secondary">{{ labelOf(content.kinds, f.kind) }}</span>
            </th>
            <td class="num">{{ size(f) }}</td>
            <td class="num" :class="{ muted: f.change === null }">
              {{ f.change === null ? content.pending : points(f.change, locale) }}
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </section>
</template>

<style scoped>
.weights th[scope="row"], .flags th[scope="row"] {
  font-weight: 400;
}

.flags th[scope="row"] .secondary {
  display: block;
  font-size: 0.8rem;
}

.sub {
  margin-top: 20px;
}
</style>
