<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from "vue"
import { useIntlayer, useLocale } from "vue-intlayer"
import { LANGUAGES, type Locale } from "../locales"

const content = useIntlayer("locale-switcher")
const { locale, availableLocales, setLocale } = useLocale()

// A disclosure: the button opens a list of the languages, each under its own name and flag.
const open = ref(false)
const root = ref<HTMLElement | null>(null)
const button = ref<HTMLButtonElement | null>(null)

function choose(next: Locale) {
  setLocale(next)
  open.value = false
  button.value?.focus()
}
function onKeydown(event: KeyboardEvent) {
  if (event.key !== "Escape" || !open.value) return
  open.value = false
  button.value?.focus()
}
function onPointerDown(event: PointerEvent) {
  if (open.value && !root.value?.contains(event.target as Node)) open.value = false
}
onMounted(() => {
  document.addEventListener("keydown", onKeydown)
  document.addEventListener("pointerdown", onPointerDown)
})
onBeforeUnmount(() => {
  document.removeEventListener("keydown", onKeydown)
  document.removeEventListener("pointerdown", onPointerDown)
})
</script>

<template>
  <div ref="root" class="switcher">
    <button
      ref="button"
      class="chip lang"
      type="button"
      :aria-expanded="open"
      :aria-controls="open ? 'languages' : undefined"
      :aria-label="content.current({ name: LANGUAGES[locale].name }).value"
      @click="open = !open"
    >
      <img alt="" class="flag" :src="LANGUAGES[locale].flag" />
      <span :lang="locale" class="name">{{ LANGUAGES[locale].name }}</span>
      <span aria-hidden="true" class="caret" />
    </button>
    <ul v-if="open" id="languages" class="menu" :aria-label="content.menu.value">
      <li v-for="option in availableLocales" :key="option">
        <button type="button" :lang="option" :aria-current="option === locale" @click="choose(option)">
          <img alt="" class="flag" :src="LANGUAGES[option].flag" />
          <span>{{ LANGUAGES[option].name }}</span>
        </button>
      </li>
    </ul>
  </div>
</template>

<style scoped>
.switcher {
  position: relative;
  flex: none;
}

.lang {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 6px 12px;
  font-size: 0.8rem;
}

.flag {
  width: 18px;
  height: 13.5px;
  border-radius: 2px;
  object-fit: cover;
  box-shadow: 0 0 0 1px rgba(255, 255, 255, 0.14);
}

.caret {
  width: 6px;
  height: 6px;
  margin: -3px 1px 0;
  border-right: 1.5px solid currentColor;
  border-bottom: 1.5px solid currentColor;
  transform: rotate(45deg);
  opacity: 0.7;
  transition: transform 0.2s;
}

.lang[aria-expanded="true"] .caret {
  margin-top: 3px;
  transform: rotate(225deg);
}

.menu {
  position: absolute;
  right: 0;
  top: calc(100% + 8px);
  z-index: 30;
  min-width: 12rem;
  /* The list is long enough to pass the bottom of a short window: scroll it rather than clip it. */
  max-height: calc(100vh - 5rem);
  overflow-y: auto;
  overscroll-behavior: contain;
  margin: 0;
  padding: 6px;
  list-style: none;
  border-radius: 14px;
  background: var(--glass);
  -webkit-backdrop-filter: blur(18px) saturate(1.3);
  backdrop-filter: blur(18px) saturate(1.3);
  box-shadow: inset 0 0 0 1px var(--border),
    0 18px 40px -18px rgba(0, 0, 0, 0.9);
  animation: rise 0.25s var(--ease-out) both;
}

.menu button {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 10px;
  border: none;
  border-radius: 9px;
  background: none;
  color: var(--ink-2);
  font: inherit;
  font-size: 0.88rem;
  text-align: left;
  cursor: pointer;
}

.menu button:hover {
  background: rgba(255, 255, 255, 0.07);
  color: var(--ink);
}

.menu button[aria-current="true"] {
  color: var(--ink);
  background: rgba(255, 255, 255, 0.1);
  box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.09);
}

/* On a phone the bar is tight: the flag says enough. */
@media (max-width: 480px) {
  .lang .name {
    display: none;
  }
}
</style>
