import { describe, expect, it } from "vitest"
import { CREDITS, initials, portrait } from "./portraits"

const files = Object.keys(import.meta.glob("./assets/portraits/*.webp")).map((path) =>
  path.replace("./assets/portraits/", "").replace(".webp", ""),
)

describe("portraits", () => {
  it("credits every portrait file, and has a file for every credit", () => {
    expect(files.toSorted()).toEqual(Object.keys(CREDITS).toSorted())
    for (const id of files) expect(portrait(id)?.src).toBeTruthy()
  })

  it("names an author, a licence and the Commons page for each", () => {
    for (const credit of Object.values(CREDITS)) {
      expect(credit.author).not.toBe("")
      expect(credit.license).not.toBe("")
      expect(credit.file).toMatch(/^https:\/\/commons\.wikimedia\.org\/wiki\/File:/)
    }
  })

  it("has no portrait for someone without one", () => {
    expect(portrait("nobody")).toBeNull()
  })

  it("takes initials from the first and last words", () => {
    expect(initials("Selma Labib")).toBe("SL")
    expect(initials("Dominique de Villepin")).toBe("DV")
    expect(initials("Madonna")).toBe("M")
  })
})
