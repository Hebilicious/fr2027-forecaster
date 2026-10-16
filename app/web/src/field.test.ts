import { describe, expect, it } from "vitest"
import type { CandidateForecast } from "./api/schemas"
import { filterField, matches, standing } from "./field"

const candidate = (
  id: string,
  slot: string | undefined,
  pRun: number,
  status = "declared",
  name = id,
  party = "P",
): CandidateForecast => ({
  candidate_id: id,
  name,
  party,
  bloc: "left",
  status,
  slot,
  p_run: pRun,
  simulated: true,
  p_qualify_r1: 0,
  p_win: 0,
  r1_share: null,
})

// The field as config/candidates.yaml had it on 16 October 2026, in part.
const lePen = candidate("le_pen", "rn", 0.91, "declared", "Marine Le Pen", "Rassemblement national (RN)")
const bardella = candidate("bardella", "rn", 0.07, "possible", "Jordan Bardella", "Rassemblement national (RN)")
const glucksmann = candidate("glucksmann", "ps_primary", 0.95)
const faure = candidate("faure", "ps_primary", 0)
const philippe = candidate("philippe", "centre", 0.85)
const attal = candidate("attal", "centre", 0.2)
const hollande = candidate("hollande", "hollande", 0.2, "possible")
const melenchon = candidate("melenchon", "melenchon", 0.95, "declared", "Jean-Luc Mélenchon", "La France insoumise")
const field = [lePen, bardella, glucksmann, faure, philippe, attal, hollande, melenchon]

describe("standing", () => {
  it("sets aside an undeclared candidate who could only replace someone likelier to run", () => {
    expect(standing(bardella, field)).toBe("standIn")
    expect(standing(lePen, field)).toBe("running")
  })

  it("counts a primary's losers out", () => {
    expect(standing(faure, field)).toBe("out")
    expect(standing(glucksmann, field)).toBe("running")
  })

  it("keeps a declared candidate who shares a slot, and an undeclared one alone in theirs", () => {
    expect(standing(attal, field)).toBe("running")
    expect(standing(hollande, field)).toBe("running")
  })

  it("keeps everyone with a chance when the forecast has no slots", () => {
    const old = [
      { ...lePen, slot: undefined },
      { ...bardella, slot: undefined },
    ]
    expect(old.map((c) => standing(c, old))).toEqual(["running", "running"])
  })
})

describe("matches", () => {
  it("finds a name or party without case or accents", () => {
    expect(matches(melenchon, "melenchon")).toBe(true)
    expect(matches(melenchon, "INSOUMISE")).toBe(true)
    expect(matches(lePen, "  ")).toBe(true)
    expect(matches(lePen, "marine rn")).toBe(true)
    expect(matches(lePen, "marine lfi")).toBe(false)
  })
})

describe("filterField", () => {
  it("hides stand-ins and those out until asked, whatever the search", () => {
    const view = filterField(field, { query: "", showAll: false })
    expect([...view.setAside]).toEqual(["bardella", "faure"])
    expect(view.visible.map((c) => c.candidate_id)).not.toContain("bardella")
    expect(view.hidden).toEqual(new Set(["bardella", "faure"]))
  })

  it("shows everyone the search finds once the toggle is on", () => {
    const view = filterField(field, { query: "rassemblement", showAll: true })
    expect(view.visible.map((c) => c.candidate_id)).toEqual(["le_pen", "bardella"])
    expect(view.setAside.size).toBe(0)
    expect(view.notRunning).toBe(2)
  })
})
