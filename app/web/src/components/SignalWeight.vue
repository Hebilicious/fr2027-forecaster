<script setup lang="ts">
import { computed } from "vue"
import { useIntlayer, useLocale } from "vue-intlayer"
import { percent } from "../format"

// The slider that sets how strong the news and social signals are, as a multiple of the published
// weights. The forecast is run at each of `scales`, so the slider steps between them.
const props = defineProps<{ scales: readonly number[] }>()
const scale = defineModel<number>({ required: true })
const content = useIntlayer("signal-weight")
const { locale } = useLocale()

const step = computed({
  get: () => Math.max(0, props.scales.indexOf(scale.value)),
  set: (i: number) => {
    scale.value = props.scales[i] ?? 1
  },
})
</script>

<template>
  <div class="signal-weight">
    <label class="row">
      <span class="label">{{ content.label({ weight: percent(scale, 0, locale) }) }}</span>
      <input
        v-model.number="step"
        min="0"
        type="range"
        :max="scales.length - 1"
        step="1"
        :aria-label="content.slider.value"
        :aria-valuetext="percent(scale, 0, locale)"
      />
    </label>
    <p class="small secondary">
      {{ content.help }}
      <button v-if="scale !== 1" class="reset" type="button" @click="scale = 1">
        {{ content.reset({ default: percent(1, 0, locale) }) }}
      </button>
    </p>
  </div>
</template>

<style scoped>
.signal-weight {
  display: grid;
  gap: 6px;
  margin: 4px 0 14px;
}

.row {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px 16px;
}

/* Both sliders' labels share one width, so their tracks line up. */
.label {
  font-weight: 600;
  flex: 0 0 min(100%, 23rem);
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
