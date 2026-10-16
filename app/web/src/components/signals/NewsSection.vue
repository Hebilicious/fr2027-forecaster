<script setup lang="ts">
import { computed, shallowRef } from "vue"
import { useIntlayer, useLocale } from "vue-intlayer"
import type { News } from "../../api/schemas"
import { count, dateTime, safeUrl } from "../../format"
import { latestHeadlines, nameOf, newsCounts } from "../../signals"
import CandidateAvatar from "../CandidateAvatar.vue"

const props = defineProps<{ news: News; names: ReadonlyMap<string, string> }>()
const content = useIntlayer("news-section")
const { locale } = useLocale()

const counts = computed(() => newsCounts(props.news.daily, props.names))
const maxCount = computed(() => Math.max(1, ...counts.value.map((c) => c.items)))
const filter = shallowRef<string | null>(null)
const headlines = computed(() => latestHeadlines(props.news.headlines, filter.value))
const failing = computed(() => props.news.feeds.filter((f) => f.status !== "ok").length)
const pct = (x: number) => `${(x * 100).toFixed(1)}%`
</script>

<template>
  <section aria-labelledby="news-title" class="card">
    <header class="card-head">
      <h2 id="news-title">{{ content.title }}</h2>
    </header>
    <p v-if="news.headlines.length === 0 && counts.length === 0" class="secondary small">{{ content.empty }}</p>
    <div v-else class="layout">
      <div v-if="counts.length" class="block">
        <h3 class="sub">{{ content.counts }}</h3>
        <ul class="bars">
          <li v-for="(c, i) in counts" :key="c.candidateId" :style="{ '--i': i }">
            <span class="bar-name">
              <CandidateAvatar :id="c.candidateId" :name="c.name" :size="22" />
              <span class="bar-label">{{ c.name }}</span>
            </span>
            <span class="bar-track"><span class="bar" :style="{ width: pct(c.items / maxCount) }" /></span>
            <span class="num bar-value">{{ count(c.items, locale) }}</span>
          </li>
        </ul>
      </div>

      <div class="block">
        <h3 class="sub">{{ content.latestHeadlines }}</h3>
        <div v-if="counts.length" class="chips" role="group" :aria-label="content.filter.value">
          <span class="small muted">{{ content.filter }}</span>
          <button class="chip" type="button" :aria-pressed="filter === null" @click="filter = null">
            {{ content.allCandidates }}
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
          {{ filter ? content.noHeadlines({ name: nameOf(names, filter) }) : content.empty }}
        </p>
      </div>
    </div>
    <p v-if="news.fetched_at" class="muted small foot">
      {{ content.feedStatus({ n: news.feeds.length, m: failing, when: dateTime(news.fetched_at, locale) }) }}
    </p>
  </section>
</template>

<style scoped>
.layout {
  display: grid;
  gap: 28px 40px;
  grid-template-columns: repeat(auto-fit, minmax(min(100%, 380px), 1fr));
  align-items: start;
}

.block {
  display: grid;
  gap: 14px;
}

.sub {
  font-size: 0.72rem;
  font-weight: 600;
  letter-spacing: 0.14em;
  text-transform: uppercase;
  color: var(--muted);
}

.bars {
  list-style: none;
  margin: 0;
  padding: 0;
  display: grid;
  gap: 10px;
  font-size: 0.88rem;
}

.bars li {
  display: grid;
  grid-template-columns: minmax(0, 12.5rem) minmax(0, 1fr) 2.5em;
  gap: 12px;
  align-items: center;
}

.bar-name {
  display: flex;
  align-items: center;
  gap: 10px;
  min-width: 0;
  color: var(--ink);
}

.bar-label {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.bar-track {
  height: 6px;
  background: rgba(243, 239, 230, 0.06);
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
  animation: grow 1s var(--ease-out) both;
  animation-delay: calc(var(--i, 0) * 40ms + 200ms);
}

.bar-value {
  text-align: right;
  color: var(--ink-2);
}

.headlines {
  list-style: none;
  margin: 0;
  padding: 0;
  display: grid;
  max-height: 34rem;
  overflow-y: auto;
  scrollbar-width: thin;
  scrollbar-color: rgba(255, 255, 255, 0.15) transparent;
  /* The list scrolls: its last lines fade out to say so. */
  -webkit-mask-image: linear-gradient(180deg, #000 calc(100% - 56px), transparent);
  mask-image: linear-gradient(180deg, #000 calc(100% - 56px), transparent);
  padding-bottom: 32px;
}

.headlines li {
  display: grid;
  gap: 4px;
  padding: 12px 0;
  border-bottom: 1px solid var(--grid);
}

.headlines li:last-child {
  border-bottom: none;
}

.headlines a {
  color: var(--ink);
  text-decoration: none;
  overflow-wrap: anywhere;
  line-height: 1.4;
}

.headlines a:hover {
  color: #ffffff;
  text-decoration: underline;
  text-decoration-color: rgba(243, 239, 230, 0.4);
}

.foot {
  margin-top: 20px;
}

@media (max-width: 760px) {
  .bars li {
    grid-template-columns: minmax(0, 11rem) minmax(0, 1fr) 2.5em;
  }
}
</style>
