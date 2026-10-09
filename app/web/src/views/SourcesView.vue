<script setup lang="ts">
import { computed } from "vue"
import type { Forecast, Health, PollRow } from "../api/schemas"
import { dateTime, longDate, safeUrl, shortDate } from "../format"
import { useI18n } from "../i18n"

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
        <dl class="facts small">
          <template v-for="c in health?.collectors ?? []" :key="c.file">
            <dt>{{ c.name }}</dt>
            <dd>{{ c.modified ? `${t("lastWritten")} ${dateTime(c.modified, locale)}` : "—" }}</dd>
          </template>
        </dl>
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

tr.unused td {
  color: var(--muted);
}

ul {
  margin: 0;
  padding-left: 18px;
}
</style>
