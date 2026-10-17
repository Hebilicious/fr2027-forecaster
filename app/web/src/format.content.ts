import { insert, t, type Dictionary } from "intlayer"

// The words and signs around numbers, for format.ts.
export default {
  key: "format",
  content: {
    points: insert(
      t({
        fr: "{{value}} pt",
        "en-GB": "{{value}} pt",
        de: "{{value}} Pp.",
        it: "{{value}} p.p.",
        "es-ES": "{{value}} p. p.",
        "pt-PT": "{{value}} p.p.",
        ja: "{{value}}ポイント",
        ko: "{{value}}%p",
        "zh-Hans": "{{value}} 个百分点",
        pl: "{{value}} p.p.",
        ru: "{{value}} п. п.",
        th: "{{value}} จุด",
      }),
    ),
    under: insert(
      t({
        fr: "< {{value}}",
        "en-GB": "<{{value}}",
        de: "< {{value}}",
        it: "< {{value}}",
        "es-ES": "< {{value}}",
        "pt-PT": "< {{value}}",
        ja: "{{value}}未満",
        ko: "{{value}} 미만",
        "zh-Hans": "<{{value}}",
        pl: "< {{value}}",
        ru: "< {{value}}",
        th: "<{{value}}",
      }),
    ),
    over: insert(
      t({
        fr: "> {{value}}",
        "en-GB": ">{{value}}",
        de: "> {{value}}",
        it: "> {{value}}",
        "es-ES": "> {{value}}",
        "pt-PT": "> {{value}}",
        ja: "{{value}}超",
        ko: "{{value}} 초과",
        "zh-Hans": ">{{value}}",
        pl: "> {{value}}",
        ru: "> {{value}}",
        th: ">{{value}}",
      }),
    ),
  },
} satisfies Dictionary
