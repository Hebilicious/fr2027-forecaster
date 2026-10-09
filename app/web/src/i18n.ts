import { computed, ref } from "vue"

export type Locale = "fr" | "en"

const en = {
  title: "France 2027 forecast",
  subtitle: "Probabilistic forecast of the presidential election",
  daysToRound1: "{n} days to round 1",
  roundDates: "Round 1 on {r1}, round 2 on {r2}",
  updated: "Updated {when}",
  asOf: "polls published by {date}",
  language: "Français",
  navOverview: "Overview",
  navRound1: "Round 1",
  navRound2: "Round 2",
  navSources: "Sources and health",
  loading: "Loading…",
  noForecast: "No forecast yet. Run `moon run repo:forecast`, then reload.",
  stale: "Showing the last data that loaded. The latest refresh failed: {error}",
  uncertainty:
    "These are probabilities, not predictions: a 70% favourite still loses about 3 times in 10. Intervals cover 80% of simulated outcomes.",
  candidate: "Candidate",
  runs: "Runs",
  qualifies: "Reaches round 2",
  wins: "Wins",
  r1Share: "Round 1 share if running",
  notPolled: "not yet tested by any poll",
  whatMoved: "What moved since the previous run",
  noPrevious: "This is the first forecast run.",
  newPolls: "New polls: {list}",
  noNewPolls: "No new polls.",
  noChange: "No candidate moved by more than 0.5 point.",
  modelNote:
    "Polls-only model {version}: {polls} polls ({scenarios} ballots tested), {sims} simulated elections. Market, news and X signals are not in this version.",
  warnings: "Warnings",
  round1Title: "Round 1: polling average by candidate",
  round1Help:
    "Each panel shows a candidate's share in the most likely field (with them in it), from the polls known on each date, with an 80% band. Dots are individual polls of any scenario that tested the candidate; select one to open its source.",
  highlightFirm: "Highlight a pollster",
  allFirms: "None",
  pollTooltip: "{firm}, {date}: {share} (scenario {scenario})",
  round2Title: "Round 2 matchups",
  round2Help:
    "Rows and columns are the candidates most likely to reach round 2. Each cell gives the probability that this pairing happens and how often the row candidate wins it.",
  matchupCell: "{p} chance · row wins {w}",
  topMatchups: "Most likely matchups",
  pairing: "Pairing",
  happens: "Happens",
  winner: "Who wins it",
  polledBadge: "polled head-to-head",
  modelledBadge: "from vote transfers",
  sourcesTitle: "Sources and health",
  latestRun: "Latest run",
  file: "File",
  generatedAt: "Generated",
  lastFieldwork: "Last fieldwork",
  collectors: "Collectors",
  lastWritten: "Last written",
  quarantine: "Quarantined files",
  none: "None",
  pollsTitle: "Polls in the model",
  published: "Published",
  firm: "Pollster",
  sponsor: "Sponsor",
  fieldwork: "Fieldwork",
  sample: "Sample",
  ballots: "Ballots",
  entry: "Entry",
  entryPrimary: "checked against the pollster",
  entryIndex: "from a poll index",
  source: "Source",
  notice: "Notice",
  healthy: "No warnings.",
}

type Messages = typeof en

