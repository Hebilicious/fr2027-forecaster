<script setup lang="ts">
import { computed } from "vue"
import { useIntlayer, useLocale } from "vue-intlayer"
import type { Forecast } from "../api/schemas"
import { FAMILIES } from "../candidates"
import { count, dateTime, daysUntil, decimal, longDate, safeUrl } from "../format"
import { labelOf } from "../labels"

// How the model works, told with the numbers of the forecast on screen: every figure is read from that run and its
// recorded settings (`forecast.parameters`), so the section cannot describe a model other than the one shown.
// method.test.ts checks that every setting the forecast schema lists has a label here.

const props = defineProps<{ forecast: Forecast }>()
const content = useIntlayer("method-section")
const common = useIntlayer("common")
const { locale } = useLocale()

const LOG_URL = "https://github.com/Hebilicious/fr2027-forecaster/blob/main/research/LOG.md"
const numerals = ["I", "II", "III", "IV", "V"]

const f = computed(() => props.forecast)
const parameters = computed(() => f.value.parameters)
const lede = computed(() =>
  content.lede({
    version: f.value.model_version,
    date: dateTime(f.value.generated_at, locale.value),
    asOf: longDate(f.value.as_of, locale.value),
  }),
)

const firms = computed(() => new Set(f.value.aggregation.house_effects.map((h) => h.firm)).size)
const possible = computed(() => f.value.candidates.filter((c) => c.p_run > 0).length)

// How much wider a candidate's round 1 error is today than on the eve of the vote.
const daysToRound1 = computed(() => Math.max(0, daysUntil(f.value.election.round1, new Date(f.value.generated_at))))
const widening = computed(() => {
  const error = parameters.value?.round1_error
  if (!error || error.candidate_sd <= 0) return null
  const months = daysToRound1.value / 30
  return Math.sqrt(error.candidate_sd ** 2 + error.horizon_candidate_sd ** 2 * months) / error.candidate_sd
})

const runoffs = computed(() => {
  const pairs = f.value.pairs
  const polled = pairs.filter((p) => p.polled).length
  const anchored = pairs.filter((p) => !p.polled && p.anchor).length
  return { polled, anchored, transfers: pairs.length - polled - anchored }
})

type Group = "round1_error" | "round2"
const settings = (group: Group): { key: string; value: string }[] => {
  const values = parameters.value?.[group]
  if (!values) return []
  return Object.entries(values)
    .filter(([key]) => key !== "transfers")
    .map(([key, value]) => ({ key, value: decimal(value as number, Number.isInteger(value) ? 0 : 2, locale.value) }))
}
const label = (group: Group, key: string) => labelOf(content.parameters[group], key)

// The transfer table: voter families in rows, finalist families and abstention in columns, each row's weights
// shaded against that row's largest so a row reads as where its voters go.
const transferColumns = computed(() => {
  const table = parameters.value?.round2.transfers ?? {}
  const keys = new Set(Object.values(table).flatMap((row) => Object.keys(row)))
  return [...FAMILIES.filter((k) => keys.has(k)), ...(keys.has("abstain") ? ["abstain"] : [])]
})
const transferRows = computed(() => {
  const table = parameters.value?.round2.transfers ?? {}
  return FAMILIES.filter((k) => table[k]).map((family) => {
    const row = table[family]!
    const max = Math.max(...Object.values(row), 0.0001)
    return {
      family,
      cells: transferColumns.value.map((k) => ({ key: k, value: row[k] ?? 0, shade: (row[k] ?? 0) / max })),
    }
  })
})
const columnLabel = (key: string) => (key === "abstain" ? content.abstain.value : labelOf(common.families, key))
</script>

