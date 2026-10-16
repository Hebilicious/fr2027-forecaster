//! Field changes applied to copies of config/candidates.yaml and config/sources.yaml as they were
//! on 2026-10-16 (`tests/fixtures/`), so the tests keep working as the real field changes.

use std::fs;
use std::path::Path;

use fr2027_collectors::{
    Repo, Schemas,
    config::CandidatesConfig,
    field_change::{self, FieldFiles},
};
use jiff::civil::date;
use serde_json::{Value, json};

const LOG: &str =
    "# Research log\n\nIntro.\n\n## 2026-10-16\n\n### Older entry\n\nText.\n\n## 2026-10-09\n\n### Oldest\n";

fn repo_root() -> &'static Path {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/.."))
}

fn schemas() -> Schemas {
    Schemas::load(&Repo::new(repo_root())).unwrap()
}

fn files() -> FieldFiles {
    FieldFiles {
        candidates: fs::read_to_string(repo_root().join("collectors/tests/fixtures/candidates.yaml")).unwrap(),
        sources: fs::read_to_string(repo_root().join("collectors/tests/fixtures/sources.yaml")).unwrap(),
        log: LOG.into(),
    }
}

fn change(operations: Value) -> Value {
    json!({
        "schema_version": "1.0",
        "change_id": "2026-10-17-test-change",
        "event_ids": ["2026-10-17-some-event"],
        "sources": [{"url": "https://www.franceinfo.fr/a", "title": "A [headline]"}],
        "summary": "A test change to the field, for the tests.",
        "log": "What changed and why, in a short paragraph.",
        "agent": {"name": "Grok Bot fr2027", "skill_version": "checks-1", "run_id": "checks-2026-10-17T08:30Z"},
        "operations": operations
    })
}

fn apply(payload: Value) -> Result<FieldFiles, Vec<String>> {
    let schemas = schemas();
    let change = field_change::parse(payload, &schemas)?;
    field_change::apply(
        &change,
        &files(),
        &schemas,
        |id| id == "2026-10-17-some-event",
        date(2026, 10, 17),
    )
}

/// Lines of `after` that are not in `before`, and the reverse.
fn line_diff(before: &str, after: &str) -> (Vec<String>, Vec<String>) {
    let b: Vec<&str> = before.lines().collect();
    let a: Vec<&str> = after.lines().collect();
    let added = a.iter().filter(|l| !b.contains(l)).map(|l| l.to_string()).collect();
    let removed = b.iter().filter(|l| !a.contains(l)).map(|l| l.to_string()).collect();
    (added, removed)
}

