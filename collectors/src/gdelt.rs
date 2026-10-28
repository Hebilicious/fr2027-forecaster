//! News volume and tone from GDELT: `data/raw/gdelt/YYYY-MM-DD.json` → `data/clean/gdelt.csv`, one
//! row per candidate per day: how many French-language articles GDELT read named the candidate,
//! from every one of its raw files (`fetch/src/gdelt.rs`), and their mean tone against the day's.
//! When two fetches cover the same day, the later one wins; fetches before 2026-10-18 read one file
//! in four and carry no tone, and every day they covered has been fetched again since.
//!
//! The RSS headlines in `news.csv` are what the Signals view lists; this is the daily count the
//! model's event detection reads, because GDELT also has it for 2017 and 2022, so the backtest
//! fits the same series the live forecast uses.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    Rejection, Repo, Result, SchemaKind, Schemas, config::CandidatesConfig, csv_bytes, read_csv, read_json_documents,
    write, write_quarantine,
};

pub const CLEAN_FILE: &str = "gdelt.csv";
const HEADER: &str = "date,candidate_id,query,articles,total_articles,tone,total_tone,files,source_file";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fetch {
    pub schema_version: String,
    pub fetched_at: String,
    pub start: String,
    pub end: String,
    pub candidates: Vec<CandidateCounts>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CandidateCounts {
    pub candidate_id: String,
    /// How the candidate was matched.
    pub query: String,
    pub status: String,
    pub error: Option<String>,
    pub days: Vec<DayCount>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DayCount {
    pub date: String,
    /// French articles naming the candidate.
    pub articles: u64,
    /// Every French article of the day, for scale.
    pub total_articles: u64,
    /// Mean GDELT tone of the articles naming the candidate; `None` without any, or in a fetch
    /// before tone was kept.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tone: Option<f64>,
    /// Mean GDELT tone of every French article of the day.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_tone: Option<f64>,
    /// GDELT files read for the day; `None` in a fetch before every file was read.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GdeltRow {
    pub date: String,
    pub candidate_id: String,
    pub query: String,
    pub articles: u64,
    pub total_articles: u64,
    pub tone: Option<f64>,
    pub total_tone: Option<f64>,
    pub files: Option<u32>,
    pub source_file: String,
}

#[derive(Debug, Default)]
pub struct GdeltIngest {
    pub rows: Vec<GdeltRow>,
    pub accepted: usize,
    pub rejections: Vec<Rejection>,
    pub latest_fetch: Option<String>,
}

pub fn check(stem: Option<&str>, fetch: &Fetch, config: &CandidatesConfig) -> Vec<String> {
    let mut errors = Vec::new();
    if let Some(stem) = stem {
        let expected = fetch.fetched_at.get(..10).unwrap_or_default();
        // `<date>-backfill` holds an earlier stretch fetched that day (`fr2027 collect ... --backfill`).
        if stem != expected && stem != format!("{expected}-backfill") {
            errors.push(format!("file name `{stem}` must be the fetch date `{expected}`"));
        }
    }
    errors.extend(check_days(fetch));
    for candidate in &fetch.candidates {
        if config.candidate(&candidate.candidate_id).is_none() {
            errors.push(format!("unknown candidate `{}`", candidate.candidate_id));
        }
    }
    errors
}

/// The checks that do not depend on the candidate list: dates inside the fetch's range, and
/// matching articles never more than the day's total.
pub fn check_days(fetch: &Fetch) -> Vec<String> {
    let mut errors = Vec::new();
    if fetch.start > fetch.end {
        errors.push("start is after end".into());
    }
    for candidate in &fetch.candidates {
        for day in &candidate.days {
            if day.date < fetch.start || day.date > fetch.end {
                errors.push(format!(
                    "{}: {} is outside {}–{}",
                    candidate.candidate_id, day.date, fetch.start, fetch.end
                ));
            }
            if day.articles > day.total_articles {
                errors.push(format!(
                    "{}: {} counts more matching articles than articles",
                    candidate.candidate_id, day.date
                ));
            }
        }
    }
    errors
}

/// One row per candidate and day; for a day several fetches cover, the count of a fetch that read
/// every file over one that read one in four (those carry no `files`), then of the one fetched last
/// (by `fetched_at`, not by file name: `<date>-backfill.json` sorts before `<date>.json`).
pub fn rows<'a>(fetches: impl IntoIterator<Item = (&'a Fetch, String)>) -> Vec<GdeltRow> {
    let mut fetches: Vec<(&Fetch, String)> = fetches.into_iter().collect();
    let every_file = |f: &Fetch| f.candidates.iter().flat_map(|c| &c.days).any(|d| d.files.is_some());
    fetches.sort_by(|a, b| (every_file(a.0), &a.0.fetched_at).cmp(&(every_file(b.0), &b.0.fetched_at)));
    let mut by_day: BTreeMap<(String, String), GdeltRow> = BTreeMap::new();
    for (fetch, source_file) in fetches {
        for candidate in &fetch.candidates {
            for day in &candidate.days {
                by_day.insert(
                    (day.date.clone(), candidate.candidate_id.clone()),
                    GdeltRow {
                        date: day.date.clone(),
                        candidate_id: candidate.candidate_id.clone(),
                        query: candidate.query.clone(),
                        articles: day.articles,
                        total_articles: day.total_articles,
                        tone: day.tone,
                        total_tone: day.total_tone,
                        files: day.files,
                        source_file: source_file.clone(),
                    },
                );
            }
        }
    }
    by_day.into_values().collect()
}

