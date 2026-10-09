<script setup lang="ts">
import { computed, shallowRef } from "vue"
import type { Forecast, Markets, Question, Venue } from "../../api/schemas"
import { dateTime, probability } from "../../format"
import { useI18n, type MessageKey } from "../../i18n"
import { marketRows, unmatchedRows, VENUES, type Point } from "../../signals"
import Sparkline from "../Sparkline.vue"

const props = defineProps<{ forecast: Forecast; markets: Markets }>()
const { t, locale } = useI18n()

const questions: readonly { id: Question; label: MessageKey }[] = [
  { id: "win", label: "wins" },
  { id: "qualify", label: "qualifies" },
  { id: "on_ballot", label: "onBallot" },
]
const question = shallowRef<Question>("win")
const questionLabel = (id: Question) => t(questions.find((q) => q.id === id)?.label ?? "wins")
const venueName: Record<Venue, string> = { polymarket: "Polymarket", kalshi: "Kalshi" }

const table = computed(() => marketRows(props.forecast, props.markets, question.value))
const unmatched = computed(() => unmatchedRows(props.markets, question.value))
const failing = computed(() => props.markets.sources.filter((s) => s.status !== "ok"))
const price = (p: number | null) => (p === null ? "—" : probability(p, locale.value))
const trendLabel = (venue: Venue, points: readonly Point[]) =>
  t("priceTrend", {
    venue: venueName[venue],
    days: 30,
    from: price(points[0]?.value ?? null),
    to: price(points.at(-1)?.value ?? null),
  })
const unmatchedPrices = (prices: Readonly<Record<Venue, number | null>>) =>
  VENUES.filter((v) => prices[v] !== null)
    .map((v) => `${venueName[v]} ${price(prices[v])}`)
    .join(" · ")
</script>

<template>
  <section aria-labelledby="markets-title" class="card stack">
    <h3 id="markets-title">{{ t("marketsTitle") }}</h3>
    <p v-if="!markets.fetched_at" class="secondary small">{{ t("marketsEmpty") }}</p>
    <template v-else>
      <div class="chips" role="group" :aria-label="t('marketsQuestion')">
        <button
          v-for="q in questions"
          :key="q.id"
          class="chip"
          type="button"
          :aria-pressed="question === q.id"
          @click="question = q.id"
        >
          {{ t(q.label) }}
        </button>
      </div>

      <div class="table-wrap">
        <table>
          <thead>
            <tr>
              <th scope="col">{{ t("candidate") }}</th>
              <th class="num" scope="col">{{ t("model") }}</th>
              <th v-for="venue in VENUES" :key="venue" class="num" scope="col">{{ venueName[venue] }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="row in table.rows" :key="row.candidateId">
              <th scope="row">{{ row.name }}</th>
              <td class="num strong">{{ price(row.model) }}</td>
              <td v-for="venue in VENUES" :key="venue" class="num">
                <div class="price">
                  <Sparkline
                    v-if="table.window && row.history[venue].length"
                    :points="row.history[venue]"
                    :window="table.window"
                    :label="trendLabel(venue, row.history[venue])"
                    :min-span="0.05"
                  />
                  <span class="value">{{ price(row.prices[venue]) }}</span>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <div v-if="unmatched.length" class="stack-tight">
        <h4>{{ t("unmatchedTitle") }}</h4>
        <ul class="unmatched small">
          <li v-for="u in unmatched" :key="u.label">
            <span>{{ u.label }}</span>
            <span class="num secondary">{{ unmatchedPrices(u.prices) }}</span>
          </li>
        </ul>
      </div>

      <div class="stack-tight small">
        <p class="muted">{{ t("fetchedAt", { when: dateTime(markets.fetched_at, locale) }) }}</p>
        <p v-for="s in failing" :key="s.id" class="notice" role="status">
          <span aria-hidden="true" class="icon">!</span>
          <span>{{ t("marketFailed", { venue: venueName[s.venue], question: questionLabel(s.question), error: s.error ?? s.status }) }}</span>
        </p>
        <p class="secondary">{{ t("marketsNote") }}</p>
      </div>
    </template>
  </section>
</template>

<style scoped>
th[scope="row"] {
  font-weight: 600;
  color: var(--ink);
}

.strong {
  font-weight: 600;
}

.price {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 8px;
}

.value {
  min-width: 3.2em;
}

h4 {
  margin: 0;
  font-size: 0.85rem;
  font-weight: 600;
  color: var(--ink-2);
}

.stack-tight {
  display: grid;
  gap: 6px;
}

.unmatched {
  list-style: none;
  margin: 0;
  padding: 0;
  display: grid;
  gap: 2px 16px;
  grid-template-columns: repeat(auto-fill, minmax(min(100%, 260px), 1fr));
}

.unmatched li {
  display: flex;
  justify-content: space-between;
  gap: 8px;
}

@media (max-width: 720px) {
  .price {
    flex-direction: column-reverse;
    align-items: flex-end;
    gap: 2px;
  }

  .value {
    min-width: 0;
  }

  th,
  td {
    padding: 8px 6px;
  }
}
</style>
