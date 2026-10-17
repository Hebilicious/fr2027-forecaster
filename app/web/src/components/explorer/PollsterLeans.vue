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
        <span>{{ content.lower }}</span>
        <span class="bar" :style="{ background: gradient }" />
        <span>{{ content.higher }}</span>
        <span class="muted">({{ relativeChange(-leans.max, locale) }} … {{ relativeChange(leans.max, locale) }})</span>
      </div>

      <div class="table-wrap">
        <table class="heat" :aria-label="content.title.value">
          <thead>
            <tr>
              <th class="corner" scope="col">{{ content.pollster }}</th>
              <th v-for="bloc in leans.blocs" :key="bloc" scope="col">{{ family(bloc) }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="firm in leans.firms" :key="firm">
              <th class="firm" scope="row">{{ firm }}</th>
              <td v-for="bloc in leans.blocs" :key="bloc">
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

.cell {
  display: grid;
  place-items: center;
  min-width: 4.5rem;
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
</style>