#[test]
fn set_status_and_set_slot_touch_only_their_blocks() {
    let out = apply(change(json!([
        {"op": "set_status", "candidate_id": "faure", "status": "withdrawn",
         "notes": "Lost the primary to Glucksmann on 11 Oct 2026 and backs him.",
         "source": {"url": "https://www.publicsenat.fr/x", "date": "2026-10-12"}},
        {"op": "set_slot", "slot_id": "le_maire", "rationale": "Said on 17 October 2026 he will not run.",
         "options": [{"run": ["le_maire"], "p": 0}]}
    ])))
    .unwrap();
    let before = files();
    let (added, removed) = line_diff(&before.candidates, &out.candidates);
    assert_eq!(
        removed,
        [
            "as_of: 2026-10-16",
            "    notes: \"PS first secretary; candidate in the PS-PP-GRS primary (declared on TF1, 30 Aug 2026).\"",
            "    rationale: \"Said he would decide in October 2026; no decision found by 2026-10-09.\"",
            "      - { run: [le_maire], p: 0.1 }",
        ]
    );
    assert_eq!(
        added,
        [
            "as_of: 2026-10-17",
            "    status: withdrawn",
            "    notes: \"Lost the primary to Glucksmann on 11 Oct 2026 and backs him.\"",
            "      - url: \"https://www.publicsenat.fr/x\"",
            "        date: 2026-10-12",
            "    rationale: \"Said on 17 October 2026 he will not run.\"",
            "      - { run: [le_maire], p: 0 }",
        ]
    );
    // Faure's block is rewritten whole (status, notes, one more source); the rest is untouched.
    assert_eq!(out.candidates.lines().count(), before.candidates.lines().count() + 2);
    assert!(
        out.candidates
            .starts_with("# The canonical candidate list and field model.")
    );
    let config = CandidatesConfig::from_text(&out.candidates, &schemas()).unwrap();
    assert_eq!(config.candidate("faure").unwrap().status, "withdrawn");
    assert_eq!(config.candidate("faure").unwrap().sources.len(), 2);
    assert_eq!(out.sources, before.sources);

    // The log entry goes first under today's new heading.
    let entry = out
        .log
        .find("## 2026-10-17\n\n### Field: 2026-10-17-test-change")
        .unwrap();
    assert!(entry < out.log.find("## 2026-10-16").unwrap());
    assert!(out.log.contains("via Grok Bot field_change `2026-10-17-test-change`"));
    assert!(
        out.log
            .contains("`faure` status declared → withdrawn; slot `le_maire` le_maire 0.1 → le_maire 0")
    );
    assert!(out.log.contains("[A (headline)](https://www.franceinfo.fr/a)"));
    assert!(out.log.contains("run `checks-2026-10-17T08:30Z`"));
    assert!(field_change::already_applied(&out.log, "2026-10-17-test-change"));
}

#[test]
fn add_candidate_adds_a_block_a_slot_and_names() {
    let out = apply(change(json!([
        {"op": "add_candidate",
         "candidate": {"id": "knafo", "name": "Sarah Knafo", "party": "Reconquête", "bloc": "far_right",
                       "status": "declared", "declared_on": "2026-10-17", "notes": "Said \"I am a candidate\" on TF1.",
                       "sources": [{"url": "https://www.tf1info.fr/x", "date": "2026-10-17"}]},
         "names": ["Sarah Knafo", "Knafo"], "wikipedia": "Sarah_Knafo",
         "slot": {"slot_id": "knafo", "p": 0.3, "rationale": "Declared 17 October 2026."}},
        {"op": "add_candidate",
         "candidate": {"id": "darmanin", "name": "Gérald Darmanin", "party": "Renaissance (RE)", "bloc": "centre",
                       "status": "possible", "sources": [{"url": "https://www.lemonde.fr/x"}]},
         "names": ["Gérald Darmanin"], "wikipedia": null,
         "slot": {"slot_id": "centre", "p": 0.01, "rationale": "Both declared; Darmanin floated a run."}}
    ])))
    .unwrap();
    let before = files();
    let (added, removed) = line_diff(&before.candidates, &out.candidates);
    assert_eq!(
        removed,
        [
            "as_of: 2026-10-16",
            "    rationale: \"Both declared, no agreed tie-break; Philippe leads Attal in every poll that tests both. Probabilities follow the thin Polymarket 'Attal vs Philippe' market on 2026-10-09 (Philippe only 0.795, Attal only 0.1355, both 0.07, neither 0.028).\"",
        ]
    );
    assert!(added.contains(&"    notes: 'Said \"I am a candidate\" on TF1.'".to_string()));
    assert!(added.contains(&"      - { run: [darmanin], p: 0.01 }".to_string()));
    assert!(out.candidates.ends_with(
        "  - id: knafo\n    rationale: \"Declared 17 October 2026.\"\n    options:\n      - { run: [knafo], p: 0.3 }\n"
    ));
    // The new candidates come right after the last one, before `slots:`.
    let darmanin = out.candidates.find("  - id: darmanin\n").unwrap();
    assert!(out.candidates.find("  - id: bonnal\n").unwrap() < darmanin);
    assert!(darmanin < out.candidates.find("\nslots:\n").unwrap());
    let config = CandidatesConfig::from_text(&out.candidates, &schemas()).unwrap();
    assert_eq!(config.candidates.len(), 33);
    let (added, removed) = line_diff(&before.sources, &out.sources);
    assert!(removed.is_empty());
    assert_eq!(
        added,
        [
            "  knafo: { wikipedia: \"Sarah_Knafo\", names: [\"Sarah Knafo\", \"Knafo\"] }",
            "  darmanin: { wikipedia: null, names: [\"Gérald Darmanin\"] }",
        ]
    );
}

