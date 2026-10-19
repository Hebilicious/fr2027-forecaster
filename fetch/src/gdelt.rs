//! Daily counts of French-language articles naming each candidate, from GDELT's raw Global
//! Knowledge Graph files: the "translation" stream, every non-English article GDELT reads, one file
//! per 15 minutes since February 2015, about 7 MB zipped each.
//! `GET https://data.gdeltproject.org/gdeltv2/<YYYYMMDDHHMMSS>.translation.gkg.csv.zip`.
//!
//! GDELT's DOC API answers the same question in one request per candidate, but it refuses most
//! requests (HTTP 429) from shared addresses, a home connection behind a carrier's NAT and CI
//! runners alike; the files are static downloads with no limit.
//!
//! Every file is read: the four of each hour, so a year is about 35,000 files, each downloaded,
//! read for its French records and dropped (nothing is kept on disk). French articles are those
//! whose translation note reads `srclc:fra`. An article counts for a candidate when its headline
//! names them, by the rule RSS headlines are matched with (`collectors/src/names.rs`): its title, or
//! the slug of its address (the longest part of the URL path, read without regard to case), since
//! the files hold no titles before 2018 but every article's address. GDELT's own list of the people
//! an article names is not used: it is read from GDELT's English machine translation, which writes
//! Mélenchon as `luc mlenchon` and Le Drian as `mondrian`, and never lists Marine Le Pen or
//! Emmanuel Macron at all.
//!
//! Each article also carries GDELT's tone (V2Tone): the share of positive words minus the share of
//! negative words, in percent, scored on its English translation. A candidate's day keeps the mean
//! tone of the articles naming them, and the mean tone of every French article that day, so the
//! candidate's coverage can be read against the day's mood.

use std::collections::BTreeMap;
use std::io::{Cursor, Read};
use std::sync::Mutex;

use fr2027_collectors::{
    gdelt::{ArticleRecord, DayCount, DayStance, Reached},
    names::NameMatcher,
    sources::SourcesConfig,
};
use jiff::{ToSpan, civil::Date};

use crate::{Client, Error, Result, article, typesafe::Jev};

const FILES: &str = "https://data.gdeltproject.org/gdeltv2";
/// How long one file may take to download.
pub const TIMEOUT: std::time::Duration = std::time::Duration::from_secs(120);
/// Files read at once.
pub const WORKERS: usize = 16;
/// The first day the translation stream covers.
pub const FIRST_DAY: Date = jiff::civil::date(2015, 2, 19);

/// Columns of an article's address, its tone, its translation note, and the extras holding its
/// title (0-based, GKG 2.1).
const DATE: usize = 1;
const URL: usize = 4;
const TONE: usize = 15;
const TRANSLATION: usize = 25;
const EXTRAS: usize = 26;
/// The minutes of each hour GDELT publishes a file at.
const MINUTES: [u8; 4] = [0, 15, 30, 45];

/// A candidate the counts are kept for, with the full name Jev is asked about.
#[derive(Clone, Debug)]
pub struct Person {
    pub candidate_id: String,
    pub full_name: String,
}

impl Person {
    /// How the candidate is matched, for the record: `names` as in config/sources.yaml.
    pub fn description(&self, names: &[String]) -> String {
        format!(
            "GDELT GKG translation stream, srclc:fra, every file; title or URL slug: {}",
            names.join(" | ")
        )
    }
}

/// Recognises the candidates in an article's headline.
pub struct Matcher {
    people: Vec<Person>,
    names: NameMatcher,
}

impl Matcher {
    /// `candidates`: each candidate's id and names, as in config/sources.yaml.
    pub fn new(candidates: &BTreeMap<String, fr2027_collectors::sources::CandidateSources>) -> Self {
        let people = candidates
            .iter()
            .map(|(id, c)| Person {
                candidate_id: id.clone(),
                full_name: c.names.first().cloned().unwrap_or_else(|| id.clone()),
            })
            .collect();
        let sources = SourcesConfig {
            candidates: candidates.clone(),
            ..SourcesConfig::default()
        };
        Matcher {
            people,
            names: NameMatcher::new(&sources),
        }
    }

    pub fn people(&self) -> &[Person] {
        &self.people
    }

