<script setup lang="ts">
import { computed } from "vue"
import { initials, portrait } from "../portraits"

const props = withDefaults(
  defineProps<{
    id: string
    name: string
    /** Diameter in CSS pixels. */
    size?: number
  }>(),
  { size: 28 },
)
const photo = computed(() => portrait(props.id))
</script>

<template>
  <!-- Decorative: the name always sits beside it. -->
  <span aria-hidden="true" class="avatar" :style="{ '--size': `${size}px` }">
    <img v-if="photo" :src="photo.src" alt="" decoding="async" loading="lazy" :width="size" :height="size" />
    <span v-else class="initials">{{ initials(name) }}</span>
  </span>
</template>

<style scoped>
.avatar {
  position: relative;
  flex: none;
  display: inline-grid;
  place-items: center;
  width: var(--size);
  height: var(--size);
  border-radius: 50%;
  overflow: hidden;
  background: linear-gradient(160deg, #2a2d35, #14161b);
  box-shadow: 0 0 0 1px rgba(255, 255, 255, 0.16),
    0 3px 10px rgba(0, 0, 0, 0.45);
}

/* Black and white for everyone: one look for all, and no party colours from the backdrops. */
img {
  display: block;
  width: 100%;
  height: 100%;
  object-fit: cover;
  filter: grayscale(1) contrast(1.06) brightness(0.94);
}

/* The same vignette on every portrait evens out white and dark backdrops. */
.avatar::after {
  content: "";
  position: absolute;
  inset: 0;
  border-radius: inherit;
  background: radial-gradient(circle at 50% 42%, transparent 55%, rgba(7, 9, 13, 0.5) 100%);
  pointer-events: none;
}

.initials {
  font-family: var(--font-display);
  font-style: italic;
  font-size: calc(var(--size) * 0.4);
  line-height: 1;
  color: var(--ink-2);
}
</style>
