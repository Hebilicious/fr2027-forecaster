//! Daily counts of French-language articles naming each candidate, from GDELT's raw Global
//! Knowledge Graph files: the "translation" stream, every non-English article GDELT reads, one file
//! per 15 minutes since February 2015, about 7 MB zipped each.
//! `GET https://data.gdeltproject.org/gdeltv2/<YYYYMMDDHHMMSS>.translation.gkg.csv.zip`.
//!
//! GDELT's DOC API answers the same question in one request per candidate, but it refuses most
//! requests (HTTP 429) from shared addresses, a home connection behind a carrier's NAT and CI
//! runners alike; the files are static downloads with no limit.
//!
//! Only the file at minute 00 of each hour is read, a one-in-four sample, so a year is about 9,000
//! files rather than 35,000. French articles are those whose translation note reads `srclc:fra`.
//! An article counts for a candidate when its title names them, by the rule headlines are matched
//! with (`collectors/src/names.rs`), or when GDELT's list of people it names (V1Persons) holds
//! their full name. Titles are in the files from 2018 on; for 2017 only the people list counts,
//! and it misses many articles, so 2017's counts are lower and noisier.

use std::collections::BTreeMap;
use std::io::{Cursor, Read};
use std::sync::Mutex;

use fr2027_collectors::{gdelt::DayCount, names::NameMatcher, sources::SourcesConfig};
use jiff::{ToSpan, civil::Date};

use crate::{Client, Error, Result};

const FILES: &str = "https://data.gdeltproject.org/gdeltv2";
/// How long one file may take to download.
pub const TIMEOUT: std::time::Duration = std::time::Duration::from_secs(120);
/// Files read at once.
pub const WORKERS: usize = 16;
/// The first day the translation stream covers.
pub const FIRST_DAY: Date = jiff::civil::date(2015, 2, 19);

/// Columns of the people an article names, its translation note, and the extras holding its
/// title (0-based, GKG 2.1).
const PERSONS: usize = 11;
const TRANSLATION: usize = 25;
const EXTRAS: usize = 26;

/// How a candidate is recognised: their full names, folded with [`fold`].
#[derive(Clone, Debug)]
pub struct Person {
    pub candidate_id: String,
    pub names: Vec<String>,
}

impl Person {
    /// The full names (two words or more) among the names headlines use; a surname alone is
    /// never one of GDELT's people.
    /// `names` as in config/sources.yaml: the first is the full name, which is what GDELT's
    /// people list holds; the others are for titles.
    pub fn new(candidate_id: &str, names: &[String]) -> Self {
        Person {
            candidate_id: candidate_id.to_string(),
            names: names.first().map(|n| fold(n)).into_iter().collect(),
        }
    }

    /// How the candidate is matched, for the record.
    pub fn description(&self, title_names: &[String]) -> String {
        format!(
            "GDELT GKG translation stream, srclc:fra, hourly sample; title: {}; persons: {}",
            title_names.join(" | "),
            self.names.join("; ")
        )
    }
}

