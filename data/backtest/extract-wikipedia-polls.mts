// Extracts the 2002, 2007, 2012, 2017 and 2022 poll tables of the English Wikipedia ("Opinion
// polling for the <year> French presidential election", one pinned revision each) into
// data/backtest/<year>/polls.csv, in the columns of data/clean/polls.csv.
//
// Wikipedia is used as an index only: every row says so (`entry: index`), keeps the pollster
// link the table gives as `source_url`, and records the revision it came from (`index_url`,
// `content_hash` of the wikitext, `retrieved_at`). research/sources/README.md describes the checks.
//
// Run from the repository root with Node 24 (type stripping, no dependencies):
//   node data/backtest/extract-wikipedia-polls.mts            fetch the pinned revisions
//   node data/backtest/extract-wikipedia-polls.mts <dir>      read <dir>/en-<oldid>.wikitext instead
//   node data/backtest/extract-wikipedia-polls.mts <dir> 2002 2007    only these years
// RETRIEVED_AT=<UTC minute> sets retrieved_at when reading local files (2017 and 2022 were
// retrieved at 2026-10-16T13:57Z, 2002–2012 at 2026-10-17T00:43Z).
//
// Rules applied, all visible in the output:
// - One poll is one firm cell (with its rowspan); each of its rows is a scenario.
// - "–" or an empty cell means the candidate was not tested; "<0.5%" is written 0.5, as in
//   data/raw/polls.
// - published_at is not in the tables: it is set to the day after fieldwork ends.
// - Polls without a sample size, rows that sum outside 85–103 (round 1) or 98–102 (round 2),
//   and cells naming no known candidate are left out and counted in the summary.
// - Sections "By region", "By department", "By commune", "By constituency" and "By first round
//   vote" are sub-national or vote-transfer tables and are skipped.
// - Firms are named as they are today: Sofres and TNS Sofres are kantar, Louis Harris and its
//   successor LH2 are lh2 (not Harris Interactive, a different firm).
// - 2012: a column headed "(nominee)" with the NPA's link (polls before the party chose its
//   candidate) is read as Poutou, who ran (NOMINEES).
// - 2007: the TNS Sofres poll of 20–21 April, which the page says was an internal survey not
//   published before the vote, is left out (UNPUBLISHED).

import { createHash } from "node:crypto"
import { mkdirSync, readFileSync, writeFileSync } from "node:fs"
import { join } from "node:path"

interface Page {
  year: number
  title: string
  oldid: number
  round1Date: string
  round2Date: string
}

const PAGES: Page[] = [
  {
    year: 2002,
    title: "Opinion_polling_for_the_2002_French_presidential_election",
    oldid: 1329460026,
    round1Date: "2002-04-21",
    round2Date: "2002-05-05",
  },
  {
    year: 2007,
    title: "Opinion_polling_for_the_2007_French_presidential_election",
    oldid: 1341388824,
    round1Date: "2007-04-22",
    round2Date: "2007-05-06",
  },
  {
    year: 2012,
    title: "Opinion_polling_for_the_2012_French_presidential_election",
    oldid: 1279812574,
    round1Date: "2012-04-22",
    round2Date: "2012-05-06",
  },
  {
    year: 2017,
    title: "Opinion_polling_for_the_2017_French_presidential_election",
    oldid: 1213598918,
    round1Date: "2017-04-23",
    round2Date: "2017-05-07",
  },
  {
    year: 2022,
    title: "Opinion_polling_for_the_2022_French_presidential_election",
    oldid: 1363121499,
    round1Date: "2022-04-10",
    round2Date: "2022-04-24",
  },
]

