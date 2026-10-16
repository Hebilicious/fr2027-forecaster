//! Campaign events reported by an agent: `data/raw/events/<event_id>.json` →
//! `data/clean/events.csv`. Shown in the UI as reported; the model does not read them.

use serde::{Deserialize, Serialize};

use crate::{
    Rejection, Repo, Result, SchemaKind, Schemas, config::CandidatesConfig, csv_bytes, read_csv, read_json_documents,
    write, write_quarantine,
};

pub const CLEAN_FILE: &str = "events.csv";
const HEADER: &str = "date,event_id,kind,candidate_ids,summary,source_url,source_title,proposed_change,reported_by,reported_at,source_file";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub schema_version: String,
    pub event_id: String,
    pub date: String,
    pub kind: String,
    pub candidate_ids: Vec<String>,
    pub summary: String,
    pub sources: Vec<EventSource>,
    #[serde(default)]
    pub proposed_change: Option<String>,
    pub reported_by: String,
    pub reported_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSource {
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventRow {
    pub date: String,
    pub event_id: String,
    pub kind: String,
    /// Space-separated.
    pub candidate_ids: String,
    pub summary: String,
    pub source_url: String,
    pub source_title: Option<String>,
    pub proposed_change: Option<String>,
    pub reported_by: String,
    pub reported_at: String,
    pub source_file: String,
}

#[derive(Debug, Default)]
pub struct EventsIngest {
    pub rows: Vec<EventRow>,
    pub rejections: Vec<Rejection>,
}

pub fn check(stem: Option<&str>, event: &Event, config: &CandidatesConfig) -> Vec<String> {
    let mut errors = Vec::new();
    if let Some(stem) = stem
        && stem != event.event_id
    {
        errors.push(format!("file name `{stem}` must equal event_id `{}`", event.event_id));
    }
    if !event.event_id.starts_with(&event.date) {
        errors.push(format!(
            "event_id `{}` must start with its date {}",
            event.event_id, event.date
        ));
    }
    for id in &event.candidate_ids {
        if config.candidate(id).is_none() {
            errors.push(format!("unknown candidate `{id}`"));
        }
    }
    errors
}

pub fn ingest(repo: &Repo, schemas: &Schemas, config: &CandidatesConfig) -> Result<EventsIngest> {
    let (accepted, rejections) =
        read_json_documents::<Event>(&repo.raw_events_dir(), schemas, SchemaKind::Event, |stem, event| {
            check(Some(stem), event, config)
        })?;
    let mut rows: Vec<EventRow> = accepted
        .iter()
        .map(|a| {
            let event = &a.document;
            let first = event.sources.first();
            EventRow {
                date: event.date.clone(),
                event_id: event.event_id.clone(),
                kind: event.kind.clone(),
                candidate_ids: event.candidate_ids.join(" "),
                summary: event.summary.clone(),
                source_url: first.map(|s| s.url.clone()).unwrap_or_default(),
                source_title: first.and_then(|s| s.title.clone()),
                proposed_change: event.proposed_change.clone(),
                reported_by: event.reported_by.clone(),
                reported_at: event.reported_at.clone(),
                source_file: repo.relative(&a.path),
            }
        })
        .collect();
    rows.sort_by(|a, b| (&a.date, &a.event_id).cmp(&(&b.date, &b.event_id)));
    Ok(EventsIngest { rows, rejections })
}

pub fn clean_bytes(ingest: &EventsIngest) -> Result<Vec<u8>> {
    csv_bytes(&ingest.rows, HEADER)
}

pub fn write_clean(repo: &Repo, ingest: &EventsIngest) -> Result<()> {
    write(&repo.clean_dir().join(CLEAN_FILE), &clean_bytes(ingest)?)?;
    write_quarantine(repo, "events", &ingest.rejections)
}

pub fn read_clean_rows(repo: &Repo) -> Result<Vec<EventRow>> {
    read_csv(&repo.clean_dir().join(CLEAN_FILE))
}
