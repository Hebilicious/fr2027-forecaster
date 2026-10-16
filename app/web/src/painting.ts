// Where Delacroix's painting sits behind each view's masthead: one image, a different detail per
// view, so moving between views pans across the canvas. Pure, so it can be tested headlessly.

export interface Size {
  readonly width: number
  readonly height: number
}

/** The image in `assets/liberte.avif`: the Louvre's canvas after its 2024 restoration. */
export const PAINTING: Size = { width: 3200, height: 2560 }

/** A point of the painting, as fractions of its width and height. */
export interface Point {
  readonly x: number
  readonly y: number
}

export interface Framing {
  /** The detail to show. */
  readonly focus: Point
  /** How far to zoom past the smallest scale that covers the masthead (1 just covers it). */
  readonly zoom: number
  /** Where the detail lands in a wide masthead, as fractions of its width and of its height below the top bar. */
  readonly wide: Point
  /**
   * Where it lands in a narrow masthead, where the text sits below the painting rather than beside
   * it: x as a fraction of the width, y as a fraction of the strip of painting above the text.
   */
  readonly narrow: Point
}

export type View = "overview" | "round1" | "round2" | "signals" | "sources"

export const FRAMINGS: Readonly<Record<View, Framing>> = {
  // Liberty raising the flag.
  overview: { focus: { x: 0.6, y: 0.14 }, zoom: 1.15, wide: { x: 0.7, y: 0.3 }, narrow: { x: 0.6, y: 0.32 } },
  // The man kneeling at her feet, looking up, dressed in the flag's blue, white and red.
  round1: { focus: { x: 0.44, y: 0.53 }, zoom: 2, wide: { x: 0.76, y: 0.55 }, narrow: { x: 0.62, y: 0.6 } },
  // The boy with the pistols, and Notre-Dame in the smoke.
  round2: { focus: { x: 0.79, y: 0.37 }, zoom: 2, wide: { x: 0.7, y: 0.42 }, narrow: { x: 0.45, y: 0.5 } },
  // The flag itself.
  signals: { focus: { x: 0.47, y: 0.09 }, zoom: 2, wide: { x: 0.72, y: 0.4 }, narrow: { x: 0.5, y: 0.5 } },
  // The signature, "Eug. Delacroix 1830".
  sources: { focus: { x: 0.885, y: 0.68 }, zoom: 2.6, wide: { x: 0.76, y: 0.52 }, narrow: { x: 0.5, y: 0.5 } },
}

export interface Placement {
  /** The painting's top left corner, from the masthead's, in CSS pixels. */
  readonly x: number
  readonly y: number
  /** CSS pixels per painting pixel. */
  readonly scale: number
}

/** An offset no further than `slack` (negative) nor past 0, so the painting covers the box. */
const cover = (offset: number, slack: number) => Math.min(0, Math.max(slack, offset))

/**
 * The scale and offset that put `focus` at `anchor` in the part of a `box` below its top `top`
 * pixels (the masthead runs under the sticky top bar), zoomed `zoom` times past covering that part,
 * then moved back as little as needed so the painting still covers it. Above, the painting may
 * show behind the bar or stop at its edge.
 */
export function place(box: Size, image: Size, focus: Point, zoom: number, anchor: Point, top = 0): Placement {
  const visible = { width: box.width, height: Math.max(1, box.height - top) }
  const scale = Math.max(visible.width / image.width, visible.height / image.height) * Math.max(1, zoom)
  const width = image.width * scale
  const height = image.height * scale
  return {
    x: cover(anchor.x * visible.width - focus.x * width, visible.width - width),
    y: top + cover(anchor.y * visible.height - focus.y * height, visible.height - height),
    scale,
  }
}

/** The widest the text column gets, as --content in style.css. */
export const CONTENT = 1240

/**
 * Where the obsidian slab's slanted edge meets the top and the bottom of a wide masthead, in
 * pixels from its left. The text sits on the slab, the painting shows beyond it. The slab covers
 * a share of the text column, a larger share on medium screens, where the text needs more room.
 */
export function slabEdge(width: number, tall: boolean): { readonly top: number; readonly bottom: number } {
  const inner = Math.min(width, CONTENT)
  const start = (width - inner) / 2
  // Between 900 and 1300 pixels the slab narrows from its medium to its wide proportions.
  const wide = Math.min(1, Math.max(0, (width - 900) / 400))
  const [top, bottom] = tall
    ? [lerp(0.62, 0.53, wide), lerp(0.52, 0.42, wide)]
    : [lerp(0.68, 0.6, wide), lerp(0.58, 0.5, wide)]
  return { top: start + inner * top, bottom: start + inner * bottom }
}

const lerp = (from: number, to: number, t: number) => from + (to - from) * t