<template>
  <section aria-labelledby="method-title" class="card method rise" :style="{ '--i': 4 }">
    <header class="card-head">
      <p class="eyebrow"><span class="tricolore-rule" />{{ content.eyebrow }}</p>
      <h2 id="method-title">{{ content.title }}</h2>
      <p class="secondary">{{ lede }}</p>
    </header>

    <ol class="steps">
      <li class="step">
        <span aria-hidden="true" class="numeral">{{ numerals[0] }}</span>
        <div class="body">
          <h3>{{ content.polls.title }}</h3>
          <p class="figure">
            {{
              content.polls.figure({
                polls: count(f.aggregation.polls_used, locale),
                scenarios: count(f.aggregation.scenarios_used, locale),
              })
            }}
          </p>
          <p class="secondary">{{ content.polls.text({ firms: count(firms, locale) }) }}</p>
          <p class="small muted">
            {{
              content.polls.fitted({
                deff: decimal(f.aggregation.design_effect, 1, locale),
                rw: decimal(f.aggregation.random_walk_sd, 3, locale),
              })
            }}
          </p>
        </div>
      </li>

      <li class="step">
        <span aria-hidden="true" class="numeral">{{ numerals[1] }}</span>
        <div class="body">
          <h3>{{ content.field.title }}</h3>
          <p class="figure">{{ content.field.figure({ count: count(possible, locale) }) }}</p>
          <p class="secondary">{{ content.field.text }}</p>
          <ul v-if="f.warnings.length" class="warnings">
            <li v-for="w in f.warnings" :key="w" class="notice small">
              <span aria-hidden="true" class="icon">!</span>
              <span>{{ w }}</span>
            </li>
          </ul>
        </div>
      </li>

      <li class="step">
        <span aria-hidden="true" class="numeral">{{ numerals[2] }}</span>
        <div class="body">
          <h3>{{ content.round1.title }}</h3>
          <template v-if="parameters && widening !== null">
            <p class="figure">{{ content.round1.figure({ ratio: decimal(widening, 1, locale) }) }}</p>
            <p class="secondary">
              {{
                content.round1.text({
                  days: count(daysToRound1, locale),
                  ratio: decimal(widening, 1, locale),
                  df: decimal(parameters.round1_error.student_t_df, 0, locale),
                })
              }}
            </p>
            <dl class="settings small">
              <template v-for="s in settings('round1_error')" :key="s.key">
                <dt>{{ label("round1_error", s.key) }}</dt>
                <dd class="num">{{ s.value }}</dd>
              </template>
            </dl>
          </template>
          <p v-else class="secondary">{{ content.noSettings }}</p>
        </div>
      </li>

      <li class="step">
        <span aria-hidden="true" class="numeral">{{ numerals[3] }}</span>
        <div class="body">
          <h3>{{ content.round2.title }}</h3>
          <p class="figure">
            {{
              content.round2.figure({
                polled: count(runoffs.polled, locale),
                anchored: count(runoffs.anchored, locale),
                transfers: count(runoffs.transfers, locale),
              })
            }}
          </p>
          <p class="secondary">{{ content.round2.text }}</p>
          <div v-if="parameters" class="round2-detail">
            <dl class="settings small">
              <template v-for="s in settings('round2')" :key="s.key">
                <dt>{{ label("round2", s.key) }}</dt>
                <dd class="num">{{ s.value }}</dd>
              </template>
            </dl>
            <figure class="transfers">
              <figcaption>
                <h4>{{ label("round2", "transfers") }}</h4>
                <p class="small muted">{{ content.transfersHelp }}</p>
              </figcaption>
              <div class="table-wrap">
                <table class="matrix small">
                  <thead>
                    <tr>
                      <th scope="col"><span class="sr-only">{{ content.voters }}</span></th>
                      <th v-for="key in transferColumns" :key="key" scope="col">{{ columnLabel(key) }}</th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr v-for="row in transferRows" :key="row.family">
                      <th scope="row">{{ labelOf(common.families, row.family) }}</th>
                      <td v-for="cell in row.cells" :key="cell.key">
                        <span class="shade num" :style="{ '--shade': cell.shade }">{{ decimal(cell.value, 2, locale) }}</span>
                      </td>
                    </tr>
                  </tbody>
                </table>
              </div>
            </figure>
          </div>
        </div>
      </li>

      <li class="step">
        <span aria-hidden="true" class="numeral">{{ numerals[4] }}</span>
        <div class="body">
          <h3>{{ content.simulation.title }}</h3>
          <p class="figure">{{ content.simulation.figure({ sims: count(f.simulations, locale) }) }}</p>
          <p class="secondary">{{ content.simulation.text }}</p>
        </div>
      </li>
    </ol>

    <footer class="outside">
      <p class="small secondary">{{ content.scaleNote }}</p>
      <p class="small secondary">
        {{ content.outside }}
        <a :href="safeUrl(LOG_URL)" rel="noopener noreferrer" target="_blank">{{ content.logLink }}</a>
      </p>
    </footer>
  </section>