const fr: Messages = {
  title: "Prévision France 2027",
  subtitle: "Prévision probabiliste de l'élection présidentielle",
  daysToRound1: "J-{n} avant le premier tour",
  roundDates: "Premier tour le {r1}, second tour le {r2}",
  updated: "Mise à jour {when}",
  asOf: "sondages publiés jusqu'au {date}",
  language: "English",
  navOverview: "Vue d'ensemble",
  navRound1: "Premier tour",
  navRound2: "Second tour",
  navSources: "Sources et état",
  loading: "Chargement…",
  noForecast: "Aucune prévision pour l'instant. Lancez `moon run repo:forecast`, puis rechargez.",
  stale: "Affichage des dernières données chargées. La dernière actualisation a échoué : {error}",
  uncertainty:
    "Ce sont des probabilités, pas des prédictions : un favori à 70 % perd encore environ 3 fois sur 10. Les intervalles couvrent 80 % des issues simulées.",
  candidate: "Candidat",
  runs: "Se présente",
  qualifies: "Accède au second tour",
  wins: "Gagne",
  r1Share: "Score au premier tour s'il se présente",
  notPolled: "pas encore testé par un sondage",
  whatMoved: "Ce qui a bougé depuis le calcul précédent",
  noPrevious: "C'est le premier calcul.",
  newPolls: "Nouveaux sondages : {list}",
  noNewPolls: "Aucun nouveau sondage.",
  noChange: "Aucun candidat n'a bougé de plus de 0,5 point.",
  modelNote:
    "Modèle sondages seuls {version} : {polls} sondages ({scenarios} hypothèses testées), {sims} élections simulées. Marchés, presse et signaux X ne sont pas dans cette version.",
  warnings: "Avertissements",
  round1Title: "Premier tour : moyenne des sondages par candidat",
  round1Help:
    "Chaque panneau montre le score d'un candidat dans le scénario le plus probable (en l'y incluant), d'après les sondages connus à chaque date, avec une bande à 80 %. Les points sont les sondages de toute hypothèse qui l'a testé ; sélectionnez-en un pour ouvrir sa source.",
  highlightFirm: "Mettre en avant un institut",
  allFirms: "Aucun",
  pollTooltip: "{firm}, {date} : {share} (hypothèse {scenario})",
  round2Title: "Duels du second tour",
  round2Help:
    "Lignes et colonnes : les candidats les plus susceptibles d'accéder au second tour. Chaque case donne la probabilité de ce duel et la fréquence à laquelle le candidat de la ligne l'emporte.",
  matchupCell: "{p} de chances · la ligne gagne {w}",
  topMatchups: "Duels les plus probables",
  pairing: "Duel",
  happens: "Probabilité",
  winner: "Qui l'emporte",
  polledBadge: "sondé en duel",
  modelledBadge: "par reports de voix",
  sourcesTitle: "Sources et état",
  latestRun: "Dernier calcul",
  file: "Fichier",
  generatedAt: "Généré",
  lastFieldwork: "Dernier terrain",
  collectors: "Collecteurs",
  lastWritten: "Dernière écriture",
  quarantine: "Fichiers en quarantaine",
  none: "Aucun",
  pollsTitle: "Sondages utilisés",
  published: "Publié",
  firm: "Institut",
  sponsor: "Commanditaire",
  fieldwork: "Terrain",
  sample: "Échantillon",
  ballots: "Hypothèses",
  entry: "Saisie",
  entryPrimary: "vérifiée auprès de l'institut",
  entryIndex: "depuis un index de sondages",
  source: "Source",
  notice: "Notice",
  healthy: "Aucun avertissement.",
}

export type MessageKey = keyof Messages

const STORAGE_KEY = "fr2027.locale"

function initialLocale(): Locale {
  try {
    const stored = globalThis.localStorage?.getItem(STORAGE_KEY)
    if (stored === "fr" || stored === "en") return stored
  } catch {
    // Storage can be unavailable (private windows, blocked site data); fall through.
  }
  return globalThis.navigator?.language?.toLowerCase().startsWith("fr") ? "fr" : "en"
}

const locale = ref<Locale>(initialLocale())

export function useI18n() {
  const messages = computed(() => (locale.value === "fr" ? fr : en))
  const t = (key: MessageKey, values: Record<string, string | number> = {}): string =>
    messages.value[key].replace(/\{(\w+)\}/g, (match, name: string) => {
      const value = values[name]
      return value === undefined ? match : String(value)
    })
  const toggle = () => {
    locale.value = locale.value === "fr" ? "en" : "fr"
    try {
      globalThis.localStorage?.setItem(STORAGE_KEY, locale.value)
    } catch {
      // Remembering the choice is a convenience only.
    }
    document.documentElement.lang = locale.value
  }
  return { locale, t, toggle }
}

export const messages = { en, fr }