/// Lowercase, without accents, hyphens as spaces: how GDELT writes people's names.
pub fn fold(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    for c in name.chars().flat_map(char::to_lowercase) {
        let plain = match c {
            'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' => 'a',
            'ç' => 'c',
            'è' | 'é' | 'ê' | 'ë' => 'e',
            'ì' | 'í' | 'î' | 'ï' => 'i',
            'ñ' => 'n',
            'ò' | 'ó' | 'ô' | 'õ' | 'ö' => 'o',
            'ù' | 'ú' | 'û' | 'ü' => 'u',
            'ý' | 'ÿ' => 'y',
            '-' | '\'' | '’' => ' ',
            other => other,
        };
        out.push(plain);
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Recognises the candidates in an article: by title and by GDELT's people list.
pub struct Matcher {
    people: Vec<Person>,
    titles: NameMatcher,
}

impl Matcher {
    /// `candidates`: each candidate's id and names, as in config/sources.yaml.
    pub fn new(candidates: &BTreeMap<String, fr2027_collectors::sources::CandidateSources>) -> Self {
        let people = candidates.iter().map(|(id, c)| Person::new(id, &c.names)).collect();
        let sources = SourcesConfig {
            candidates: candidates.clone(),
            ..SourcesConfig::default()
        };
        Matcher {
            people,
            titles: NameMatcher::new(&sources),
        }
    }

    pub fn people(&self) -> &[Person] {
        &self.people
    }
}

/// The French articles in one file, and how many name each candidate (in `matcher.people()`
/// order).
pub fn count_file(zipped: &[u8], matcher: &Matcher) -> Result<(u64, Vec<u64>)> {
    let mut archive = zip::ZipArchive::new(Cursor::new(zipped)).map_err(|e| Error::parse("gdelt gkg", e))?;
    let mut text = Vec::new();
    archive
        .by_index(0)
        .map_err(|e| Error::parse("gdelt gkg", e))?
        .read_to_end(&mut text)
        .map_err(|e| Error::parse("gdelt gkg", e))?;
    Ok(count_lines(&text, matcher))
}

fn count_lines(text: &[u8], matcher: &Matcher) -> (u64, Vec<u64>) {
    let people = &matcher.people;
    let mut french = 0;
    let mut counts = vec![0; people.len()];
    for line in text.split(|b| *b == b'\n') {
        let fields: Vec<&[u8]> = line.split(|b| *b == b'\t').collect();
        let Some(note) = fields.get(TRANSLATION) else { continue };
        if !note.starts_with(b"srclc:fra") {
            continue;
        }
        french += 1;
        let persons = String::from_utf8_lossy(fields.get(PERSONS).copied().unwrap_or_default());
        let named: Vec<String> = persons.split(';').map(fold).collect();
        let extras = String::from_utf8_lossy(fields.get(EXTRAS).copied().unwrap_or_default());
        let in_title = title(&extras).map(|t| matcher.titles.find(&t)).unwrap_or_default();
        for (i, person) in people.iter().enumerate() {
            if in_title.contains(&person.candidate_id) || person.names.iter().any(|n| named.contains(n)) {
                counts[i] += 1;
            }
        }
    }
    (french, counts)
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

/// One day's sampled counts.
#[derive(Clone, Debug, Default)]
struct Day {
    french: u64,
    counts: Vec<u64>,
    files: u32,
}

/// Each person's daily count of French articles naming them, from `start` to `end` (both included,
/// none before [`FIRST_DAY`]), reading `workers` files at a time. `total_articles` is the day's
/// French articles in the same sample. A day with no file left (GDELT has gaps) is left out.
pub fn count_days(
    client: &Client,
    matcher: &Matcher,
    start: Date,
    end: Date,
    workers: usize,
    progress: &(dyn Fn(&str) + Sync),
) -> Result<Vec<(String, Vec<DayCount>)>> {
    let mut hours = Vec::new();
    let mut day = start.max(FIRST_DAY);
    while day <= end {
        for hour in 0..24 {
            hours.push((day, hour));
        }
        day = day.tomorrow().map_err(|e| Error::parse("gdelt gkg", e))?;
    }
    let total = hours.len();
    let people = matcher.people();
    let queue = Mutex::new(hours.into_iter());
    let days: Mutex<BTreeMap<Date, Day>> = Mutex::new(BTreeMap::new());
    let failure: Mutex<Option<Error>> = Mutex::new(None);
    let done = Mutex::new(0usize);
    std::thread::scope(|scope| {
        for _ in 0..workers.max(1) {
            scope.spawn(|| {
                loop {
                    let Some((day, hour)) = queue.lock().expect("queue").next() else {
                        break;
                    };
                    if failure.lock().expect("failure").is_some() {
                        break;
                    }
                    let url = format!(
                        "{FILES}/{}{hour:02}0000.translation.gkg.csv.zip",
                        day.strftime("%Y%m%d")
                    );
                    let counted = match client.get_bytes(&url, None) {
                        Ok(bytes) => count_file(&bytes, matcher).map(Some),
                        // GDELT's stream has gaps; a missing file is not a failure.
                        Err(error) if error.status() == Some(404) => Ok(None),
                        Err(error) => Err(error),
                    };
                    match counted {
                        Ok(Some((french, counts))) => {
                            let mut days = days.lock().expect("days");
                            let entry = days.entry(day).or_insert_with(|| Day {
                                counts: vec![0; people.len()],
                                ..Day::default()
                            });
                            entry.french += french;
                            entry.files += 1;
                            for (sum, c) in entry.counts.iter_mut().zip(counts) {
                                *sum += c;
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
                    if (*done).is_multiple_of(500) {
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
    Ok(people
        .iter()
        .enumerate()
        .map(|(i, person)| {
            let counts = days
                .iter()
                .map(|(date, day)| DayCount {
                    date: date.to_string(),
                    articles: day.counts[i],
                    total_articles: day.french,
                })
                .collect();
            (person.candidate_id.clone(), counts)
        })
        .collect())
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

    #[test]
    fn folds_names_as_gdelt_writes_them() {
        assert_eq!(fold("Jean-Luc Mélenchon"), "jean luc melenchon");
        assert_eq!(fold("Benoît Hamon"), "benoit hamon");
        assert_eq!(fold("Marine  Le Pen"), "marine le pen");
    }

    #[test]
    fn counts_french_articles_by_title_or_people() {
        let names = |n: &[&str]| fr2027_collectors::sources::CandidateSources {
            wikipedia: None,
            names: n.iter().map(|s| s.to_string()).collect(),
            market_names: Vec::new(),
        };
        let matcher = Matcher::new(&BTreeMap::from([
            ("le_pen".to_string(), names(&["Marine Le Pen", "Le Pen"])),
            ("melenchon".to_string(), names(&["Jean-Luc Mélenchon", "Mélenchon"])),
        ]));
        assert_eq!(matcher.people()[0].names, ["marine le pen"], "the full name only");
        let row = |persons: &str, note: &str, title: &str| {
            let mut fields = vec![""; 27];
            fields[PERSONS] = persons;
            fields[TRANSLATION] = note;
            let extras = format!("<PAGE_TITLE>{title}</PAGE_TITLE>");
            let mut fields: Vec<String> = fields.into_iter().map(String::from).collect();
            fields[EXTRAS] = extras;
            fields.join("\t")
        };
        let fra = "srclc:fra;eng:GT-FRA 1.0";
        let text = [
            row("marine le pen;emmanuel macron", fra, ""),
            row("jean marie le pen", fra, "Hommage au fondateur"),
            row("", fra, "M&#xE9;lenchon en meeting &#xE0; Lille"),
            row("marine le pen", "srclc:spa;eng:GT-SPA 1.0", "Le Pen en Espagne"),
        ]
        .join("\n");
        let (french, counts) = count_lines(text.as_bytes(), &matcher);
        assert_eq!(french, 3);
        assert_eq!(
            counts,
            [1, 1],
            "Jean-Marie is not Marine; Spanish articles do not count"
        );
    }

    #[test]
    fn decodes_title_entities() {
        assert_eq!(decode_entities("M&#xE9;lenchon &amp; co &#233;"), "Mélenchon & co é");
        assert_eq!(decode_entities("a & b"), "a & b");
    }
}
