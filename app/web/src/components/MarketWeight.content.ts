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
        fr: "Le modèle compte déjà les sondages, les pics de presse et d'attention, et l'élan de l'attention et du soutien sur X. Ce curseur y mêle les marchés de prédiction : 0 % le modèle seul, 100 % les marchés seuls. Le site publie {{default}} ; les chances ci-dessous suivent le curseur.",
        "en-GB":
          "The model already counts the polls, news and attention spikes, and momentum in attention and in X support. This slider mixes in the prediction markets: 0% the model alone, 100% the markets alone. The site publishes {{default}}; the chances below follow the slider.",
        de: "Das Modell berücksichtigt bereits die Umfragen, Spitzen in Presse und Aufmerksamkeit sowie die Dynamik der Aufmerksamkeit und der Unterstützung auf X. Dieser Regler mischt die Prognosemärkte hinzu: 0 % nur das Modell, 100 % nur die Märkte. Die Seite veröffentlicht {{default}}; die Chancen unten folgen dem Regler.",
        it: "Il modello tiene già conto dei sondaggi, dei picchi di stampa e di attenzione e dello slancio dell'attenzione e del sostegno su X. Questo cursore vi mescola i mercati di previsione: 0 % solo il modello, 100 % solo i mercati. Il sito pubblica {{default}}; le probabilità qui sotto seguono il cursore.",
        "es-ES":
          "El modelo ya tiene en cuenta las encuestas, los picos de prensa y de atención y el impulso de la atención y del apoyo en X. Este control añade los mercados de predicción: 0 %, solo el modelo; 100 %, solo los mercados. El sitio publica {{default}}; las probabilidades de abajo siguen el control.",
        "pt-PT":
          "O modelo já tem em conta as sondagens, os picos de imprensa e de atenção e o impulso da atenção e do apoio no X. Este cursor junta-lhe os mercados de previsão: 0 %, apenas o modelo; 100 %, apenas os mercados. O site publica {{default}}; as probabilidades abaixo seguem o cursor.",
        nl: "Het model telt al de peilingen, de pieken in pers en aandacht, en de dynamiek in aandacht en in steun op X. Deze schuifregelaar mengt er de voorspellingsmarkten bij: 0% alleen het model, 100% alleen de markten. De site publiceert {{default}}; de kansen hieronder volgen de schuifregelaar.",
        pl: "Model uwzględnia już sondaże, skoki doniesień i zainteresowania oraz impet zainteresowania i poparcia na X. Ten suwak dodaje do tego rynki predykcyjne: 0% to sam model, 100% same rynki. Serwis publikuje {{default}}; szanse poniżej zmieniają się wraz z suwakiem.",
        ru: "Модель уже учитывает опросы, всплески публикаций и интереса, а также динамику интереса и поддержки в X. Этот ползунок подмешивает рынки прогнозов: 0 % — только модель, 100 % — только рынки. Сайт публикует {{default}}; шансы ниже следуют за ползунком.",
        uk: "Модель уже враховує опитування, сплески публікацій та інтересу, а також динаміку інтересу й підтримки в X. Цей повзунок додає ринки прогнозів: 0% — лише модель, 100% — лише ринки. Сайт публікує {{default}}; шанси нижче змінюються за повзунком.",
        ja: "モデルはすでに世論調査、報道と関心の急増、関心とXでの支持の勢いを反映しています。このスライダーは予測市場を混ぜます。0%はモデルのみ、100%は市場のみです。サイトの公表値は{{default}}で、下の確率はスライダーに連動します。",
        ko: "모델에는 이미 여론조사, 보도·관심 급증, 관심과 X 지지의 추세가 반영되어 있습니다. 이 슬라이더는 여기에 예측시장을 섞습니다. 0%는 모델만, 100%는 시장만입니다. 사이트의 공표 기준은 {{default}}이며, 아래 확률은 슬라이더에 따라 바뀝니다.",
        "zh-Hans":
          "模型已纳入民调、新闻与关注度激增，以及关注度和 X 上支持度的势头。此滑块再混入预测市场：0% 为仅用模型，100% 为仅用市场。本站公布值采用 {{default}}，下方概率随滑块变化。",
        th: "แบบจำลองนับรวมโพล การพุ่งขึ้นของข่าวและความสนใจ และแรงส่งของความสนใจและแรงสนับสนุนบน X อยู่แล้ว แถบเลื่อนนี้ผสมตลาดคาดการณ์เข้าไป: 0% ใช้แบบจำลองล้วน 100% ใช้ตลาดล้วน เว็บไซต์เผยแพร่ที่ {{default}} และโอกาสด้านล่างจะเปลี่ยนตามแถบเลื่อน",
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
