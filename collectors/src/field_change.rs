//! Field changes an agent sends through the inbox (`schemas/field_change.schema.json`): a few
//! closed operations on `config/candidates.yaml`, applied to the affected blocks only, with an
//! entry in `research/LOG.md`. A change is checked as a whole and either applied or rejected
//! with every reason; nothing is written for a rejected change.

use std::collections::BTreeSet;

use fr2027_model::SlotOption;
use jiff::civil::Date;
use serde::Deserialize;

use crate::{
    SchemaKind, Schemas,
    config::{CandidateEntry, CandidatesConfig, SlotEntry, SourceRef},
    polls::Agent,
    sources::SourcesConfig,
    validate_value,
    yaml_text::{self, Edits, plain_id, quote},
};

#[derive(Clone, Debug, Deserialize)]
pub struct FieldChange {
    pub schema_version: String,
    pub change_id: String,
    pub event_ids: Vec<String>,
    pub sources: Vec<ChangeSource>,
    pub summary: String,
    pub log: String,
    pub agent: Agent,
    pub operations: Vec<Operation>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ChangeSource {
    pub url: String,
    pub title: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum Operation {
    SetStatus {
        candidate_id: String,
        status: String,
        #[serde(default)]
        notes: Option<String>,
        #[serde(default)]
        declared_on: Option<Date>,
        #[serde(default)]
        source: Option<SourceRef>,
    },
    SetSlot {
        slot_id: String,
        options: Vec<SlotOption>,
        rationale: String,
    },
    AddCandidate {
        candidate: CandidateEntry,
        names: Vec<String>,
        wikipedia: Option<String>,
        slot: Placement,
    },
}

#[derive(Clone, Debug, Deserialize)]
pub struct Placement {
    pub slot_id: String,
    pub p: f64,
    #[serde(default)]
    pub label: Option<String>,
    pub rationale: String,
}

/// The three files a field change may touch, as text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldFiles {
    pub candidates: String,
    pub sources: String,
    pub log: String,
}

/// Checks a payload against the schema.
pub fn parse(payload: serde_json::Value, schemas: &Schemas) -> Result<FieldChange, Vec<String>> {
    validate_value(payload, schemas, SchemaKind::FieldChange)
}

/// The text that marks a change in the research log; a change found there is already applied.
pub fn log_marker(change_id: &str) -> String {
    format!("field_change `{change_id}`")
}

pub fn already_applied(log: &str, change_id: &str) -> bool {
    log.contains(&log_marker(change_id))
}

/// Applies `change` to the files, or returns every reason it cannot be applied. `event_known`
/// says whether an accepted event with that id is in the repository; `today` becomes `as_of`
/// (unless the file is already newer) and the log entry's date.
pub fn apply(
    change: &FieldChange,
    files: &FieldFiles,
    schemas: &Schemas,
    event_known: impl Fn(&str) -> bool,
    today: Date,
) -> Result<FieldFiles, Vec<String>> {
    let before = CandidatesConfig::from_text(&files.candidates, schemas).map_err(|e| {
        vec![format!(
            "config/candidates.yaml does not validate before the change: {e}"
        )]
    })?;
    let mut errors: Vec<String> = change
        .event_ids
        .iter()
        .filter(|id| !event_known(id))
        .map(|id| format!("event `{id}` is not in data/raw/events/; send the event first"))
        .collect();

    let mut after = before.clone();
    let mut new_names: Vec<(String, Vec<String>, Option<String>)> = Vec::new();
    for (n, operation) in change.operations.iter().enumerate() {
        let at = format!("operation {}", n + 1);
        match operation {
            Operation::SetStatus {
                candidate_id,
                status,
                notes,
                declared_on,
                source,
            } => {
                let Some(candidate) = after.candidates.iter_mut().find(|c| c.id == *candidate_id) else {
                    errors.push(format!("{at}: unknown candidate `{candidate_id}`"));
                    continue;
                };
                candidate.status.clone_from(status);
                if notes.is_some() {
                    candidate.notes.clone_from(notes);
                }
                if declared_on.is_some() {
                    candidate.declared_on = *declared_on;
                }
                if let Some(source) = source
                    && !candidate.sources.iter().any(|s| s.url == source.url)
                {
                    candidate.sources.push(source.clone());
                }
            }
            Operation::SetSlot {
                slot_id,
                options,
                rationale,
            } => {
                let Some(slot) = after.slots.iter_mut().find(|s| s.id == *slot_id) else {
                    errors.push(format!(
                        "{at}: unknown slot `{slot_id}` (set_slot changes an existing slot)"
                    ));
                    continue;
                };
                let members = |options: &[SlotOption]| -> BTreeSet<String> {
                    options.iter().flat_map(|o| o.run.iter().cloned()).collect()
                };
                for dropped in members(&slot.options).difference(&members(options)) {
                    errors.push(format!(
                        "{at}: slot `{slot_id}` would lose `{dropped}`; a candidate who no longer runs stays in the slot with p 0"
                    ));
                }
                slot.options.clone_from(options);
                slot.rationale = Some(rationale.clone());
            }
            Operation::AddCandidate {
                candidate,
                names,
                wikipedia,
                slot,
            } => {
                if after.candidate(&candidate.id).is_some() {
                    errors.push(format!("{at}: candidate `{}` is already in the list", candidate.id));
                    continue;
                }
                if !after.blocs.contains(&candidate.bloc) {
                    errors.push(format!(
                        "{at}: unknown bloc `{}` (one of {})",
                        candidate.bloc,
                        after.blocs.join(", ")
                    ));
                }
                after.candidates.push(candidate.clone());
                let option = SlotOption {
                    run: vec![candidate.id.clone()],
                    p: slot.p,
                };
                match after.slots.iter_mut().find(|s| s.id == slot.slot_id) {
                    Some(existing) => {
                        existing.options.push(option);
                        existing.rationale = Some(slot.rationale.clone());
                        if slot.label.is_some() {
                            existing.label.clone_from(&slot.label);
                        }
                    }
                    None => after.slots.push(SlotEntry {
                        id: slot.slot_id.clone(),
                        label: slot.label.clone(),
                        rationale: Some(slot.rationale.clone()),
                        options: vec![option],
                    }),
                }
                new_names.push((candidate.id.clone(), names.clone(), wikipedia.clone()));
            }
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    after.as_of = after.as_of.max(today);
    if let Err(e) = after.field() {
        return Err(vec![format!("the field after the change is not valid: {e}")]);
    }

    let candidates = edit_candidates(&files.candidates, &before, &after).map_err(|e| vec![e])?;
    let config = CandidatesConfig::from_text(&candidates, schemas)
        .map_err(|e| vec![format!("config/candidates.yaml after the change: {e}")])?;
    if serde_json::to_value(&config).ok() != serde_json::to_value(&after).ok() {
        return Err(vec![
            "internal error: the edited config/candidates.yaml does not read back as the change; nothing written"
                .into(),
        ]);
    }
    let sources = add_source_names(&files.sources, &new_names).map_err(|e| vec![e])?;
    SourcesConfig::from_text(&sources, schemas, &config)
        .map_err(|e| vec![format!("config/sources.yaml after the change: {e}")])?;
    let log = insert_log_entry(&files.log, today, &log_entry(change, &before, today));
    Ok(FieldFiles {
        candidates,
        sources,
        log,
    })
}

/// Rewrites the candidate and slot blocks that differ between `before` and `after`, appends new
/// ones, and sets `as_of`; every other line stays as it is.
fn edit_candidates(text: &str, before: &CandidatesConfig, after: &CandidatesConfig) -> Result<String, String> {
    let lines = yaml_text::lines(text);
    let mut edits = Edits::default();

    let section = yaml_text::section(&lines, "candidates", true)?;
    let pad = " ".repeat(section.indent);
    for candidate in &after.candidates {
        let block = section.items.iter().find(|i| i.value["id"] == candidate.id.as_str());
        match (block, before.candidate(&candidate.id)) {
            (Some(item), Some(old)) if !same(old, candidate) => {
                edits.replace(item.start, item.end, render_candidate(candidate, &pad));
            }
            (Some(_), Some(_)) => {}
            (None, None) => edits.insert(
                section.items.last().map_or(0, |i| i.end),
                render_candidate(candidate, &pad),
            ),
            _ => return Err(format!("candidate `{}`: no single block in the file", candidate.id)),
        }
    }

    let section = yaml_text::section(&lines, "slots", true)?;
    let pad = " ".repeat(section.indent);
    for slot in &after.slots {
        let block = section.items.iter().find(|i| i.value["id"] == slot.id.as_str());
        match (block, before.slots.iter().find(|s| s.id == slot.id)) {
            (Some(item), Some(old)) if !same(old, slot) => {
                edits.replace(item.start, item.end, render_slot(slot, &pad));
            }
            (Some(_), Some(_)) => {}
            (None, None) => edits.insert(section.items.last().map_or(0, |i| i.end), render_slot(slot, &pad)),
            _ => return Err(format!("slot `{}`: no single block in the file", slot.id)),
        }
    }

    if after.as_of != before.as_of {
        let line = lines
            .iter()
            .position(|l| l.starts_with("as_of:"))
            .ok_or("no top-level `as_of:` line")?;
        edits.replace(line, line + 1, format!("as_of: {}\n", after.as_of));
    }
    Ok(edits.apply(&lines))
}

/// Whether two entries are equal, compared through their JSON form.
fn same<T: serde::Serialize>(a: &T, b: &T) -> bool {
    serde_json::to_value(a).ok() == serde_json::to_value(b).ok()
}

/// A candidate block in the file's style.
fn render_candidate(c: &CandidateEntry, pad: &str) -> String {
    let mut out = format!("{pad}- id: {}\n", plain_id(&c.id));
    let key = |out: &mut String, k: &str, v: String| out.push_str(&format!("{pad}  {k}: {v}\n"));
    key(&mut out, "name", quote(&c.name));
    key(&mut out, "party", quote(&c.party));
    key(&mut out, "bloc", plain_id(&c.bloc));
    key(&mut out, "status", c.status.clone());
    if let Some(date) = c.declared_on {
        key(&mut out, "declared_on", date.to_string());
    }
    if let Some(notes) = &c.notes {
        key(&mut out, "notes", quote(notes));
    }
    if c.sources.is_empty() {
        key(&mut out, "sources", "[]".into());
    } else {
        out.push_str(&format!("{pad}  sources:\n"));
        for s in &c.sources {
            out.push_str(&format!("{pad}    - url: {}\n", quote(&s.url)));
            if let Some(date) = s.date {
                out.push_str(&format!("{pad}      date: {date}\n"));
            }
            if let Some(note) = &s.note {
                out.push_str(&format!("{pad}      note: {}\n", quote(note)));
            }
        }
    }
    out
}

/// A slot block in the file's style: one flow mapping per option.
fn render_slot(s: &SlotEntry, pad: &str) -> String {
    let mut out = format!("{pad}- id: {}\n", plain_id(&s.id));
    if let Some(label) = &s.label {
        out.push_str(&format!("{pad}  label: {}\n", quote(label)));
    }
    if let Some(rationale) = &s.rationale {
        out.push_str(&format!("{pad}  rationale: {}\n", quote(rationale)));
    }
    out.push_str(&format!("{pad}  options:\n"));
    for option in &s.options {
        let run: Vec<String> = option.run.iter().map(|id| plain_id(id)).collect();
        out.push_str(&format!(
            "{pad}    - {{ run: [{}], p: {} }}\n",
            run.join(", "),
            option.p
        ));
    }
    out
}

/// Adds a `candidates:` entry to config/sources.yaml for each new candidate.
fn add_source_names(text: &str, new: &[(String, Vec<String>, Option<String>)]) -> Result<String, String> {
    if new.is_empty() {
        return Ok(text.to_string());
    }
    let lines = yaml_text::lines(text);
    let section = yaml_text::section(&lines, "candidates", false)?;
    let pad = " ".repeat(section.indent);
    let at = section.items.last().map_or(0, |i| i.end);
    let mut edits = Edits::default();
    for (id, names, wikipedia) in new {
        let names: Vec<String> = names.iter().map(|n| quote(n)).collect();
        let wikipedia = wikipedia.as_deref().map_or("null".to_string(), quote);
        edits.insert(
            at,
            format!(
                "{pad}{}: {{ wikipedia: {wikipedia}, names: [{}] }}\n",
                plain_id(id),
                names.join(", ")
            ),
        );
    }
    Ok(edits.apply(&lines))
}

/// The research log entry for a change.
fn log_entry(change: &FieldChange, before: &CandidatesConfig, today: Date) -> String {
    let operations: Vec<String> = change
        .operations
        .iter()
        .map(|operation| match operation {
            Operation::SetStatus {
                candidate_id, status, ..
            } => {
                let old = before.candidate(candidate_id).map_or("?", |c| c.status.as_str());
                format!("`{candidate_id}` status {old} → {status}")
            }
            Operation::SetSlot { slot_id, options, .. } => {
                let old = before
                    .slots
                    .iter()
                    .find(|s| s.id == *slot_id)
                    .map_or(String::new(), |s| describe_options(&s.options));
                format!("slot `{slot_id}` {old} → {}", describe_options(options))
            }
            Operation::AddCandidate { candidate, slot, .. } => format!(
                "new candidate `{}` ({}, {}, {}) in slot `{}` at p {}",
                candidate.id, candidate.name, candidate.bloc, candidate.status, slot.slot_id, slot.p
            ),
        })
        .collect();
    let sources: Vec<String> = change
        .sources
        .iter()
        .map(|s| format!("[{}]({})", s.title.replace('[', "(").replace(']', ")"), s.url))
        .collect();
    let events = if change.event_ids.is_empty() {
        "none".to_string()
    } else {
        change
            .event_ids
            .iter()
            .map(|id| format!("`{id}`"))
            .collect::<Vec<_>>()
            .join(", ")
    };
    format!(
        "### Field: {} (via Grok Bot {})\n\n{}\n\n{}\n\n- `config/candidates.yaml`: {}; `as_of` {today}.\n- Sources: {}.\n- Events: {events}.\n- Sent by {} ({}, run `{}`); checked and applied by the Live workflow.\n",
        change.change_id,
        log_marker(&change.change_id),
        change.summary.trim(),
        change.log.trim(),
        operations.join("; "),
        sources.join(", "),
        change.agent.name,
        change.agent.skill_version,
        change.agent.run_id,
    )
}

fn describe_options(options: &[SlotOption]) -> String {
    options
        .iter()
        .map(|o| format!("{} {}", o.run.join("+"), o.p))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Puts `entry` first under today's `## YYYY-MM-DD` heading, adding the heading above the
/// newest day when today has none yet.
pub fn insert_log_entry(log: &str, today: Date, entry: &str) -> String {
    let heading = format!("## {today}");
    let lines: Vec<&str> = log.lines().collect();
    let mut out = String::new();
    let push_lines = |out: &mut String, lines: &[&str]| {
        for line in lines {
            out.push_str(line);
            out.push('\n');
        }
    };
    if let Some(at) = lines.iter().position(|l| *l == heading) {
        // After the heading and the blank lines that follow it.
        let mut next = at + 1;
        while next < lines.len() && lines[next].trim().is_empty() {
            next += 1;
        }
        push_lines(&mut out, &lines[..=at]);
        out.push('\n');
        out.push_str(entry);
        if next < lines.len() {
            out.push('\n');
        }
        push_lines(&mut out, &lines[next..]);
    } else {
        let at = lines.iter().position(|l| l.starts_with("## ")).unwrap_or(lines.len());
        push_lines(&mut out, &lines[..at]);
        if !out.ends_with("\n\n") && !out.is_empty() {
            out.push('\n');
        }
        out.push_str(&format!("{heading}\n\n"));
        out.push_str(entry);
        if at < lines.len() {
            out.push('\n');
        }
        push_lines(&mut out, &lines[at..]);
    }
    out
}
