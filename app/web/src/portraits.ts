// Candidates' portraits. Each is the lead image of the candidate's French Wikipedia article (the
// titles in config/sources.yaml), taken only when the file is under a free licence, and cropped
// square around the face by one rule for everyone: the face fills the same share of the frame, at
// the same height (research/LOG.md, 2026-10-16). The UI shows them in black and white.

/** Who took a portrait and under what terms: the credits the licences ask for. */
export interface PortraitCredit {
  readonly author: string
  readonly license: string
  readonly licenseUrl: string | null
  /** The file's page on Wikimedia Commons. */
  readonly file: string
}

export const CREDITS: Readonly<Record<string, PortraitCredit>> = {
  arthaud: {
    author: "Kokluts",
    license: "CC BY-SA 4.0",
    licenseUrl: "https://creativecommons.org/licenses/by-sa/4.0",
    file: "https://commons.wikimedia.org/wiki/File:Nathalie_Arthaud_(LO)_19-05-2024.jpg",
  },
  asselineau: {
    author: "Union Populaire Républicaine",
    license: "CC BY-SA 4.0",
    licenseUrl: "https://creativecommons.org/licenses/by-sa/4.0",
    file: "https://commons.wikimedia.org/wiki/File:Fran%C3%A7ois_ASSELINEAU.jpg",
  },
  attal: {
    author: "Ismail Aissoub",
    license: "CC BY 4.0",
    licenseUrl: "https://creativecommons.org/licenses/by/4.0",
    file: "https://commons.wikimedia.org/wiki/File:Gabriel_Attal_2025_(close_crop).jpg",
  },
  bardella: {
    author: "European Union",
    license: "© European Union, European Parliament",
    licenseUrl: "https://www.europarl.europa.eu/legal-notice/",
    file: "https://commons.wikimedia.org/wiki/File:MEP_Jordan_Bardella.jpg",
  },
  batho: {
    author: "DeuxSevres79",
    license: "CC BY-SA 4.0",
    licenseUrl: "https://creativecommons.org/licenses/by-sa/4.0",
    file: "https://commons.wikimedia.org/wiki/File:Delphine_Batho_(cropped-2).png",
  },
  becht: {
    author: "Dominique Giannelli",
    license: "CC BY 4.0",
    licenseUrl: "https://creativecommons.org/licenses/by/4.0",
    file: "https://commons.wikimedia.org/wiki/File:Olivier_Becht_en_2026_(cropped).png",
  },
  bertrand: {
    author: "Claudio Centonze / European Union, 2025 / EC - Audiovisual Service",
    license: "CC BY 4.0",
    licenseUrl: "https://creativecommons.org/licenses/by/4.0",
    file: "https://commons.wikimedia.org/wiki/File:Xavier_Bertrand_-_2025_(cropped).jpg",
  },
  bouamrane: {
    author: "Oumma",
    license: "CC BY 4.0",
    licenseUrl: "https://creativecommons.org/licenses/by/4.0",
    file: "https://commons.wikimedia.org/wiki/File:Karim_Bouamrane_en_2026_(cropped).jpg",
  },
  cazeneuve: {
    author: "Jérémy Barande",
    license: "CC BY-SA 2.0",
    licenseUrl: "https://creativecommons.org/licenses/by-sa/2.0",
    file: "https://commons.wikimedia.org/wiki/File:Bernard_Cazeneuve,_(42399145362)_(cropped).jpg",
  },
  dupont_aignan: {
    author: "Debout la France",
    license: "CC BY-SA 4.0",
    licenseUrl: "https://creativecommons.org/licenses/by-sa/4.0",
    file: "https://commons.wikimedia.org/wiki/File:Nicolas_Dupont-Aignan,_homme_politique_fran%C3%A7ais.jpg",
  },
  faure: {
    author: "KimiRaikkonen3",
    license: "Public domain",
    licenseUrl: null,
    file: "https://commons.wikimedia.org/wiki/File:OF_MATIGNON_-_MAXIME_CLAM_(cropped).png",
  },
  glucksmann: {
    author: "European Union 2024 - Source : EP",
    license: "© European Union, European Parliament",
    licenseUrl: "https://www.europarl.europa.eu/legal-notice/",
    file: "https://commons.wikimedia.org/wiki/File:1720448398743_20240708_GLUCKSMANN_Raphael_FR_006.jpg",
  },
  guedj: {
    author: "Audrey AK",
    license: "CC BY-SA 2.0",
    licenseUrl: "https://creativecommons.org/licenses/by-sa/2.0",
    file: "https://commons.wikimedia.org/wiki/File:J%C3%A9r%C3%B4me_Guedj_2010.jpg",
  },
  hollande: {
    author: "Christophe Licoppe / European Union, 2024 / EC - Audiovisual Service",
    license: "CC BY 4.0",
    licenseUrl: "https://creativecommons.org/licenses/by/4.0",
    file: "https://commons.wikimedia.org/wiki/File:Fran%C3%A7ois_Hollande_-_Ceremony_of_Honour_for_Jacques_Delors_-_2024.jpg",
  },
  kazib: {
    author: "PabloMorao27",
    license: "CC BY-SA 4.0",
    licenseUrl: "https://creativecommons.org/licenses/by-sa/4.0",
    file: "https://commons.wikimedia.org/wiki/File:Anasse_Kazib,_d%C3%A9cembre_2021.jpg",
  },
  le_maire: {
    author: "Belgian Presidency of the Council of the EU 2024 from Belgium",
    license: "CC BY 2.0",
    licenseUrl: "https://creativecommons.org/licenses/by/2.0",
    file: "https://commons.wikimedia.org/wiki/File:EU2024BE_240223_Informal_meeting_of_economic_and_financial_affairs_ministers_Gent_JNZT_0040_(53545889372).jpg",
  },
  le_pen: {
    author: "Vox España",
    license: "CC0",
    licenseUrl: "http://creativecommons.org/publicdomain/zero/1.0/deed.en",
    file: "https://commons.wikimedia.org/wiki/File:Marine_Le_Pen_2025_(cropped).jpg",
  },
  lisnard: {
    author: "Grégor Sauvagère",
    license: "CC BY-SA 4.0",
    licenseUrl: "https://creativecommons.org/licenses/by-sa/4.0",
    file: "https://commons.wikimedia.org/wiki/File:David_Lisnard_en_2026.jpg",
  },
  maurel: {
    author: "Echwander",
    license: "CC BY-SA 4.0",
    licenseUrl: "https://creativecommons.org/licenses/by-sa/4.0",
    file: "https://commons.wikimedia.org/wiki/File:Emmanuel_Maurel_en_2016.jpg",
  },
  melenchon: {
    author: "jlm2017.fr",
    license: "CC BY 4.0",
    licenseUrl: "https://creativecommons.org/licenses/by/4.0",
    file: "https://commons.wikimedia.org/wiki/File:Jean_Luc_M%C3%A9lenchon_en_mai_2026-_55261894422_(cropped).jpg",
  },
  philippe: {
    author: "Wasasaq8",
    license: "CC0",
    licenseUrl: "http://creativecommons.org/publicdomain/zero/1.0/deed.en",
    file: "https://commons.wikimedia.org/wiki/File:Edouard_Philippe_3x4_crop.jpg",
  },
  philippot: {
    author: "Thomas Bresson",
    license: "CC BY 4.0",
    licenseUrl: "https://creativecommons.org/licenses/by/4.0",
    file: "https://commons.wikimedia.org/wiki/File:2022-04-16_16-49-26_MAM-Paris_02.jpg",
  },
  retailleau: {
    author: "Anthonymontardyfr",
    license: "CC BY 4.0",
    licenseUrl: "https://creativecommons.org/licenses/by/4.0",
    file: "https://commons.wikimedia.org/wiki/File:Bruno_Retailleau_-_Ministre_de_l%27Int%C3%A9rieur_fran%C3%A7ais_(cropped).jpg",
  },
  roussel: {
    author: "Zouhair NAKARA",
    license: "CC BY-SA 4.0",
    licenseUrl: "https://creativecommons.org/licenses/by-sa/4.0",
    file: "https://commons.wikimedia.org/wiki/File:Roussel_Fabien_1.jpg",
  },
  royal: {
    author: "Jackolan1",
    license: "CC BY-SA 3.0",
    licenseUrl: "https://creativecommons.org/licenses/by-sa/3.0",
    file: "https://commons.wikimedia.org/wiki/File:Royal_Toulouse_2012.JPG",
  },
  ruffin: {
    author: "Thinkerview",
    license: "CC BY-SA 4.0",
    licenseUrl: "https://creativecommons.org/licenses/by-sa/4.0",
    file: "https://commons.wikimedia.org/wiki/File:Fran%C3%A7ois_Ruffin_(cropped).jpg",
  },
  tondelier: {
    author: "Selbymay",
    license: "CC BY 4.0",
    licenseUrl: "https://creativecommons.org/licenses/by/4.0",
    file: "https://commons.wikimedia.org/wiki/File:Huma2023MarineTondelier_(cropped).jpg",
  },
  villepin: {
    author: "Georges Seguin (Okki)",
    license: "CC BY-SA 3.0",
    licenseUrl: "https://creativecommons.org/licenses/by-sa/3.0",
    file: "https://commons.wikimedia.org/wiki/File:Dominique_de_Villepin_20100330_Salon_du_livre_de_Paris_2_cropped.jpg",
  },
  zemmour: {
    author: "Anh De France",
    license: "CC0",
    licenseUrl: "http://creativecommons.org/publicdomain/zero/1.0/deed.en",
    file: "https://commons.wikimedia.org/wiki/File:Portrait_d%27%C3%89ric_Zemmour,_avril_2022.jpg",
  },
}

