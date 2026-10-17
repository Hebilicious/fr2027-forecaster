//! News volume from GDELT: `data/raw/gdelt/YYYY-MM-DD.json` → `data/clean/gdelt.csv`, one row
//! per candidate per day: how many French-language articles GDELT read named the candidate, in a
//! one-in-four sample of its raw files (`fetch/src/gdelt.rs`). When two fetches cover the same day,
//! the later one wins.
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
const HEADER: &str = "date,candidate_id,query,articles,total_articles,source_file";

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
    /// French articles naming the candidate, in the day's sample.
    pub articles: u64,
    /// Every French article in the day's sample, for scale.
    pub total_articles: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GdeltRow {
    pub date: String,
    pub candidate_id: String,
    pub query: String,
    pub articles: u64,
    pub total_articles: u64,
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

/// One row per candidate and day; for a day several fetches cover, the last fetch's count.
pub fn rows<'a>(fetches: impl IntoIterator<Item = (&'a Fetch, String)>) -> Vec<GdeltRow> {
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

    #[test]
    fn later_fetch_wins() {
        let fetch = |fetched_at: &str, articles: u64| Fetch {
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
                }],
            }],
        };
        let (a, b) = (fetch("2026-10-02T06:00:00Z", 3), fetch("2026-10-03T06:00:00Z", 5));
        let rows = rows([(&a, "a".to_string()), (&b, "b".to_string())]);
        assert_eq!(rows.len(), 1);
        assert_eq!((rows[0].articles, rows[0].source_file.as_str()), (5, "b"));
    }
}
