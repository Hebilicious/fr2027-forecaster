<script setup lang="ts">
import { computed } from "vue"
import { useIntlayer, useLocale } from "vue-intlayer"
import type { Forecast } from "../api/schemas"
import { byFamily } from "../candidates"
import CandidateAvatar from "../components/CandidateAvatar.vue"
import { list, probability, safeUrl, shareWithInterval } from "../format"
import { labelOf } from "../labels"
import { PARTY_SITES, PROGRAMMES } from "../links"
import { ARTICLES, CREDITS } from "../portraits"

const props = defineProps<{ forecast: Forecast }>()
const content = useIntlayer("candidates-view")
const common = useIntlayer("common")
const { locale } = useLocale()

const groups = computed(() => byFamily(props.forecast.candidates))
const byName = (a: { name: string }, b: { name: string }) => a.name.localeCompare(b.name, "fr")
const credited = computed(() =>
  props.forecast.candidates
    .flatMap((c) => {
      const credit = CREDITS[c.candidate_id]
      return credit ? [{ id: c.candidate_id, name: c.name, credit }] : []
    })
    .toSorted(byName),
)
const uncredited = computed(() =>
  props.forecast.candidates
    .filter((c) => !CREDITS[c.candidate_id])
    .toSorted(byName)
    .map((c) => c.name),
)
</script>

<template>
  <div class="candidates">
    <section
      v-for="(group, i) in groups"
      :key="group.family"
      :aria-labelledby="`family-${group.family}`"
      class="group rise"
      :style="{ '--i': i }"
    >
      <header class="group-head">
        <h2 :id="`family-${group.family}`">{{ labelOf(content.families, group.family) }}</h2>
        <span class="count num">{{ group.candidates.length }}</span>
      </header>
      <ul class="cards">
        <li v-for="c in group.candidates" :key="c.candidate_id" class="card person">
          <div class="who">
            <CandidateAvatar :id="c.candidate_id" :name="c.name" :size="64" />
            <div class="identity">
              <h3>{{ c.name }}</h3>
              <p class="party">{{ c.party }}</p>
              <p><span class="badge">{{ labelOf(content.statuses, c.status) }}</span></p>
            </div>
          </div>
          <dl class="stats">
            <div>
              <dt>{{ common.runs }}</dt>
              <dd>{{ probability(c.p_run, locale) }}</dd>
            </div>
            <div>
              <dt>{{ content.round2 }}</dt>
              <dd>{{ probability(c.p_qualify_r1, locale) }}</dd>
            </div>
            <div>
              <dt>{{ common.wins }}</dt>
              <dd class="strong">{{ probability(c.p_win, locale) }}</dd>
            </div>
          </dl>
          <p class="share">
            <span class="muted">{{ common.r1Share }}</span>
            <span v-if="c.r1_share" class="num">{{
                shareWithInterval(c.r1_share.mean, c.r1_share.lo80, c.r1_share.hi80, locale)
              }}</span>
            <span v-else class="muted">{{ c.simulated ? "—" : common.notPolled }}</span>
          </p>
          <p class="links">
            <a v-if="PARTY_SITES[c.candidate_id]" :href="safeUrl(PARTY_SITES[c.candidate_id])" rel="noopener noreferrer">
              {{ content.partySite }}<span aria-hidden="true"> ↗</span>
            </a>
            <a v-if="PROGRAMMES[c.candidate_id]" :href="safeUrl(PROGRAMMES[c.candidate_id])" rel="noopener noreferrer">
              {{ content.programme }}<span aria-hidden="true"> ↗</span>
            </a>
            <span v-else class="muted">{{ content.programmeNone }}</span>
            <a v-if="ARTICLES[c.candidate_id]" :href="safeUrl(ARTICLES[c.candidate_id])" rel="noopener noreferrer">
              {{ content.wikipedia }}<span aria-hidden="true"> ↗</span>
            </a>
          </p>
        </li>
      </ul>
    </section>

    <section aria-labelledby="credits-title" class="card rise" :style="{ '--i': groups.length }">
      <header class="card-head">
        <h2 id="credits-title">{{ content.photoCredits }}</h2>
        <p class="secondary small">{{ content.photoCreditsNote }}</p>
      </header>
      <ul class="credits small">
        <li v-for="c in credited" :key="c.id">
          <span class="credit-name">{{ c.name }}</span>
          <span class="secondary">{{ c.credit.author }}</span>
          <span>
            <a v-if="c.credit.licenseUrl" :href="safeUrl(c.credit.licenseUrl)" rel="noopener noreferrer">{{ c.credit.license }}</a>
            <span v-else>{{ c.credit.license }}</span>
            · <a :href="safeUrl(c.credit.file)" rel="noopener noreferrer">{{ content.commons }}</a>
          </span>
        </li>
      </ul>
      <p v-if="uncredited.length" class="muted small none">{{ content.photoCreditsNone({ names: list(uncredited, locale) }) }}</p>
    </section>
  </div>
