<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watchEffect } from "vue"
import Cocarde from "./components/Cocarde.vue"
import Masthead from "./components/Masthead.vue"
import { daysUntil, dateTime, longDate } from "./format"
import { useI18n, type MessageKey } from "./i18n"
import { FRAMINGS, type View } from "./painting"
import { useLiveData } from "./useLiveData"
import OverviewView from "./views/OverviewView.vue"
import Round1View from "./views/Round1View.vue"
import Round2View from "./views/Round2View.vue"
import SignalsView from "./views/SignalsView.vue"
import SourcesView from "./views/SourcesView.vue"

interface ViewEntry {
  id: View
  label: MessageKey
  /** The plate number, as in a salon catalogue. */
  numeral: string
  /** The masthead's heading and lede; the overview has its own hero instead. */
  plate: { title: MessageKey; lede: MessageKey } | null
}
const views: readonly ViewEntry[] = [
  { id: "overview", label: "navOverview", numeral: "I", plate: null },
  { id: "round1", label: "navRound1", numeral: "II", plate: { title: "round1Title", lede: "round1Help" } },
  { id: "round2", label: "navRound2", numeral: "III", plate: { title: "round2Title", lede: "round2Help" } },
  { id: "signals", label: "navSignals", numeral: "IV", plate: { title: "signalsTitle", lede: "signalsIntro" } },
  { id: "sources", label: "navSources", numeral: "V", plate: { title: "sourcesTitle", lede: "sourcesIntro" } },
]

const { t, locale, toggle } = useI18n()
const { forecast, series, polls, health, signals, loading, error, missing } = useLiveData()

