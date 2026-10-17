import { insert, t, type Dictionary } from "intlayer"

export default {
  key: "locale-switcher",
  content: {
    current: insert(
      t({
        fr: "Langue : {{name}}",
        "en-GB": "Language: {{name}}",
        de: "Sprache: {{name}}",
        it: "Lingua: {{name}}",
        "es-ES": "Idioma: {{name}}",
        "pt-PT": "Idioma: {{name}}",
        nl: "Taal: {{name}}",
        pl: "Język: {{name}}",
        ru: "Язык: {{name}}",
        uk: "Мова: {{name}}",
        ja: "言語：{{name}}",
        ko: "언어: {{name}}",
        "zh-Hans": "语言：{{name}}",
        th: "ภาษา: {{name}}",
      }),
    ),
    menu: t({
      fr: "Langues du site",
      "en-GB": "Site languages",
      de: "Sprachen der Website",
      it: "Lingue del sito",
      "es-ES": "Idiomas del sitio",
      "pt-PT": "Idiomas do site",
      nl: "Talen van de site",
      pl: "Języki serwisu",
      ru: "Языки сайта",
      uk: "Мови сайту",
      ja: "サイトの言語",
      ko: "사이트 언어",
      "zh-Hans": "网站语言",
      th: "ภาษาของเว็บไซต์",
    }),
  },
} satisfies Dictionary
