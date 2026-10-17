import { describe, expect, it } from "vitest"
import {
  count,
  dateOrTime,
  daysUntil,
  percent,
  points,
  probability,
  relativeChange,
  relativeTime,
  safeUrl,
  shareWithInterval,
} from "./format"

describe("probability", () => {
  it("never rounds to certainty", () => {
    expect(probability(0.999, "en-GB")).toBe(">99%")
    expect(probability(0.001, "en-GB")).toBe("<1%")
    expect(probability(0, "en-GB")).toBe("0%")
    expect(probability(0.426, "en-GB")).toBe("43%")
  })

  it("follows each locale's typography", () => {
    // French and German set a no-break space before the percent sign.
    expect(probability(0.426, "fr")).toBe("43\u00a0%")
    expect(probability(0.001, "fr")).toBe("< 1\u00a0%")
    expect(probability(0.001, "ja")).toBe("1%未満")
    expect(probability(0.999, "de")).toBe("> 99\u00a0%")
  })
})

describe("shares", () => {
  it("formats a share with its interval", () => {
    expect(shareWithInterval(0.243, 0.201, 0.287, "en-GB")).toBe("24% (20–29)")
  })

  it("formats decimals per locale", () => {
    expect(percent(0.1234, 1, "en-GB")).toBe("12.3%")
    expect(percent(0.1234, 1, "fr")).toBe("12,3\u00a0%")
  })

  it("signs point changes", () => {
    expect(points(0.034, "en-GB")).toBe("+3.4 pt")
    expect(points(-0.01, "en-GB")).toBe("−1.0 pt")
    expect(points(0.034, "ja")).toBe("+3.4ポイント")
  })
})

describe("counts and changes", () => {
  it("signs a relative change as a whole percentage", () => {
    expect(relativeChange(0.124, "en-GB")).toBe("+12%")
    expect(relativeChange(-0.08, "fr")).toBe("−8\u00a0%")
    expect(relativeChange(0.001, "en-GB")).toBe("±0%")
  })

  it("groups digits per locale", () => {
    expect(count(12345, "en-GB")).toBe("12,345")
    expect(count(12345.4, "fr")).toBe("12 345")
  })
})

describe("dateOrTime", () => {
  it("shows a bare date as that date, whatever the time zone", () => {
    expect(dateOrTime("2026-10-06", "en-GB")).toBe("6 Oct")
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

describe("relativeTime", () => {
  const now = Date.parse("2026-10-17T12:00:00Z")
  it("counts back from the reader's clock", () => {
    expect(relativeTime("2026-10-17T11:59:30Z", now, "en-GB")).toBe("now")
    expect(relativeTime("2026-10-17T11:40:00Z", now, "en-GB")).toBe("20 minutes ago")
    expect(relativeTime("2026-10-17T09:00:00Z", now, "en-GB")).toBe("3 hours ago")
    expect(relativeTime("2026-10-16T09:00:00Z", now, "en-GB")).toBe("yesterday")
    expect(relativeTime("2026-10-17T09:00:00Z", now, "fr")).toBe("il y a 3 heures")
  })
})
