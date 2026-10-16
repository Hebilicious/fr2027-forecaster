import { getIntlayer } from "intlayer"
import type { Locale } from "./locales"

// Numbers and dates follow each locale's own conventions through `Intl`; the few words and signs around them
// ("<1%", "+3.4 pt") come from the `format` dictionary.

/** A probability as a whole percentage, never claiming certainty it doesn't have. */
export function probability(p: number, locale: Locale): string {
  if (p <= 0) return percent(0, 0, locale)
  const words = getIntlayer("format", locale)
  if (p < 0.005) return words.under({ value: percent(0.01, 0, locale) })
  if (p >= 0.995) return words.over({ value: percent(0.99, 0, locale) })
  return percent(p, 0, locale)
}

/** A share of the vote (0–1) as a percentage. */
export function percent(x: number, digits: number, locale: Locale): string {
  return new Intl.NumberFormat(locale, {
    style: "percent",
    minimumFractionDigits: digits,
    maximumFractionDigits: digits,
  }).format(x)
}

const wholePoints = (x: number) => Math.round(x * 100).toString()

/** A share with its 80% interval, e.g. "24% (20–28)". */
export function shareWithInterval(mean: number, lo: number, hi: number, locale: Locale): string {
  return `${percent(mean, 0, locale)} (${wholePoints(lo)}–${wholePoints(hi)})`
}

const signOf = (x: number) => (x > 0 ? "+" : x < 0 ? "−" : "±")

/** A signed change in percentage points. */
export function points(delta: number, locale: Locale): string {
  const value = Math.abs(delta * 100).toLocaleString(locale, { minimumFractionDigits: 1, maximumFractionDigits: 1 })
  return getIntlayer("format", locale).points({ value: `${signOf(delta)}${value}` })
}

/** A signed relative change as a whole percentage, e.g. "+12%" or "−8%". */
export function relativeChange(ratio: number, locale: Locale): string {
  const whole = Math.round(ratio * 100)
  return `${signOf(whole)}${percent(Math.abs(whole) / 100, 0, locale)}`
}

/** A whole count with the locale's digit grouping. */
export function count(n: number, locale: Locale): string {
  return Math.round(n).toLocaleString(locale)
}

/** Names or codes joined as the locale writes a list, e.g. "A, B and C". */
export function list(items: readonly string[], locale: Locale): string {
  return new Intl.ListFormat(locale, { type: "conjunction" }).format(items)
}

const DAY_MS = 86_400_000

/** Whole days from `today` to an ISO calendar date, both read as UTC dates. */
export function daysUntil(isoDate: string, today: Date): number {
  const target = Date.parse(`${isoDate}T00:00:00Z`)
  const start = Date.UTC(today.getUTCFullYear(), today.getUTCMonth(), today.getUTCDate())
  return Math.round((target - start) / DAY_MS)
}

export function longDate(isoDate: string, locale: Locale): string {
  return new Date(`${isoDate}T00:00:00Z`).toLocaleDateString(locale, {
    day: "numeric",
    month: "long",
    year: "numeric",
    timeZone: "UTC",
  })
}

export function shortDate(isoDate: string, locale: Locale): string {
  return new Date(`${isoDate}T00:00:00Z`).toLocaleDateString(locale, {
    day: "numeric",
    month: "short",
    timeZone: "UTC",
  })
}

export function dateTime(isoTimestamp: string, locale: Locale): string {
  return new Date(isoTimestamp).toLocaleString(locale, {
    day: "numeric",
    month: "short",
    hour: "2-digit",
    minute: "2-digit",
  })
}

/** A timestamp in local time, or a bare calendar date (YYYY-MM-DD) as that date. */
export function dateOrTime(value: string, locale: Locale): string {
  return /^\d{4}-\d{2}-\d{2}$/.test(value) ? shortDate(value, locale) : dateTime(value, locale)
}

/** `url` when it is an http(s) link, else `undefined`: data files are not trusted to carry safe links. */
export function safeUrl(url: string | null | undefined): string | undefined {
  if (!url) return undefined
  try {
    const parsed = new URL(url)
    return parsed.protocol === "https:" || parsed.protocol === "http:" ? parsed.href : undefined
  } catch {
    return undefined
  }
}
