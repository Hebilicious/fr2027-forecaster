import cn from "flag-icons/flags/4x3/cn.svg"
import de from "flag-icons/flags/4x3/de.svg"
import es from "flag-icons/flags/4x3/es.svg"
import fr from "flag-icons/flags/4x3/fr.svg"
import gb from "flag-icons/flags/4x3/gb.svg"
import it from "flag-icons/flags/4x3/it.svg"
import jp from "flag-icons/flags/4x3/jp.svg"
import kr from "flag-icons/flags/4x3/kr.svg"
import nl from "flag-icons/flags/4x3/nl.svg"
import pl from "flag-icons/flags/4x3/pl.svg"
import pt from "flag-icons/flags/4x3/pt.svg"
import ru from "flag-icons/flags/4x3/ru.svg"
import th from "flag-icons/flags/4x3/th.svg"
import ua from "flag-icons/flags/4x3/ua.svg"
import { getLocaleFromStorageClient, isDeclaredLocale, localeResolver, type DeclaredLocales } from "intlayer"

/** A locale the site is written in, as listed in intlayer.config.ts. */
export type Locale = DeclaredLocales

/**
 * Each language as the language menu shows it: its own name, which reads the same whatever the page's language,
 * and the flag of the country whose variant the site uses.
 */
export const LANGUAGES: Readonly<Record<Locale, { name: string; flag: string }>> = {
  fr: { name: "Français", flag: fr },
  "en-GB": { name: "English", flag: gb },
  de: { name: "Deutsch", flag: de },
  it: { name: "Italiano", flag: it },
  "es-ES": { name: "Español", flag: es },
  "pt-PT": { name: "Português", flag: pt },
  nl: { name: "Nederlands", flag: nl },
  pl: { name: "Polski", flag: pl },
  ru: { name: "Русский", flag: ru },
  uk: { name: "Українська", flag: ua },
  ja: { name: "日本語", flag: jp },
  ko: { name: "한국어", flag: kr },
  "zh-Hans": { name: "简体中文", flag: cn },
  th: { name: "ไทย", flag: th },
}

/**
 * The language to open the site in: the visitor's earlier choice, else the first of their browser's languages the
 * site has (matching "en-US" to "en-GB", "zh-CN" to "zh-Hans"), else French.
 */
export function initialLocale(): Locale {
  const stored = getLocaleFromStorageClient()
  if (isDeclaredLocale(stored)) return stored
  return localeResolver([...(globalThis.navigator?.languages ?? [])])
}