</template>

<style scoped>
.method h2 {
  font-size: clamp(1.5rem, 2.4vw, 2rem);
}

.steps {
  list-style: none;
  margin: 0;
  padding: 0;
  display: grid;
}

/* A roman numeral in the margin, the step beside it, a hairline between steps. */
.step {
  display: grid;
  grid-template-columns: 3.2rem minmax(0, 1fr);
  gap: 18px;
  padding: 22px 0;
  border-top: 1px solid var(--grid);
}

.numeral {
  font-family: var(--font-display);
  font-style: italic;
  font-size: 1.6rem;
  line-height: 1.1;
  color: var(--muted);
  text-align: right;
}

.body {
  display: grid;
  gap: 8px;
  min-width: 0;
}

.body > p {
  max-width: 78ch;
}

h3 {
  font-family: var(--font-sans);
  font-size: 1.02rem;
  font-weight: 600;
  margin: 0;
}

h4 {
  font-family: var(--font-sans);
  font-size: 0.9rem;
  font-weight: 600;
  margin: 0 0 4px;
  text-transform: none;
  letter-spacing: 0;
  color: var(--ink);
}

.figure {
  font-family: var(--font-display);
  font-size: clamp(1.25rem, 2vw, 1.6rem);
  line-height: 1.2;
  color: var(--ink);
}

.settings {
  display: grid;
  grid-template-columns: minmax(0, max-content) auto;
  gap: 6px 18px;
  margin: 6px 0 0;
  padding: 12px 14px;
  border-radius: var(--radius-sm);
  background: rgba(255, 255, 255, 0.025);
  box-shadow: inset 0 0 0 1px var(--border);
  align-self: start;
}

.settings dt {
  color: var(--ink-2);
}

.settings dd {
  margin: 0;
  text-align: right;
  color: var(--ink);
  font-weight: 600;
}

.round2-detail {
  display: grid;
  grid-template-columns: minmax(0, 22rem) minmax(0, 1fr);
  gap: 18px;
  align-items: start;
}

.transfers {
  margin: 6px 0 0;
}

.matrix {
  background: none;
  color: var(--ink);
  font-family: var(--font-sans);
  font-style: normal;
  border-collapse: separate;
  border-spacing: 2px;
}

.matrix th {
  font-weight: 500;
  color: var(--ink-2);
  padding: 4px 6px;
  white-space: nowrap;
}

.matrix thead th {
  font-size: 0.72rem;
}

.matrix td {
  padding: 0;
}

/* One hue, light to dark, by the share of the row's largest weight. */
.shade {
  display: grid;
  place-items: center;
  min-width: 3.4rem;
  height: 28px;
  border-radius: 4px;
  background: rgba(88, 124, 241, calc(0.08 + var(--shade) * 0.72));
  color: var(--ink);
}

.warnings {
  list-style: none;
  margin: 4px 0 0;
  padding: 0;
  display: grid;
  gap: 8px;
}

.outside {
  display: grid;
  gap: 8px;
  padding-top: 18px;
  border-top: 1px solid var(--grid);
}

.outside p {
  max-width: 88ch;
}

@media (max-width: 860px) {
  .round2-detail {
    grid-template-columns: 1fr;
  }

  .step {
    grid-template-columns: 2.2rem minmax(0, 1fr);
    gap: 12px;
  }
}
</style>
