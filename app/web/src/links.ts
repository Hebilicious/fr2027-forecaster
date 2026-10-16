// Links on each candidate's card in the Candidates view. Checked on 2026-10-16 (research/LOG.md).

/**
 * The website of each candidate's party, from Wikidata's "official website" where it has one,
 * checked by loading it. Missing: La Convention (Bernard Cazeneuve), whose site could not be
 * confirmed, and Sébastien Bonnal, who has no party.
 */
export const PARTY_SITES: Readonly<Record<string, string>> = {
  arthaud: "https://www.lutte-ouvriere.org/",
  kazib: "https://www.revolutionpermanente.fr/",
  labib: "https://npa-revolutionnaires.org/",
  melenchon: "https://lafranceinsoumise.fr/",
  glucksmann: "https://place-publique.eu/",
  faure: "https://www.parti-socialiste.fr/",
  royal: "https://www.parti-socialiste.fr/",
  guedj: "https://www.parti-socialiste.fr/",
  hollande: "https://www.parti-socialiste.fr/",
  bouamrane: "https://www.parti-socialiste.fr/",
  maurel: "https://g-r-s.fr/",
  roussel: "https://www.pcf.fr/",
  tondelier: "https://lesecologistes.fr/",
  ruffin: "https://debout.fr/",
  batho: "https://www.generationecologie.fr/",
  philippe: "https://horizonsleparti.fr/",
  attal: "https://parti-renaissance.fr/",
  le_maire: "https://parti-renaissance.fr/",
  becht: "https://parti-renaissance.fr/",
  retailleau: "https://republicains.fr/",
  lisnard: "https://www.unenouvelleenergie.fr/",
  bertrand: "https://www.nousfrance.fr/",
  villepin: "https://lafrancehumaniste.fr/",
  le_pen: "https://rassemblementnational.fr/",
  bardella: "https://rassemblementnational.fr/",
  zemmour: "https://www.parti-reconquete.fr/",
  dupont_aignan: "https://www.debout-la-france.fr/",
  philippot: "https://les-patriotes.fr/",
  asselineau: "https://upr.fr/",
}

/**
 * Each candidate's 2027 programme, once published on their own or their party's site. None was
 * on 2026-10-16: La France insoumise announces its 2027 edition of L'Avenir en commun for
 * 6–7 November, and Édouard Philippe has presented proposals on television but published no
 * programme. Add the link here when one is released; the card says "not published yet" until then.
 */
export const PROGRAMMES: Readonly<Record<string, string>> = {}
