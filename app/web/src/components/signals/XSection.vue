<script setup lang="ts">
import { computed, ref } from "vue"
import { useIntlayer, useLocale } from "vue-intlayer"
import type { XRow } from "../../api/schemas"
import { count, dateTime, percent } from "../../format"
import { latestXWindow, nameOf, sentiment, type Sentiment } from "../../signals"
import CandidateAvatar from "../CandidateAvatar.vue"

// The latest X drop, one line per candidate: a full-width bar stacking the favourable, neutral and
// unfavourable shares of the posts read, then the volume. The most mentioned first; the rest behind a toggle.
const props = defineProps<{ rows: readonly XRow[]; names: ReadonlyMap<string, string> }>()
const content = useIntlayer("x-section")
const common = useIntlayer("common")
const { locale } = useLocale()

const SHOWN = 12
const showAll = ref(false)
const latest = computed(() => latestXWindow(props.rows))
const all = computed(() =>
  (latest.value?.rows ?? []).map((row) => ({ row, name: nameOf(props.names, row.candidate_id), mood: sentiment(row) })),
)
const rows = computed(() => (showAll.value ? all.value : all.value.slice(0, SHOWN)))
const share = (x: number) => percent(x, 0, locale.value)
const pct = (x: number) => `${(x * 100).toFixed(1)}%`
// A segment shows its share inside when it is wide enough to hold the label.
const labelled = (x: number) => x >= 0.12
const moodLabel = (s: Sentiment) =>
  content.sentimentLabel({ pos: share(s.pos), neu: share(s.neu), neg: share(s.neg) }).value
</script>

<template>
  <section aria-labelledby="x-title" class="card">
    <header class="card-head head">
      <h2 id="x-title">{{ content.title }}</h2>
      <p v-if="latest" class="muted small">
        {{ content.window({ start: dateTime(latest.start, locale), end: dateTime(latest.end, locale) }) }}
      </p>
    </header>
    <p v-if="!latest" class="secondary small">{{ content.empty }}</p>
    <template v-else>
      <p aria-hidden="true" class="legend small">
        <span class="key"><span class="swatch pos" />{{ content.positive }}</span>
        <span class="key"><span class="swatch neu" />{{ content.neutral }}</span>
        <span class="key"><span class="swatch neg" />{{ content.negative }}</span>
      </p>
      <div class="table-wrap">
        <table class="x">
          <thead>
            <tr>
              <th scope="col">{{ common.candidate }}</th>
              <th class="bar-col" scope="col">{{ content.sentiment }}</th>
              <th class="num" scope="col">{{ content.mentions }}</th>
              <th class="num" scope="col">{{ content.posts }}</th>
              <th class="num" scope="col">{{ content.botShare }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="{ row, name, mood } in rows" :key="row.candidate_id">
              <th scope="row">
                <span class="who">
                  <CandidateAvatar :id="row.candidate_id" :name="name" :size="22" />
                  {{ name }}
                </span>
              </th>
              <td class="bar-col">
                <div v-if="mood" class="mood" role="img" :aria-label="moodLabel(mood)" :title="moodLabel(mood)">
                  <span v-if="mood.pos > 0" class="pos" :style="{ width: pct(mood.pos) }">
                    <template v-if="labelled(mood.pos)">{{ share(mood.pos) }}</template>
                  </span>
                  <span v-if="mood.neu > 0" class="neu" :style="{ width: pct(mood.neu) }">
                    <template v-if="labelled(mood.neu)">{{ share(mood.neu) }}</template>
                  </span>
                  <span v-if="mood.neg > 0" class="neg" :style="{ width: pct(mood.neg) }">
                    <template v-if="labelled(mood.neg)">{{ share(mood.neg) }}</template>
                  </span>
                </div>
                <span v-else class="muted">—</span>
              </td>
              <td
                class="num strong"
                :title="row.mentions_method ? content.methods[row.mentions_method].value : undefined"
              >
                {{ row.mentions === null ? "—" : count(row.mentions, locale) }}
              </td>
              <td class="num secondary">{{ row.sample_size === null ? "—" : count(row.sample_size, locale) }}</td>
              <td class="num secondary">
                {{ row.bot_share_estimate === null ? "—" : share(row.bot_share_estimate) }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>
      <button v-if="all.length > SHOWN" class="more small" type="button" @click="showAll = !showAll">
        {{ showAll ? content.showFewer : content.showAll({ n: count(all.length, locale) }) }}
      </button>
    </template>
  </section>
</template>

<style scoped>
.head {
  display: flex;
  flex-wrap: wrap;
  align-items: baseline;
  justify-content: space-between;
  gap: 4px 16px;
}

.x th, .x td {
  padding-top: 7px;
  padding-bottom: 7px;
}

.who {
  display: flex;
  align-items: center;
  gap: 10px;
  white-space: nowrap;
}

th[scope="row"] {
  font-weight: 500;
  color: var(--ink);
}

.strong {
  font-weight: 650;
  color: var(--ink);
}

.bar-col {
  width: 55%;
}

/* A 100% bar: favourable in bleu, neutral in grey, unfavourable in rouge, with the shares written inside. */
.mood {
  display: flex;
  gap: 2px;
  height: 20px;
  min-width: 160px;
}

.mood span {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  overflow: hidden;
  font-size: 0.72rem;
  font-weight: 600;
  font-variant-numeric: tabular-nums;
  color: #ffffff;
  white-space: nowrap;
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
  margin-bottom: 10px;
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

.more {
  margin-top: 10px;
  background: none;
  border: none;
  padding: 0;
  font: inherit;
  color: var(--link);
  text-decoration: underline;
  cursor: pointer;
}

@media (max-width: 760px) {
  .bar-col {
    width: auto;
  }

  .mood {
    min-width: 110px;
  }

  th,
  td {
    padding: 7px 6px;
  }
}
</style>
