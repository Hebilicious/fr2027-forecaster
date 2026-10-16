<script setup lang="ts">
import { computed, shallowRef } from "vue"
import type { Forecast, Markets, Question, Venue } from "../../api/schemas"
import { dateTime, probability } from "../../format"
import { useI18n, type MessageKey } from "../../i18n"
import { marketRows, unmatchedRows, VENUES, type Point } from "../../signals"
import CandidateAvatar from "../CandidateAvatar.vue"
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
  <section aria-labelledby="markets-title" class="card">
    <header class="card-head head">
      <h2 id="markets-title">{{ t("marketsTitle") }}</h2>
      <div v-if="markets.fetched_at" class="segmented" role="group" :aria-label="t('marketsQuestion')">
        <button
          v-for="q in questions"
          :key="q.id"
          type="button"
          :aria-pressed="question === q.id"
          @click="question = q.id"
        >
          {{ t(q.label) }}
        </button>
      </div>
    </header>
    <p v-if="!markets.fetched_at" class="secondary small">{{ t("marketsEmpty") }}</p>
    <template v-else>
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
              <th scope="row">
                <span class="who">
                  <CandidateAvatar :id="row.candidateId" :name="row.name" :size="26" />
                  {{ row.name }}
                </span>
              </th>
              <td class="num model">{{ price(row.model) }}</td>
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

      <div v-if="unmatched.length" class="unmatched-block">
        <h3 class="sub">{{ t("unmatchedTitle") }}</h3>
        <ul class="unmatched small">
          <li v-for="u in unmatched" :key="u.label">
            <span>{{ u.label }}</span>
            <span class="num secondary">{{ unmatchedPrices(u.prices) }}</span>
          </li>
        </ul>
      </div>

      <footer class="foot small">
        <p v-for="s in failing" :key="s.id" class="notice" role="status">
          <span aria-hidden="true" class="icon">!</span>
          <span>{{ t("marketFailed", { venue: venueName[s.venue], question: questionLabel(s.question), error: s.error ?? s.status }) }}</span>
        </p>
        <p class="secondary">{{ t("marketsNote") }}</p>
        <p class="muted">{{ t("fetchedAt", { when: dateTime(markets.fetched_at, locale) }) }}</p>
      </footer>
    </template>
  </section>
</template>

<style scoped>
.who {
  display: flex;
  align-items: center;
  gap: 10px;
}

.head {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 14px 24px;
}

/* The question the markets answer: one choice of three, set like a segmented glass control. */
.segmented {
  display: inline-flex;
  padding: 4px;
  gap: 2px;
  border-radius: 999px;
  background: rgba(0, 0, 0, 0.3);
  box-shadow: inset 0 0 0 1px var(--border);
}

.segmented button {
  font: inherit;
  font-size: 0.82rem;
  font-weight: 500;
  padding: 7px 14px;
  border: none;
  border-radius: 999px;
  background: transparent;
  color: var(--ink-2);
  cursor: pointer;
  transition:
    background-color 0.2s,
    color 0.2s;
}

.segmented button:hover {
  color: var(--ink);
}

.segmented button[aria-pressed="true"] {
  background: var(--ink);
  color: var(--page);
  box-shadow: 0 4px 14px -4px rgba(243, 239, 230, 0.4);
}

th[scope="row"] {
  font-weight: 500;
  color: var(--ink);
}

.model {
  font-weight: 650;
  color: var(--ink);
}

.price {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 12px;
}

.value {
  min-width: 3.2em;
  color: var(--ink-2);
}

.sub {
  font-size: 0.72rem;
  font-weight: 600;
  letter-spacing: 0.14em;
  text-transform: uppercase;
  color: var(--muted);
}

.unmatched-block {
  display: grid;
  gap: 10px;
  margin-top: 22px;
}

.unmatched {
  list-style: none;
  margin: 0;
  padding: 0;
  display: grid;
  gap: 4px 24px;
  grid-template-columns: repeat(auto-fill, minmax(min(100%, 280px), 1fr));
}

.unmatched li {
  display: flex;
  justify-content: space-between;
  gap: 8px;
  padding: 6px 0;
  border-bottom: 1px solid var(--grid);
}

.foot {
  display: grid;
  gap: 8px;
  margin-top: 22px;
}

@media (max-width: 760px) {
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
    padding: 10px 6px;
  }
}
</style>
