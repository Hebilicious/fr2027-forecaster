import { describe, expect, it } from "vitest"
import { CONTENT, FRAMINGS, PAINTING, place, slabEdge } from "./painting"

const image = { width: 1000, height: 800 }

describe("place", () => {
  it("puts the focus point on the anchor when the painting can move that far", () => {
    const box = { width: 1000, height: 400 }
    const at = place(box, image, { x: 0.5, y: 0.5 }, 2, { x: 0.5, y: 0.5 })
    expect(at.scale).toBe(2)
    expect(500 * at.scale + at.x).toBeCloseTo(500)
    expect(400 * at.scale + at.y).toBeCloseTo(200)
  })

  it("never uncovers an edge of the masthead", () => {
    const box = { width: 1000, height: 400 }
    const left = place(box, image, { x: 0, y: 0 }, 1, { x: 0.9, y: 0.9 })
    expect(left.x).toBe(0)
    expect(left.y).toBe(0)
    const right = place(box, image, { x: 1, y: 1 }, 1, { x: 0.1, y: 0.1 })
    expect(right.x).toBeCloseTo(box.width - image.width * right.scale)
    expect(right.y).toBeCloseTo(box.height - image.height * right.scale)
  })

  it("frames the part below the top bar, letting the painting stop at the bar's edge", () => {
    const box = { width: 1000, height: 464 }
    const at = place(box, image, { x: 0.5, y: 0 }, 1, { x: 0.5, y: 0.5 }, 64)
    expect(at.y).toBe(64)
    expect(800 * at.scale + at.y).toBeGreaterThanOrEqual(box.height)
    const lower = place(box, image, { x: 0.5, y: 0.9 }, 2, { x: 0.5, y: 0.5 }, 64)
    expect(lower.y).toBeLessThan(0)
  })

  it("covers a box of either shape before zooming, and never zooms out", () => {
    expect(place({ width: 500, height: 1000 }, image, { x: 0.5, y: 0.5 }, 1, { x: 0.5, y: 0.5 }).scale).toBe(1.25)
    expect(place({ width: 2000, height: 400 }, image, { x: 0.5, y: 0.5 }, 0.5, { x: 0.5, y: 0.5 }).scale).toBe(2)
  })
})

describe("FRAMINGS", () => {
  it("keeps every focus point and anchor inside the painting and the masthead", () => {
    for (const framing of Object.values(FRAMINGS)) {
      for (const point of [framing.focus, framing.wide, framing.narrow]) {
        expect(point.x).toBeGreaterThanOrEqual(0)
        expect(point.x).toBeLessThanOrEqual(1)
        expect(point.y).toBeGreaterThanOrEqual(0)
        expect(point.y).toBeLessThanOrEqual(1)
      }
      expect(framing.zoom).toBeGreaterThanOrEqual(1)
    }
    expect(PAINTING.width / PAINTING.height).toBeCloseTo(1.25)
  })
})

describe("slabEdge", () => {
  it("slants left to right, top to bottom, and leaves the text column room", () => {
    for (const width of [770, 900, 1100, 1300, 1440, 1920]) {
      for (const tall of [true, false]) {
        const edge = slabEdge(width, tall)
        const start = (width - Math.min(width, CONTENT)) / 2
        expect(edge.top).toBeGreaterThan(edge.bottom)
        expect(edge.bottom - start).toBeGreaterThanOrEqual(0.42 * Math.min(width, CONTENT))
        expect(edge.top).toBeLessThan(width)
      }
    }
  })

  it("gives medium screens a larger share than wide ones", () => {
    expect(slabEdge(800, true).bottom / 800).toBeCloseTo(0.52)
    expect(slabEdge(1240, true).bottom / 1240).toBeCloseTo(0.435)
    expect(slabEdge(1400, true).bottom).toBeCloseTo(80 + 1240 * 0.42)
  })
})
