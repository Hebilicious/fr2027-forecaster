<script setup lang="ts">
import { computed } from "vue"
import type { Forecast, Health, PollRow } from "../api/schemas"
import { dateOrTime, dateTime, longDate, safeUrl, shortDate } from "../format"
import { useI18n, type MessageKey } from "../i18n"

const props = defineProps<{ forecast: Forecast; polls: readonly PollRow[]; health: Health | null }>()
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

const summaries = computed<PollSummary[]>(() => {
  const byPoll = new Map<string, { row: PollRow; scenarios: Set<string> }>()
  for (const row of props.polls) {
    const entry = byPoll.get(row.poll_id) ?? { row, scenarios: new Set<string>() }
    entry.scenarios.add(`${row.round}:${row.scenario_id}`)
    byPoll.set(row.poll_id, entry)
  }
  return [...byPoll.values()]
    .map(({ row, scenarios }) => ({
      id: row.poll_id,
      firm: row.firm,
      sponsor: row.sponsor,
      fieldStart: row.field_start,
      fieldEnd: row.field_end,
      published: row.published_at,
      sample: row.sample_size,
      ballots: scenarios.size,
      entry: row.entry,
      source: row.source_url,
      notice: row.notice_url,
    }))
    .toSorted((a, b) => b.published.localeCompare(a.published) || a.id.localeCompare(b.id))
})
const used = computed(() => new Set(props.forecast.aggregation.poll_ids))
// Collector names and cadences come from `fr2027`; known ones are translated, others shown as sent.
const collectorNames: Record<string, MessageKey> = {
  polls: "collectorPolls",
  markets: "collectorMarkets",
  news: "collectorNews",
  attention: "collectorAttention",
  x: "collectorX",
  events: "collectorEvents",
}
const cadences: Record<string, MessageKey> = {
  "when a poll is published": "cadencePoll",
  hourly: "cadenceHourly",
  daily: "cadenceDaily",
  "Grok Bot, every 6 hours": "cadenceGrok6h",
  "Grok Bot, daily": "cadenceGrokDaily",
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
  <section class="stack">
    <h2>{{ t("sourcesTitle") }}</h2>

    <div v-if="health?.warnings.length" class="stack">
      <p v-for="w in health.warnings" :key="w" class="notice" role="status">
        <span aria-hidden="true" class="icon">!</span>
        <span>{{ w }}</span>
      </p>
    </div>
    <p v-else-if="health" class="secondary">{{ t("healthy") }}</p>

    <div class="grid-2">
      <div class="card stack">
        <h3>{{ t("latestRun") }}</h3>
        <dl class="facts small">
          <dt>{{ t("file") }}</dt>
          <dd class="num">data/forecasts/{{ health?.latest_forecast?.file ?? "—" }}</dd>
          <dt>{{ t("generatedAt") }}</dt>
          <dd>{{ dateTime(forecast.generated_at, locale) }}</dd>
          <dt>{{ t("lastFieldwork") }}</dt>
          <dd>{{ forecast.aggregation.last_field_end ? longDate(forecast.aggregation.last_field_end, locale) : "—" }}</dd>
          <dt>inputs_hash</dt>
          <dd class="num hash">{{ forecast.inputs_hash }}</dd>
        </dl>
      </div>
      <div class="card stack">
        <h3>{{ t("collectors") }}</h3>
        <div class="table-wrap">
          <table class="collectors small">
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
                <td class="secondary">{{ translated(cadences, c.cadence) }}</td>
                <td>
                  <span class="when" :class="{ muted: !c.updated }">{{ c.updated ? dateOrTime(c.updated, locale) : t("never") }}</span>
                  <span v-if="c.stale" class="badge stale"><span aria-hidden="true" class="dot" />{{ t("staleBadge") }}</span>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
        <h3>{{ t("quarantine") }}</h3>
        <ul v-if="quarantined.length" class="small">
          <li v-for="q in quarantined" :key="q" class="num">data/quarantine/{{ q }}</li>
        </ul>
        <p v-else class="small secondary">{{ t("none") }}</p>
      </div>
    </div>

    <div class="card table-wrap">
      <h3>{{ t("pollsTitle") }}</h3>
      <table>
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
            <td>{{ p.firm }}</td>
            <td class="secondary">{{ p.sponsor ?? "—" }}</td>
            <td class="num">{{ shortDate(p.fieldStart, locale) }}–{{ shortDate(p.fieldEnd, locale) }}</td>
            <td class="num">{{ p.sample.toLocaleString(locale === "fr" ? "fr-FR" : "en-GB") }}</td>
            <td class="num">{{ p.ballots }}</td>
            <td class="small">{{ p.entry === "primary" ? t("entryPrimary") : t("entryIndex") }}</td>
            <td class="small">
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
</template>

<style scoped>
.grid-2 {
  display: grid;
  gap: 16px;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  align-items: start;
}

.facts {
  display: grid;
  grid-template-columns: max-content 1fr;
  gap: 4px 12px;
  margin: 0;
}

.facts dt {
  color: var(--ink-2);
}

.facts dd {
  margin: 0;
  min-width: 0;
}

.hash {
  overflow-wrap: anywhere;
}

.collectors th[scope="row"] {
  font-weight: 400;
  color: var(--ink);
}

.collectors th, .collectors td {
  padding: 6px 8px 6px 0;
}

.when {
  white-space: nowrap;
}

.stale {
  margin-left: 6px;
  display: inline-flex;
  align-items: center;
  gap: 4px;
}

.stale .dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--warning);
}

tr.unused td {
  color: var(--muted);
}

ul {
  margin: 0;
  padding-left: 18px;
}
</style>
