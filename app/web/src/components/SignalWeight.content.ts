import { insert, t, type Dictionary } from "intlayer"

export default {
  key: "signal-weight",
  content: {
    label: insert(
      t({
        fr: "Poids de la presse et des réseaux : {{weight}}",
        "en-GB": "Weight of news and social signals: {{weight}}",
        de: "Gewicht von Presse und sozialen Netzwerken: {{weight}}",
        it: "Peso di stampa e social: {{weight}}",
        "es-ES": "Peso de la prensa y las redes sociales: {{weight}}",
        "pt-PT": "Peso da imprensa e das redes sociais: {{weight}}",
        nl: "Gewicht van pers en sociale media: {{weight}}",
        pl: "Waga prasy i mediów społecznościowych: {{weight}}",
        ru: "Вес прессы и соцсетей: {{weight}}",
        uk: "Вага преси й соцмереж: {{weight}}",
        ja: "報道とソーシャルメディアの重み：{{weight}}",
        ko: "언론·소셜 신호 가중치: {{weight}}",
        "zh-Hans": "新闻与社交媒体信号的权重：{{weight}}",
        th: "น้ำหนักของสัญญาณจากข่าวและโซเชียลมีเดีย: {{weight}}",
      }),
    ),
    slider: t({
      fr: "Poids de la presse et des réseaux",
      "en-GB": "Weight of news and social signals",
      de: "Gewicht von Presse und sozialen Netzwerken",
      it: "Peso di stampa e social",
      "es-ES": "Peso de la prensa y las redes sociales",
      "pt-PT": "Peso da imprensa e das redes sociais",
      nl: "Gewicht van pers en sociale media",
      pl: "Waga prasy i mediów społecznościowych",
      ru: "Вес прессы и соцсетей",
      uk: "Вага преси й соцмереж",
      ja: "報道とソーシャルメディアの重み",
      ko: "언론·소셜 신호 가중치",
      "zh-Hans": "新闻与社交媒体信号的权重",
      th: "น้ำหนักของสัญญาณจากข่าวและโซเชียลมีเดีย",
    }),
    help: t({
      fr: "0 % : les sondages seuls ; 100 % : les poids publiés ; 200 % : deux fois plus forts. Règle combien un pic de presse ou d'attention élargit l'incertitude d'un candidat et combien l'élan de l'attention et du soutien sur X incline son score.",
      "en-GB":
        "0%: the polls alone; 100%: the published weights; 200%: twice as strong. Sets how much a news or attention spike widens a candidate's uncertainty and how much momentum in attention and X support tilts their vote.",
      de: "0 %: nur die Umfragen; 100 %: die veröffentlichten Gewichte; 200 %: doppelt so stark. Bestimmt, wie stark eine Spitze in Presse oder Aufmerksamkeit die Unsicherheit eines Kandidaten vergrößert und wie stark die Dynamik von Aufmerksamkeit und Unterstützung auf X seine Stimmen verschiebt.",
      it: "0 %: solo i sondaggi; 100 %: i pesi pubblicati; 200 %: il doppio. Regola quanto un picco di stampa o di attenzione allarga l'incertezza di un candidato e quanto lo slancio dell'attenzione e del sostegno su X ne sposta il voto.",
      "es-ES":
        "0 %: solo las encuestas; 100 %: los pesos publicados; 200 %: el doble. Fija cuánto amplía un pico de prensa o de atención la incertidumbre de un candidato y cuánto inclina su voto el impulso de la atención y del apoyo en X.",
      "pt-PT":
        "0 %: apenas as sondagens; 100 %: os pesos publicados; 200 %: o dobro. Define quanto um pico de imprensa ou de atenção alarga a incerteza de um candidato e quanto o impulso da atenção e do apoio no X inclina o seu voto.",
      nl: "0%: alleen de peilingen; 100%: de gepubliceerde gewichten; 200%: twee keer zo sterk. Bepaalt hoeveel een piek in pers of aandacht de onzekerheid rond een kandidaat vergroot en hoeveel de dynamiek in aandacht en steun op X zijn stemmen verschuift.",
      pl: "0%: same sondaże; 100%: opublikowane wagi; 200%: dwa razy silniej. Określa, jak bardzo skok doniesień lub zainteresowania zwiększa niepewność co do kandydata i jak bardzo impet zainteresowania i poparcia na X przechyla jego wynik.",
      ru: "0 %: только опросы; 100 %: опубликованные веса; 200 %: вдвое сильнее. Задаёт, насколько всплеск публикаций или интереса расширяет неопределённость по кандидату и насколько динамика интереса и поддержки в X смещает его результат.",
      uk: "0%: лише опитування; 100%: опубліковані ваги; 200%: удвічі сильніше. Визначає, наскільки сплеск публікацій чи інтересу розширює невизначеність щодо кандидата і наскільки динаміка інтересу й підтримки в X зміщує його результат.",
      ja: "0%：世論調査のみ、100%：公表している重み、200%：その2倍。報道や関心の急増が候補者の不確実性をどれだけ広げ、関心とXでの支持の勢いが得票をどれだけ傾けるかを決めます。",
      ko: "0%: 여론조사만, 100%: 공표된 가중치, 200%: 두 배. 보도나 관심 급증이 후보의 불확실성을 얼마나 키우는지, 관심과 X 지지의 추세가 득표를 얼마나 기울이는지를 정합니다.",
      "zh-Hans":
        "0%：仅用民调；100%：公布的权重；200%：加倍。决定新闻或关注度激增会使候选人的不确定性扩大多少，以及关注度和 X 上支持度的势头会使其得票倾斜多少。",
      th: "0%: ใช้โพลล้วน 100%: น้ำหนักที่เผยแพร่ 200%: แรงขึ้นสองเท่า กำหนดว่าการพุ่งขึ้นของข่าวหรือความสนใจทำให้ความไม่แน่นอนของผู้สมัครกว้างขึ้นเท่าใด และแรงส่งของความสนใจและแรงสนับสนุนบน X ทำให้คะแนนเอียงเท่าใด",
    }),
    reset: insert(
      t({
        fr: "Revenir à {{default}}",
        "en-GB": "Back to {{default}}",
        de: "Zurück auf {{default}}",
        it: "Torna a {{default}}",
        "es-ES": "Volver a {{default}}",
        "pt-PT": "Voltar a {{default}}",
        nl: "Terug naar {{default}}",
        pl: "Wróć do {{default}}",
        ru: "Вернуть {{default}}",
        uk: "Повернути {{default}}",
        ja: "{{default}}に戻す",
        ko: "{{default}}로 되돌리기",
        "zh-Hans": "恢复为 {{default}}",
        th: "กลับไปที่ {{default}}",
      }),
    ),
  },
} satisfies Dictionary
