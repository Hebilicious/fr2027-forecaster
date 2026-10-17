<script setup lang="ts">
import { computed, ref, useId } from "vue"
import { useIntlayer, useLocale } from "vue-intlayer"
import type { Blend, Forecast, HistoryRun, TrackRecord as TrackRecordData } from "../../api/schemas"
import { percent } from "../../format"
import ChancesOverTime from "./ChancesOverTime.vue"
import PathsToVictory from "./PathsToVictory.vue"
import PollsterLeans from "./PollsterLeans.vue"
import TrackRecord from "./TrackRecord.vue"

const props = defineProps<{
  forecast: Forecast
  history: readonly HistoryRun[]
  trackRecord: TrackRecordData | null
  blend?: Blend | null
}>()
const content = useIntlayer("model-explorer")
const { locale } = useLocale()

const tabs = ["paths", "time", "pollsters", "record"] as const
type Tab = (typeof tabs)[number]
const active = ref<Tab>("paths")
const id = useId()
const tabId = (tab: Tab) => `${id}-tab-${tab}`
const panelId = (tab: Tab) => `${id}-panel-${tab}`
const buttons = ref<HTMLButtonElement[]>([])

// Arrow keys move between tabs and select them (the panels are cheap to show), as in the WAI-ARIA tabs pattern.
function onKey(event: KeyboardEvent, index: number) {
  const next =
    event.key === "ArrowRight"
      ? (index + 1) % tabs.length
      : event.key === "ArrowLeft"
        ? (index - 1 + tabs.length) % tabs.length
        : event.key === "Home"
          ? 0
          : event.key === "End"
            ? tabs.length - 1
            : null
  if (next === null) return
  event.preventDefault()
  active.value = tabs[next]!
  buttons.value[next]?.focus()
}

const activeIndex = computed(() => tabs.indexOf(active.value))
const hasRecord = computed(() => (props.trackRecord?.by_horizon.length ?? 0) > 0)
</script>

<template>
  <section aria-labelledby="explorer-title" class="card explorer rise" :style="{ '--i': 1 }">
    <header class="card-head">
      <p class="eyebrow"><span class="tricolore-rule" />{{ content.eyebrow }}</p>
      <h2 id="explorer-title">{{ content.title }}</h2>
      <p class="secondary">{{ content.help }}</p>
      <!-- The headline chances mix in the markets and follow the reader's settings; these are the model's. -->
      <p v-if="blend" class="small muted model-only">
        {{ content.modelOnly({ weight: percent(1 - blend.model_weight, 0, locale) }) }}
      </p>
    </header>

    <div class="tablist" role="tablist" :aria-label="content.tabsLabel.value" :style="{ '--active': activeIndex }">
      <button
        v-for="(tab, i) in tabs"
        :id="tabId(tab)"
        :key="tab"
        ref="buttons"
        class="tab"
        role="tab"
        type="button"
        :aria-selected="active === tab"
        :aria-controls="panelId(tab)"
        :tabindex="active === tab ? 0 : -1"
        @click="active = tab"
        @keydown="onKey($event, i)"
      >
        <span class="tab-label">{{ content.tabs[tab].label }}</span>
        <span class="tab-hint">{{ content.tabs[tab].hint }}</span>
      </button>
      <span aria-hidden="true" class="indicator" />
    </div>

    <div
      v-for="tab in tabs"
      v-show="active === tab"
      :id="panelId(tab)"
      :key="tab"
      class="panel"
      role="tabpanel"
      :aria-labelledby="tabId(tab)"
      tabindex="0"
    >
      <template v-if="active === tab">
        <PathsToVictory v-if="tab === 'paths'" :forecast="forecast" />
        <ChancesOverTime v-else-if="tab === 'time'" :forecast="forecast" :history="history" />
        <PollsterLeans v-else-if="tab === 'pollsters'" :forecast="forecast" />
        <template v-else>
          <TrackRecord v-if="trackRecord && hasRecord" :record="trackRecord" />
          <p v-else class="secondary">{{ content.noRecord }}</p>
        </template>
      </template>
    </div>
  </section>
</template>

<style scoped>
.model-only {
  margin-top: 8px;
  max-width: 70ch;
}

.explorer h2 {
  font-size: clamp(1.5rem, 2.4vw, 2rem);
}

/* Four equal tabs on a hairline, with a bar that slides under the selected one. */
.tablist {
  position: relative;
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  margin: 0 0 26px;
  border-bottom: 1px solid var(--grid);
}

.tab {
  display: grid;
  gap: 4px;
  justify-items: start;
  text-align: left;
  padding: 12px 14px 16px;
  border: none;
  border-radius: var(--radius-sm) var(--radius-sm) 0 0;
  background: transparent;
  color: var(--muted);
  font: inherit;
  cursor: pointer;
  transition:
    color 0.2s,
    background-color 0.2s;
}

.tab:hover {
  color: var(--ink-2);
  background: rgba(255, 255, 255, 0.03);
}

.tab[aria-selected="true"] {
  color: var(--ink);
}

.tab:focus-visible {
  outline: 2px solid var(--focus);
  outline-offset: -2px;
}

.tab-label {
  font-weight: 600;
  font-size: 0.98rem;
}

.tab-hint {
  font-size: 0.78rem;
  line-height: 1.35;
  color: var(--muted);
}

.indicator {
  position: absolute;
  left: 0;
  bottom: -1px;
  width: 25%;
  height: 2px;
  border-radius: 2px;
  background: var(--tricolore);
  transform: translateX(calc(var(--active) * 100%));
  transition: transform 0.45s var(--ease-out);
}

.panel:focus-visible {
  outline: 2px solid var(--focus);
  outline-offset: 6px;
  border-radius: var(--radius-sm);
}

.panel > * {
  animation: fade 0.45s var(--ease-out) both;
}

@keyframes fade {
  from {
    opacity: 0;
    transform: translateY(6px);
  }
}

@media (max-width: 760px) {
  .tablist {
    grid-template-columns: repeat(2, minmax(0, 1fr));
    border-bottom: none;
    gap: 4px;
  }

  .tab {
    border-radius: var(--radius-sm);
    box-shadow: inset 0 0 0 1px var(--border);
    padding: 10px 12px;
  }

  .tab[aria-selected="true"] {
    background: rgba(255, 255, 255, 0.06);
    box-shadow: inset 0 0 0 1px rgba(243, 239, 230, 0.4);
  }

  .tab-hint,
  .indicator {
    display: none;
  }
}

@media (prefers-reduced-motion: reduce) {
  .indicator,
  .panel > * {
    transition: none;
    animation: none;
  }
}
</style>