// Link target (or header text) → candidate id. Ids reuse config/candidates.yaml where the person
// is also a 2027 candidate.
const CANDIDATES: Record<string, string> = {
  "Nathalie Arthaud": "arthaud",
  "Philippe Poutou": "poutou",
  "Jean-Luc Mélenchon": "melenchon",
  "Pierre Laurent": "laurent",
  "Fabien Roussel": "roussel",
  "Cécile Duflot": "duflot",
  "Yannick Jadot": "jadot",
  "Nicolas Hulot": "hulot",
  "Éric Piolle": "piolle",
  "Sandrine Rousseau": "rousseau",
  "Delphine Batho": "batho",
  "François Hollande": "hollande",
  "Manuel Valls": "valls",
  "Arnaud Montebourg": "montebourg",
  "Benoît Hamon": "hamon",
  "Vincent Peillon": "peillon",
  "Anne Hidalgo": "hidalgo",
  "Christiane Taubira": "taubira",
  "Ségolène Royal": "royal",
  "Bernard Cazeneuve": "cazeneuve",
  "Olivier Faure": "faure",
  "Raphaël Glucksmann": "glucksmann",
  "François Ruffin": "ruffin",
  "Emmanuel Macron": "macron",
  "François Bayrou": "bayrou",
  "Édouard Philippe": "philippe",
  "Jean-Christophe Lagarde": "lagarde",
  "Alain Juppé": "juppe",
  "Nicolas Sarkozy": "sarkozy",
  "François Fillon": "fillon",
  "Bruno Le Maire": "le_maire",
  "Nathalie Kosciusko-Morizet": "kosciusko_morizet",
  "Jean-François Copé": "cope",
  "Xavier Bertrand": "bertrand",
  "Valérie Pécresse": "pecresse",
  "Michel Barnier": "barnier",
  "Laurent Wauquiez": "wauquiez",
  "Éric Ciotti": "ciotti",
  "Philippe Juvin": "juvin",
  "Denis Payre": "payre",
  "François Baroin": "baroin",
  "Rachida Dati": "dati",
  "Jean-Frédéric Poisson": "poisson",
  "Christine Boutin": "boutin",
  "Jean Lassalle": "lassalle",
  "Nicolas Dupont-Aignan": "dupont_aignan",
  "François Asselineau": "asselineau",
  "Jacques Cheminade": "cheminade",
  "Florian Philippot": "philippot",
  "Marine Le Pen": "le_pen",
  "Marion Maréchal": "marechal",
  "Marion Maréchal-Le Pen": "marechal",
  "Éric Zemmour": "zemmour",
  "Rama Yade": "yade",
  "Jean-Marie Bigard": "bigard",
  "Henri Guaino": "guaino",
  "Michèle Alliot-Marie": "alliot_marie",
  "Hervé Morin": "morin",
  "Cyril Hanouna": "hanouna",
  "Anasse Kazib": "kazib",
  "Hélène Thouy": "thouy",
  "Clara Egger": "egger",
  "Antoine Waechter": "waechter",
  "Alexandre Langlois": "langlois",
  "Georges Kuzmanovic": "kuzmanovic",
  "Gaspard Koenig": "koenig",
  "Jean-Christophe Fromantin": "fromantin",
  "Corinne Lepage": "lepage",
  "Charlotte Marchandise": "marchandise",
  "Michel Onfray": "onfray",
  "Didier Raoult": "raoult",
  "Jean-Pierre Chevènement": "chevenement",
  "Pierre Larrouturou": "larrouturou",
  "Sylvia Pinel": "pinel",
  "Olivier Besancenot": "besancenot",
  "Eva Joly": "joly",
  "Martine Aubry": "aubry",
  "Philippe de Villiers": "de_villiers",
  "Bruno Retailleau": "retailleau",
  // 2002–2012
  "Arlette Laguiller": "laguiller",
  "Daniel Gluckstein": "gluckstein",
  "Gérard Schivardi": "schivardi",
  "Robert Hue": "hue",
  "Marie-George Buffet": "buffet",
  "Clémentine Autain": "autain",
  "Patrick Braouezec": "braouezec",
  "José Bové": "bove",
  "Lionel Jospin": "jospin",
  "Laurent Fabius": "fabius",
  "Dominique Strauss-Kahn": "strauss_kahn",
  "Jack Lang (French politician)": "lang",
  "Henri Emmanuelli": "emmanuelli",
  "Bertrand Delanoë": "delanoe",
  "Noël Mamère": "mamere",
  "Alain Lipietz": "lipietz",
  "Dominique Voynet": "voynet",
  "Brice Lalonde": "lalonde",
  "Jean Saint-Josse": "saint_josse",
  "Frédéric Nihous": "nihous",
  "Jacques Chirac": "chirac",
  "Alain Madelin": "madelin",
  "Philippe Séguin": "seguin",
  "François Léotard": "leotard",
  "Édouard Balladur": "balladur",
  "Charles Pasqua": "pasqua",
  "Dominique de Villepin": "villepin",
  "Jean-Louis Borloo": "borloo",
  "Nicolas Miguet": "miguet",
  "Bernard Tapie": "tapie",
  "Dieudonné (comedian)": "dieudonne",
  "Dieudonné M'bala M'bala": "dieudonne",
  "Myriam Martin": "martin",
  "Bruno Mégret": "megret",
  "Jean-Marie Le Pen": "jm_le_pen",
}

