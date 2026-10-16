<script setup lang="ts">
import { computed } from "vue"
import { useIntlayer, useLocale } from "vue-intlayer"
import type { Attention } from "../../api/schemas"
import { count, longDate, relativeChange } from "../../format"
import { attentionRows, type AttentionRow } from "../../signals"
import CandidateAvatar from "../CandidateAvatar.vue"
import Sparkline from "../Sparkline.vue"

const props = defineProps<{ attention: Attention; names: ReadonlyMap<string, string> }>()
const content = useIntlayer("attention-section")
const common = useIntlayer("common")
const { locale } = useLocale()

const table = computed(() => attentionRows(props.attention.rows, props.names))
const trendLabel = (row: AttentionRow) =>
  content.trend({
    days: 90,
    from: count(row.daily[0]?.value ?? 0, locale.value),
    to: count(row.daily.at(-1)?.value ?? 0, locale.value),
  }).value
</script>

<template>
  <section aria-labelledby="attention-title" class="card">
    <header class="card-head">
      <h2 id="attention-title">{{ content.title }}</h2>
      <p class="secondary small">{{ content.help }}</p>
    </header>
    <p v-if="table.rows.length === 0" class="secondary small">{{ content.empty }}</p>
    <template v-else>
      <div class="table-wrap">
        <table>
          <thead>
            <tr>
              <th scope="col">{{ common.candidate }}</th>
              <th class="num" scope="col">{{ content.last7Days }}</th>
              <th class="num" scope="col">{{ content.vsPrevious7 }}</th>
              <th scope="col">{{ content.last90Days }}</th>
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
              <td class="num strong">{{ count(row.total, locale) }}</td>
              <td class="num secondary">{{ row.change === null ? "—" : relativeChange(row.change, locale) }}</td>
              <td>
                <Sparkline
                  v-if="table.window && row.daily.length >= 2"
                  :points="row.daily"
                  :window="table.window"
                  :label="trendLabel(row)"
                />
              </td>
            </tr>
          </tbody>
        </table>
      </div>
      <p v-if="table.window" class="muted small foot">{{ content.dataTo({ date: longDate(table.window[1], locale) }) }}</p>
    </template>
  </section>
</template>

<style scoped>
.who {
  display: flex;
  align-items: center;
  gap: 10px;
}

th[scope="row"] {
  font-weight: 500;
  color: var(--ink);
}

.strong {
  font-weight: 650;
  color: var(--ink);
}

.foot {
  margin-top: 16px;
}

@media (max-width: 760px) {
  th,
  td {
    padding: 10px 6px;
  }
}
</style>