    /// Whether an article's text names the candidate, in any case.
    fn names_in_text(&self, text: &str, id: &str) -> bool {
        self.names
            .find(text)
            .iter()
            .chain(&self.names.find_any_case(text))
            .any(|found| found == id)
    }

    /// The candidates an article's title or address names.
    fn named(&self, title: Option<&str>, url: &str) -> Vec<String> {
        let mut ids = title.map(|t| self.names.find(t)).unwrap_or_default();
        for id in self.names.find_any_case(&slug(url)) {
            if !ids.contains(&id) {
                ids.push(id);
            }
        }
        ids
    }
}

/// What one file holds for the candidates: its French articles and their summed tone, and per
/// candidate (in `matcher.people()` order) the articles naming them and those articles' summed tone.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FileCounts {
    pub french: u64,
    pub french_tone: f64,
    pub counts: Vec<u64>,
    pub tone: Vec<f64>,
    /// The articles whose headline names a candidate.
    pub matched: Vec<Matched>,
}

/// An article whose headline names at least one candidate.
#[derive(Clone, Debug, PartialEq)]
pub struct Matched {
    /// The record's DATE, as an ISO timestamp.
    pub seen_at: String,
    pub url: String,
    pub title: Option<String>,
    pub candidate_ids: Vec<String>,
    pub tone: f64,
}

pub fn count_file(zipped: &[u8], matcher: &Matcher) -> Result<FileCounts> {
    let mut archive = zip::ZipArchive::new(Cursor::new(zipped)).map_err(|e| Error::parse("gdelt gkg", e))?;
    let mut text = Vec::new();
    archive
        .by_index(0)
        .map_err(|e| Error::parse("gdelt gkg", e))?
        .read_to_end(&mut text)
        .map_err(|e| Error::parse("gdelt gkg", e))?;
    Ok(count_lines(&text, matcher))
}

fn count_lines(text: &[u8], matcher: &Matcher) -> FileCounts {
    let people = &matcher.people;
    let mut file = FileCounts {
        counts: vec![0; people.len()],
        tone: vec![0.0; people.len()],
        ..FileCounts::default()
    };
    for line in text.split(|b| *b == b'\n') {
        let fields: Vec<&[u8]> = line.split(|b| *b == b'\t').collect();
        let Some(note) = fields.get(TRANSLATION) else { continue };
        if !note.starts_with(b"srclc:fra") {
            continue;
        }
        let tone = fields.get(TONE).map_or(0.0, |t| article_tone(t));
        file.french += 1;
        file.french_tone += tone;
        let url = String::from_utf8_lossy(fields.get(URL).copied().unwrap_or_default());
        let extras = String::from_utf8_lossy(fields.get(EXTRAS).copied().unwrap_or_default());
        let title = title(&extras);
        let named = matcher.named(title.as_deref(), &url);
        for (i, person) in people.iter().enumerate() {
            if named.contains(&person.candidate_id) {
                file.counts[i] += 1;
                file.tone[i] += tone;
            }
        }
        if !named.is_empty() {
            file.matched.push(Matched {
                seen_at: seen_at(fields.get(DATE).copied().unwrap_or_default()),
                url: url.into_owned(),
                title,
                candidate_ids: named,
                tone,
            });
        }
    }
    file
}

/// A record's DATE (`20261016121500`) as `2026-10-16T12:15:00Z`; empty when malformed.
fn seen_at(field: &[u8]) -> String {
    let d = String::from_utf8_lossy(field);
    if d.len() != 14 || !d.bytes().all(|b| b.is_ascii_digit()) {
        return String::new();
    }
    format!(
        "{}-{}-{}T{}:{}:{}Z",
        &d[0..4],
        &d[4..6],
        &d[6..8],
        &d[8..10],
        &d[10..12],
        &d[12..14]
    )
}

/// The first number of V2Tone, the article's tone; 0 when the field is empty or unreadable.
fn article_tone(field: &[u8]) -> f64 {
    let first = field.split(|b| *b == b',').next().unwrap_or_default();
    std::str::from_utf8(first)
        .ok()
        .and_then(|t| t.trim().parse::<f64>().ok())
        .filter(|t| t.is_finite())
        .unwrap_or(0.0)
}

