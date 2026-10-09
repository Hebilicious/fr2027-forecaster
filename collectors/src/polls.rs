//! Polls: `data/raw/polls/<poll_id>.yaml` → `data/clean/polls.csv`.
//!
//! One clean row per candidate per scenario, the spec's poll row plus the provenance columns
//! (`index_url`, `entry`, `retrieved_at`, `content_hash`).

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use fr2027_model::{Poll, Scenario, Share};
use jiff::civil::Date;
use serde::{Deserialize, Serialize};

use crate::{
    Error, Rejection, Repo, Result, SchemaKind, Schemas, config::CandidatesConfig, content_hash, list_files, read,
    schema::yaml_to_json, write, write_quarantine,
};

pub const CLEAN_FILE: &str = "polls.csv";

#[derive(Clone, Debug, Deserialize)]
pub struct PollEntry {
    pub poll_id: String,
    #[serde(default)]
    pub supersedes: Option<String>,
    pub firm: String,
    pub sponsor: Option<String>,
    pub field_start: Date,
    pub field_end: Date,
    pub published_at: Date,
    pub sample_size: u32,
    pub method: Option<String>,
    pub population: Option<String>,
    pub source_url: String,
    #[serde(default)]
    pub notice_url: Option<String>,
    #[serde(default)]
    pub index_url: Option<String>,
    pub retrieved_at: String,
    pub entry: String,
    pub scenarios: Vec<ScenarioEntry>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ScenarioEntry {
    pub scenario_id: String,
    pub round: u8,
    pub shares: BTreeMap<String, f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PollRow {
    pub poll_id: String,
    pub firm: String,
    pub sponsor: Option<String>,
    pub field_start: Date,
    pub field_end: Date,
    pub published_at: Date,
    pub sample_size: u32,
    pub method: Option<String>,
    pub population: Option<String>,
    pub round: u8,
    pub scenario_id: String,
    pub candidate_id: String,
    pub share: f64,
    pub source_url: String,
    pub notice_url: Option<String>,
    pub index_url: Option<String>,
    pub entry: String,
    pub retrieved_at: String,
    pub content_hash: String,
}

#[derive(Clone, Debug, Default)]
pub struct PollIngest {
    pub rows: Vec<PollRow>,
    pub accepted: Vec<String>,
    pub superseded: Vec<String>,
    pub rejections: Vec<Rejection>,
}

/// Validates every raw poll file. Reads only; see [`write_clean`].
pub fn ingest(repo: &Repo, schemas: &Schemas, config: &CandidatesConfig) -> Result<PollIngest> {
    let mut entries: Vec<(PollEntry, String)> = Vec::new();
    let mut rejections = Vec::new();
    for path in list_files(&repo.raw_polls_dir(), "yaml")? {
        let bytes = read(&path)?;
        match validate_file(&path, &bytes, schemas, config) {
            Ok(entry) => {
                if entries.iter().any(|(e, _)| e.poll_id == entry.poll_id) {
                    rejections.push(Rejection {
                        path,
                        errors: vec![format!("poll_id `{}` is already used by another file", entry.poll_id)],
                    });
                } else {
                    entries.push((entry, content_hash(&bytes)));
                }
            }
            Err(errors) => rejections.push(Rejection { path, errors }),
        }
    }
    let superseded: BTreeSet<String> = entries.iter().filter_map(|(e, _)| e.supersedes.clone()).collect();
    for id in &superseded {
        if !entries.iter().any(|(e, _)| e.poll_id == *id) {
            let (entry, _) = entries
                .iter()
                .find(|(e, _)| e.supersedes.as_ref() == Some(id))
                .expect("present");
            rejections.push(Rejection {
                path: repo.raw_polls_dir().join(format!("{}.yaml", entry.poll_id)),
                errors: vec![format!("supersedes unknown poll `{id}`")],
            });
        }
    }
    let mut ingest = PollIngest {
        superseded: superseded.iter().cloned().collect(),
        rejections,
        ..Default::default()
    };
    for (entry, hash) in entries {
        if superseded.contains(&entry.poll_id) {
            continue;
        }
        ingest.accepted.push(entry.poll_id.clone());
        for scenario in &entry.scenarios {
            for (candidate_id, share) in &scenario.shares {
                ingest.rows.push(PollRow {
                    poll_id: entry.poll_id.clone(),
                    firm: entry.firm.clone(),
                    sponsor: entry.sponsor.clone(),
                    field_start: entry.field_start,
                    field_end: entry.field_end,
                    published_at: entry.published_at,
                    sample_size: entry.sample_size,
                    method: entry.method.clone(),
                    population: entry.population.clone(),
                    round: scenario.round,
                    scenario_id: scenario.scenario_id.clone(),
                    candidate_id: candidate_id.clone(),
                    share: *share,
                    source_url: entry.source_url.clone(),
                    notice_url: entry.notice_url.clone(),
                    index_url: entry.index_url.clone(),
                    entry: entry.entry.clone(),
                    retrieved_at: entry.retrieved_at.clone(),
                    content_hash: hash.clone(),
                });
            }
        }
    }
    ingest.rows.sort_by(|a, b| {
        (a.published_at, &a.poll_id, a.round, &a.scenario_id, &a.candidate_id).cmp(&(
            b.published_at,
            &b.poll_id,
            b.round,
            &b.scenario_id,
            &b.candidate_id,
        ))
    });
    ingest.rejections.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(ingest)
}

fn validate_file(
    path: &Path,
    bytes: &[u8],
    schemas: &Schemas,
    config: &CandidatesConfig,
) -> Result<PollEntry, Vec<String>> {
    let text = std::str::from_utf8(bytes).map_err(|_| vec!["not UTF-8".to_string()])?;
    let value = yaml_to_json(text).map_err(|e| vec![format!("not YAML: {e}")])?;
    let violations = schemas.violations(SchemaKind::Poll, &value);
    if !violations.is_empty() {
        return Err(violations);
    }
    let entry: PollEntry = serde_json::from_value(value).map_err(|e| vec![e.to_string()])?;
    let mut errors = Vec::new();
    let stem = path
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    if stem != entry.poll_id {
        errors.push(format!("file name `{stem}` must equal poll_id `{}`", entry.poll_id));
    }
    if entry.field_start > entry.field_end {
        errors.push("field_start is after field_end".into());
    }
    if entry.field_end > entry.published_at {
        errors.push("published_at is before field_end".into());
    }
    let mut scenario_ids = BTreeSet::new();
    for scenario in &entry.scenarios {
        if !scenario_ids.insert((scenario.round, scenario.scenario_id.clone())) {
            errors.push(format!(
                "scenario `{}` appears twice in round {}",
                scenario.scenario_id, scenario.round
            ));
        }
        for candidate in scenario.shares.keys() {
            if config.candidate(candidate).is_none() {
                errors.push(format!(
                    "scenario `{}`: unknown candidate `{candidate}` (add it to config/candidates.yaml)",
                    scenario.scenario_id
                ));
            }
        }
        let total: f64 = scenario.shares.values().sum();
        match scenario.round {
            1 if !(85.0..=103.0).contains(&total) => errors.push(format!(
                "scenario `{}`: round 1 shares sum to {total:.1}, expected 85–103",
                scenario.scenario_id
            )),
            2 if scenario.shares.len() != 2 => errors.push(format!(
                "scenario `{}`: a round 2 scenario has exactly two candidates",
                scenario.scenario_id
            )),
            2 if !(98.0..=102.0).contains(&total) => errors.push(format!(
                "scenario `{}`: round 2 shares sum to {total:.1}, expected 98–102",
                scenario.scenario_id
            )),
            _ => {}
        }
    }
    if errors.is_empty() { Ok(entry) } else { Err(errors) }
}

/// The bytes of `data/clean/polls.csv` for this ingest.
pub fn clean_bytes(ingest: &PollIngest) -> Result<Vec<u8>> {
    if ingest.rows.is_empty() {
        return Ok(b"poll_id,firm,sponsor,field_start,field_end,published_at,sample_size,method,population,round,scenario_id,candidate_id,share,source_url,notice_url,index_url,entry,retrieved_at,content_hash\n".to_vec());
    }
    let mut writer = csv::Writer::from_writer(Vec::new());
    for row in &ingest.rows {
        writer.serialize(row)?;
    }
    writer.into_inner().map_err(|e| Error::Csv(e.into_error().into()))
}

/// Writes `data/clean/polls.csv` and rewrites `data/quarantine/polls/`.
pub fn write_clean(repo: &Repo, ingest: &PollIngest) -> Result<()> {
    write(&repo.clean_dir().join(CLEAN_FILE), &clean_bytes(ingest)?)?;
    write_quarantine(repo, "polls", &ingest.rejections)
}

/// Reads `data/clean/polls.csv`.
pub fn read_clean_rows(repo: &Repo) -> Result<Vec<PollRow>> {
    let path = repo.clean_dir().join(CLEAN_FILE);
    let bytes = read(&path)?;
    let mut reader = csv::Reader::from_reader(bytes.as_slice());
    reader
        .deserialize()
        .collect::<Result<Vec<PollRow>, _>>()
        .map_err(|e| Error::invalid(&path, e.to_string()))
}

/// Groups clean rows into the model's polls, in file order.
pub fn to_polls(rows: &[PollRow]) -> Vec<Poll> {
    let mut polls: Vec<Poll> = Vec::new();
    for row in rows {
        let index = match polls.iter().position(|p| p.poll_id == row.poll_id) {
            Some(index) => index,
            None => {
                polls.push(Poll {
                    poll_id: row.poll_id.clone(),
                    firm: row.firm.clone(),
                    field_start: row.field_start,
                    field_end: row.field_end,
                    published_at: row.published_at,
                    sample_size: row.sample_size,
                    scenarios: Vec::new(),
                });
                polls.len() - 1
            }
        };
        let poll = &mut polls[index];
        let scenario = match poll
            .scenarios
            .iter()
            .position(|s| s.round == row.round && s.scenario_id == row.scenario_id)
        {
            Some(s) => &mut poll.scenarios[s],
            None => {
                poll.scenarios.push(Scenario {
                    scenario_id: row.scenario_id.clone(),
                    round: row.round,
                    shares: Vec::new(),
                });
                poll.scenarios.last_mut().expect("just pushed")
            }
        };
        scenario.shares.push(Share {
            candidate_id: row.candidate_id.clone(),
            share: row.share,
        });
    }
    polls
}
