<script setup lang="ts">
import { computed, ref } from "vue"
import { useIntlayer, useLocale } from "vue-intlayer"
import type { Forecast } from "../../api/schemas"
import { DIVERGING, divergingColour, pollsterLeans, type Lean } from "../../explorer"
import { relativeChange } from "../../format"
import { labelOf } from "../../labels"

const props = defineProps<{ forecast: Forecast }>()
const content = useIntlayer("pollster-leans")
const common = useIntlayer("common")
const { locale } = useLocale()

const leans = computed(() => pollsterLeans(props.forecast))
// Body columns: the axis, then the blocs outside it.
const columns = computed(() => [...leans.value.spectrum, ...leans.value.outside])
const cell = (firm: string, bloc: string) => leans.value.cells.find((c) => c.firm === firm && c.bloc === bloc)
const family = (bloc: string) => labelOf(common.families, bloc)

const pointed = ref<Lean | null>(null)
const describe = (lean: Lean) =>
  content.cellLabel({
    firm: lean.firm,
    family: family(lean.bloc),
    value: relativeChange(lean.relative, locale.value),
    lo: relativeChange(lean.lo, locale.value),
    hi: relativeChange(lean.hi, locale.value),
  }).value
const gradient = `linear-gradient(90deg, ${DIVERGING.low}, ${DIVERGING.mid}, ${DIVERGING.high})`
</script>

<template>
  <div class="leans">
    <div class="intro">
      <h3>{{ content.title }}</h3>
      <p class="secondary small">{{ content.help }}</p>
    </div>

    <p v-if="!leans.cells.length" class="secondary">{{ content.empty }}</p>
    <template v-else>
      <div aria-hidden="true" class="scale small">
        <span class="lower">{{ content.lower }}</span>
        <span class="bar" :style="{ background: gradient }" />
        <span class="higher">{{ content.higher }}</span>
        <span class="range muted">({{ relativeChange(-leans.max, locale) }} … {{ relativeChange(leans.max, locale) }})</span>
      </div>

      <div class="table-wrap">
        <table class="heat" :aria-label="content.title.value">
          <thead>
            <!-- The political families on their left–right axis, then, apart, those outside it. -->
            <tr>
              <th class="corner" rowspan="2" scope="col">{{ content.pollster }}</th>
              <th class="axis" :colspan="leans.spectrum.length" scope="colgroup">
                <span>{{ content.axisLeft }}</span>
                <span aria-hidden="true" class="axis-line" />
                <span>{{ content.axisRight }}</span>
              </th>
              <th v-if="leans.outside.length" class="axis first-outside" :colspan="leans.outside.length" scope="colgroup">
                {{ content.offAxis }}
              </th>
            </tr>
            <tr>
              <th v-for="bloc in leans.spectrum" :key="bloc" scope="col">{{ family(bloc) }}</th>
              <th
                v-for="(bloc, i) in leans.outside"
                :key="bloc"
                class="outside"
                :class="{ 'first-outside': i === 0 }"
                scope="col"
              >
                {{ family(bloc) }}
              </th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="firm in leans.firms" :key="firm">
              <th class="firm" scope="row">{{ firm }}</th>
              <td v-for="bloc in columns" :key="bloc" :class="{ 'first-outside': bloc === leans.outside[0] }">
                <span
                  v-if="cell(firm, bloc)"
                  class="cell num"
                  :class="{ faint: !cell(firm, bloc)!.clear, pointed: pointed === cell(firm, bloc) }"
                  :style="{ background: divergingColour(cell(firm, bloc)!.relative, leans.max) }"
                  tabindex="0"
                  :aria-label="describe(cell(firm, bloc)!)"
                  @pointerenter="pointed = cell(firm, bloc)!"
                  @pointerleave="pointed = null"
                  @focus="pointed = cell(firm, bloc)!"
                  @blur="pointed = null"
                >{{ relativeChange(cell(firm, bloc)!.relative, locale) }}</span>
                <span v-else aria-hidden="true" class="cell empty muted">·</span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
      <p aria-live="polite" class="readout small secondary">{{ pointed ? describe(pointed) : content.note }}</p>
    </template>
  </div>
