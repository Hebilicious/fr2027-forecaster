<script setup lang="ts">
import { useIntlayer } from "vue-intlayer"
import { RANGES } from "../../signals"
import { useSignalRange } from "../../useSignalRange"

// The period of the X and press sections: one choice, shown in each section's header.
defineProps<{ disabled?: boolean }>()
const content = useIntlayer("range-control")
const range = useSignalRange()
</script>

<template>
  <div class="segmented" role="group" :aria-label="content.label.value">
    <button
      v-for="r in RANGES"
      :key="r"
      type="button"
      :aria-pressed="!disabled && range === r"
      :disabled="disabled"
      @click="range = r"
    >
      {{ content.ranges[r] }}
    </button>
  </div>
</template>