// A column for a party's candidate before the party named one (header "(nominee)" and the party's
// link), read as the candidate the party ran, by year.
const NOMINEES: Record<number, Record<string, string>> = {
  2012: { "New Anticapitalist Party": "poutou" },
}

// Polls the page itself says were not published before the vote (firm, fieldwork start and end).
const UNPUBLISHED = new Set(["kantar|2007-04-20|2007-04-21"])

const FIRMS: [RegExp, string][] = [
  [/ifop/i, "ifop"],
  [/elabe/i, "elabe"],
  [/lh2|louis harris/i, "lh2"],
  [/odoxa/i, "odoxa"],
  [/opinion\s*way/i, "opinionway"],
  [/ipsos/i, "ipsos"],
  [/harris/i, "harris-interactive"],
  [/bva/i, "bva"],
  [/kantar|tns|sofres/i, "kantar"],
  [/yougov/i, "yougov"],
  [/cluster\s*17/i, "cluster17"],
  [/viavoice/i, "viavoice"],
  [/cevipof/i, "ipsos"],
  [/csa/i, "csa"],
  [/futuribles/i, "futuribles"],
  [/montaigne/i, "montaigne"],
  [/dentsu/i, "odoxa"],
  [/le terrain|scan research/i, "le-terrain"],
  [/atlas\s*intel/i, "atlasintel"],
  [/future thinking/i, "future-thinking"],
]

interface Cell {
  header: boolean
  attrs: string
  text: string
  rowspan: number
  colspan: number
}

interface GridCell {
  cell: Cell
  id: number
}

interface Row {
  poll_id: string
  firm: string
  sponsor: string
  field_start: string
  field_end: string
  published_at: string
  sample_size: number
  method: string
  population: string
  round: number
  scenario_id: string
  candidate_id: string
  share: number
  source_url: string
  notice_url: string
  index_url: string
  entry: string
  retrieved_at: string
  content_hash: string
}

interface Summary {
  tables: number
  polls: number
  scenarios: number
  skipped: Record<string, number>
  unknownHeaders: Set<string>
  firms: Map<string, number>
}

const MONTHS: Record<string, number> = {
  jan: 1,
  feb: 2,
  mar: 3,
  apr: 4,
  may: 5,
  jun: 6,
  jul: 7,
  aug: 8,
  sep: 9,
  oct: 10,
  nov: 11,
  dec: 12,
}

function skip(summary: Summary, reason: string): void {
  summary.skipped[reason] = (summary.skipped[reason] ?? 0) + 1
}

/** Splits `text` on `separator` where it is not inside [[...]], {{...}} or [...]. */
function splitTopLevel(text: string, separator: string): string[] {
  const parts: string[] = []
  let depthLink = 0
  let depthTemplate = 0
  let start = 0
  for (let i = 0; i < text.length; i++) {
    if (text.startsWith("[[", i)) {
      depthLink++
      i++
    } else if (text.startsWith("]]", i) && depthLink > 0) {
      depthLink--
      i++
    } else if (text.startsWith("{{", i)) {
      depthTemplate++
      i++
    } else if (text.startsWith("}}", i) && depthTemplate > 0) {
      depthTemplate--
      i++
    } else if (depthLink === 0 && depthTemplate === 0 && text.startsWith(separator, i)) {
      parts.push(text.slice(start, i))
      start = i + separator.length
      i += separator.length - 1
    }
  }
  parts.push(text.slice(start))
  return parts
}

