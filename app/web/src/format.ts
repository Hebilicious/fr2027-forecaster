import type { Locale } from "./i18n"

/** A probability as a whole percentage, never claiming certainty it doesn't have. */
export function probability(p: number, locale: Locale): string {
  if (p <= 0) return locale === "fr" ? "0 %" : "0%"
  if (p < 0.005) return locale === "fr" ? "< 1 %" : "<1%"
  if (p >= 0.995) return locale === "fr" ? "> 99 %" : ">99%"
  return percent(p, 0, locale)
}

/** A share of the vote (0–1) as a percentage. */
export function percent(x: number, digits: number, locale: Locale): string {
  const value = (x * 100).toLocaleString(locale === "fr" ? "fr-FR" : "en-GB", {
    minimumFractionDigits: digits,
    maximumFractionDigits: digits,
  })
  return locale === "fr" ? `${value} %` : `${value}%`
}

const wholePoints = (x: number) => Math.round(x * 100).toString()

/** A share with its 80% interval, e.g. "24% (20–28)". */
export function shareWithInterval(mean: number, lo: number, hi: number, locale: Locale): string {
  return `${percent(mean, 0, locale)} (${wholePoints(lo)}–${wholePoints(hi)})`
}

/** A signed change in percentage points. */
export function points(delta: number, locale: Locale): string {
  const value = Math.abs(delta * 100).toLocaleString(locale === "fr" ? "fr-FR" : "en-GB", {
    minimumFractionDigits: 1,
    maximumFractionDigits: 1,
  })
  const sign = delta > 0 ? "+" : delta < 0 ? "−" : "±"
  return `${sign}${value} pt`
}

const DAY_MS = 86_400_000

/** Whole days from `today` to an ISO calendar date, both read as UTC dates. */
export function daysUntil(isoDate: string, today: Date): number {
  const target = Date.parse(`${isoDate}T00:00:00Z`)
  const start = Date.UTC(today.getUTCFullYear(), today.getUTCMonth(), today.getUTCDate())
  return Math.round((target - start) / DAY_MS)
}

export function longDate(isoDate: string, locale: Locale): string {
  return new Date(`${isoDate}T00:00:00Z`).toLocaleDateString(locale === "fr" ? "fr-FR" : "en-GB", {
    day: "numeric",
    month: "long",
    year: "numeric",
    timeZone: "UTC",
  })
}

export function shortDate(isoDate: string, locale: Locale): string {
  return new Date(`${isoDate}T00:00:00Z`).toLocaleDateString(locale === "fr" ? "fr-FR" : "en-GB", {
    day: "numeric",
    month: "short",
    timeZone: "UTC",
  })
}

export function dateTime(isoTimestamp: string, locale: Locale): string {
  return new Date(isoTimestamp).toLocaleString(locale === "fr" ? "fr-FR" : "en-GB", {
    day: "numeric",
    month: "short",
    hour: "2-digit",
    minute: "2-digit",
  })
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
