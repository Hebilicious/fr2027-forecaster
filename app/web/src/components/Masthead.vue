<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, shallowRef } from "vue"
import liberteAvif from "../assets/liberte.avif"
import liberteWebp from "../assets/liberte.webp"
import { CONTENT as CONTENT_WIDTH, PAINTING, place, slabEdge, type Framing, type Size } from "../painting"

const props = defineProps<{
  framing: Framing
  /** The overview's hero: taller, with more of the painting. */
  tall: boolean
}>()

// The same query as the stylesheets' narrow layout, so the script and the CSS switch together.
const NARROW_QUERY = "(max-width: 760px)"
// In a narrow masthead the painting shows above the text: this many pixels of it.
const REGION = { tall: 300, short: 196 }

const root = ref<HTMLElement | null>(null)
const box = shallowRef<Size>({ width: 0, height: 0 })
// The masthead runs under the sticky top bar; its height comes from --topbar-h.
const topbar = ref(64)
// The page gutter, clamp(16px, 4vw, 40px) in style.css, which the text column starts after.
const gutter = ref(40)
const narrow = ref(false)
const loaded = ref(false)
// The first placement is not animated: the painting fades in where it belongs, then pans.
const settled = ref(false)
let observer: ResizeObserver | undefined
let query: MediaQueryList | undefined
const onQuery = (event: MediaQueryListEvent) => (narrow.value = event.matches)

onMounted(() => {
  const el = root.value
  if (!el) return
  const measure = () => {
    box.value = { width: el.clientWidth, height: el.clientHeight }
    topbar.value = Number.parseFloat(getComputedStyle(el).getPropertyValue("--topbar-h")) || 64
    gutter.value = Math.min(40, Math.max(16, window.innerWidth * 0.04))
  }
  query = window.matchMedia(NARROW_QUERY)
  narrow.value = query.matches
  query.addEventListener("change", onQuery)
  measure()
  observer = new ResizeObserver(measure)
  observer.observe(el)
  requestAnimationFrame(() => requestAnimationFrame(() => (settled.value = true)))
})
onBeforeUnmount(() => {
  observer?.disconnect()
  query?.removeEventListener("change", onQuery)
})

const region = computed(() => (props.tall ? REGION.tall : REGION.short))

const placement = computed(() => {
  const { width, height } = box.value
  if (width === 0 || height === 0) return null
  const below = Math.max(1, height - topbar.value)
  const anchor = narrow.value
    ? { x: props.framing.narrow.x, y: Math.min(1, (props.framing.narrow.y * region.value) / below) }
    : props.framing.wide
  return place(box.value, PAINTING, props.framing.focus, props.framing.zoom, anchor, topbar.value)
})
const canvasStyle = computed(() => {
  const at = placement.value
  return at
    ? { transform: `translate3d(${at.x.toFixed(1)}px, ${at.y.toFixed(1)}px, 0) scale(${at.scale.toFixed(5)})` }
    : {}
})

// The obsidian slab the text sits on: the left of a wide masthead, the bottom of a narrow one.
// Its polished edge is drawn from the same numbers, so the highlight runs exactly along it.
const slab = computed(() => {
  const { width: w, height: h } = box.value
  if (w === 0 || h === 0) return { x1: 0, y1: 0, x2: 0, y2: 0, clip: "none", textMax: null }
  if (narrow.value) {
    const left = Math.min(h, topbar.value + region.value + 18)
    const right = Math.max(0, topbar.value + region.value - 26)
    return {
      x1: 0,
      y1: left,
      x2: w,
      y2: right,
      clip: `polygon(0 ${left}px, ${w}px ${right}px, ${w}px ${h}px, 0 ${h}px)`,
      textMax: null,
    }
  }
  const { top, bottom } = slabEdge(w, props.tall)
  // The text column ends a little before the slab's edge at its narrowest, the bottom.
  const textStart = (w - Math.min(w, CONTENT_WIDTH)) / 2 + gutter.value
  return {
    x1: top,
    y1: 0,
    x2: bottom,
    y2: h,
    clip: `polygon(0 0, ${top}px 0, ${bottom}px ${h}px, 0 ${h}px)`,
    textMax: Math.max(240, bottom - textStart - 28),
  }
})
const mastheadStyle = computed(() => ({
  "--region": `${region.value}px`,
  ...(slab.value.textMax === null ? {} : { "--text-max": `${Math.round(slab.value.textMax)}px` }),
}))
</script>

