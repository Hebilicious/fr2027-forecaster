import { insert, t, type Dictionary } from "intlayer"

export default {
  key: "market-weight",
  content: {
    label: insert(
      t({
        fr: "Poids des marchés de prédiction : {{weight}}",
        "en-GB": "Weight of the prediction markets: {{weight}}",
        de: "Gewicht der Prognosemärkte: {{weight}}",
        it: "Peso dei mercati di previsione: {{weight}}",
        "es-ES": "Peso de los mercados de predicción: {{weight}}",
        "pt-PT": "Peso dos mercados de previsão: {{weight}}",
        nl: "Gewicht van de voorspellingsmarkten: {{weight}}",
        pl: "Waga rynków predykcyjnych: {{weight}}",
        ru: "Вес рынков прогнозов: {{weight}}",
        uk: "Вага ринків прогнозів: {{weight}}",
        ja: "予測市場の重み：{{weight}}",
        ko: "예측시장 가중치: {{weight}}",
        "zh-Hans": "预测市场的权重：{{weight}}",
        th: "น้ำหนักของตลาดคาดการณ์: {{weight}}",
      }),
    ),
    slider: t({
      fr: "Poids des marchés de prédiction",
      "en-GB": "Weight of the prediction markets",
      de: "Gewicht der Prognosemärkte",
      it: "Peso dei mercati di previsione",
      "es-ES": "Peso de los mercados de predicción",
      "pt-PT": "Peso dos mercados de previsão",
      nl: "Gewicht van de voorspellingsmarkten",
      pl: "Waga rynków predykcyjnych",
      ru: "Вес рынков прогнозов",
      uk: "Вага ринків прогнозів",
      ja: "予測市場の重み",
      ko: "예측시장 가중치",
      "zh-Hans": "预测市场的权重",
      th: "น้ำหนักของตลาดคาดการณ์",
    }),
    help: insert(
      t({
        fr: "0 % : le modèle seul ; 100 % : les marchés seuls. Le site publie {{default}}. Les chances de victoire et de qualification ci-dessous suivent le curseur.",
        "en-GB":
          "0%: the model alone; 100%: the markets alone. The site publishes {{default}}. The chances of winning and of reaching round 2 below follow the slider.",
        de: "0 %: nur das Modell; 100 %: nur die Märkte. Die Seite veröffentlicht {{default}}. Die Sieg- und Stichwahlchancen unten folgen dem Regler.",
        it: "0 %: solo il modello; 100 %: solo i mercati. Il sito pubblica {{default}}. Le probabilità di vittoria e di accesso al secondo turno qui sotto seguono il cursore.",
        "es-ES":
          "0 %: solo el modelo; 100 %: solo los mercados. El sitio publica {{default}}. Las probabilidades de ganar y de pasar a la segunda vuelta que aparecen abajo siguen el control deslizante.",
        "pt-PT":
          "0 %: apenas o modelo; 100 %: apenas os mercados. O site publica {{default}}. As probabilidades de vitória e de passagem à segunda volta abaixo seguem o cursor.",
        nl: "0%: alleen het model; 100%: alleen de markten. De site publiceert {{default}}. De kansen op winst en op de tweede ronde hieronder volgen de schuifregelaar.",
        pl: "0%: sam model; 100%: same rynki. Serwis publikuje {{default}}. Szanse na zwycięstwo i na awans do drugiej tury poniżej zmieniają się wraz z suwakiem.",
        ru: "0 %: только модель; 100 %: только рынки. Сайт публикует {{default}}. Шансы на победу и на выход во второй тур ниже следуют за ползунком.",
        uk: "0%: лише модель; 100%: лише ринки. Сайт публікує {{default}}. Шанси на перемогу та на вихід у другий тур нижче змінюються за повзунком.",
        ja: "0%：モデルのみ、100%：市場のみ。サイトの公表値は{{default}}です。下の勝利確率と決選投票進出確率はスライダーに連動します。",
        ko: "0%: 모델만, 100%: 시장만. 사이트의 공표 기준은 {{default}}입니다. 아래의 승리 확률과 결선 진출 확률은 슬라이더에 따라 바뀝니다.",
        "zh-Hans":
          "0%：仅采用模型；100%：仅采用市场。本站公布值采用 {{default}}。下方的胜选概率和进入第二轮的概率随滑块变化。",
        th: "0%: ใช้แบบจำลองล้วน 100%: ใช้ตลาดล้วน เว็บไซต์เผยแพร่ที่ {{default}} โอกาสชนะและโอกาสเข้ารอบสองด้านล่างจะเปลี่ยนตามแถบเลื่อน",
      }),
    ),
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
    stale: insert(
      t({
        fr: "Le dernier relevé des marchés ({{when}}) est trop ancien : seul le modèle compte, quel que soit le curseur.",
        "en-GB": "The latest market snapshot ({{when}}) is too old: only the model counts, whatever the slider says.",
        de: "Die letzte Marktabfrage ({{when}}) ist zu alt: Es zählt nur das Modell, unabhängig vom Regler.",
        it: "L'ultima rilevazione dei mercati ({{when}}) è troppo vecchia: conta solo il modello, qualunque sia la posizione del cursore.",
        "es-ES":
          "La última captura de los mercados ({{when}}) es demasiado antigua: solo cuenta el modelo, sea cual sea la posición del control.",
        "pt-PT":
          "O último registo dos mercados ({{when}}) é demasiado antigo: conta apenas o modelo, seja qual for a posição do cursor.",
        nl: "De laatste momentopname van de markten ({{when}}) is te oud: alleen het model telt, wat de schuifregelaar ook aangeeft.",
        pl: "Ostatni odczyt rynków ({{when}}) jest zbyt stary: liczy się tylko model, niezależnie od suwaka.",
        ru: "Последний срез рынков ({{when}}) слишком старый: учитывается только модель, что бы ни показывал ползунок.",
        uk: "Останній зріз ринків ({{when}}) застарий: враховується лише модель, хоч би що показував повзунок.",
        ja: "最新の市場データ（{{when}}）は古すぎるため、スライダーの位置にかかわらずモデルのみを反映します。",
        ko: "최근 시장 자료({{when}})가 너무 오래되어 슬라이더와 관계없이 모델 값만 반영합니다.",
        "zh-Hans": "最新的市场数据（{{when}}）过旧：无论滑块位置如何，仅采用模型。",
        th: "ข้อมูลตลาดล่าสุด ({{when}}) เก่าเกินไป จึงใช้เฉพาะแบบจำลองไม่ว่าแถบเลื่อนจะอยู่ที่ใด",
      }),
    ),
  },
} satisfies Dictionary
