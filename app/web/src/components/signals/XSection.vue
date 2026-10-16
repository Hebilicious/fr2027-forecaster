<script setup lang="ts">
import { computed } from "vue"
import type { XRow } from "../../api/schemas"
import { count, dateTime, percent } from "../../format"
import { useI18n, type MessageKey } from "../../i18n"
import { latestXWindow, nameOf, sentiment, type Sentiment } from "../../signals"

const props = defineProps<{ rows: readonly XRow[]; names: ReadonlyMap<string, string> }>()
const { t, locale } = useI18n()

const latest = computed(() => latestXWindow(props.rows))
const rows = computed(() =>
  (latest.value?.rows ?? []).map((row) => ({ row, name: nameOf(props.names, row.candidate_id), mood: sentiment(row) })),
)
const hasSentiment = computed(() => rows.value.some((r) => r.mood !== null))
// The sample is the posts read for sentiment and the bot share, so it sits under the sentiment bar.
const hasSample = computed(() => hasSentiment.value || rows.value.some((r) => r.row.sample_size !== null))
const methods: Record<NonNullable<XRow["mentions_method"]>, MessageKey> = {
  x_counts_endpoint: "methodXCounts",
  native_count: "methodNative",
  sample_estimate: "methodSample",
}
const share = (x: number) => percent(x, 0, locale.value)
const pct = (x: number) => `${(x * 100).toFixed(1)}%`
const moodLabel = (s: Sentiment) => t("sentimentLabel", { pos: share(s.pos), neu: share(s.neu), neg: share(s.neg) })
</script>

<template>
  <section aria-labelledby="x-title" class="card">
    <header class="card-head">
      <h2 id="x-title">{{ t("xTitle") }}</h2>
      <p v-if="latest" class="muted small">
        {{ t("xWindow", { start: dateTime(latest.start, locale), end: dateTime(latest.end, locale) }) }}
      </p>
    </header>
    <p v-if="!latest" class="secondary small">{{ t("xEmpty") }}</p>
    <template v-else>
      <p v-if="hasSentiment" aria-hidden="true" class="legend small">
        <span class="key"><span class="swatch pos" />{{ t("positive") }}</span>
        <span class="key"><span class="swatch neu" />{{ t("neutral") }}</span>
        <span class="key"><span class="swatch neg" />{{ t("negative") }}</span>
      </p>
      <div class="table-wrap">
        <table>
          <thead>
            <tr>
              <th scope="col">{{ t("candidate") }}</th>
              <th class="num" scope="col">{{ t("mentions") }}</th>
              <th v-if="hasSample" scope="col">{{ hasSentiment ? t("sentiment") : t("sample") }}</th>
              <th class="num" scope="col">{{ t("botShare") }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="{ row, name, mood } in rows" :key="row.candidate_id">
              <th scope="row">{{ name }}</th>
              <td class="num">
                <span class="strong">{{ row.mentions === null ? "—" : count(row.mentions, locale) }}</span>
                <span v-if="row.mentions_method" class="method muted small">{{ t(methods[row.mentions_method]) }}</span>
              </td>
              <td v-if="hasSample">
                <div v-if="mood" class="mood" role="img" :aria-label="moodLabel(mood)" :title="moodLabel(mood)">
                  <span v-if="mood.pos > 0" class="pos" :style="{ width: pct(mood.pos) }" />
                  <span v-if="mood.neu > 0" class="neu" :style="{ width: pct(mood.neu) }" />
                  <span v-if="mood.neg > 0" class="neg" :style="{ width: pct(mood.neg) }" />
                </div>
                <span v-else-if="hasSentiment" class="muted">—</span>
                <template v-if="row.sample_size !== null">
                  <span aria-hidden="true" class="posts muted small num">n = {{ count(row.sample_size, locale) }}</span>
                  <span class="sr-only">{{ t("postsRead", { n: count(row.sample_size, locale) }) }}</span>
                </template>
              </td>
              <td class="num secondary">{{ row.bot_share_estimate === null ? "—" : share(row.bot_share_estimate) }}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </template>
  </section>
</template>

<style scoped>
th[scope="row"] {
  font-weight: 500;
  color: var(--ink);
}

.strong {
  font-weight: 650;
  color: var(--ink);
}

.method, .posts {
  display: block;
  white-space: nowrap;
}

.posts {
  margin-top: 4px;
}

/* Diverging: bleu for positive and rouge for negative around a grey neutral, 2px gaps between. */
.mood {
  display: flex;
  gap: 2px;
  height: 8px;
  min-width: 120px;
}

.mood span {
  display: block;
  height: 100%;
}

.mood span:first-child {
  border-radius: 4px 0 0 4px;
}

.mood span:last-child {
  border-radius: 0 4px 4px 0;
}

.pos {
  background: var(--series);
}

.neu {
  background: var(--dot);
}

.neg {
  background: var(--accent);
}

.legend {
  display: flex;
  flex-wrap: wrap;
  gap: 4px 18px;
  margin-bottom: 14px;
  color: var(--ink-2);
}

.key {
  display: inline-flex;
  align-items: center;
  gap: 8px;
}

.swatch {
  display: inline-block;
  width: 12px;
  height: 8px;
  border-radius: 2px;
}

@media (max-width: 760px) {
  .mood {
    min-width: 44px;
  }

  .method {
    white-space: normal;
  }

  th,
  td {
    padding: 10px 6px;
  }
}
</style>