#[test]
fn an_invalid_change_is_rejected_with_every_reason() {
    let errors = apply(json!({
        "schema_version": "1.0",
        "change_id": "2026-10-17-bad",
        "event_ids": ["2026-10-01-unknown-event"],
        "sources": [{"url": "https://www.franceinfo.fr/a", "title": "A"}],
        "summary": "A change that cannot be applied.",
        "log": "This change has several problems on purpose.",
        "agent": {"name": "Grok Bot fr2027", "skill_version": "checks-1", "run_id": "checks-2026-10-17T08:30Z"},
        "operations": [
            {"op": "set_status", "candidate_id": "nobody", "status": "withdrawn"},
            {"op": "set_slot", "slot_id": "ps_primary", "rationale": "Drops the losers.",
             "options": [{"run": ["glucksmann"], "p": 0.95}]},
            {"op": "set_slot", "slot_id": "nope", "rationale": "x", "options": [{"run": ["arthaud"], "p": 0.5}]},
            {"op": "add_candidate",
             "candidate": {"id": "melenchon", "name": "J", "party": "P", "bloc": "left", "status": "declared",
                           "sources": [{"url": "https://example.org"}]},
             "names": ["Jean"], "wikipedia": null, "slot": {"slot_id": "x", "p": 0.1, "rationale": "x"}}
        ]
    }))
    .unwrap_err();
    assert_eq!(
        errors,
        [
            "event `2026-10-01-unknown-event` is not in data/raw/events/; send the event first",
            "operation 1: unknown candidate `nobody`",
            "operation 2: slot `ps_primary` would lose `faure`; a candidate who no longer runs stays in the slot with p 0",
            "operation 2: slot `ps_primary` would lose `guedj`; a candidate who no longer runs stays in the slot with p 0",
            "operation 2: slot `ps_primary` would lose `maurel`; a candidate who no longer runs stays in the slot with p 0",
            "operation 2: slot `ps_primary` would lose `royal`; a candidate who no longer runs stays in the slot with p 0",
            "operation 3: unknown slot `nope` (set_slot changes an existing slot)",
            "operation 4: candidate `melenchon` is already in the list",
        ]
    );
}

#[test]
fn slot_sums_and_schema_are_checked() {
    let errors = apply(change(json!([
        {"op": "set_slot", "slot_id": "rn", "rationale": "Too much.",
         "options": [{"run": ["le_pen"], "p": 0.95}, {"run": ["bardella"], "p": 0.1}]}
    ])))
    .unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(
        errors[0].starts_with("the field after the change is not valid"),
        "{errors:?}"
    );

    let mut payload = change(json!([{"op": "set_status", "candidate_id": "faure", "status": "gone"}]));
    payload["summary"] = json!("Two\nlines");
    assert!(apply(payload).unwrap_err().len() >= 2);
    assert!(apply(change(json!([]))).is_err());
}

#[test]
fn log_entry_goes_first_under_an_existing_heading() {
    let out = field_change::insert_log_entry(LOG, date(2026, 10, 16), "### New\n\nBody.\n");
    assert_eq!(
        out,
        "# Research log\n\nIntro.\n\n## 2026-10-16\n\n### New\n\nBody.\n\n### Older entry\n\nText.\n\n## 2026-10-09\n\n### Oldest\n"
    );
    let empty = field_change::insert_log_entry("# Research log\n", date(2026, 10, 16), "### New\n");
    assert_eq!(empty, "# Research log\n\n## 2026-10-16\n\n### New\n");
}