function viewFromHash(): View {
  const id = location.hash.replace(/^#\/?/, "")
  return views.find((v) => v.id === id)?.id ?? "overview"
}
const view = ref<View>(viewFromHash())
const onHash = () => {
  view.value = viewFromHash()
  window.scrollTo({ top: 0 })
}
onMounted(() => {
  window.addEventListener("hashchange", onHash)
  document.documentElement.lang = locale.value
})
onBeforeUnmount(() => window.removeEventListener("hashchange", onHash))

const current = computed(() => views.find((v) => v.id === view.value) ?? views[0]!)
// The views live in the hash, so the skip link moves focus itself rather than following "#main".
const main = ref<HTMLElement | null>(null)
const skipToMain = () => main.value?.focus()
watchEffect(() => {
  document.title = current.value.plate ? `${t(current.value.label)} · ${t("title")}` : t("title")
})

const daysLeft = computed(() => (forecast.value ? daysUntil(forecast.value.election.round1, new Date()) : null))
// "France 2027" set on two lines in the hero.
const brandLines = computed(() => {
  const brand = t("brand")
  const split = brand.lastIndexOf(" ")
  return split > 0 ? [brand.slice(0, split), brand.slice(split + 1)] : [brand, ""]
})
// The painting's title stays in French, set in italics inside the credit.
const PAINTING_TITLE = "Le 28 juillet. La Liberté guidant le peuple"
const credit = computed(() => t("footerPainting", { title: "\u0000" }).split("\u0000"))
</script>

<template>
  <div aria-hidden="true" class="backdrop" />
  <a class="skip-link" href="#main" @click.prevent="skipToMain">{{ t("skipToContent") }}</a>

  <header class="topbar">
    <div aria-hidden="true" class="flagline" />
    <div class="topbar-inner">
      <a class="brand" href="#/overview">
        <Cocarde />
        <span class="brand-name">{{ t("brand") }}</span>
      </a>
      <nav :aria-label="t('navLabel')" class="nav">
        <a v-for="v in views" :key="v.id" :href="`#/${v.id}`" :aria-current="view === v.id ? 'page' : undefined">
          <span aria-hidden="true" class="numeral">{{ v.numeral }}</span>
          <span>{{ t(v.label) }}</span>
        </a>
      </nav>
      <button class="chip lang" type="button" :lang="locale === 'fr' ? 'en' : 'fr'" @click="toggle">
        {{ t("language") }}
      </button>
    </div>
  </header>

  <main id="main" ref="main" class="main" tabindex="-1">
    <Masthead :framing="FRAMINGS[view]" :tall="view === 'overview'">
      <div :class="current.plate ? 'plate' : 'hero'">
        <template v-if="!current.plate">
          <p class="eyebrow"><span class="tricolore-rule" />{{ t("heroEyebrow") }}</p>
          <h1 class="hero-title">
            <span>{{ brandLines[0] }}</span>
            <em>{{ brandLines[1] }}</em>
          </h1>
          <p class="hero-lede">{{ t("subtitle") }}</p>
          <div v-if="forecast" class="hero-facts">
            <p v-if="daysLeft !== null && daysLeft >= 0" class="countdown">
              <span class="countdown-n">{{ daysLeft }}</span>
              <span class="countdown-label">{{ t("countdownLabel") }}</span>
            </p>
            <dl class="rounds">
              <div>
                <dt>{{ t("navRound1") }}</dt>
                <dd>{{ longDate(forecast.election.round1, locale) }}</dd>
              </div>
              <div>
                <dt>{{ t("navRound2") }}</dt>
                <dd>{{ longDate(forecast.election.round2, locale) }}</dd>
              </div>
            </dl>
          </div>
        </template>
        <template v-else>
          <p class="eyebrow">
            <span class="numeral">{{ current.numeral }}</span>
            <span class="tricolore-rule" />
            {{ t(current.label) }}
          </p>
          <h1>{{ t(current.plate.title) }}</h1>
          <p class="lede">{{ t(current.plate.lede) }}</p>
          <p v-if="forecast" class="updated">
            {{ t("roundDates", { r1: longDate(forecast.election.round1, locale), r2: longDate(forecast.election.round2, locale) }) }}
            <template v-if="daysLeft !== null && daysLeft >= 0"> · {{ t("daysToRound1", { n: daysLeft }) }}</template>
          </p>
        </template>
        <p v-if="forecast" class="updated">
          {{ t("updated", { when: dateTime(forecast.generated_at, locale) }) }} ·
          {{ t("asOf", { date: longDate(forecast.as_of, locale) }) }}
        </p>
        <p v-if="error && forecast" class="notice" role="status">
          <span aria-hidden="true" class="icon">!</span>
          <span>{{ t("stale", { error }) }}</span>
        </p>
      </div>
    </Masthead>

    <div class="content">
      <p v-if="loading && !forecast" class="loading">{{ t("loading") }}</p>
      <div v-else-if="missing || (!forecast && error)" class="notice" role="alert">
        <span aria-hidden="true" class="icon">!</span>
        <span>{{ missing ? t("noForecast") : error }}</span>
      </div>
      <template v-else-if="forecast">
        <OverviewView v-if="view === 'overview'" :forecast="forecast" />
        <Round1View v-else-if="view === 'round1'" :forecast="forecast" :series="series" :polls="polls" />
        <Round2View v-else-if="view === 'round2'" :forecast="forecast" />
        <SignalsView v-else-if="view === 'signals'" :forecast="forecast" :signals="signals" />
        <SourcesView v-else :forecast="forecast" :polls="polls" :health="health" />
      </template>
    </div>
  </main>

  <footer class="footer">
    <div class="footer-inner">
      <p class="footer-brand">
        <Cocarde />
        <span class="brand-name">{{ t("brand") }}</span>
        <span class="muted">{{ t("subtitle") }}</span>
      </p>
      <p class="credit">
        {{ credit[0] }}<cite>{{ PAINTING_TITLE }}</cite>{{ credit[1] }}
      </p>
      <p>
        <a href="https://github.com/Hebilicious/fr2027-forecaster" rel="noopener noreferrer">{{ t("footerCode") }}</a>
      </p>
    </div>
  </footer>
</template>

<style scoped>
.skip-link {
  position: absolute;
  left: 16px;
  top: -48px;
  z-index: 50;
  padding: 8px 14px;
  border-radius: 999px;
  background: var(--ink);
  color: var(--page);
  font-weight: 600;
  text-decoration: none;
  box-shadow: 0 0 0 6px var(--page);
}

.skip-link:focus {
  top: 12px;
}

/* The top bar: frosted glass over the painting, with the flag drawn along its top edge. */
.topbar {
  position: sticky;
  top: 0;
  z-index: 20;
  height: var(--topbar-h);
  background: linear-gradient(180deg, rgba(7, 9, 13, 0.62), rgba(7, 9, 13, 0.38));
  -webkit-backdrop-filter: blur(18px) saturate(170%);
  backdrop-filter: blur(18px) saturate(170%);
  box-shadow: inset 0 -1px 0 rgba(255, 255, 255, 0.07),
    0 12px 32px -20px rgba(0, 0, 0, 0.8);
}

.flagline {
  height: 2px;
  background: var(--tricolore);
  opacity: 0.9;
}

.topbar-inner {
  max-width: var(--content);
  height: calc(100% - 2px);
  margin: 0 auto;
  padding: 0 var(--gutter);
  display: flex;
  align-items: center;
  gap: 20px;
}

.brand {
  display: inline-flex;
  align-items: center;
  gap: 10px;
  color: var(--ink);
  text-decoration: none;
  flex: none;
}

.brand-name {
  font-family: var(--font-display);
  font-size: 1.2rem;
  font-weight: 600;
  letter-spacing: 0.01em;
}

/* The views shrink and scroll sideways before anything else in the bar is pushed out. */
.nav {
  display: flex;
  gap: 2px;
  margin-left: auto;
  min-width: 0;
  overflow-x: auto;
  scrollbar-width: none;
}

.nav a {
  position: relative;
  display: inline-flex;
  align-items: baseline;
  gap: 7px;
  padding: 8px 13px;
  border-radius: 999px;
  color: var(--ink-2);
  text-decoration: none;
  font-size: 0.86rem;
  font-weight: 500;
  white-space: nowrap;
  transition:
    color 0.2s,
    background-color 0.2s;
}

.nav a:hover {
  color: var(--ink);
  background: rgba(255, 255, 255, 0.05);
}

.nav .numeral {
  font-family: var(--font-display);
  font-style: italic;
  font-size: 0.82rem;
  color: var(--muted);
}

.nav a[aria-current="page"] {
  color: var(--ink);
  background: rgba(255, 255, 255, 0.075);
  box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.09);
}