function parseCell(raw: string, header: boolean): Cell {
  const parts = splitTopLevel(raw, "|")
  let attrs = ""
  let text = raw
  if (parts.length > 1 && /^\s*([a-z-]+\s*=|$)/i.test(parts[0]) && !parts[0].includes("[")) {
    attrs = parts[0]
    text = parts.slice(1).join("|")
  }
  const span = (name: string) => {
    const match = attrs.match(new RegExp(`${name}\\s*=\\s*"?(\\d+)`, "i"))
    return match ? Number(match[1]) : 1
  }
  return { header, attrs, text: text.trim(), rowspan: span("rowspan"), colspan: span("colspan") }
}

/** The rows of one `{| ... |}` table, each a list of cells. */
function parseTable(body: string): Cell[][] {
  const rows: Cell[][] = []
  let current: Cell[] = []
  let pending: { raw: string; header: boolean } | null = null
  const flush = () => {
    if (pending) {
      current.push(parseCell(pending.raw, pending.header))
      pending = null
    }
  }
  for (const line of body.split("\n")) {
    if (line.startsWith("|-")) {
      flush()
      if (current.length > 0) rows.push(current)
      current = []
    } else if (line.startsWith("|+")) {
      flush()
    } else if (line.startsWith("!")) {
      flush()
      const cells = splitTopLevel(line.slice(1), "!!")
      for (const raw of cells.slice(0, -1)) current.push(parseCell(raw, true))
      pending = { raw: cells[cells.length - 1], header: true }
    } else if (line.startsWith("|")) {
      flush()
      const cells = splitTopLevel(line.slice(1), "||")
      for (const raw of cells.slice(0, -1)) current.push(parseCell(raw, false))
      pending = { raw: cells[cells.length - 1], header: false }
    } else if (pending) {
      pending.raw += `\n${line}`
    }
  }
  flush()
  if (current.length > 0) rows.push(current)
  return rows
}

/** Lays the rows out on a grid, repeating row- and column-spanning cells. */
function toGrid(rows: Cell[][]): GridCell[][] {
  const grid: GridCell[][] = []
  const carry: { cell: GridCell; left: number }[] = []
  let id = 0
  for (const row of rows) {
    const out: GridCell[] = []
    let column = 0
    const fillCarry = () => {
      while (carry[column] && carry[column].left > 0) {
        out[column] = carry[column].cell
        carry[column].left--
        column++
      }
    }
    for (const cell of row) {
      fillCarry()
      const placed = { cell, id: id++ }
      for (let k = 0; k < cell.colspan; k++) {
        out[column] = placed
        carry[column] = { cell: placed, left: cell.rowspan - 1 }
        column++
      }
    }
    fillCarry()
    grid.push(out)
  }
  return grid
}

