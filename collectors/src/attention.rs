//! Wikipedia page views: `data/raw/attention/YYYY-MM-DD.json` → `data/clean/attention.csv`, one
//! row per candidate per day. When two fetches cover the same day, the later one wins.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    Rejection, Repo, Result, SchemaKind, Schemas, config::CandidatesConfig, csv_bytes, read_csv, read_json_documents,
    write, write_quarantine,
};

pub const CLEAN_FILE: &str = "attention.csv";
const HEADER: &str = "date,candidate_id,article,views,source_file";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fetch {
    pub schema_version: String,
    pub fetched_at: String,
    pub project: String,
    pub start: String,
    pub end: String,
    pub articles: Vec<ArticleViews>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArticleViews {
    pub candidate_id: String,
    pub article: String,
    pub status: String,
    pub error: Option<String>,
    pub days: Vec<DayViews>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DayViews {
    pub date: String,
    pub views: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttentionRow {
    pub date: String,
    pub candidate_id: String,
    pub article: String,
    pub views: u64,
    pub source_file: String,
}

#[derive(Debug, Default)]
pub struct AttentionIngest {
    pub rows: Vec<AttentionRow>,
    pub accepted: usize,
    pub rejections: Vec<Rejection>,
    pub latest_fetch: Option<String>,
}

pub fn check(stem: Option<&str>, fetch: &Fetch, config: &CandidatesConfig) -> Vec<String> {
    let mut errors = Vec::new();
    if let Some(stem) = stem {
        let expected = fetch.fetched_at.get(..10).unwrap_or_default();
        if stem != expected {
            errors.push(format!("file name `{stem}` must be the fetch date `{expected}`"));
        }
    }
    if fetch.start > fetch.end {
        errors.push("start is after end".into());
    }
    for article in &fetch.articles {
        if config.candidate(&article.candidate_id).is_none() {
            errors.push(format!("unknown candidate `{}`", article.candidate_id));
        }
        for day in &article.days {
            if day.date < fetch.start || day.date > fetch.end {
                errors.push(format!(
                    "{}: {} is outside {}–{}",
                    article.article, day.date, fetch.start, fetch.end
                ));
            }
        }
    }
    errors
}

pub fn ingest(repo: &Repo, schemas: &Schemas, config: &CandidatesConfig) -> Result<AttentionIngest> {
    let (accepted, rejections) = read_json_documents::<Fetch>(
        &repo.raw_attention_dir(),
        schemas,
        SchemaKind::Attention,
        |stem, fetch| check(Some(stem), fetch, config),
    )?;
    let mut by_day: BTreeMap<(String, String), AttentionRow> = BTreeMap::new();
    for a in &accepted {
        let source_file = repo.relative(&a.path);
        for article in &a.document.articles {
            for day in &article.days {
                by_day.insert(
                    (day.date.clone(), article.candidate_id.clone()),
                    AttentionRow {
                        date: day.date.clone(),
                        candidate_id: article.candidate_id.clone(),
                        article: article.article.clone(),
                        views: day.views,
                        source_file: source_file.clone(),
                    },
                );
            }
        }
    }
    Ok(AttentionIngest {
        rows: by_day.into_values().collect(),
        accepted: accepted.len(),
        latest_fetch: accepted.last().map(|a| a.document.fetched_at.clone()),
        rejections,
    })
}

pub fn clean_bytes(ingest: &AttentionIngest) -> Result<Vec<u8>> {
    csv_bytes(&ingest.rows, HEADER)
}

pub fn write_clean(repo: &Repo, ingest: &AttentionIngest) -> Result<()> {
    write(&repo.clean_dir().join(CLEAN_FILE), &clean_bytes(ingest)?)?;
    write_quarantine(repo, "attention", &ingest.rejections)
}

pub fn read_clean_rows(repo: &Repo) -> Result<Vec<AttentionRow>> {
    read_csv(&repo.clean_dir().join(CLEAN_FILE))
}
