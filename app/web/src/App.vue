<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from "vue"
import { daysUntil, dateTime, longDate } from "./format"
import { useI18n, type MessageKey } from "./i18n"
import { useLiveData } from "./useLiveData"
import OverviewView from "./views/OverviewView.vue"
import Round1View from "./views/Round1View.vue"
import Round2View from "./views/Round2View.vue"
import SourcesView from "./views/SourcesView.vue"

type View = "overview" | "round1" | "round2" | "sources"
const views: readonly { id: View; label: MessageKey }[] = [
  { id: "overview", label: "navOverview" },
  { id: "round1", label: "navRound1" },
  { id: "round2", label: "navRound2" },
  { id: "sources", label: "navSources" },
]

const { t, locale, toggle } = useI18n()
const { forecast, series, polls, health, loading, error, missing } = useLiveData()

function viewFromHash(): View {
  const id = location.hash.replace(/^#\/?/, "")
  return views.find((v) => v.id === id)?.id ?? "overview"
}
const view = ref<View>(viewFromHash())
const onHash = () => (view.value = viewFromHash())
onMounted(() => {
  window.addEventListener("hashchange", onHash)
  document.documentElement.lang = locale.value
})
onBeforeUnmount(() => window.removeEventListener("hashchange", onHash))

const daysLeft = computed(() => (forecast.value ? daysUntil(forecast.value.election.round1, new Date()) : null))
</script>

<template>
  <header class="header">
    <div class="header-inner">
      <div>
        <h1>{{ t("title") }}</h1>
        <p class="secondary small">
          <template v-if="forecast">
            {{ t("roundDates", { r1: longDate(forecast.election.round1, locale), r2: longDate(forecast.election.round2, locale) }) }}
            <template v-if="daysLeft !== null && daysLeft >= 0"> · {{ t("daysToRound1", { n: daysLeft }) }}</template>
          </template>
          <template v-else>{{ t("subtitle") }}</template>
        </p>
      </div>
      <div class="header-side">
        <p v-if="forecast" class="muted small">
          {{ t("updated", { when: dateTime(forecast.generated_at, locale) }) }} ·
          {{ t("asOf", { date: longDate(forecast.as_of, locale) }) }}
        </p>
        <button class="chip" type="button" :lang="locale === 'fr' ? 'en' : 'fr'" @click="toggle">
          {{ t("language") }}
        </button>
      </div>
    </div>
    <nav aria-label="Views" class="nav">
      <a v-for="v in views" :key="v.id" :href="`#/${v.id}`" :aria-current="view === v.id ? 'page' : undefined">
        {{ t(v.label) }}
      </a>
    </nav>
  </header>

  <main class="main">
    <div v-if="error && forecast" class="notice" role="status">
      <span aria-hidden="true" class="icon">!</span>
      <span>{{ t("stale", { error }) }}</span>
    </div>
    <p v-if="loading && !forecast" class="muted">{{ t("loading") }}</p>
    <div v-else-if="missing || (!forecast && error)" class="notice" role="alert">
      <span aria-hidden="true" class="icon">!</span>
      <span>{{ missing ? t("noForecast") : error }}</span>
    </div>
    <template v-else-if="forecast">
      <OverviewView v-if="view === 'overview'" :forecast="forecast" />
      <Round1View v-else-if="view === 'round1'" :forecast="forecast" :series="series" :polls="polls" />
      <Round2View v-else-if="view === 'round2'" :forecast="forecast" />
      <SourcesView v-else :forecast="forecast" :polls="polls" :health="health" />
    </template>
  </main>
</template>

<style scoped>
.header {
  background: var(--surface);
  border-bottom: 1px solid var(--border);
}

.header-inner {
  max-width: 1200px;
  margin: 0 auto;
  padding: 16px 16px 8px;
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  justify-content: space-between;
  align-items: flex-start;
}

.header h1 {
  font-size: 1.35rem;
}

.header-side {
  display: flex;
  gap: 12px;
  align-items: center;
  flex-wrap: wrap;
}

.nav {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 16px;
  display: flex;
  flex-wrap: wrap;
  gap: 0 4px;
}

.nav a {
  padding: 8px 10px;
  color: var(--ink-2);
  text-decoration: none;
  border-bottom: 2px solid transparent;
  white-space: nowrap;
}

.nav a[aria-current="page"] {
  color: var(--ink);
  border-bottom-color: var(--series);
  font-weight: 600;
}

.main {
  max-width: 1200px;
  margin: 0 auto;
  padding: 16px;
  display: grid;
  gap: 16px;
}
</style>
