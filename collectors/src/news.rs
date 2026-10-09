//! Headlines: `data/raw/news/YYYY-MM-DDTHH.json` → `data/clean/news.csv`, one row per headline
//! that names a candidate, oldest first, each link once.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::{
    Rejection, Repo, Result, SchemaKind, Schemas, config::CandidatesConfig, csv_bytes, read_csv, read_json_documents,
    sources::SourcesConfig, write, write_quarantine,
};

pub const CLEAN_FILE: &str = "news.csv";
const HEADER: &str = "published_at,fetched_at,feed,outlet,candidate_ids,title,url,source_file";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Snapshot {
    pub schema_version: String,
    pub fetched_at: String,
    pub feeds: Vec<FeedStatus>,
    pub items: Vec<Item>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeedStatus {
    pub id: String,
    pub status: String,
    pub items: u32,
    pub error: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub feed: String,
    pub title: String,
    pub url: String,
    pub published_at: Option<String>,
    pub candidate_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NewsRow {
    /// Publication time from the feed, or the fetch time when the feed gives none.
    pub published_at: String,
    pub fetched_at: String,
    pub feed: String,
    pub outlet: String,
    /// Space-separated.
    pub candidate_ids: String,
    pub title: String,
    pub url: String,
    pub source_file: String,
}

#[derive(Debug, Default)]
pub struct NewsIngest {
    pub rows: Vec<NewsRow>,
    pub accepted: usize,
    pub rejections: Vec<Rejection>,
    /// Every link already recorded, for the collector's de-duplication.
    pub seen_urls: BTreeSet<String>,
    /// The newest accepted snapshot's fetch time and feed statuses.
    pub latest: Option<(String, Vec<FeedStatus>)>,
}

/// `2026-10-09T20:17:03Z` → `2026-10-09T20`, the file stem of a snapshot.
pub fn file_stem(fetched_at: &str) -> String {
    fetched_at.get(..13).unwrap_or(fetched_at).to_string()
}

pub fn check(stem: Option<&str>, snapshot: &Snapshot, config: &CandidatesConfig) -> Vec<String> {
    let mut errors = Vec::new();
    if let Some(stem) = stem {
        let expected = file_stem(&snapshot.fetched_at);
        if stem != expected {
            errors.push(format!("file name `{stem}` must be the fetch hour `{expected}`"));
        }
    }
    for item in &snapshot.items {
        for id in &item.candidate_ids {
            if config.candidate(id).is_none() {
                errors.push(format!("{}: unknown candidate `{id}`", item.url));
            }
        }
    }
    errors
}

pub fn ingest(
    repo: &Repo,
    schemas: &Schemas,
    config: &CandidatesConfig,
    sources: &SourcesConfig,
) -> Result<NewsIngest> {
    let (accepted, rejections) =
        read_json_documents::<Snapshot>(&repo.raw_news_dir(), schemas, SchemaKind::News, |stem, snapshot| {
            check(Some(stem), snapshot, config)
        })?;
    let mut ingest = NewsIngest {
        accepted: accepted.len(),
        rejections,
        ..Default::default()
    };
    for a in &accepted {
        let source_file = repo.relative(&a.path);
        for item in &a.document.items {
            if !ingest.seen_urls.insert(item.url.clone()) {
                continue;
            }
            ingest.rows.push(NewsRow {
                published_at: item
                    .published_at
                    .clone()
                    .unwrap_or_else(|| a.document.fetched_at.clone()),
                fetched_at: a.document.fetched_at.clone(),
                feed: item.feed.clone(),
                outlet: sources.feed(&item.feed).map(|f| f.outlet.clone()).unwrap_or_default(),
                candidate_ids: item.candidate_ids.join(" "),
                title: item.title.clone(),
                url: item.url.clone(),
                source_file: source_file.clone(),
            });
        }
    }
    ingest
        .rows
        .sort_by(|a, b| (&a.published_at, &a.url).cmp(&(&b.published_at, &b.url)));
    ingest.latest = accepted
        .into_iter()
        .last()
        .map(|a| (a.document.fetched_at, a.document.feeds));
    Ok(ingest)
}

pub fn clean_bytes(ingest: &NewsIngest) -> Result<Vec<u8>> {
    csv_bytes(&ingest.rows, HEADER)
}

pub fn write_clean(repo: &Repo, ingest: &NewsIngest) -> Result<()> {
    write(&repo.clean_dir().join(CLEAN_FILE), &clean_bytes(ingest)?)?;
    write_quarantine(repo, "news", &ingest.rejections)
}

pub fn read_clean_rows(repo: &Repo) -> Result<Vec<NewsRow>> {
    read_csv(&repo.clean_dir().join(CLEAN_FILE))
}
