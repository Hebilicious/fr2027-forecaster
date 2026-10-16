<script setup lang="ts">
import { computed } from "vue"
import type { Forecast, Health, Poll } from "../api/schemas"
import { dateOrTime, dateTime, longDate, safeUrl, shortDate } from "../format"
import { cadenceMessages, useI18n, type MessageKey } from "../i18n"

const props = defineProps<{ forecast: Forecast; polls: readonly Poll[]; health: Health | null }>()
const { t, locale } = useI18n()

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
// Collector names come from `fr2027`; known ones are translated, others shown as sent.
const collectorNames: Record<string, MessageKey> = {
  polls: "collectorPolls",
  markets: "collectorMarkets",
  news: "collectorNews",
  attention: "collectorAttention",
  x: "collectorX",
  events: "collectorEvents",
}
const translated = (keys: Record<string, MessageKey>, value: string) => {
  const key = keys[value]
  return key ? t(key) : value
}
const quarantined = computed(() =>
  Object.entries(props.health?.quarantine ?? {}).flatMap(([area, files]) => files.map((f) => `${area}/${f}`)),
)
</script>

<template>
  <div class="sources">
    <div v-if="health?.warnings.length" class="stack rise">
      <p v-for="w in health.warnings" :key="w" class="notice" role="status">
        <span aria-hidden="true" class="icon">!</span>
        <span>{{ w }}</span>
      </p>
    </div>
    <p v-else-if="health" class="healthy rise">
      <span aria-hidden="true" class="tick">✓</span>
      {{ t("healthy") }}
    </p>

    <div class="grid-2">
      <section aria-labelledby="run-title" class="card rise" :style="{ '--i': 1 }">
        <header class="card-head">
          <h2 id="run-title">{{ t("latestRun") }}</h2>
        </header>
        <dl class="facts">
          <dt>{{ t("file") }}</dt>
          <dd class="mono">data/forecasts/{{ health?.latest_forecast?.file ?? "—" }}</dd>
          <dt>{{ t("generatedAt") }}</dt>
          <dd>{{ dateTime(forecast.generated_at, locale) }}</dd>
          <dt>{{ t("lastFieldwork") }}</dt>
          <dd>{{ forecast.aggregation.last_field_end ? longDate(forecast.aggregation.last_field_end, locale) : "—" }}</dd>
          <dt>inputs_hash</dt>
          <dd class="mono hash">{{ forecast.inputs_hash }}</dd>
        </dl>
      </section>

      <section aria-labelledby="collectors-title" class="card rise" :style="{ '--i': 2 }">
        <header class="card-head">
          <h2 id="collectors-title">{{ t("collectors") }}</h2>
        </header>
        <div class="table-wrap">
          <table class="collectors">
            <thead>
              <tr>
                <th scope="col">{{ t("source") }}</th>
                <th scope="col">{{ t("cadence") }}</th>
                <th scope="col">{{ t("lastUpdate") }}</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="c in health?.collectors ?? []" :key="c.name">
                <th scope="row">{{ translated(collectorNames, c.name) }}</th>
                <td class="secondary">{{ t(cadenceMessages[c.cadence]) }}</td>
                <td>
                  <span class="when" :class="{ muted: !c.updated }">{{ c.updated ? dateOrTime(c.updated, locale) : t("never") }}</span>
                  <span v-if="c.stale" class="badge stale"><span aria-hidden="true" class="dot" />{{ t("staleBadge") }}</span>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
        <h3 class="sub">{{ t("quarantine") }}</h3>
        <ul v-if="quarantined.length" class="quarantine small">
          <li v-for="q in quarantined" :key="q" class="mono">data/quarantine/{{ q }}</li>
        </ul>
        <p v-else class="small secondary">{{ t("none") }}</p>
      </section>
    </div>

    <section aria-labelledby="polls-title" class="card rise" :style="{ '--i': 3 }">
      <header class="card-head">
        <h2 id="polls-title">{{ t("pollsTitle") }}</h2>
      </header>
      <div class="table-wrap">
        <table class="polls">
          <thead>
            <tr>
              <th scope="col">{{ t("published") }}</th>
              <th scope="col">{{ t("firm") }}</th>
              <th scope="col">{{ t("sponsor") }}</th>
              <th scope="col">{{ t("fieldwork") }}</th>
              <th class="num" scope="col">{{ t("sample") }}</th>
              <th class="num" scope="col">{{ t("ballots") }}</th>
              <th scope="col">{{ t("entry") }}</th>
              <th scope="col">{{ t("source") }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="p in summaries" :key="p.id" :class="{ unused: !used.has(p.id) }">
              <td class="num">{{ shortDate(p.published, locale) }}</td>
              <td class="firm">{{ p.firm }}</td>
              <td class="secondary">{{ p.sponsor ?? "—" }}</td>
              <td class="num nowrap">{{ shortDate(p.fieldStart, locale) }}–{{ shortDate(p.fieldEnd, locale) }}</td>
              <td class="num">{{ p.sample.toLocaleString(locale === "fr" ? "fr-FR" : "en-GB") }}</td>
              <td class="num">{{ p.ballots }}</td>
              <td>
                <span class="badge" :class="{ checked: p.entry === 'primary' }">
                  {{ p.entry === "primary" ? t("entryPrimary") : t("entryIndex") }}
                </span>
              </td>
              <td class="small nowrap">
                <a :href="safeUrl(p.source)" rel="noopener noreferrer" target="_blank">{{ t("source") }}</a>
                <template v-if="p.notice">
                  · <a :href="safeUrl(p.notice)" rel="noopener noreferrer" target="_blank">{{ t("notice") }}</a>
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

.healthy {
  justify-self: start;
  display: inline-flex;
  align-items: center;
  gap: 10px;
  padding: 10px 18px 10px 12px;
  border-radius: 999px;
  background: rgba(12, 163, 12, 0.08);
  box-shadow: inset 0 0 0 1px rgba(12, 163, 12, 0.3);
  color: var(--ink);
  font-size: 0.9rem;
}

.tick {
  display: grid;
  place-items: center;
  width: 20px;
  height: 20px;
  border-radius: 50%;
  background: #0ca30c;
  color: #ffffff;
  font-size: 0.72rem;
  font-weight: 700;
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

.stale {
  margin-left: 8px;
}

.stale .dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--warning);
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
