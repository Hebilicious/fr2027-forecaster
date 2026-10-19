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
const HEADER: &str = "date,candidate_id,query,articles,total_articles,tone,total_tone,files,\
stance_read,favourable,unfavourable,neutral,stance_not_about,stance_unreachable,source_file";
pub const ARTICLES_FILE: &str = "gdelt_articles.csv";
const ARTICLES_HEADER: &str =
    "date,seen_at,url,title,candidate_id,tone,reached,favourable,unfavourable,neutral,not_about,source_file";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fetch {
    pub schema_version: String,
    pub fetched_at: String,
    pub start: String,
    pub end: String,
    pub candidates: Vec<CandidateCounts>,
    /// The articles behind the counts, with how they were read (fetches since 2026-10-19).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub articles: Vec<ArticleRecord>,
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
    /// How the articles naming the candidate portray them, by Jev reading each article's text; `None`
    /// in a fetch that did not read them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stance: Option<DayStance>,
}

/// A candidate's day of articles as Jev read them (`fetch/src/typesafe.rs`): how many were read and
/// are about the candidate, the sums of their favourable, unfavourable and neutral probabilities
/// (renormalised without "not about"), and the articles Jev found not about the candidate (a name
/// match on someone else) or whose text could not be reached, live or in the Internet Archive.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DayStance {
    pub read: u64,
    pub favourable: f64,
    pub unfavourable: f64,
    pub neutral: f64,
    pub not_about: u64,
    pub unreachable: u64,
}

/// Where an article's text was read.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Reached {
    /// At its address.
    Live,
    /// In the Internet Archive, the page being gone or refusing the collector.
    Archive,
    /// Neither.
    Unreachable,
}

/// Jev's probabilities for how an article portrays one candidate.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stance {
    pub favourable: f64,
    pub unfavourable: f64,
    pub neutral: f64,
    pub not_about: f64,
}

impl Stance {
    /// Most of the probability on "not about": a name match on someone else, or a passing mention.
    pub fn is_about(&self) -> bool {
        self.not_about < 0.5
    }
}

/// One French article GDELT read whose headline names a candidate, as the live collector keeps it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArticleRecord {
    /// The day of the GDELT file that listed it (UTC).
    pub date: String,
    /// When GDELT first listed it that day (its record's DATE, UTC).
    pub seen_at: String,
    pub url: String,
    pub title: Option<String>,
    pub candidate_ids: Vec<String>,
    /// GDELT's tone of the article.
    pub tone: f64,
    pub reached: Reached,
    /// By candidate id, when the text was read and classified.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub stance: BTreeMap<String, Stance>,
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
    /// [`DayStance`], flat; empty for a fetch that did not read the articles.
    pub stance_read: Option<u64>,
    pub favourable: Option<f64>,
    pub unfavourable: Option<f64>,
    pub neutral: Option<f64>,
    pub stance_not_about: Option<u64>,
    pub stance_unreachable: Option<u64>,
    pub source_file: String,
}

impl GdeltRow {
    pub fn stance(&self) -> Option<DayStance> {
        Some(DayStance {
            read: self.stance_read?,
            favourable: self.favourable?,
            unfavourable: self.unfavourable?,
            neutral: self.neutral?,
            not_about: self.stance_not_about?,
            unreachable: self.stance_unreachable?,
        })
    }
}

/// One row of `data/clean/gdelt_articles.csv`: an article GDELT read and one candidate its headline
/// names, with how Jev read it (empty when its text could not be reached).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArticleRow {
    pub date: String,
    pub seen_at: String,
    pub url: String,
    pub title: Option<String>,
    pub candidate_id: String,
    pub tone: f64,
    pub reached: Reached,
    pub favourable: Option<f64>,
    pub unfavourable: Option<f64>,
    pub neutral: Option<f64>,
    pub not_about: Option<f64>,
    pub source_file: String,
}