/// An article's slug as words: the longest part of its URL's path, percent-decoded, with hyphens,
/// underscores and dots as spaces (`…/politique/marine-le-pen/nous-ne-pouvons-plus…` reads as
/// `nous ne pouvons plus…`, not the section it is filed under).
fn slug(url: &str) -> String {
    let after_scheme = url.split_once("://").map_or(url, |(_, rest)| rest);
    let path = after_scheme.split_once('/').map_or("", |(_, path)| path);
    let path = path.split(['?', '#']).next().unwrap_or_default();
    let longest = path.split('/').max_by_key(|segment| segment.len()).unwrap_or_default();
    percent_decode(longest)
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn percent_decode(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        let hex = |b: u8| (b as char).to_digit(16);
        if bytes[i] == b'%'
            && let (Some(hi), Some(lo)) = (
                bytes.get(i + 1).copied().and_then(hex),
                bytes.get(i + 2).copied().and_then(hex),
            )
        {
            out.push((hi * 16 + lo) as u8);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// The page title in a record's extras, with its HTML entities decoded.
fn title(extras: &str) -> Option<String> {
    let start = extras.find("<PAGE_TITLE>")? + "<PAGE_TITLE>".len();
    let end = extras[start..]
        .find("</PAGE_TITLE>")
        .map_or(extras.len(), |e| start + e);
    Some(decode_entities(&extras[start..end]))
}

fn decode_entities(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find('&') {
        out.push_str(&rest[..at]);
        rest = &rest[at..];
        let Some(end) = rest.find(';').filter(|e| *e <= 10) else {
            out.push('&');
            rest = &rest[1..];
            continue;
        };
        let entity = &rest[1..end];
        let decoded = match entity {
            "amp" => Some('&'),
            "quot" => Some('"'),
            "apos" => Some('\''),
            "lt" => Some('<'),
            "gt" => Some('>'),
            "nbsp" => Some('\u{a0}'),
            _ => entity
                .strip_prefix("#x")
                .or_else(|| entity.strip_prefix("#X"))
                .and_then(|h| u32::from_str_radix(h, 16).ok())
                .or_else(|| entity.strip_prefix('#').and_then(|d| d.parse().ok()))
                .and_then(char::from_u32),
        };
        match decoded {
            Some(c) => {
                out.push(c);
                rest = &rest[end + 1..];
            }
            None => {
                out.push('&');
                rest = &rest[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

/// One day's counts, summed over the files read.
#[derive(Clone, Debug, Default)]
struct Day {
    french: u64,
    french_tone: f64,
    counts: Vec<u64>,
    tone: Vec<f64>,
    files: u32,
    /// The day's matched articles, once each.
    matched: BTreeMap<String, Matched>,
}

/// How to read the matched articles: the client their pages are fetched with, Jev, and how many to
/// read at once.
pub struct Reading<'a> {
    pub pages: &'a Client,
    pub jev: &'a Jev,
    pub workers: usize,
}

/// What [`count_days`] found: each candidate's days, and, when the articles were read, every
/// matched article with how it was read.
#[derive(Debug, Default)]
pub struct Counted {
    pub candidates: Vec<(String, Vec<DayCount>)>,
    pub articles: Vec<ArticleRecord>,
}

/// Each person's daily count of French articles naming them and their mean tone, from `start` to
/// `end` (both included, none before [`FIRST_DAY`]), reading every file of those days, `workers` at
/// a time. `total_articles` and `total_tone` are the day's French articles and their mean tone;
/// `files` is how many files the day had (GDELT's stream has gaps: a file it never published is not
/// a failure, any other download error is). A day with no file at all is left out.
///
/// With `reading`, every article whose headline names a candidate is then read
/// ([`crate::article::read`]) and Jev judges how it portrays each candidate it names; the days
/// carry the sums ([`DayStance`]) and [`Counted::articles`] every article.
pub fn count_days(
    client: &Client,
    matcher: &Matcher,
    start: Date,
    end: Date,
    workers: usize,
    reading: Option<&Reading>,
    progress: &(dyn Fn(&str) + Sync),
) -> Result<Counted> {
    let mut files = Vec::new();
    let mut day = start.max(FIRST_DAY);
    while day <= end {
        for hour in 0..24u8 {
            for minute in MINUTES {
                files.push((day, hour, minute));
            }
        }
        day = day.tomorrow().map_err(|e| Error::parse("gdelt gkg", e))?;
    }
    let total = files.len();
    let people = matcher.people();
    let queue = Mutex::new(files.into_iter());
    let days: Mutex<BTreeMap<Date, Day>> = Mutex::new(BTreeMap::new());
    let failure: Mutex<Option<Error>> = Mutex::new(None);
    let done = Mutex::new(0usize);
    std::thread::scope(|scope| {
        for _ in 0..workers.max(1) {
            scope.spawn(|| {
                loop {
                    let Some((day, hour, minute)) = queue.lock().expect("queue").next() else {
                        break;
                    };
                    if failure.lock().expect("failure").is_some() {
                        break;
                    }
                    let url = format!(
                        "{FILES}/{}{hour:02}{minute:02}00.translation.gkg.csv.zip",
                        day.strftime("%Y%m%d")
                    );
                    let counted = match client.get_bytes(&url, None) {
                        Ok(bytes) => count_file(&bytes, matcher).map(Some),
                        // GDELT's stream has gaps; a missing file is not a failure.
                        Err(error) if error.status() == Some(404) => Ok(None),
                        Err(error) => Err(error),
                    };
                    match counted {
                        Ok(Some(file)) => {
                            let mut days = days.lock().expect("days");
                            let entry = days.entry(day).or_insert_with(|| Day {
                                counts: vec![0; people.len()],
                                tone: vec![0.0; people.len()],
                                ..Day::default()
                            });
                            entry.french += file.french;
                            entry.french_tone += file.french_tone;
                            entry.files += 1;
                            for (sum, c) in entry.counts.iter_mut().zip(file.counts) {
                                *sum += c;
                            }
                            for (sum, t) in entry.tone.iter_mut().zip(file.tone) {
                                *sum += t;
                            }
                            // An article several files list keeps its earliest listing; a record
                            // without a readable DATE takes its file's.
                            for mut article in file.matched {
                                if article.seen_at.is_empty() {
                                    article.seen_at = format!("{day}T{hour:02}:{minute:02}:00Z");
                                }
                                match entry.matched.get(&article.url) {
                                    Some(kept) if kept.seen_at <= article.seen_at => {}
                                    _ => {
                                        entry.matched.insert(article.url.clone(), article);
                                    }
                                }
                            }
                        }
                        Ok(None) => {}
                        Err(error) => {
                            *failure.lock().expect("failure") = Some(error);
                            break;
                        }
                    }
                    let mut done = done.lock().expect("done");
                    *done += 1;
                    if (*done).is_multiple_of(2000) {
                        progress(&format!("{done} of {total} files"));
                    }
                }
            });
        }
    });
    if let Some(error) = failure.into_inner().expect("failure") {
        return Err(error);
    }
    let days = days.into_inner().expect("days");
    let articles = match reading {
        Some(reading) => read_articles(matcher, &days, reading, progress)?,
        None => Vec::new(),
    };
    // Each candidate's day of read articles.
    let mut stances: BTreeMap<(String, String), DayStance> = BTreeMap::new();
    for article in &articles {
        for id in &article.candidate_ids {
            let entry = stances.entry((article.date.clone(), id.clone())).or_default();
            match article.stance.get(id) {
                None => entry.unreachable += 1,
                Some(stance) if !stance.is_about() => entry.not_about += 1,
                Some(stance) => {
                    let about = stance.favourable + stance.unfavourable + stance.neutral;
                    if about > 0.0 {
                        entry.read += 1;
                        entry.favourable += stance.favourable / about;
                        entry.unfavourable += stance.unfavourable / about;
                        entry.neutral += stance.neutral / about;
                    }
                }
            }
        }
    }
    let round = |x: f64| (x * 10_000.0).round() / 10_000.0;
    let candidates = matcher
        .people()
        .iter()
        .enumerate()
        .map(|(i, person)| {
            let counts = days
                .iter()
                .map(|(date, day)| DayCount {
                    date: date.to_string(),
                    articles: day.counts[i],
                    total_articles: day.french,
                    tone: mean(day.tone[i], day.counts[i]),
                    total_tone: mean(day.french_tone, day.french),
                    files: Some(day.files),
                    stance: reading.map(|_| {
                        let s = stances
                            .get(&(date.to_string(), person.candidate_id.clone()))
                            .copied()
                            .unwrap_or_default();
                        DayStance {
                            favourable: round(s.favourable),
                            unfavourable: round(s.unfavourable),
                            neutral: round(s.neutral),
                            ..s
                        }
                    }),
                })
                .collect();
            (person.candidate_id.clone(), counts)
        })
        .collect();
    Ok(Counted { candidates, articles })
}

/// Reads every matched article of `days`, `reading.workers` at a time, and asks Jev about the
/// candidates each one's headline names. An article whose text cannot be reached has no stance;
/// a Jev failure (after its retries) stops the count.
fn read_articles(
    matcher: &Matcher,
    days: &BTreeMap<Date, Day>,
    reading: &Reading,
    progress: &(dyn Fn(&str) + Sync),
) -> Result<Vec<ArticleRecord>> {
    let full_names: BTreeMap<&str, &str> = matcher
        .people()
        .iter()
        .map(|p| (p.candidate_id.as_str(), p.full_name.as_str()))
        .collect();
    let queue: Vec<(Date, &Matched)> = days
        .iter()
        .flat_map(|(date, day)| day.matched.values().map(move |m| (*date, m)))
        .collect();
    let total = queue.len();
    let queue = Mutex::new(queue.into_iter());
    let records: Mutex<Vec<ArticleRecord>> = Mutex::new(Vec::new());
    let failure: Mutex<Option<Error>> = Mutex::new(None);
    std::thread::scope(|scope| {
        for _ in 0..reading.workers.max(1) {
            scope.spawn(|| {
                loop {
                    if failure.lock().expect("failure").is_some() {
                        break;
                    }
                    let Some((date, matched)) = queue.lock().expect("queue").next() else {
                        break;
                    };
                    let names_a_candidate =
                        |text: &str| matched.candidate_ids.iter().any(|id| matcher.names_in_text(text, id));
                    let text = article::read(reading.pages, &matched.url, date, &names_a_candidate);
                    let mut record = ArticleRecord {
                        date: date.to_string(),
                        seen_at: matched.seen_at.clone(),
                        url: matched.url.clone(),
                        title: matched.title.clone().or(text.title.clone()),
                        candidate_ids: matched.candidate_ids.clone(),
                        tone: matched.tone,
                        reached: text.reached,
                        stance: BTreeMap::new(),
                    };
                    if text.reached != Reached::Unreachable {
                        let asked: Vec<(String, String)> = matched
                            .candidate_ids
                            .iter()
                            .map(|id| {
                                (
                                    id.clone(),
                                    full_names.get(id.as_str()).copied().unwrap_or(id).to_string(),
                                )
                            })
                            .collect();
                        match reading.jev.stance(record.title.as_deref(), &text.body, &asked) {
                            Ok(stance) => record.stance = stance,
                            Err(error) => {
                                *failure.lock().expect("failure") = Some(error);
                                break;
                            }
                        }
                    }
                    let mut records = records.lock().expect("records");
                    records.push(record);
                    if records.len().is_multiple_of(500) {
                        progress(&format!("{} of {total} articles read", records.len()));
                    }
                }
            });
        }
    });
    if let Some(error) = failure.into_inner().expect("failure") {
        return Err(error);
    }
    let mut records = records.into_inner().expect("records");
    records.sort_by(|a, b| (&a.date, &a.url).cmp(&(&b.date, &b.url)));
    Ok(records)
}

/// A summed tone over `n` articles as their mean, to four decimals; `None` without articles.
fn mean(sum: f64, n: u64) -> Option<f64> {
    (n > 0).then(|| (sum / n as f64 * 10_000.0).round() / 10_000.0)
}

/// The last `days` complete days before `today`.
pub fn recent(today: Date, days: i64) -> Result<(Date, Date)> {
    let end = today.yesterday().map_err(|e| Error::parse("gdelt gkg", e))?;
    let start = end
        .checked_sub((days - 1).days())
        .map_err(|e| Error::parse("gdelt gkg", e))?;
    Ok((start, end))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sources(names: &[&str]) -> fr2027_collectors::sources::CandidateSources {
        fr2027_collectors::sources::CandidateSources {
            wikipedia: None,
            names: names.iter().map(|s| s.to_string()).collect(),
            market_names: Vec::new(),
        }
    }

    /// A GKG line with the fields the counts read.
    fn row(url: &str, tone: &str, note: &str, title: Option<&str>) -> String {
        let mut fields = vec![String::new(); 27];
        fields[DATE] = "20261016121500".into();
        fields[URL] = url.into();
        fields[TONE] = tone.into();
        fields[TRANSLATION] = note.into();
        fields[EXTRAS] = title
            .map(|t| format!("<PAGE_TITLE>{t}</PAGE_TITLE>"))
            .unwrap_or_default();
        fields.join("\t")
    }

    const FRA: &str = "srclc:fra;eng:GT-FRA 1.0";

    #[test]
    fn counts_french_articles_by_title_or_address_with_their_tone() {
        let matcher = Matcher::new(&BTreeMap::from([
            ("le_pen".to_string(), sources(&["Marine Le Pen", "Le Pen"])),
            ("le_maire".to_string(), sources(&["Bruno Le Maire"])),
            ("melenchon".to_string(), sources(&["Jean-Luc Mélenchon", "Mélenchon"])),
        ]));
        // Lines as GDELT's translation stream writes them; 2017's have no title.
        let text = [
            row(
                "https://www.franceinfo.fr/politique/marine-le-pen/nous-ne-pouvons-plus-aider-l-ukraine.html",
                "-2.5,1,3.5,4.5,20,1,300",
                FRA,
                Some("« Nous ne pouvons plus aider l'Ukraine »"),
            ),
            row(
                "https://www.sudouest.fr/a/1.html",
                "0.5,2,1.5,3.5,20,1,100",
                FRA,
                Some("M&#xE9;lenchon en meeting &#xE0; Lille"),
            ),
            row(
                "http://www.corsematin.com/article/article/marine-le-pen-je-serai-en-tete.2156247.html",
                "1,2,1,3,20,1,100",
                FRA,
                None,
            ),
            row(
                "http://tempsreel.nouvelobs.com/a-chaud/36006-venezuela-m%C3%A9lenchon-demande.html",
                "-8,1,9,10,20,1,200",
                FRA,
                None,
            ),
            row(
                "https://www.ouest-france.fr/le-maire-de-rennes-inaugure.html",
                "0,1,1,2,20,1,100",
                FRA,
                Some("Le maire de Rennes inaugure"),
            ),
            row(
                "https://www.marca.com/le-pen-en-espagne.html",
                "-9,0,9,9,20,1,100",
                "srclc:spa;eng:GT-SPA 1.0",
                Some("Le Pen en Espagne"),
            ),
        ]
        .join("\n");
        let file = count_lines(text.as_bytes(), &matcher);
        assert_eq!(file.french, 5, "Spanish articles do not count");
        assert_eq!(
            file.counts,
            [0, 1, 2],
            "by id: a mayor is not Le Maire; Le Pen by her slug, not the section an article is filed under"
        );
        assert!((file.french_tone - (-2.5 + 0.5 + 1.0 - 8.0)).abs() < 1e-9);
        assert!((file.tone[1] - 1.0).abs() < 1e-9);
        assert!((file.tone[2] - (0.5 - 8.0)).abs() < 1e-9);
    }

    #[test]
    fn reads_an_address_as_its_slug() {
        assert_eq!(
            slug("http://www.lefigaro.fr/flash-eco/2017/04/20/97002-20170420FILWWW00304-fmi-lagarde.php?x=1#y"),
            "97002 20170420FILWWW00304 fmi lagarde php"
        );
        assert_eq!(slug("https://example.org/a/%C3%A9douard-philippe"), "édouard philippe");
        assert_eq!(slug("https://example.org"), "");
    }

    #[test]
    fn reads_when_gdelt_listed_an_article() {
        assert_eq!(seen_at(b"20261016121500"), "2026-10-16T12:15:00Z");
        assert_eq!(seen_at(b"2026101612"), "");
    }

    #[test]
    fn reads_the_tone_or_none() {
        assert_eq!(
            article_tone(b"-3.84615384615385,0.64,4.48,5.12,26.9,1.28,154"),
            -3.84615384615385
        );
        assert_eq!(article_tone(b""), 0.0);
        assert_eq!(article_tone(b"nan,1"), 0.0);
        assert_eq!(mean(-10.0, 3), Some(-3.3333));
        assert_eq!(mean(1.0, 0), None);
    }

    #[test]
    fn decodes_title_entities() {
        assert_eq!(decode_entities("M&#xE9;lenchon &amp; co &#233;"), "Mélenchon & co é");
        assert_eq!(decode_entities("a & b"), "a & b");
    }
}