pub fn ingest(repo: &Repo, schemas: &Schemas, config: &CandidatesConfig) -> Result<GdeltIngest> {
    let (accepted, rejections) =
        read_json_documents::<Fetch>(&repo.raw_gdelt_dir(), schemas, SchemaKind::Gdelt, |stem, fetch| {
            check(Some(stem), fetch, config)
        })?;
    Ok(GdeltIngest {
        rows: rows(accepted.iter().map(|a| (&a.document, repo.relative(&a.path)))),
        accepted: accepted.len(),
        latest_fetch: accepted.last().map(|a| a.document.fetched_at.clone()),
        rejections,
    })
}

pub fn clean_bytes(ingest: &GdeltIngest) -> Result<Vec<u8>> {
    csv_bytes(&ingest.rows, HEADER)
}

pub fn write_clean(repo: &Repo, ingest: &GdeltIngest) -> Result<()> {
    write(&repo.clean_dir().join(CLEAN_FILE), &clean_bytes(ingest)?)?;
    write_quarantine(repo, "gdelt", &ingest.rejections)
}

pub fn read_clean_rows(repo: &Repo) -> Result<Vec<GdeltRow>> {
    read_csv(&repo.clean_dir().join(CLEAN_FILE))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sampled(fetched_at: &str, articles: u64, files: Option<u32>) -> Fetch {
        Fetch {
            schema_version: "1.0".into(),
            fetched_at: fetched_at.into(),
            start: "2026-10-01".into(),
            end: "2026-10-02".into(),
            candidates: vec![CandidateCounts {
                candidate_id: "le_pen".into(),
                query: "q".into(),
                status: "ok".into(),
                error: None,
                days: vec![DayCount {
                    date: "2026-10-02".into(),
                    articles,
                    total_articles: 1000,
                    tone: None,
                    total_tone: None,
                    files,
                }],
            }],
        }
    }

    #[test]
    fn later_fetch_wins() {
        let fetch = |fetched_at: &str, articles: u64| sampled(fetched_at, articles, None);
        let (a, b) = (fetch("2026-10-02T06:00:00Z", 3), fetch("2026-10-03T06:00:00Z", 5));
        for order in [[(&a, "a"), (&b, "b")], [(&b, "b"), (&a, "a")]] {
            let rows = rows(order.map(|(f, name)| (f, name.to_string())));
            assert_eq!(rows.len(), 1);
            assert_eq!(
                (rows[0].articles, rows[0].source_file.as_str()),
                (5, "b"),
                "whatever the file order"
            );
        }
    }

    #[test]
    fn a_fetch_of_every_file_wins_over_a_later_sample() {
        let full = sampled("2026-10-02T06:00:00Z", 20, Some(96));
        let later_sample = sampled("2026-10-03T06:00:00Z", 5, None);
        let rows = rows([(&full, "full".to_string()), (&later_sample, "sample".to_string())]);
        assert_eq!((rows[0].articles, rows[0].source_file.as_str()), (20, "full"));
    }
}
