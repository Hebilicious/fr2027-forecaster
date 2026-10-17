import { Locales, type IntlayerConfig } from "intlayer"

// French is the reference language: every dictionary is written in French first, and French is what a
// visitor sees when nothing else matches. `strict` makes `t()` a type error unless it carries every locale.
const config: IntlayerConfig = {
  internationalization: {
    locales: [
      Locales.FRENCH,
      Locales.ENGLISH_UNITED_KINGDOM,
      Locales.GERMAN,
      Locales.ITALIAN,
      Locales.SPANISH_SPAIN,
      Locales.PORTUGUESE_PORTUGAL,
      Locales.DUTCH,
      Locales.POLISH,
      Locales.RUSSIAN,
      Locales.UKRAINIAN,
      Locales.JAPANESE,
      Locales.KOREAN,
      Locales.CHINESE_SIMPLIFIED,
      Locales.THAI,
    ],
    defaultLocale: Locales.FRENCH,
    strictMode: "strict",
  },
  // The views live in the URL hash and the site is static, so the locale stays out of the path and is
  // remembered in localStorage only.
  routing: {
    mode: "no-prefix",
    storage: ["localStorage"],
    enableProxy: false,
  },
  content: {
    contentDir: ["src"],
  },
  editor: {
    enabled: false,
  },
}

export default config
