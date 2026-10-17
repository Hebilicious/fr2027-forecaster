<script setup lang="ts">
import { computed } from "vue"
import { useIntlayer, useLocale } from "vue-intlayer"
import type { Forecast, Health, Poll } from "../api/schemas"
import { count, dateTime, longDate, relativeTime, safeUrl, shortDate } from "../format"
import { labelOf } from "../labels"
import { useNow } from "../useNow"

const props = defineProps<{ forecast: Forecast; polls: readonly Poll[]; health: Health | null }>()
const content = useIntlayer("sources-view")
const common = useIntlayer("common")
const { locale } = useLocale()
const now = useNow()
// A source dated by publication (polls) has a day, not a time.
const isDay = (value: string) => /^\d{4}-\d{2}-\d{2}$/.test(value)

interface PollSummary {
  id: string
  firm: string
  sponsor: string | null
  fieldStart: string
  fieldEnd: string
  published: string
  sample: number
  ballots: number
  entry: string
  source: string
  notice: string | null
}

const summaries = computed<PollSummary[]>(() =>
  props.polls
    .map((poll) => ({
      id: poll.poll_id,
      firm: poll.firm,
      sponsor: poll.sponsor,
      fieldStart: poll.field_start,
      fieldEnd: poll.field_end,
      published: poll.published_at,
      sample: poll.sample_size,
      ballots: poll.scenarios.length,
      entry: poll.entry,
      source: poll.source_url,
      notice: poll.notice_url,
    }))
    .toSorted((a, b) => b.published.localeCompare(a.published) || a.id.localeCompare(b.id)),
)
const used = computed(() => new Set(props.forecast.aggregation.poll_ids))
const quarantined = computed(() =>
  Object.entries(props.health?.quarantine ?? {}).flatMap(([area, files]) => files.map((f) => `${area}/${f}`)),
)
</script>

<template>
  <div class="sources">
    <div class="grid-2">
      <section aria-labelledby="run-title" class="card rise" :style="{ '--i': 1 }">
        <header class="card-head">
          <h2 id="run-title">{{ content.latestRun }}</h2>
        </header>
        <dl class="facts">
          <dt>{{ content.file }}</dt>
          <dd class="mono" translate="no">data/forecasts/{{ health?.latest_forecast?.file ?? "—" }}</dd>
          <dt>{{ content.generatedAt }}</dt>
          <dd :title="dateTime(forecast.generated_at, locale)">{{ relativeTime(forecast.generated_at, now, locale) }}</dd>
          <dt>{{ content.lastFieldwork }}</dt>
          <dd>{{ forecast.aggregation.last_field_end ? longDate(forecast.aggregation.last_field_end, locale) : "—" }}</dd>
          <dt translate="no">inputs_hash</dt>
          <dd class="mono hash">{{ forecast.inputs_hash }}</dd>
        </dl>
      </section>

      <section aria-labelledby="collectors-title" class="card rise" :style="{ '--i': 2 }">
        <header class="card-head">
          <h2 id="collectors-title">{{ content.collectorsTitle }}</h2>
        </header>
        <div class="table-wrap">
          <table class="collectors">
            <thead>
              <tr>
                <th scope="col">{{ common.source }}</th>
                <th scope="col">{{ content.cadence }}</th>
                <th scope="col">{{ content.lastUpdate }}</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="c in health?.collectors ?? []" :key="c.name">
                <th scope="row">{{ labelOf(content.collectors, c.name) }}</th>
                <td class="secondary">{{ content.cadences[c.cadence] }}</td>
                <td>
                  <span v-if="!c.updated" class="when muted">{{ content.never }}</span>
                  <span v-else-if="isDay(c.updated)" class="when">{{ shortDate(c.updated, locale) }}</span>
                  <span v-else class="when" :title="dateTime(c.updated, locale)">{{ relativeTime(c.updated, now, locale) }}</span>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
        <h3 class="sub">{{ content.quarantine }}</h3>
        <ul v-if="quarantined.length" class="quarantine small">
          <li v-for="q in quarantined" :key="q" class="mono" translate="no">data/quarantine/{{ q }}</li>
        </ul>
        <p v-else class="small secondary">{{ common.none }}</p>
      </section>
    </div>

    <section aria-labelledby="polls-title" class="card rise" :style="{ '--i': 3 }">
      <header class="card-head">
        <h2 id="polls-title">{{ content.pollsTitle }}</h2>
      </header>
      <div class="table-wrap">
        <table class="polls">
          <thead>
            <tr>
              <th scope="col">{{ content.published }}</th>
              <th scope="col">{{ common.firm }}</th>
              <th scope="col">{{ content.sponsor }}</th>
              <th scope="col">{{ content.fieldwork }}</th>
              <th class="num" scope="col">{{ common.sample }}</th>
              <th class="num" scope="col">{{ content.ballots }}</th>
              <th scope="col">{{ content.entry }}</th>
              <th scope="col">{{ common.source }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="p in summaries" :key="p.id" :class="{ unused: !used.has(p.id) }">
              <td class="num">{{ shortDate(p.published, locale) }}</td>
              <td class="firm">{{ p.firm }}</td>
              <td class="secondary">{{ p.sponsor ?? "—" }}</td>
              <td class="num nowrap">{{ shortDate(p.fieldStart, locale) }}–{{ shortDate(p.fieldEnd, locale) }}</td>
              <td class="num">{{ count(p.sample, locale) }}</td>
              <td class="num">{{ p.ballots }}</td>
              <td>
                <span class="badge" :class="{ checked: p.entry === 'primary' }">
                  {{ p.entry === "primary" ? content.entryPrimary : content.entryIndex }}
                </span>
              </td>
              <td class="small nowrap">
                <a :href="safeUrl(p.source)" rel="noopener noreferrer" target="_blank">{{ common.source }}</a>
                <template v-if="p.notice">
                  · <a :href="safeUrl(p.notice)" rel="noopener noreferrer" target="_blank">{{ common.notice }}</a>
                </template>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </section>
  </div>
</template>

<style scoped>
.sources {
  display: grid;
  gap: clamp(20px, 2.4vw, 28px);
}

.facts {
  display: grid;
  grid-template-columns: max-content 1fr;
  gap: 12px 20px;
  margin: 0;
  font-size: 0.9rem;
}

.facts dt {
  color: var(--muted);
  font-size: 0.78rem;
  font-weight: 600;
  letter-spacing: 0.1em;
  text-transform: uppercase;
  padding-top: 2px;
}

.facts dd {
  margin: 0;
  min-width: 0;
  color: var(--ink);
  overflow-wrap: anywhere;
}

.hash {
  overflow-wrap: anywhere;
  color: var(--ink-2);
}

.collectors th[scope="row"] {
  font-weight: 500;
  color: var(--ink);
}

.when {
  white-space: nowrap;
}

.sub {
  margin: 22px 0 10px;
  font-size: 0.72rem;
  font-weight: 600;
  letter-spacing: 0.14em;
  text-transform: uppercase;
  color: var(--muted);
}

.quarantine {
  margin: 0;
  padding-left: 18px;
  display: grid;
  gap: 4px;
}

.firm {
  font-weight: 500;
  color: var(--ink);
}

.nowrap {
  white-space: nowrap;
}

.badge.checked {
  color: var(--ink);
  background: rgba(88, 124, 241, 0.12);
  box-shadow: inset 0 0 0 1px rgba(88, 124, 241, 0.35);
}

tr.unused td {
  color: var(--muted);
}
</style>
