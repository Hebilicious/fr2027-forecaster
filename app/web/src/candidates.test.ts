import { describe, expect, it } from "vitest"
import type { CandidateForecast } from "./api/schemas"
import { ARTICLES } from "./articles"
import { articleFor, byFamily } from "./candidates"

const candidate = (id: string, bloc: string, pRun: number): CandidateForecast => ({
  candidate_id: id,
  name: id.toUpperCase(),
  party: "P",
  bloc,
  status: "declared",
  p_run: pRun,
  simulated: true,
  p_qualify_r1: 0,
  p_win: 0,
  r1_share: null,
})

describe("byFamily", () => {
  it("orders families left to right, unknown ones last, and the likeliest to run first", () => {
    const groups = byFamily([
      candidate("e", "far_right", 0.9),
      candidate("b", "left", 0.2),
      candidate("x", "regional", 1),
      candidate("a", "left", 0.9),
      candidate("c", "left", 0.9),
      candidate("o", "other", 0.5),
    ])
    expect(groups.map((g) => g.family)).toEqual(["left", "far_right", "other", "regional"])
    expect(groups[0]?.candidates.map((c) => c.candidate_id)).toEqual(["a", "c", "b"])
  })

  it("lists LFI and PCF with the far left", () => {
    const groups = byFamily([
      candidate("melenchon", "left", 1),
      candidate("roussel", "left", 1),
      candidate("faure", "left", 1),
    ])
    expect(groups.map((g) => [g.family, g.candidates.map((c) => c.candidate_id)])).toEqual([
      ["far_left", ["MELENCHON", "ROUSSEL"].map((n) => n.toLowerCase())],
      ["left", ["faure"]],
    ])
  })
})

describe("articleFor", () => {
  it("links the reader's own Wikipedia when it has the article", () => {
    expect(articleFor("le_pen", "de")).toEqual({ url: "https://de.wikipedia.org/wiki/Marine_Le_Pen", lang: "de" })
  })

  it("falls back to the French article, saying so", () => {
    const thai = Object.entries(ARTICLES).find(([, links]) => !links.th)
    expect(thai).toBeDefined()
    expect(articleFor(thai![0], "th")).toEqual({ url: thai![1].fr, lang: "fr" })
  })

  it("has no link for someone French Wikipedia lacks", () => {
    expect(articleFor("nobody", "fr")).toBeNull()
  })

  it("links each language to its own Wikipedia", () => {
    const wikis = {
      fr: "fr",
      "en-GB": "en",
      de: "de",
      it: "it",
      "es-ES": "es",
      "pt-PT": "pt",
      ja: "ja",
      ko: "ko",
      th: "th",
    }
    for (const links of Object.values(ARTICLES)) {
      for (const [locale, wiki] of Object.entries(wikis)) {
        const url = links[locale as keyof typeof links]
        if (url) expect(url).toMatch(new RegExp(`^https://${wiki}\\.wikipedia\\.org/wiki/`))
      }
      if (links["zh-Hans"]) expect(links["zh-Hans"]).toMatch(/^https:\/\/zh\.wikipedia\.org\/zh-hans\//)
    }
  })
})
