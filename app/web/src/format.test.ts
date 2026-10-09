import { describe, expect, it } from "vitest"
import {
  count,
  dateOrTime,
  daysUntil,
  percent,
  points,
  probability,
  relativeChange,
  safeUrl,
  shareWithInterval,
} from "./format"

describe("probability", () => {
  it("never rounds to certainty", () => {
    expect(probability(0.999, "en")).toBe(">99%")
    expect(probability(0.001, "en")).toBe("<1%")
    expect(probability(0, "en")).toBe("0%")
    expect(probability(0.426, "en")).toBe("43%")
  })

  it("follows French typography", () => {
    expect(probability(0.426, "fr")).toBe("43 %")
    expect(probability(0.001, "fr")).toBe("< 1 %")
  })
})

describe("shares", () => {
  it("formats a share with its interval", () => {
    expect(shareWithInterval(0.243, 0.201, 0.287, "en")).toBe("24% (20–29)")
  })

  it("formats decimals per locale", () => {
    expect(percent(0.1234, 1, "en")).toBe("12.3%")
    expect(percent(0.1234, 1, "fr")).toBe("12,3 %")
  })

  it("signs point changes", () => {
    expect(points(0.034, "en")).toBe("+3.4 pt")
    expect(points(-0.01, "en")).toBe("−1.0 pt")
  })
})

describe("counts and changes", () => {
  it("signs a relative change as a whole percentage", () => {
    expect(relativeChange(0.124, "en")).toBe("+12%")
    expect(relativeChange(-0.08, "fr")).toBe("−8 %")
    expect(relativeChange(0.001, "en")).toBe("±0%")
  })

  it("groups digits per locale", () => {
    expect(count(12345, "en")).toBe("12,345")
    expect(count(12345.4, "fr")).toBe("12 345")
  })
})

describe("dateOrTime", () => {
  it("shows a bare date as that date, whatever the time zone", () => {
    expect(dateOrTime("2026-10-06", "en")).toBe("6 Oct")
  })
})

describe("daysUntil", () => {
  it("counts calendar days in UTC", () => {
    expect(daysUntil("2027-04-18", new Date("2026-10-09T23:30:00Z"))).toBe(191)
    expect(daysUntil("2027-04-18", new Date("2027-04-18T08:00:00Z"))).toBe(0)
  })
})

describe("safeUrl", () => {
  it("keeps http(s) links and drops everything else", () => {
    expect(safeUrl("https://example.org/a?b=1")).toBe("https://example.org/a?b=1")
    expect(safeUrl("javascript:alert(1)")).toBeUndefined()
    expect(safeUrl("not a url")).toBeUndefined()
    expect(safeUrl(null)).toBeUndefined()
  })
})