<template>
  <section ref="root" class="masthead" :class="{ tall, narrow }" :style="mastheadStyle">
    <div aria-hidden="true" class="painting">
      <picture>
        <source :srcset="liberteAvif" type="image/avif" />
        <img
          :src="liberteWebp"
          alt=""
          class="canvas"
          :class="{ loaded: loaded && placement, settled }"
          :style="canvasStyle"
          decoding="async"
          :width="PAINTING.width"
          :height="PAINTING.height"
          @load="loaded = true"
        />
      </picture>
      <div class="glass" />
      <div class="slab" :style="{ clipPath: slab.clip }" />
      <svg class="edge" :viewBox="`0 0 ${box.width || 1} ${box.height || 1}`" preserveAspectRatio="none">
        <defs>
          <linearGradient
            id="slab-edge"
            gradientUnits="userSpaceOnUse"
            :x1="slab.x1"
            :y1="slab.y1"
            :x2="slab.x2"
            :y2="slab.y2"
          >
            <stop offset="0" stop-color="#ffffff" stop-opacity="0.04" />
            <stop offset="0.3" stop-color="#ffffff" stop-opacity="0.6" />
            <stop offset="0.62" stop-color="#ffffff" stop-opacity="0.14" />
            <stop offset="1" stop-color="#ffffff" stop-opacity="0" />
          </linearGradient>
        </defs>
        <line :x1="slab.x1" :y1="slab.y1" :x2="slab.x2" :y2="slab.y2" class="edge-shadow" />
        <line :x1="slab.x1" :y1="slab.y1" :x2="slab.x2" :y2="slab.y2" class="edge-light" />
      </svg>
      <div class="veil" />
    </div>
    <div class="masthead-inner">
      <slot />
    </div>
  </section>
</template>

<style scoped>
.masthead {
  position: relative;
  margin-top: calc(-1 * var(--topbar-h));
  min-height: clamp(420px, 52vh, 480px);
  overflow: hidden;
  isolation: isolate;
  display: flex;
  align-items: center;
}

.masthead.tall {
  min-height: clamp(640px, 94vh, 860px);
}

.masthead.narrow {
  min-height: 0;
  align-items: flex-end;
}

.painting {
  position: absolute;
  inset: 0;
  z-index: -1;
  background: var(--page);
}

.canvas {
  position: absolute;
  left: 0;
  top: 0;
  width: 3200px;
  height: 2560px;
  max-width: none;
  transform-origin: 0 0;
  opacity: 0;
  filter: saturate(1.05) contrast(1.05) brightness(0.9);
  transition: opacity 1.6s ease;
}

.canvas.loaded {
  opacity: 1;
}

.canvas.settled {
  transition:
    opacity 1.6s ease,
    transform 1.9s var(--ease-out);
}

/* The painting sits behind glass: a faint diagonal reflection, and its far edge sinks into the dark. */
.glass {
  position: absolute;
  inset: 0;
  background: linear-gradient(108deg, rgba(255, 255, 255, 0) 44%, rgba(255, 255, 255, 0.07) 52%, rgba(255, 255, 255, 0) 61%),
    linear-gradient(270deg, rgba(4, 5, 8, 0.55) 0, rgba(4, 5, 8, 0) 24%);
}

.narrow .glass {
  background: linear-gradient(118deg, rgba(255, 255, 255, 0) 30%, rgba(255, 255, 255, 0.07) 46%, rgba(255, 255, 255, 0) 60%);
}

/* Obsidian: the painting behind it is darkened and blurred, as in a polished black stone. */
.slab {
  position: absolute;
  inset: 0;
  background:
    linear-gradient(118deg, rgba(255, 255, 255, 0.06), rgba(255, 255, 255, 0) 34%),
    linear-gradient(90deg, rgba(4, 5, 8, 0.55), rgba(4, 5, 8, 0.3)),
    rgba(6, 7, 10, 0.28);
  -webkit-backdrop-filter: blur(16px) brightness(0.42) saturate(1.2);
  backdrop-filter: blur(16px) brightness(0.42) saturate(1.2);
}

.narrow .slab {
  background:
    linear-gradient(185deg, rgba(255, 255, 255, 0.06), rgba(255, 255, 255, 0) 30%),
    rgba(6, 7, 10, 0.45);
}

.edge {
  position: absolute;
  inset: 0;
  width: 100%;
  height: 100%;
  overflow: visible;
}

.edge-light {
  stroke: url(#slab-edge);
  stroke-width: 1.25;
  vector-effect: non-scaling-stroke;
}

.edge-shadow {
  stroke: rgba(0, 0, 0, 0.55);
  stroke-width: 6;
  vector-effect: non-scaling-stroke;
  filter: blur(4px);
}

/* Shade under the top bar, and a long fade into the page so the cards below can overlap. */
.veil {
  position: absolute;
  inset: 0;
  background: linear-gradient(180deg, rgba(7, 9, 13, 0.75) 0, rgba(7, 9, 13, 0) calc(var(--topbar-h) + 72px)),
    linear-gradient(0deg, var(--page) 0, rgba(7, 9, 13, 0.75) 14%, rgba(7, 9, 13, 0) 44%);
}

.masthead-inner {
  width: 100%;
  max-width: var(--content);
  margin: 0 auto;
  padding: calc(var(--topbar-h) + 28px) var(--gutter) 88px;
}

.tall .masthead-inner {
  padding-bottom: 150px;
}

.narrow .masthead-inner {
  padding-top: calc(var(--topbar-h) + var(--region) + 40px);
  padding-bottom: 56px;
}

.narrow.tall .masthead-inner {
  padding-bottom: 96px;
}
</style>