</template>

<style scoped>
.leans {
  display: grid;
  /* One column as wide as the panel: the data table scrolls inside it instead of widening the page. */
  grid-template-columns: minmax(0, 1fr);
  gap: 16px;
}

h3 {
  font-family: var(--font-sans);
  font-size: 1.05rem;
  font-weight: 600;
  margin: 0 0 6px;
}

.intro p {
  max-width: 72ch;
}

.scale {
  display: flex;
  align-items: center;
  gap: 10px;
  color: var(--ink-2);
  flex-wrap: wrap;
}

.bar {
  width: 140px;
  height: 8px;
  border-radius: 4px;
}

/* The heatmap is a real table, styled as a grid of tiles with a 2px gap. */
.heat {
  background: none;
  color: var(--ink);
  font-family: var(--font-sans);
  font-style: normal;
  font-size: 0.84rem;
  border-collapse: separate;
  border-spacing: 2px;
  width: 100%;
  /* Equal family columns, so the axis reads symmetric. */
  table-layout: fixed;
  min-width: 640px;
}

.heat .corner {
  width: 8.5rem;
}


.heat th {
  font-weight: 500;
  color: var(--ink-2);
  padding: 6px 8px;
  text-align: center;
  white-space: nowrap;
}

.heat .corner, .heat .firm {
  text-align: left;
  color: var(--muted);
}

.heat .firm {
  color: var(--ink-2);
  font-family: var(--font-mono);
  font-size: 0.8rem;
}

.heat td {
  padding: 0;
}

/* The axis over the political families: left on the left, right on the right. */
.heat th.axis {
  padding: 0 0 2px;
  font-size: 0.72rem;
  letter-spacing: 0.12em;
  text-transform: uppercase;
}

.axis > span {
  vertical-align: middle;
}

.axis-line {
  display: inline-block;
  width: min(40%, 280px);
  height: 1px;
  margin: 0 12px;
  background: linear-gradient(90deg, var(--axis), var(--grid), var(--axis));
}

/* Over a single column, the label of the families outside the axis wraps. */
.heat th.axis.first-outside {
  white-space: normal;
  letter-spacing: 0.06em;
  line-height: 1.3;
}

/* The families outside the axis stand apart, behind a hairline. */
.heat .first-outside {
  padding-left: 18px;
  background: linear-gradient(var(--axis), var(--axis)) 8px 0 / 1px 100% no-repeat;
}

.heat th.outside {
  color: var(--muted);
}

.cell {
  display: grid;
  place-items: center;
  height: 36px;
  border-radius: 4px;
  color: var(--ink);
  font-weight: 600;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.45);
  transition: box-shadow 0.15s, opacity 0.15s;
}

/* A lean smaller than its own uncertainty: shown, but faded. */
.cell.faint {
  opacity: 0.45;
  font-weight: 400;
}

.cell.pointed, .cell:focus-visible {
  outline: none;
  opacity: 1;
  box-shadow: inset 0 0 0 2px var(--ink);
}

.cell.empty {
  background: rgba(255, 255, 255, 0.02);
}

.readout {
  min-height: 1.3em;
  margin: 0;
}

/* On a phone the two ends of the scale sit above the ends of a full-width bar. */
@media (max-width: 480px) {
  .scale {
    display: grid;
    grid-template-columns: minmax(0, 1fr) minmax(0, 1fr);
    gap: 6px 12px;
  }

  .higher {
    text-align: right;
  }

  .bar {
    grid-column: 1 / -1;
    grid-row: 2;
    width: auto;
  }

  .range {
    grid-column: 1 / -1;
  }
}
</style>
