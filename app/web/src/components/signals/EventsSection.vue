<script setup lang="ts">
import { computed } from "vue"
import type { CampaignEvent } from "../../api/schemas"
import { safeUrl, shortDate } from "../../format"
import { useI18n, type MessageKey } from "../../i18n"
import { eventsNewestFirst, nameOf } from "../../signals"

const props = defineProps<{ events: readonly CampaignEvent[]; names: ReadonlyMap<string, string> }>()
const { t, locale } = useI18n()

const kinds: Record<string, MessageKey> = {
  declaration: "kindDeclaration",
  withdrawal: "kindWithdrawal",
  endorsement: "kindEndorsement",
  primary_result: "kindPrimaryResult",
  sponsorships: "kindSponsorships",
  legal: "kindLegal",
  poll_published: "kindPollPublished",
  other: "kindOther",
}
const kindLabel = (kind: string) => {
  const key = kinds[kind]
  return key ? t(key) : kind
}
const events = computed(() =>
  eventsNewestFirst(props.events).map((e) => ({
    ...e,
    who: e.candidate_ids.map((id) => nameOf(props.names, id)).join(", "),
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
  <section aria-labelledby="events-title" class="card stack">
    <div class="stack-tight">
      <h3 id="events-title">{{ t("eventsTitle") }}</h3>
      <p class="secondary small">{{ t("eventsReported") }}</p>
    </div>
    <p v-if="events.length === 0" class="secondary small">{{ t("eventsEmpty") }}</p>
    <ul v-else class="events" :aria-label="t('eventsReported')">
      <li v-for="e in events" :key="e.event_id">
        <p class="meta small">
          <span class="num">{{ shortDate(e.date, locale) }}</span>
          <span class="badge">{{ kindLabel(e.kind) }}</span>
          <span v-if="e.who" class="secondary">{{ e.who }}</span>
        </p>
        <p>{{ e.summary }}</p>
        <p class="small">
          <a v-if="safeUrl(e.source_url)" :href="safeUrl(e.source_url)" rel="noopener">{{ e.source_title ?? e.host }}</a>
          <span v-else class="muted">{{ e.source_title ?? e.source_url }}</span>
        </p>
        <p v-if="e.proposed_change" class="muted small">{{ t("proposedChange", { change: e.proposed_change }) }}</p>
      </li>
    </ul>
  </section>
</template>

<style scoped>
.stack-tight {
  display: grid;
  gap: 6px;
}

.events {
  list-style: none;
  margin: 0;
  padding: 0;
  display: grid;
}

.events li {
  display: grid;
  gap: 4px;
  padding: 10px 0;
  border-bottom: 1px solid var(--grid);
}

.events li:first-child {
  padding-top: 0;
}

.events li:last-child {
  border-bottom: none;
  padding-bottom: 0;
}

.meta {
  display: flex;
  flex-wrap: wrap;
  gap: 4px 10px;
  align-items: center;
}

a {
  overflow-wrap: anywhere;
}
</style>
