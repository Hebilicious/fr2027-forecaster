<script setup lang="ts">
import { computed, shallowRef } from "vue"
import type { News } from "../../api/schemas"
import { count, dateTime, safeUrl } from "../../format"
import { useI18n } from "../../i18n"
import { latestHeadlines, nameOf, newsCounts } from "../../signals"

const props = defineProps<{ news: News; names: ReadonlyMap<string, string> }>()
const { t, locale } = useI18n()

const counts = computed(() => newsCounts(props.news.daily, props.names))
const maxCount = computed(() => Math.max(1, ...counts.value.map((c) => c.items)))
const filter = shallowRef<string | null>(null)
const headlines = computed(() => latestHeadlines(props.news.headlines, filter.value))
const failing = computed(() => props.news.feeds.filter((f) => f.status !== "ok").length)
const pct = (x: number) => `${(x * 100).toFixed(1)}%`
</script>

<template>
  <section aria-labelledby="news-title" class="card stack">
    <h3 id="news-title">{{ t("newsTitle") }}</h3>
    <p v-if="news.headlines.length === 0 && counts.length === 0" class="secondary small">{{ t("newsEmpty") }}</p>
    <template v-else>
      <div v-if="counts.length" class="stack-tight">
        <h4>{{ t("newsCounts") }}</h4>
        <ul class="bars">
          <li v-for="c in counts" :key="c.candidateId">
            <span class="bar-name">{{ c.name }}</span>
            <span class="bar-track"><span class="bar" :style="{ width: pct(c.items / maxCount) }" /></span>
            <span class="num bar-value">{{ count(c.items, locale) }}</span>
          </li>
        </ul>
      </div>

      <div class="stack-tight">
        <h4>{{ t("latestHeadlines") }}</h4>
        <div v-if="counts.length" class="chips" role="group" :aria-label="t('newsFilter')">
          <span class="small secondary">{{ t("newsFilter") }}</span>
          <button class="chip" type="button" :aria-pressed="filter === null" @click="filter = null">
            {{ t("allCandidates") }}
          </button>
          <button
            v-for="c in counts"
            :key="c.candidateId"
            class="chip"
            type="button"
            :aria-pressed="filter === c.candidateId"
            @click="filter = filter === c.candidateId ? null : c.candidateId"
          >
            {{ c.name }}
          </button>
        </div>
        <ul v-if="headlines.length" class="headlines">
          <li v-for="(h, i) in headlines" :key="`${i}-${h.url}`">
            <a :href="safeUrl(h.url)" rel="noopener">{{ h.title }}</a>
            <span class="muted small">{{ h.outlet }} · {{ dateTime(h.published_at, locale) }}</span>
          </li>
        </ul>
        <p v-else class="secondary small">
          {{ filter ? t("noHeadlines", { name: nameOf(names, filter) }) : t("newsEmpty") }}
        </p>
      </div>
    </template>
    <p v-if="news.fetched_at" class="muted small">
      {{ t("feedStatus", { n: news.feeds.length, m: failing, when: dateTime(news.fetched_at, locale) }) }}
    </p>
  </section>
</template>

<style scoped>
h4 {
  margin: 0;
  font-size: 0.85rem;
  font-weight: 600;
  color: var(--ink-2);
}

.stack-tight {
  display: grid;
  gap: 8px;
}

.bars {
  list-style: none;
  margin: 0;
  padding: 0;
  display: grid;
  gap: 4px;
  font-size: 0.9rem;
  max-width: 760px;
}

.bars li {
  display: grid;
  grid-template-columns: minmax(0, 11rem) minmax(0, 1fr) 2.5em;
  gap: 10px;
  align-items: center;
}

.bar-name {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.bar-track {
  height: 10px;
}

.bar {
  display: block;
  height: 100%;
  min-width: 2px;
  background: var(--series);
  border-radius: 0 4px 4px 0;
}

.bar-value {
  text-align: right;
}

.headlines {
  list-style: none;
  margin: 0;
  padding: 0;
  display: grid;
}

.headlines li {
  display: grid;
  gap: 2px;
  padding: 8px 0;
  border-bottom: 1px solid var(--grid);
}

.headlines li:last-child {
  border-bottom: none;
}

.headlines a {
  overflow-wrap: anywhere;
}

@media (max-width: 720px) {
  .bars li {
    grid-template-columns: minmax(0, 9.5rem) minmax(0, 1fr) 2.5em;
  }
}
</style>
