<script setup lang="ts">
import { computed } from "vue"
import { useIntlayer, useLocale } from "vue-intlayer"
import type { Blend } from "../api/schemas"
import { dateOrTime, percent } from "../format"

// The slider that sets how much the prediction markets weigh in the forecast the reader sees, from
// 0 (the model alone) to 1 (the markets alone). `published` is the site's own weight.
const props = defineProps<{ blend: Blend; published: number }>()
const weight = defineModel<number>({ required: true })
const content = useIntlayer("market-weight")
const { locale } = useLocale()

const percentValue = computed({
  get: () => Math.round(weight.value * 100),
  set: (value: number) => {
    weight.value = value / 100
  },
})
</script>

<template>
  <div class="market-weight">
    <label class="row">
      <span class="label">{{ content.label({ weight: percent(weight, 0, locale) }) }}</span>
      <input
        v-model.number="percentValue"
        max="100"
        min="0"
        step="5"
        type="range"
        :aria-label="content.slider.value"
        :aria-valuetext="percent(weight, 0, locale)"
      />
    </label>
    <p class="small secondary">
      {{ content.help({ default: percent(published, 0, locale) }) }}
      <button v-if="Math.abs(weight - published) > 1e-9" class="reset" type="button" @click="weight = published">
        {{ content.reset({ default: percent(published, 0, locale) }) }}
      </button>
    </p>
    <p v-if="!props.blend.market_used && props.blend.market_fetched_at" class="notice small" role="status">
      <span aria-hidden="true" class="icon">!</span>
      <span>{{ content.stale({ when: dateOrTime(props.blend.market_fetched_at, locale) }) }}</span>
    </p>
  </div>
</template>

<style scoped>
.market-weight {
  display: grid;
  gap: 6px;
  margin: 4px 0 18px;
}

.row {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px 16px;
}

.label {
  font-weight: 600;
  min-width: 18rem;
}

input[type="range"] {
  flex: 1 1 220px;
  max-width: 420px;
  accent-color: var(--series);
}

.reset {
  margin-left: 8px;
  background: none;
  border: none;
  color: var(--link);
  text-decoration: underline;
  cursor: pointer;
  font: inherit;
  padding: 0;
}
</style>
