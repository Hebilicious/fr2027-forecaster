<script setup lang="ts">
import { computed } from "vue"
import { useIntlayer, useLocale } from "vue-intlayer"
import type { CampaignEvent } from "../../api/schemas"
import { list, safeUrl, shortDate } from "../../format"
import { labelOf } from "../../labels"
import { eventsNewestFirst, nameOf } from "../../signals"

const props = defineProps<{ events: readonly CampaignEvent[]; names: ReadonlyMap<string, string> }>()
const content = useIntlayer("events-section")
const { locale } = useLocale()

const events = computed(() =>
  eventsNewestFirst(props.events).map((e) => ({
    ...e,
    who: list(
      e.candidate_ids.map((id) => nameOf(props.names, id)),
      locale.value,
    ),
    host: hostOf(e.source_url),
  })),
)

function hostOf(url: string): string {
  try {
    return new URL(url).hostname.replace(/^www\./, "")
  } catch {
    return url
  }
}
</script>

<template>
  <section aria-labelledby="events-title" class="card">
    <header class="card-head">
      <h2 id="events-title">{{ content.title }}</h2>
      <p class="secondary small">{{ content.reported }}</p>
    </header>
    <p v-if="events.length === 0" class="secondary small">{{ content.empty }}</p>
    <ol v-else class="timeline" :aria-label="content.reported.value">
      <li v-for="e in events" :key="e.event_id">
        <p class="meta small">
          <span class="date num">{{ shortDate(e.date, locale) }}</span>
          <span class="badge">{{ labelOf(content.kinds, e.kind) }}</span>
          <span v-if="e.who" class="secondary">{{ e.who }}</span>
        </p>
        <p class="summary">{{ e.summary }}</p>
        <p class="small">
          <a v-if="safeUrl(e.source_url)" :href="safeUrl(e.source_url)" rel="noopener">{{ e.source_title ?? e.host }}</a>
          <span v-else class="muted">{{ e.source_title ?? e.source_url }}</span>
        </p>
        <p v-if="e.proposed_change" class="muted small">{{ content.proposedChange({ change: e.proposed_change }) }}</p>
      </li>
    </ol>
  </section>
</template>

<style scoped>
/* A rail down the left, a bead for each event. */
.timeline {
  position: relative;
  list-style: none;
  margin: 0;
  padding: 0 0 0 26px;
  display: grid;
  gap: 22px;
}

.timeline::before {
  content: "";
  position: absolute;
  left: 5px;
  top: 6px;
  bottom: 6px;
  width: 1px;
  background: linear-gradient(180deg, rgba(243, 239, 230, 0.3), rgba(243, 239, 230, 0.04));
}

.timeline li {
  position: relative;
  display: grid;
  gap: 6px;
}

.timeline li::before {
  content: "";
  position: absolute;
  left: -25px;
  top: 5px;
  width: 9px;
  height: 9px;
  border-radius: 50%;
  background: var(--ink);
  box-shadow:
    0 0 0 3px var(--surface),
    0 0 12px rgba(243, 239, 230, 0.35);
}

.meta {
  display: flex;
  flex-wrap: wrap;
  gap: 4px 12px;
  align-items: center;
}

.date {
  font-weight: 600;
  color: var(--ink);
}

.summary {
  max-width: 80ch;
  line-height: 1.55;
}

a {
  overflow-wrap: anywhere;
}
</style>
