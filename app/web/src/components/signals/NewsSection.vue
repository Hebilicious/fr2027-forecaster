<script setup lang="ts">
import { computed, shallowRef } from "vue"
import { useIntlayer, useLocale } from "vue-intlayer"
import type { News } from "../../api/schemas"
import { count, dateTime, percent, points, relativeTime, safeUrl, shortDate } from "../../format"
import { useNow } from "../../useNow"
import {
  TONE_MIN_ARTICLES,
  headlineStance,
  headlinesFor,
  headlinesInRange,
  nameOf,
  newsCounts,
  pressVolume,
  type Sentiment,
} from "../../signals"
import { useSignalRange } from "../../useSignalRange"
import CandidateAvatar from "../CandidateAvatar.vue"
import Sparkline from "../Sparkline.vue"
import RangeControl from "./RangeControl.vue"

// The press over the chosen period: headlines per candidate from the RSS feeds with their days, the headlines
// themselves, and GDELT's count of French-language articles, their tone against the day's, and the whole history.
const props = defineProps<{ news: News; names: ReadonlyMap<string, string> }>()
const content = useIntlayer("news-section")
const { locale } = useLocale()
const now = useNow()
const range = useSignalRange()

const inRange = computed(() => headlinesInRange(props.news.headlines, range.value))
const span = computed(() => {
  const newest = inRange.value[0]
  const oldest = inRange.value.at(-1)
  return newest && oldest ? { start: oldest.published_at, end: newest.published_at } : null
})
const counts = computed(() => newsCounts(props.news.headlines, props.news.daily, props.names, range.value))
const maxCount = computed(() => Math.max(1, ...counts.value.rows.map((c) => c.items)))
const filter = shallowRef<string | null>(null)
const headlines = computed(() => headlinesFor(inRange.value, filter.value))
const volume = computed(() => pressVolume(props.news.volume ?? [], props.names, range.value))
const maxVolume = computed(() => Math.max(1, ...volume.value.rows.map((v) => v.articles)))
const failing = computed(() => props.news.feeds.filter((f) => f.status !== "ok").length)
const pct = (x: number) => `${(x * 100).toFixed(1)}%`
const day = (date: string) => shortDate(date, locale.value)
// GDELT's tone is a difference of word shares in percent: shown as points, + for warmer coverage than the day's.
const tone = (t: number) => points(t / 100, locale.value)
const toneClass = (t: number) => (t > 0.05 ? "warm" : t < -0.05 ? "cold" : "")
const share = (x: number) => percent(x, 0, locale.value)
const stanceLabel = (name: string, s: Sentiment) =>
  content.stanceLabel({ name, pos: share(s.pos), neu: share(s.neu), neg: share(s.neg) }).value
// A headline's candidates with the stance Jev read in the article, when it read it.
const marks = (h: (typeof headlines.value)[number]) =>
  h.candidate_ids.flatMap((id) => {
    const stance = headlineStance(h.stance?.[id])
    return stance ? [{ id, name: nameOf(props.names, id), stance }] : []
  })
</script>

