import { insert, t, type Dictionary } from "intlayer"

export default {
  key: "share-panel",
  content: {
    label: insert(
      t({
        fr: "{{name}} : {{latest}}",
        "en-GB": "{{name}}: {{latest}}",
        de: "{{name}}: {{latest}}",
        it: "{{name}}: {{latest}}",
        "es-ES": "{{name}}: {{latest}}",
        "pt-PT": "{{name}}: {{latest}}",
        ja: "{{name}}：{{latest}}",
        ko: "{{name}}: {{latest}}",
        "zh-Hans": "{{name}}：{{latest}}",
        th: "{{name}}: {{latest}}",
      }),
    ),
    poll: insert(
      t({
        fr: "{{firm}}, {{date}} : {{share}} (hypothèse {{scenario}})",
        "en-GB": "{{firm}}, {{date}}: {{share}} (scenario {{scenario}})",
        de: "{{firm}}, {{date}}: {{share}} (Szenario {{scenario}})",
        it: "{{firm}}, {{date}}: {{share}} (scenario {{scenario}})",
        "es-ES": "{{firm}}, {{date}}: {{share}} (escenario {{scenario}})",
        "pt-PT": "{{firm}}, {{date}}: {{share}} (cenário {{scenario}})",
        ja: "{{firm}}、{{date}}：{{share}}（シナリオ{{scenario}}）",
        ko: "{{firm}}, {{date}}: {{share}} (시나리오 {{scenario}})",
        "zh-Hans": "{{firm}}，{{date}}：{{share}}（情境 {{scenario}}）",
        th: "{{firm}}, {{date}}: {{share}} (ฉากทัศน์ {{scenario}})",
      }),
    ),
  },
} satisfies Dictionary