/** Each candidate's French Wikipedia article, from config/sources.yaml. */
export const ARTICLES: Readonly<Record<string, string>> = {
  arthaud: "https://fr.wikipedia.org/wiki/Nathalie_Arthaud",
  asselineau: "https://fr.wikipedia.org/wiki/Fran%C3%A7ois_Asselineau",
  attal: "https://fr.wikipedia.org/wiki/Gabriel_Attal",
  bardella: "https://fr.wikipedia.org/wiki/Jordan_Bardella",
  batho: "https://fr.wikipedia.org/wiki/Delphine_Batho",
  becht: "https://fr.wikipedia.org/wiki/Olivier_Becht",
  bertrand: "https://fr.wikipedia.org/wiki/Xavier_Bertrand",
  bouamrane: "https://fr.wikipedia.org/wiki/Karim_Bouamrane",
  cazeneuve: "https://fr.wikipedia.org/wiki/Bernard_Cazeneuve",
  dupont_aignan: "https://fr.wikipedia.org/wiki/Nicolas_Dupont-Aignan",
  faure: "https://fr.wikipedia.org/wiki/Olivier_Faure",
  glucksmann: "https://fr.wikipedia.org/wiki/Rapha%C3%ABl_Glucksmann",
  guedj: "https://fr.wikipedia.org/wiki/J%C3%A9r%C3%B4me_Guedj",
  hollande: "https://fr.wikipedia.org/wiki/Fran%C3%A7ois_Hollande",
  kazib: "https://fr.wikipedia.org/wiki/Anasse_Kazib",
  labib: "https://fr.wikipedia.org/wiki/Selma_Labib",
  le_maire: "https://fr.wikipedia.org/wiki/Bruno_Le_Maire",
  le_pen: "https://fr.wikipedia.org/wiki/Marine_Le_Pen",
  lisnard: "https://fr.wikipedia.org/wiki/David_Lisnard",
  maurel: "https://fr.wikipedia.org/wiki/Emmanuel_Maurel",
  melenchon: "https://fr.wikipedia.org/wiki/Jean-Luc_M%C3%A9lenchon",
  philippe: "https://fr.wikipedia.org/wiki/%C3%89douard_Philippe",
  philippot: "https://fr.wikipedia.org/wiki/Florian_Philippot",
  retailleau: "https://fr.wikipedia.org/wiki/Bruno_Retailleau",
  roussel: "https://fr.wikipedia.org/wiki/Fabien_Roussel",
  royal: "https://fr.wikipedia.org/wiki/S%C3%A9gol%C3%A8ne_Royal",
  ruffin: "https://fr.wikipedia.org/wiki/Fran%C3%A7ois_Ruffin",
  tondelier: "https://fr.wikipedia.org/wiki/Marine_Tondelier",
  villepin: "https://fr.wikipedia.org/wiki/Dominique_de_Villepin",
  zemmour: "https://fr.wikipedia.org/wiki/%C3%89ric_Zemmour",
}

const files = import.meta.glob<string>("./assets/portraits/*.webp", { eager: true, import: "default" })

export interface Portrait extends PortraitCredit {
  readonly src: string
}

/** The candidate's portrait, or `null` when no freely licensed photograph was found. */
export function portrait(candidateId: string): Portrait | null {
  const credit = CREDITS[candidateId]
  const src = files[`./assets/portraits/${candidateId}.webp`]
  return credit && src ? { ...credit, src } : null
}

/** Two initials for a candidate without a portrait: the first and the last word of the name. */
export function initials(name: string): string {
  const words = name.split(/\s+/).filter(Boolean)
  const first = words[0]?.[0] ?? ""
  const last = words.length > 1 ? (words.at(-1)?.[0] ?? "") : ""
  return (first + last).toUpperCase()
}