#[derive(Debug, Default)]
pub struct GdeltIngest {
    pub rows: Vec<GdeltRow>,
    pub articles: Vec<ArticleRow>,
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
                        stance_read: day.stance.map(|s| s.read),
                        favourable: day.stance.map(|s| s.favourable),
                        unfavourable: day.stance.map(|s| s.unfavourable),
                        neutral: day.stance.map(|s| s.neutral),
                        stance_not_about: day.stance.map(|s| s.not_about),
                        stance_unreachable: day.stance.map(|s| s.unreachable),
                        source_file: source_file.clone(),
                    },
                );
            }
        }
    }
    by_day.into_values().collect()
}

/// One row per article and candidate, from the fetches that kept their articles; for an article
/// several fetches hold, the one fetched last.
pub fn article_rows<'a>(fetches: impl IntoIterator<Item = (&'a Fetch, String)>) -> Vec<ArticleRow> {
    let mut fetches: Vec<(&Fetch, String)> = fetches.into_iter().collect();
    fetches.sort_by(|a, b| a.0.fetched_at.cmp(&b.0.fetched_at));
    let mut by_article: BTreeMap<(String, String, String), ArticleRow> = BTreeMap::new();
    for (fetch, source_file) in fetches {
        for article in &fetch.articles {
            for id in &article.candidate_ids {
                let stance = article.stance.get(id);
                by_article.insert(
                    (article.date.clone(), article.url.clone(), id.clone()),
                    ArticleRow {
                        date: article.date.clone(),
                        seen_at: article.seen_at.clone(),
                        url: article.url.clone(),
                        title: article.title.clone(),
                        candidate_id: id.clone(),
                        tone: article.tone,
                        reached: article.reached,
                        favourable: stance.map(|s| s.favourable),
                        unfavourable: stance.map(|s| s.unfavourable),
                        neutral: stance.map(|s| s.neutral),
                        not_about: stance.map(|s| s.not_about),
                        source_file: source_file.clone(),
                    },
                );
            }
        }
    }
    by_article.into_values().collect()
}

pub fn ingest(repo: &Repo, schemas: &Schemas, config: &CandidatesConfig) -> Result<GdeltIngest> {
    let (accepted, rejections) =
        read_json_documents::<Fetch>(&repo.raw_gdelt_dir(), schemas, SchemaKind::Gdelt, |stem, fetch| {
            check(Some(stem), fetch, config)
        })?;
    Ok(GdeltIngest {
        rows: rows(accepted.iter().map(|a| (&a.document, repo.relative(&a.path)))),
        articles: article_rows(accepted.iter().map(|a| (&a.document, repo.relative(&a.path)))),
        accepted: accepted.len(),
        latest_fetch: accepted.last().map(|a| a.document.fetched_at.clone()),
        rejections,
    })
}

pub fn clean_bytes(ingest: &GdeltIngest) -> Result<Vec<u8>> {
    csv_bytes(&ingest.rows, HEADER)
}

pub fn articles_bytes(ingest: &GdeltIngest) -> Result<Vec<u8>> {
    csv_bytes(&ingest.articles, ARTICLES_HEADER)
}

pub fn write_clean(repo: &Repo, ingest: &GdeltIngest) -> Result<()> {
    write(&repo.clean_dir().join(CLEAN_FILE), &clean_bytes(ingest)?)?;
    write(&repo.clean_dir().join(ARTICLES_FILE), &articles_bytes(ingest)?)?;
    write_quarantine(repo, "gdelt", &ingest.rejections)
}

pub fn read_clean_rows(repo: &Repo) -> Result<Vec<GdeltRow>> {
    read_csv(&repo.clean_dir().join(CLEAN_FILE))
}

/// `data/clean/gdelt_articles.csv`; empty before the collector kept articles.
pub fn read_article_rows(repo: &Repo) -> Result<Vec<ArticleRow>> {
    let path = repo.clean_dir().join(ARTICLES_FILE);
    if !path.exists() {
        return Ok(Vec::new());
    }
    read_csv(&path)
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
                    stance: None,
                }],
            }],
            articles: Vec::new(),
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
