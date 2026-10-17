import { t, type Dictionary } from "intlayer"

export default {
  key: "signals-view",
  content: {
    missing: t({
      fr: "Les signaux n'ont pas pu être chargés.",
      "en-GB": "The signals did not load.",
      de: "Die Signale konnten nicht geladen werden.",
      it: "Non è stato possibile caricare i segnali.",
      "es-ES": "No se han podido cargar las señales.",
      "pt-PT": "Não foi possível carregar os sinais.",
      nl: "De signalen konden niet worden geladen.",
      pl: "Nie udało się wczytać sygnałów.",
      ru: "Не удалось загрузить сигналы.",
      uk: "Не вдалося завантажити сигнали.",
      ja: "シグナルを読み込めませんでした。",
      ko: "신호를 불러오지 못했습니다.",
      "zh-Hans": "信号未能加载。",
      th: "โหลดสัญญาณไม่สำเร็จ",
    }),
  },
} satisfies Dictionary