<template>
  <section aria-labelledby="news-title" class="card">
    <header class="card-head head">
      <div class="titles">
        <h2 id="news-title">{{ content.title }}</h2>
        <p v-if="span" class="muted small">
          {{
            content.window({
              start: dateTime(span.start, locale),
              end: dateTime(span.end, locale),
              n: count(inRange.length, locale),
            })
          }}
        </p>
      </div>
      <RangeControl v-if="news.headlines.length" />
    </header>
    <p v-if="news.headlines.length === 0 && volume.rows.length === 0" class="secondary small">{{ content.empty }}</p>
    <div v-else class="layout">
      <div class="column">
        <div v-if="counts.rows.length" class="block">
          <h3 class="sub">{{ content.counts }}</h3>
          <ul class="bars">
            <li v-for="(c, i) in counts.rows" :key="c.candidateId" :style="{ '--i': i }">
              <span class="bar-name">
                <CandidateAvatar :id="c.candidateId" :name="c.name" :size="22" />
                <span class="bar-label">{{ c.name }}</span>
              </span>
              <span class="bar-track"><span class="bar" :style="{ width: pct(c.items / maxCount) }" /></span>
              <span class="num bar-value">{{ count(c.items, locale) }}</span>
              <Sparkline
                v-if="counts.window"
                :points="c.daily"
                :window="counts.window"
                :label="content.countsLabel({ name: c.name, start: day(counts.window[0]), end: day(counts.window[1]) }).value"
              />
            </li>
          </ul>
        </div>

        <div v-if="volume.rows.length && volume.range" class="block">
          <h3 class="sub">{{ content.volumeTitle }}</h3>
          <p v-if="volume.window" class="muted small">
            {{
              content.volumeWindow({
                start: day(volume.range[0]),
                end: day(volume.range[1]),
                first: day(volume.window[0]),
              })
            }}
          </p>
          <ul class="bars toned">
            <li v-for="(v, i) in volume.rows" :key="v.candidateId" :style="{ '--i': i }">
              <span class="bar-name">
                <CandidateAvatar :id="v.candidateId" :name="v.name" :size="22" />
                <span class="bar-label">{{ v.name }}</span>
              </span>
              <span class="bar-track"><span class="bar" :style="{ width: pct(v.articles / maxVolume) }" /></span>
              <span
                class="num bar-value"
                :title="v.share === null ? undefined : content.volumeShare({ share: percent(v.share, 1, locale) }).value"
              >
                {{ count(v.articles, locale) }}
              </span>
              <span
                v-if="v.tone !== null"
                class="num tone"
                :class="toneClass(v.tone)"
                :title="content.toneLabel({ tone: tone(v.tone) }).value"
              >
                {{ tone(v.tone) }}
              </span>
              <span v-else class="num tone muted">—</span>
              <span
                v-if="v.stance"
                class="stance-bar"
                role="img"
                :aria-label="stanceLabel(v.name, v.stance)"
                :title="stanceLabel(v.name, v.stance)"
              >
                <span class="pos" :style="{ width: pct(v.stance.pos) }" />
                <span class="neu" :style="{ width: pct(v.stance.neu) }" />
                <span class="neg" :style="{ width: pct(v.stance.neg) }" />
              </span>
              <span v-else class="stance-bar empty" />
              <Sparkline
                v-if="volume.window"
                :points="v.daily"
                :window="volume.window"
                :label="content.volumeLabel({ name: v.name, start: day(volume.window[0]), end: day(volume.window[1]) }).value"
              />
            </li>
          </ul>
          <p class="muted small">{{ content.volumeNote }}</p>
          <p class="muted small">{{ content.toneNote({ n: count(TONE_MIN_ARTICLES, locale) }) }}</p>
          <p class="muted small">{{ content.stanceNote }}</p>
        </div>
      </div>

      <div class="block">
        <h3 class="sub">{{ content.latestHeadlines }}</h3>
        <div v-if="counts.rows.length" class="chips" role="group" :aria-label="content.filter.value">
          <span class="small muted">{{ content.filter }}</span>
          <button class="chip" type="button" :aria-pressed="filter === null" @click="filter = null">
            {{ content.allCandidates }}
          </button>
          <button
            v-for="c in counts.rows"
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
            <span class="muted small">
              {{ h.outlet }} · {{ dateTime(h.published_at, locale) }}
              <template v-if="h.source === 'gdelt'"> · {{ content.viaGdelt }}</template>
            </span>
            <span v-if="marks(h).length" class="marks small">
              <span
                v-for="m in marks(h)"
                :key="m.id"
                class="mark"
                :class="m.stance"
                :title="content.markLabel[m.stance]({ name: m.name }).value"
              >
                {{ m.name }}
              </span>
            </span>
          </li>
        </ul>
        <p v-else class="secondary small">
          {{ filter ? content.noHeadlines({ name: nameOf(names, filter) }) : content.empty }}
        </p>
      </div>
    </div>
    <p v-if="news.fetched_at" class="muted small foot">
      {{ content.feedStatus({ n: news.feeds.length, m: failing, when: relativeTime(news.fetched_at, now, locale) }) }}
    </p>
  </section>
</template>

<style scoped>
.head {
  display: flex;
  flex-wrap: wrap;
  align-items: flex-start;
  justify-content: space-between;
  gap: 12px 24px;
}

.titles {
  display: grid;
  gap: 4px;
}

.column {
  display: grid;
  gap: 28px;
  align-content: start;
}

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
  grid-template-columns: minmax(0, 12.5rem) minmax(0, 1fr) 3em auto;
  gap: 12px;
  align-items: center;
}

/* GDELT's rows carry the tone and Jev's stance between the count and the line. */
.bars.toned li {
  grid-template-columns: minmax(0, 12.5rem) minmax(0, 1fr) 3em 4.5em 4.5em auto;
}

/* Favourable in bleu, neutral in grey, unfavourable in rouge, as the X bars. */
.stance-bar {
  display: flex;
  gap: 1px;
  height: 8px;
  border-radius: 3px;
  overflow: hidden;
}

.stance-bar.empty {
  background: none;
}

.stance-bar .pos,
.mark.pos::before {
  background: var(--series);
}

.stance-bar .neu,
.mark.neu::before {
  background: var(--dot);
}

.stance-bar .neg,
.mark.neg::before {
  background: var(--accent);
}

.marks {
  display: flex;
  flex-wrap: wrap;
  gap: 2px 12px;
  color: var(--ink-2);
}

.mark {
  display: inline-flex;
  align-items: center;
  gap: 6px;
}

.mark::before {
  content: "";
  width: 7px;
  height: 7px;
  border-radius: 50%;
}

.tone {
  text-align: right;
  font-size: 0.8rem;
  white-space: nowrap;
  color: var(--ink-2);
}

.tone.warm {
  color: var(--series);
}

.tone.cold {
  color: var(--accent);
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
    grid-template-columns: minmax(0, 11rem) minmax(0, 1fr) 3em auto;
  }

  .bars.toned li {
    grid-template-columns: minmax(0, 8rem) minmax(0, 1fr) 2.5em 4em 3em auto;
  }
}
</style>
