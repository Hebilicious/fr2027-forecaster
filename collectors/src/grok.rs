//! Grok drops: `data/raw/grok/YYYY-MM-DDTHH-MM.json` → `data/clean/grok.csv`.

use std::collections::BTreeSet;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{
    Rejection, Repo, Result, SchemaKind, Schemas, config::CandidatesConfig, content_hash, csv_bytes, list_files, read,
    read_csv, write, write_quarantine,
};

pub const CLEAN_FILE: &str = "grok.csv";

#[derive(Clone, Debug, Deserialize)]
struct Drop {
    collected_at: String,
    window: Window,
    candidates: Vec<CandidateActivity>,
}

#[derive(Clone, Debug, Deserialize)]
struct Window {
    start: String,
    end: String,
}

#[derive(Clone, Debug, Deserialize)]
struct CandidateActivity {
    candidate_id: String,
    mentions: Option<u64>,
    unique_authors: Option<u64>,
    engagement: Option<u64>,
    sentiment: Option<Sentiment>,
    top_topics: Option<Vec<String>>,
    bot_share_estimate: Option<f64>,
    #[serde(default)]
    mentions_method: Option<String>,
    #[serde(default)]
    sample_size: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
struct Sentiment {
    pos: f64,
    neg: f64,
    neu: f64,
    method: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GrokRow {
    pub window_start: String,
    pub window_end: String,
    pub collected_at: String,
    pub candidate_id: String,
    pub mentions: Option<u64>,
    pub mentions_method: Option<String>,
    pub sample_size: Option<u64>,
    pub unique_authors: Option<u64>,
    pub engagement: Option<u64>,
    pub sentiment_pos: Option<f64>,
    pub sentiment_neg: Option<f64>,
    pub sentiment_neu: Option<f64>,
    pub sentiment_method: Option<String>,
    pub bot_share_estimate: Option<f64>,
    /// Topics joined with ` | `.
    pub top_topics: Option<String>,
    pub source_file: String,
    pub content_hash: String,
}

#[derive(Clone, Debug, Default)]
pub struct GrokIngest {
    pub rows: Vec<GrokRow>,
    pub accepted: usize,
    pub rejections: Vec<Rejection>,
}

pub fn ingest(repo: &Repo, schemas: &Schemas, config: &CandidatesConfig) -> Result<GrokIngest> {
    let mut ingest = GrokIngest::default();
    for path in list_files(&repo.raw_grok_dir(), "json")? {
        let bytes = read(&path)?;
        match validate_file(&path, &bytes, schemas, config) {
            Ok(drop) => {
                ingest.accepted += 1;
                let source_file = repo.relative(&path);
                let hash = content_hash(&bytes);
                for activity in drop.candidates {
                    let sentiment = activity.sentiment;
                    ingest.rows.push(GrokRow {
                        window_start: drop.window.start.clone(),
                        window_end: drop.window.end.clone(),
                        collected_at: drop.collected_at.clone(),
                        candidate_id: activity.candidate_id,
                        mentions: activity.mentions,
                        mentions_method: activity.mentions_method,
                        sample_size: activity.sample_size,
                        unique_authors: activity.unique_authors,
                        engagement: activity.engagement,
                        sentiment_pos: sentiment.as_ref().map(|s| s.pos),
                        sentiment_neg: sentiment.as_ref().map(|s| s.neg),
                        sentiment_neu: sentiment.as_ref().map(|s| s.neu),
                        sentiment_method: sentiment.map(|s| s.method),
                        bot_share_estimate: activity.bot_share_estimate,
                        top_topics: activity.top_topics.map(|t| t.join(" | ")),
                        source_file: source_file.clone(),
                        content_hash: hash.clone(),
                    });
                }
            }
            Err(errors) => ingest.rejections.push(Rejection { path, errors }),
        }
    }
    ingest
        .rows
        .sort_by(|a, b| (&a.window_end, &a.candidate_id).cmp(&(&b.window_end, &b.candidate_id)));
    Ok(ingest)
}

fn validate_file(path: &Path, bytes: &[u8], schemas: &Schemas, config: &CandidatesConfig) -> Result<Drop, Vec<String>> {
    let value: serde_json::Value = serde_json::from_slice(bytes).map_err(|e| vec![format!("not JSON: {e}")])?;
    let stem = path
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    validate_value(value, Some(&stem), schemas, config)
}

/// `2026-10-09T14:00:00Z` → `2026-10-09T14-00`, the file stem of the drop whose window ends then.
pub fn file_stem(window_end: &str) -> String {
    window_end.get(..16).map(|s| s.replace(':', "-")).unwrap_or_default()
}

/// Checks a drop against the schema, the candidate list and, when given, its file stem. Returns
/// the drop's file stem.
pub fn validate(
    value: serde_json::Value,
    stem: Option<&str>,
    schemas: &Schemas,
    config: &CandidatesConfig,
) -> Result<String, Vec<String>> {
    validate_value(value, stem, schemas, config).map(|drop| file_stem(&drop.window.end))
}

fn validate_value(
    value: serde_json::Value,
    stem: Option<&str>,
    schemas: &Schemas,
    config: &CandidatesConfig,
) -> Result<Drop, Vec<String>> {
    let violations = schemas.violations(SchemaKind::GrokDrop, &value);
    if !violations.is_empty() {
        return Err(violations);
    }
    let drop: Drop = serde_json::from_value(value).map_err(|e| vec![e.to_string()])?;
    let mut errors = Vec::new();
    let expected = file_stem(&drop.window.end);
    if let Some(stem) = stem
        && stem != expected
    {
        errors.push(format!("file name `{stem}` must be the window end `{expected}`"));
    }
    if drop.window.start >= drop.window.end {
        errors.push("window.start must be before window.end".into());
    }
    if drop.collected_at < drop.window.end {
        errors.push("collected_at must not be before window.end".into());
    }
    let mut seen = BTreeSet::new();
    for activity in &drop.candidates {
        if config.candidate(&activity.candidate_id).is_none() {
            errors.push(format!("unknown candidate `{}`", activity.candidate_id));
        }
        if !seen.insert(activity.candidate_id.clone()) {
            errors.push(format!("candidate `{}` appears twice", activity.candidate_id));
        }
        if let Some(s) = &activity.sentiment {
            let total = s.pos + s.neg + s.neu;
            if (total - 1.0).abs() > 0.02 {
                errors.push(format!(
                    "sentiment for `{}` sums to {total:.3}, expected 1",
                    activity.candidate_id
                ));
            }
        }
    }
    if errors.is_empty() { Ok(drop) } else { Err(errors) }
}

/// The bytes of `data/clean/grok.csv` for this ingest.
pub fn clean_bytes(ingest: &GrokIngest) -> Result<Vec<u8>> {
    csv_bytes(
        &ingest.rows,
        "window_start,window_end,collected_at,candidate_id,mentions,mentions_method,sample_size,unique_authors,engagement,sentiment_pos,sentiment_neg,sentiment_neu,sentiment_method,bot_share_estimate,top_topics,source_file,content_hash",
    )
}

/// Writes `data/clean/grok.csv` and rewrites `data/quarantine/grok/`.
pub fn write_clean(repo: &Repo, ingest: &GrokIngest) -> Result<()> {
    write(&repo.clean_dir().join(CLEAN_FILE), &clean_bytes(ingest)?)?;
    write_quarantine(repo, "grok", &ingest.rejections)
}

/// Reads `data/clean/grok.csv`.
pub fn read_clean_rows(repo: &Repo) -> Result<Vec<GrokRow>> {
    read_csv(&repo.clean_dir().join(CLEAN_FILE))
}