.nav a[aria-current="page"] .numeral {
  color: var(--ink);
}

.nav a[aria-current="page"]::after {
  content: "";
  position: absolute;
  left: 50%;
  bottom: -1px;
  width: 24px;
  height: 2px;
  border-radius: 2px;
  transform: translateX(-50%);
  background: var(--tricolore);
}

.lang {
  flex: none;
  padding: 6px 13px;
  font-size: 0.8rem;
}

/* The overview's hero. --text-max keeps it on the obsidian, off the painting (Masthead.vue). */
.hero {
  max-width: min(31rem, var(--text-max, 31rem));
  display: grid;
  gap: 22px;
  animation: rise 1s var(--ease-out) both;
}

.hero-title {
  display: grid;
  font-size: clamp(4.4rem, 2.6rem + 6.4vw, 8.75rem);
  font-weight: 500;
  line-height: 0.86;
  letter-spacing: -0.035em;
  text-shadow: 0 10px 40px rgba(0, 0, 0, 0.5);
}

.hero-title em {
  font-weight: 400;
  padding-right: 0.06em;
  background: linear-gradient(100deg, var(--ink) 18%, #c7d3ff 48%, var(--ink) 78%);
  -webkit-background-clip: text;
  background-clip: text;
  color: transparent;
}

.hero-lede {
  font-family: var(--font-display);
  font-style: italic;
  font-size: clamp(1.1rem, 1rem + 0.5vw, 1.35rem);
  color: var(--ink-2);
  line-height: 1.35;
  max-width: 24rem;
}

.hero-facts {
  display: flex;
  flex-wrap: wrap;
  align-items: stretch;
  gap: 18px 28px;
  padding-top: 22px;
  border-top: 1px solid rgba(243, 239, 230, 0.14);
}

.countdown {
  display: grid;
  gap: 6px;
}

.countdown-n {
  font-family: var(--font-display);
  font-size: clamp(2.8rem, 2.2rem + 2vw, 3.75rem);
  font-weight: 500;
  line-height: 0.9;
  letter-spacing: -0.02em;
}

.countdown-label {
  font-size: 0.8rem;
  color: var(--ink-2);
  max-width: 9rem;
  line-height: 1.3;
}

.rounds {
  margin: 0;
  display: grid;
  gap: 10px;
  align-content: center;
  padding-left: 28px;
  border-left: 1px solid rgba(243, 239, 230, 0.14);
}

.rounds div {
  display: grid;
  gap: 1px;
}

.rounds dt {
  font-size: 0.68rem;
  font-weight: 600;
  letter-spacing: 0.16em;
  text-transform: uppercase;
  color: var(--muted);
}

.rounds dd {
  margin: 0;
  font-size: 0.98rem;
  color: var(--ink);
}

.updated {
  font-size: 0.8rem;
  color: var(--muted);
}

/* Every other view's plate. */
.plate {
  max-width: min(38rem, var(--text-max, 38rem));
  display: grid;
  gap: 14px;
  animation: rise 0.9s var(--ease-out) both;
}

.plate h1 {
  font-size: clamp(2.1rem, 1.5rem + 2.4vw, 3.35rem);
  line-height: 1.04;
  letter-spacing: -0.02em;
  text-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
}

.plate .numeral {
  font-family: var(--font-display);
  font-style: italic;
  font-size: 1.05rem;
  font-weight: 500;
  letter-spacing: 0;
  text-transform: none;
  color: var(--ink);
}

.lede {
  color: var(--ink-2);
  font-size: 0.95rem;
  line-height: 1.6;
  max-width: 62ch;
}

.main:focus {
  outline: none;
}

.content {
  position: relative;
  z-index: 1;
  max-width: var(--content);
  margin: 0 auto;
  padding: 0 var(--gutter) 80px;
  display: grid;
  gap: clamp(20px, 2.4vw, 28px);
}

.content > * {
  min-width: 0;
}

/* The stale-data warning sits under the update time it qualifies. */
.hero .notice,
.plate .notice {
  margin-top: 4px;
}

.loading {
  justify-self: start;
  padding: 10px 18px;
  border-radius: 999px;
  color: var(--ink-2);
  background: linear-gradient(90deg, rgba(255, 255, 255, 0.03), rgba(255, 255, 255, 0.09), rgba(255, 255, 255, 0.03));
  background-size: 200% 100%;
  animation: shimmer 1.6s linear infinite;
}

@keyframes shimmer {
  to {
    background-position: -200% 0;
  }
}

.footer {
  position: relative;
  background: rgba(4, 5, 8, 0.7);
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.06);
}

