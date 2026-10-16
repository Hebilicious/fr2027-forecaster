<script setup lang="ts">
import { useIntlayer, useLocale } from "vue-intlayer"
import type { FieldView } from "../field"
import { count } from "../format"

// The search and the stand-in toggle above a list of candidates. The state is shared by every view. A view that
// lists only some candidates (those with a poll history) passes its own counts.
const props = defineProps<{ field: FieldView; total: number; shown?: number }>()
const query = defineModel<string>("query", { required: true })
const showAll = defineModel<boolean>("showAll", { required: true })
const content = useIntlayer("field-filter")
const { locale } = useLocale()
</script>

<template>
  <div class="field-filter" role="search" :aria-label="content.label.value">
    <label class="search">
      <span class="sr-only">{{ content.search }}</span>
      <span aria-hidden="true" class="glass-icon" />
      <input v-model="query" type="search" :placeholder="content.placeholder.value" autocomplete="off" />
      <button v-if="query" class="clear" type="button" :aria-label="content.clear.value" @click="query = ''">
        <span aria-hidden="true">×</span>
      </button>
    </label>
    <button v-if="field.notRunning > 0" class="chip" type="button" :aria-pressed="showAll" @click="showAll = !showAll">
      {{ content.showAll }} <span class="num tally">{{ count(field.notRunning, locale) }}</span>
    </button>
    <span class="small muted num shown" role="status">
      {{ content.shown({ shown: count(props.shown ?? field.visible.length, locale), total: count(total, locale) }) }}
    </span>
    <p v-if="(shown ?? field.visible.length) === 0" class="secondary empty">{{ content.noMatch }}</p>
  </div>
</template>

<style scoped>
.field-filter {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 10px 12px;
}

.shown {
  margin-left: auto;
}

.tally {
  margin-left: 4px;
  opacity: 0.6;
}

.empty {
  flex-basis: 100%;
}

.search {
  position: relative;
  display: flex;
  align-items: center;
  flex: 0 1 18rem;
  min-width: 12rem;
}

.search input {
  width: 100%;
  font: inherit;
  font-size: 0.88rem;
  color: var(--ink);
  padding: 8px 34px 8px 34px;
  border: none;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.045);
  box-shadow: inset 0 0 0 1px var(--border);
  appearance: none;
}

.search input::placeholder {
  color: var(--muted);
}

.search input::-webkit-search-cancel-button {
  display: none;
}

.search input:focus-visible {
  outline: 2px solid var(--focus);
  outline-offset: 2px;
}

/* A magnifying glass drawn in CSS: a ring and its handle. */
.glass-icon {
  position: absolute;
  left: 13px;
  width: 11px;
  height: 11px;
  border: 1.5px solid var(--muted);
  border-radius: 50%;
  pointer-events: none;
}

.glass-icon::after {
  content: "";
  position: absolute;
  right: -4px;
  bottom: -4px;
  width: 5px;
  height: 1.5px;
  background: var(--muted);
  transform: rotate(45deg);
}

.clear {
  position: absolute;
  right: 6px;
  width: 24px;
  height: 24px;
  border: none;
  border-radius: 50%;
  background: transparent;
  color: var(--muted);
  font-size: 1.1rem;
  line-height: 1;
  cursor: pointer;
}

.clear:hover {
  color: var(--ink);
}
</style>