</template>

<style scoped>
.candidates {
  display: grid;
  gap: clamp(28px, 3.4vw, 44px);
}

.group {
  display: grid;
  gap: 16px;
}

/* The group's name, its count, then a hairline running to the edge. */
.group-head {
  display: flex;
  align-items: center;
  gap: 14px;
}

.group-head::after {
  content: "";
  flex: 1;
  height: 1px;
  background: linear-gradient(90deg, rgba(243, 239, 230, 0.2), rgba(243, 239, 230, 0));
}

.count {
  font-size: 0.78rem;
  font-weight: 600;
  color: var(--ink-2);
  padding: 2px 9px;
  border-radius: 999px;
  box-shadow: inset 0 0 0 1px var(--border);
}

.cards {
  list-style: none;
  margin: 0;
  padding: 0;
  display: grid;
  gap: clamp(14px, 1.6vw, 20px);
  grid-template-columns: repeat(auto-fill, minmax(min(100%, 290px), 1fr));
}

.person {
  display: grid;
  gap: 16px;
  align-content: start;
  padding: 20px;
  transition: transform 0.3s var(--ease-out);
}

.identity .badge {
  margin-top: 4px;
}

.person:hover {
  transform: translateY(-2px);
}

.who {
  display: flex;
  align-items: center;
  gap: 16px;
}

.identity {
  display: grid;
  gap: 3px;
  min-width: 0;
}

h3 {
  font-size: 1.02rem;
  font-weight: 600;
}

.party {
  font-size: 0.82rem;
  color: var(--muted);
  line-height: 1.35;
}

.stats {
  margin: 0;
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 8px;
  padding: 12px 0;
  border-top: 1px solid var(--grid);
  border-bottom: 1px solid var(--grid);
}

.stats div {
  display: grid;
  gap: 3px;
}

.stats dt {
  font-size: 0.64rem;
  font-weight: 600;
  letter-spacing: 0.12em;
  text-transform: uppercase;
  color: var(--muted);
}

.stats dd {
  margin: 0;
  font-size: 1.15rem;
  font-weight: 600;
  color: var(--ink-2);
  letter-spacing: -0.01em;
}

.stats dd.strong {
  color: var(--ink);
}

.share {
  display: grid;
  gap: 2px;
  font-size: 0.84rem;
}

.links {
  display: flex;
  flex-wrap: wrap;
  gap: 4px 16px;
  font-size: 0.8rem;
}

.credits {
  list-style: none;
  margin: 0;
  padding: 0;
  display: grid;
  gap: 2px 28px;
  grid-template-columns: repeat(auto-fill, minmax(min(100%, 330px), 1fr));
}

.credits li {
  display: grid;
  gap: 1px;
  padding: 8px 0;
  border-bottom: 1px solid var(--grid);
  overflow-wrap: anywhere;
}

.credit-name {
  color: var(--ink);
  font-weight: 500;
}

.none {
  margin-top: 16px;
}
</style>