.footer::before {
  content: "";
  position: absolute;
  left: 0;
  right: 0;
  top: 0;
  height: 2px;
  background: var(--tricolore);
  opacity: 0.85;
}

.footer-inner {
  max-width: var(--content);
  margin: 0 auto;
  padding: 36px var(--gutter) 48px;
  display: grid;
  gap: 12px;
  font-size: 0.84rem;
  color: var(--muted);
}

.footer-brand {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 10px 12px;
  color: var(--ink);
}

.credit {
  max-width: 80ch;
  line-height: 1.6;
}

.credit cite {
  font-family: var(--font-display);
  font-style: italic;
  font-size: 1.05em;
  color: var(--ink-2);
}

@media (max-width: 1080px) {
  .nav .numeral {
    display: none;
  }
}

/* Below 760px the text sits under the painting rather than beside it, so it can take the width. */
@media (max-width: 760px) {
  .plate,
  .hero {
    max-width: none;
  }
}

/* Two rows: the brand and the language, then the views, which scroll sideways if they must. */
@media (max-width: 960px) {
  .topbar-inner {
    display: grid;
    grid-template-columns: 1fr auto;
    gap: 0 12px;
    padding-top: 8px;
  }

  .nav {
    grid-column: 1 / -1;
    grid-row: 2;
    margin: 0 calc(-1 * var(--gutter));
    padding: 4px var(--gutter) 8px;
    overflow-x: auto;
    scrollbar-width: none;
    -webkit-mask-image: linear-gradient(90deg, #000 calc(100% - 48px), transparent);
    mask-image: linear-gradient(90deg, #000 calc(100% - 48px), transparent);
  }

  .nav .numeral {
    display: inline;
  }

  .rounds {
    padding-left: 0;
    border-left: none;
  }
}
</style>