function stripMarkup(text: string): string {
  return text
    .replace(/<!--[\s\S]*?-->/g, "")
    .replace(/<ref[^>]*\/>/g, "")
    .replace(/<ref[^>]*>[\s\S]*?<\/ref>/g, "")
    .replace(/\{\{(efn|refn|Webarchive|cn|citation needed|sfn|r)\b[^{}]*\}\}/gi, "")
    .replace(/<[^>]+>/g, " ")
    .replace(/'''?/g, "")
    .trim()
}

function linkTargets(text: string): string[] {
  return [...text.matchAll(/\[\[([^\]|]+)(\|[^\]]*)?\]\]/g)].map((m) => m[1].trim())
}

function candidateOf(header: string): string | null {
  for (const target of linkTargets(header)) {
    if (target.startsWith("File:")) continue
    if (CANDIDATES[target]) return CANDIDATES[target]
  }
  const vert = header.match(/vert header\|[^[]*\[\[([^\]|]+)/)
  if (vert && CANDIDATES[vert[1]]) return CANDIDATES[vert[1]]
  return null
}

function nomineeOf(year: number, header: string): string | null {
  if (!/\(nominee\)/.test(header)) return null
  const party = linkTargets(header).find((target) => NOMINEES[year]?.[target])
  return party ? NOMINEES[year][party] : null
}

function parseShare(text: string): number | null | "absent" {
  const clean = stripMarkup(text)
    .replace(/\{\{[^{}]*\|([^{}|]*)\}\}/g, "$1")
    .trim()
  if (clean === "" || /^[–—-]+$/.test(clean) || /^n\/a$/i.test(clean)) return "absent"
  if (/^<\s*(0[.,]5|1)\s*%?$/.test(clean)) return 0.5
  const match = clean.match(/^(\d+(?:[.,]\d+)?)\s*%?$/)
  return match ? Number(match[1].replace(",", ".")) : null
}

function parseSample(text: string): number | null {
  const clean = stripMarkup(text).replace(/\{\{formatnum:([^}]*)\}\}/, "$1")
  const match = clean.match(/(\d{1,3}(?:[,\s]\d{3})+|\d+)/)
  return match ? Number(match[1].replace(/[,\s]/g, "")) : null
}

function iso(year: number, month: number, day: number): string {
  return `${year}-${String(month).padStart(2, "0")}-${String(day).padStart(2, "0")}`
}

/** "16–17 Nov 2016", "30 Nov–2 Dec 2016", "28 Dec 2016–2 Jan 2017", "21 Apr 2017". */
function parseDates(text: string): [string, string] | null {
  const clean = stripMarkup(text).replace(/&nbsp;/g, " ").replace(/\s+/g, " ")
  const parts = clean.split(/\s*[–—-]\s*/)
  const parse = (part: string) => {
    const match = part.match(/^(\d{1,2})(?:\s+([A-Za-z]{3})[a-z]*)?(?:\s+(\d{4}))?$/)
    if (!match) return null
    return { day: Number(match[1]), month: match[2] ? MONTHS[match[2].toLowerCase()] : undefined, year: match[3] ? Number(match[3]) : undefined }
  }
  const end = parse(parts[parts.length - 1])
  const start = parse(parts[0])
  if (!end || !start || end.month === undefined || end.year === undefined || parts.length > 2) return null
  const startMonth = start.month ?? end.month
  const startYear = start.year ?? (startMonth > end.month ? end.year - 1 : end.year)
  return [iso(startYear, startMonth, start.day), iso(end.year, end.month, end.day)]
}

function nextDay(date: string): string {
  const d = new Date(`${date}T00:00:00Z`)
  d.setUTCDate(d.getUTCDate() + 1)
  return d.toISOString().slice(0, 10)
}

function firmOf(text: string): { firm: string; url: string } | null {
  const url = text.match(/\[(https?:\/\/[^\s\]]+)/)?.[1] ?? ""
  const label = stripMarkup(text)
    .replace(/\[https?:\/\/\S+\s*([^\]]*)\]/g, "$1")
    .replace(/\[\[([^\]|]+\|)?([^\]]+)\]\]/g, "$2")
  for (const [pattern, firm] of FIRMS) {
    if (pattern.test(label)) return { firm, url }
  }
  return null
}

interface Section {
  heading: string
  round: 1 | 2
  body: string
}

/** The first- and second-round sections that hold national voting-intention tables. */
function sections(wikitext: string): Section[] {
  const out: Section[] = []
  let round: 1 | 2 | null = null
  let heading = ""
  let skipping = false
  let body: string[] = []
  const push = () => {
    if (round && !skipping && body.length > 0) out.push({ heading, round, body: body.join("\n") })
    body = []
  }
  for (const line of wikitext.split("\n")) {
    const match = line.match(/^(=+)\s*(.*?)\s*\1\s*$/)
    if (!match) {
      body.push(line)
      continue
    }
    push()
    heading = match[2]
    if (match[1].length === 2) {
      round = /^first round$/i.test(heading) ? 1 : /^second round$/i.test(heading) ? 2 : null
    }
    // Sub-national tables and vote-transfer tables are not national voting intentions.
    skipping = /by (region|department|commune|constituency|first round vote)/i.test(heading)
  }
  push()
  return out
}

function tables(body: string): string[] {
  const out: string[] = []
  let depth = 0
  let current: string[] = []
  for (const line of body.split("\n")) {
    if (line.trimStart().startsWith("{|")) {
      depth++
      if (depth === 1) {
        current = []
        continue
      }
    }
    if (line.trimStart().startsWith("|}")) {
      depth--
      if (depth === 0) {
        out.push(current.join("\n"))
        continue
      }
    }
    if (depth > 0) current.push(line)
  }
  return out
}

function extract(page: Page, wikitext: string, retrievedAt: string, summary: Summary): Row[] {
  const hash = `sha256:${createHash("sha256").update(wikitext).digest("hex")}`
  const indexUrl = `https://en.wikipedia.org/w/index.php?title=${page.title}&oldid=${page.oldid}`
  const rows: Row[] = []
  const ids = new Map<string, number>()
  const pollIds = new Map<string, string>()
  const scenarioCounts = new Map<string, number>()
  for (const section of sections(wikitext)) {
    for (const table of tables(section.body)) {
      const grid = toGrid(parseTable(table))
      const headerRows = grid.filter((row) => row.every((c) => c.cell.header))
      const width = Math.max(...grid.map((r) => r.length))
      const columns: { kind: string; candidate?: string }[] = []
      for (let column = 0; column < width; column++) {
        const text = headerRows.map((r) => r[column]?.cell.text ?? "").join(" ")
        const candidate = candidateOf(text) ?? nomineeOf(page.year, text)
        const plain = stripMarkup(text).toLowerCase()
        if (candidate) columns.push({ kind: "candidate", candidate })
        else if (/polling|firm|sondeur/.test(plain)) columns.push({ kind: "firm" })
        else if (/fieldwork|date/.test(plain)) columns.push({ kind: "date" })
        else if (/sample/.test(plain)) columns.push({ kind: "sample" })
        else {
          columns.push({ kind: "other" })
          if (linkTargets(text).some((t) => !t.startsWith("File:"))) summary.unknownHeaders.add(linkTargets(text).join(", "))
        }
      }
      const firmColumn = columns.findIndex((c) => c.kind === "firm")
      const dateColumn = columns.findIndex((c) => c.kind === "date")
      const sampleColumn = columns.findIndex((c) => c.kind === "sample")
      if (firmColumn < 0 || dateColumn < 0 || !columns.some((c) => c.kind === "candidate")) continue
      summary.tables++
      // Group data rows into polls by the firm cell they share.
      const polls = new Map<number, GridCell[][]>()
      for (const row of grid) {
        if (row.every((c) => c.cell.header)) continue
        const firmCell = row[firmColumn]
        if (!firmCell || firmCell.cell.colspan > 1) continue
        if (/election|résultats|results/i.test(firmCell.cell.text) && !firmOf(firmCell.cell.text)) continue
        const list = polls.get(firmCell.id) ?? []
        list.push(row)
        polls.set(firmCell.id, list)
      }
      for (const pollRows of polls.values()) {
        const first = pollRows[0]
        const firm = firmOf(first[firmColumn].cell.text)
        if (/Dedicated Research/.test(first[firmColumn].cell.text)) {
          // Belgian pollster; the page marks it as not subject to French regulations.
          skip(summary, "Dedicated Research (not subject to French regulations)")
          continue
        }
        if (!firm) {
          skip(summary, `unknown firm: ${stripMarkup(first[firmColumn].cell.text).slice(0, 40)}`)
          continue
        }
        const dates = parseDates(first[dateColumn].cell.text)
        if (!dates) {
          skip(summary, "unparsed fieldwork date")
          continue
        }
        if (UNPUBLISHED.has(`${firm.firm}|${dates[0]}|${dates[1]}`)) {
          skip(summary, "not published before the vote (the page says so)")
          continue
        }
        const sample = sampleColumn >= 0 ? parseSample(first[sampleColumn].cell.text) : null
        if (!sample || sample < 100) {
          skip(summary, "no sample size")
          continue
        }
        // The same poll appears in several tables (round 1, then one per runoff matchup): rows
        // with the same firm, fieldwork and sample size are one poll.
        const key = `${firm.firm}|${dates[0]}|${dates[1]}|${sample}`
        let pollId = pollIds.get(key)
        if (!pollId) {
          const base = `${firm.firm}-${dates[1]}`
          const n = (ids.get(base) ?? 0) + 1
          ids.set(base, n)
          pollId = n === 1 ? base : `${base}-${n}`
          pollIds.set(key, pollId)
        }
        let added = 0
        for (const row of pollRows) {
          const shares: [string, number][] = []
          let broken = false
          let spanned = false
          columns.forEach((column, i) => {
            if (column.kind !== "candidate" || !row[i]) return
            // A cell spanning several candidate columns is a note, not a share.
            if (row[i].cell.colspan > 1) {
              spanned = true
              return
            }
            const share = parseShare(row[i].cell.text)
            if (share === null) broken = true
            else if (share !== "absent") shares.push([column.candidate!, share])
          })
          const total = shares.reduce((sum, [, s]) => sum + s, 0)
          const unique = new Set(shares.map(([c]) => c)).size === shares.length
          if (spanned) {
            skip(summary, "one share covers two candidates")
            continue
          }
          if (broken || !unique || shares.length < 2) {
            skip(summary, "unparsed scenario row")
            continue
          }
          if (section.round === 1 ? total < 85 || total > 103 : shares.length !== 2 || total < 98 || total > 102) {
            skip(summary, `round ${section.round} shares outside the accepted sum`)
            continue
          }
          const counter = `${pollId}|${section.round}`
          const scenarioNumber = (scenarioCounts.get(counter) ?? 0) + 1
          scenarioCounts.set(counter, scenarioNumber)
          added++
          for (const [candidate, share] of shares) {
            rows.push({
              poll_id: pollId,
              firm: firm.firm,
              sponsor: "",
              field_start: dates[0],
              field_end: dates[1],
              published_at: nextDay(dates[1]),
              sample_size: sample,
              method: "",
              population: "",
              round: section.round,
              scenario_id: `${section.round === 1 ? "S" : "H"}${scenarioNumber}`,
              candidate_id: candidate,
              share,
              source_url: firm.url || indexUrl,
              notice_url: "",
              index_url: indexUrl,
              entry: "index",
              retrieved_at: retrievedAt,
              content_hash: hash,
            })
          }
        }
        summary.scenarios += added
      }
    }
  }
  const polls = new Map(rows.map((row) => [row.poll_id, row.firm]))
  summary.polls = polls.size
  for (const firm of polls.values()) summary.firms.set(firm, (summary.firms.get(firm) ?? 0) + 1)
  return rows
}

const COLUMNS: (keyof Row)[] = [
  "poll_id",
  "firm",
  "sponsor",
  "field_start",
  "field_end",
  "published_at",
  "sample_size",
  "method",
  "population",
  "round",
  "scenario_id",
  "candidate_id",
  "share",
  "source_url",
  "notice_url",
  "index_url",
  "entry",
  "retrieved_at",
  "content_hash",
]

function csvField(value: string | number): string {
  const text = String(value)
  return /[",\n]/.test(text) ? `"${text.replace(/"/g, '""')}"` : text
}

async function wikitextOf(page: Page, localDir: string | undefined): Promise<{ text: string; retrievedAt: string }> {
  if (localDir) {
    return {
      text: readFileSync(join(localDir, `en-${page.oldid}.wikitext`), "utf8"),
      retrievedAt: process.env.RETRIEVED_AT ?? new Date().toISOString().slice(0, 16) + "Z",
    }
  }
  const url = `https://en.wikipedia.org/w/index.php?oldid=${page.oldid}&action=raw`
  const response = await fetch(url, { headers: { "user-agent": "fr2027-forecaster backtest extraction" } })
  if (!response.ok) throw new Error(`${url}: HTTP ${response.status}`)
  return { text: await response.text(), retrievedAt: new Date().toISOString().slice(0, 16) + "Z" }
}

/** Parses a CSV file with a header row (quoted fields may hold commas and doubled quotes). */
function readCsv(path: string): Record<string, string>[] {
  const records: string[][] = []
  let field = ""
  let record: string[] = []
  let quoted = false
  const text = readFileSync(path, "utf8")
  for (let i = 0; i < text.length; i++) {
    const ch = text[i]
    if (quoted) {
      if (ch === '"' && text[i + 1] === '"') {
        field += '"'
        i++
      } else if (ch === '"') quoted = false
      else field += ch
    } else if (ch === '"') quoted = true
    else if (ch === ",") {
      record.push(field)
      field = ""
    } else if (ch === "\n") {
      record.push(field)
      records.push(record)
      record = []
      field = ""
    } else field += ch
  }
  if (field !== "" || record.length > 0) records.push([...record, field])
  const [header, ...rest] = records
  return rest.map((values) => Object.fromEntries(header.map((name, i) => [name, values[i] ?? ""])))
}

/**
 * Applies the primary-source checks (research/sources/README.md, "Backtest polls"):
 * data/backtest/corrections.csv replaces a share with the one the pollster published (an empty
 * primary_share removes a candidate the source did not test), and data/backtest/primary-checks.csv
 * marks the polls that now match their primary source as `entry: primary`, with the document
 * read as `source_url`. A correction whose index value is not the extracted one stops the run.
 */
function applyChecks(year: number, rows: Row[]): Row[] {
  const corrections = readCsv(join("data", "backtest", "corrections.csv")).filter((c) => Number(c.year) === year)
  for (const c of corrections) {
    const row = rows.find(
      (r) =>
        r.poll_id === c.poll_id &&
        String(r.round) === c.round &&
        r.scenario_id === c.scenario_id &&
        r.candidate_id === c.candidate_id,
    )
    if (!row || row.share !== Number(c.index_share)) {
      throw new Error(`correction ${c.poll_id} ${c.scenario_id} ${c.candidate_id}: index value not found`)
    }
    if (c.primary_share === "") row.share = Number.NaN
    else row.share = Number(c.primary_share)
  }
  const checks = readCsv(join("data", "backtest", "primary-checks.csv")).filter(
    (c) => Number(c.year) === year && c.entry_after === "primary",
  )
  for (const check of checks) {
    const pollRows = rows.filter((r) => r.poll_id === check.poll_id)
    if (pollRows.length === 0) throw new Error(`checked poll ${check.poll_id} not found`)
    for (const row of pollRows) {
      row.entry = "primary"
      row.source_url = check.primary_url
      row.retrieved_at = check.retrieved_at
    }
  }
  return rows.filter((r) => !Number.isNaN(r.share))
}

const [localDir, ...years] = process.argv.slice(2)
for (const page of PAGES.filter((p) => years.length === 0 || years.includes(String(p.year)))) {
  const { text, retrievedAt } = await wikitextOf(page, localDir)
  const summary: Summary = { tables: 0, polls: 0, scenarios: 0, skipped: {}, unknownHeaders: new Set(), firms: new Map() }
  const rows = applyChecks(page.year, extract(page, text, retrievedAt, summary))
  rows.sort((a, b) =>
    a.published_at !== b.published_at
      ? a.published_at.localeCompare(b.published_at)
      : a.poll_id !== b.poll_id
        ? a.poll_id.localeCompare(b.poll_id)
        : a.round - b.round || a.scenario_id.localeCompare(b.scenario_id) || a.candidate_id.localeCompare(b.candidate_id),
  )
  const dir = join("data", "backtest", String(page.year))
  mkdirSync(dir, { recursive: true })
  const lines = [COLUMNS.join(","), ...rows.map((row) => COLUMNS.map((c) => csvField(row[c])).join(","))]
  writeFileSync(join(dir, "polls.csv"), `${lines.join("\n")}\n`)
  console.log(
    `${page.year}: ${summary.tables} tables, ${summary.polls} polls, ${summary.scenarios} scenarios, ${rows.length} rows`,
  )
  console.log(`  firms: ${[...summary.firms].map(([f, n]) => `${f} ${n}`).join(", ")}`)
  for (const [reason, count] of Object.entries(summary.skipped)) console.log(`  skipped ${count}: ${reason}`)
  for (const header of summary.unknownHeaders) console.log(`  unmatched header: ${header}`)
}
