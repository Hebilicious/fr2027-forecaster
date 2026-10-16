//! `process` on a scratch repository: polls and their blind checks, field changes.

use std::fs;
use std::path::Path;

use fr2027_collectors::Repo;
use fr2027_fetch::inbox::{PendingItem, Resolution};
use jiff::civil::date;
use serde_json::{Value, json};

use super::{OpenProposal, Outcome, PrAction, process};

const CANDIDATES: &str = r#"# Header comment, kept.
schema_version: "1.0"
as_of: 2026-10-01
blocs: [left, centre, right]
candidates:
  - id: anna
    name: "Anna"
    party: "Left party"
    bloc: left
    status: declared
    sources: []
  # A comment between blocks, kept.
  - id: bruno
    name: "Bruno"
    party: "Centre party"
    bloc: centre
    status: declared
    sources: []
  - id: chloe
    name: "Chloé"
    party: "Right party"
    bloc: right
    status: possible
    sources: []
slots:
  - id: anna
    options:
      - { run: [anna], p: 0.9 }
  - id: centre_right
    rationale: "One of them."
    options:
      - { run: [bruno], p: 0.6 }
      - { run: [chloe], p: 0.3 }
"#;

const SOURCES: &str = r#"schema_version: "1.0"
user_agent: "fr2027-forecaster tests (+https://example.org)"
markets: []
feeds: []
wikipedia: { project: fr.wikipedia.org }
candidates:
  anna: { wikipedia: null, names: ["Anna"] }
  bruno: { wikipedia: null, names: ["Bruno"] }
  chloe: { wikipedia: null, names: ["Chloé"] }
publication_blackouts: []
"#;

const LOG: &str = "# Research log\n\n## 2026-10-01\n\n### Start\n\nText.\n";

fn scratch() -> (tempfile::TempDir, Repo) {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    for sub in ["schemas", "config", "research", "data/raw/polls", "data/raw/events"] {
        fs::create_dir_all(root.join(sub)).unwrap();
    }
    for entry in fs::read_dir(source.join("schemas")).unwrap() {
        let entry = entry.unwrap();
        fs::copy(entry.path(), root.join("schemas").join(entry.file_name())).unwrap();
    }
    fs::write(root.join("config/candidates.yaml"), CANDIDATES).unwrap();
    fs::write(root.join("config/sources.yaml"), SOURCES).unwrap();
    fs::write(root.join("research/LOG.md"), LOG).unwrap();
    fs::write(root.join("data/raw/polls/ifop-2026-09-20.yaml"), "schema_version: \"1.0\"\npoll_id: ifop-2026-09-20\nfirm: ifop\nsponsor: null\nfield_start: 2026-09-18\nfield_end: 2026-09-19\npublished_at: 2026-09-20\nsample_size: 1000\nmethod: online\npopulation: registered\nsource_url: https://example.org/old.pdf\nretrieved_at: 2026-09-20T08:00:00Z\nentry: index\nscenarios:\n  - { scenario_id: S1, round: 1, shares: { anna: 40, bruno: 35, chloe: 25 } }\n").unwrap();
    let repo = Repo::new(root);
    (dir, repo)
}

fn poll(run_id: &str) -> Value {
    json!({
        "schema_version": "1.0",
        "poll_id": "elabe-2026-10-14-10320",
        "firm": "elabe",
        "sponsor": "BFMTV",
        "field_start": "2026-10-12",
        "field_end": "2026-10-14",
        "published_at": "2026-10-15",
        "sample_size": 1500,
        "method": "online",
        "population": "registered",
        "source_url": "https://example.org/notice.pdf",
        "notice_url": "https://example.org/notice.pdf",
        "retrieved_at": "2026-10-15T08:00:00Z",
        "entry": "primary",
        "agent": {"name": "Grok Bot fr2027", "skill_version": "polls-2", "run_id": run_id},
        "scenarios": [
            {"scenario_id": "S1", "round": 1, "shares": {"anna": 40, "bruno": 35, "chloe": 25}},
            {"scenario_id": "H1", "round": 2, "shares": {"anna": 52, "bruno": 48}}
        ]
    })
}

fn item(id: &str, kind: &str, payload: Value) -> PendingItem {
    PendingItem {
        id: id.into(),
        kind: kind.into(),
        received_at: "2026-10-15T09:00:00Z".into(),
        payload,
    }
}

/// The proposal of `poll(run)` as Live proposed it in an earlier run, in pull request `pr`.
fn open_proposal(pr: u64, run: &str) -> OpenProposal {
    let yaml = super::poll_yaml(&poll(run)).unwrap();
    OpenProposal {
        pr,
        poll_id: "elabe-2026-10-14-10320".into(),
        bytes: yaml.into_bytes(),
        inbox_item: Some("item-poll".into()),
    }
}

fn status<'a>(list: &'a [Resolution], id: &str) -> &'a Resolution {
    list.iter()
        .find(|r| r.id == id)
        .unwrap_or_else(|| panic!("no resolution for {id} in {list:?}"))
}

fn run(repo: &Repo, items: &[PendingItem], open: &[OpenProposal]) -> Outcome {
    process(repo, items, open, date(2026, 10, 16)).unwrap()
}

const RAW: &str = "data/raw/polls/elabe-2026-10-14-10320.yaml";

#[test]
fn a_matching_check_in_the_same_pull_lands_the_poll() {
    let (_dir, repo) = scratch();
    let mut check = poll("checks-2026-10-15T08:30Z");
    // Another order and other scenario ids; urls and notes are not compared.
    check["scenarios"] = json!([
        {"scenario_id": "R2-a", "round": 2, "shares": {"bruno": 48, "anna": 52}},
        {"scenario_id": "T1", "round": 1, "shares": {"chloe": 25, "bruno": 35, "anna": 40}}
    ]);
    check["source_url"] = json!("https://example.org/report.pdf");
    let out = run(
        &repo,
        &[
            item("item-check", "poll_check", check),
            item("item-poll", "poll", poll("polls-2026-10-15T07:30Z")),
        ],
        &[],
    );
    assert!(out.now.is_empty(), "{:?}", out.now);
    assert_eq!(status(&out.after_push, "item-poll").status, "accepted");
    assert_eq!(status(&out.after_push, "item-check").status, "accepted");
    let landed = fs::read_to_string(repo.root().join(RAW)).unwrap();
    assert!(
        landed.contains("run_id: polls-2026-10-15T07:30Z"),
        "the proposal lands, not the check"
    );
    assert!(!repo.proposals_dir().join("polls/elabe-2026-10-14-10320.yaml").exists());
    assert!(!repo.proposals_dir().join("polls/elabe-2026-10-14-10320.json").exists());
}

#[test]
fn a_matching_check_of_an_open_pull_request_lands_and_closes_it() {
    let (_dir, repo) = scratch();
    let open = [open_proposal(7, "polls-2026-10-15T07:30Z")];
    let out = run(
        &repo,
        &[item("item-check", "poll_check", poll("checks-2026-10-15T08:30Z"))],
        &open,
    );
    assert!(out.now.is_empty(), "{:?}", out.now);
    assert_eq!(status(&out.after_push, "item-check").status, "accepted");
    let followup = status(&out.after_push, "item-poll");
    assert!(followup.followup && followup.status == "accepted");
    assert_eq!(fs::read(repo.root().join(RAW)).unwrap(), open[0].bytes);
    assert_eq!(out.pr_actions.len(), 1);
    let action: &PrAction = &out.pr_actions[0];
    assert!(action.close && action.needs_commit && action.number == 7);
    assert!(action.comment.contains("matches this proposal"));
    assert!(action.comment.contains("committed to main in {commit}"));
}

#[test]
fn a_differing_check_rejects_both_and_keeps_the_proposal_open() {
    let (_dir, repo) = scratch();
    let open = [open_proposal(7, "polls-2026-10-15T07:30Z")];
    let mut check = poll("checks-2026-10-15T08:30Z");
    check["sample_size"] = json!(1501);
    check["scenarios"][0]["shares"] = json!({"anna": 41, "bruno": 34, "chloe": 25});
    let out = run(&repo, &[item("item-check", "poll_check", check)], &open);
    let rejected = status(&out.now, "item-check");
    assert_eq!(rejected.status, "rejected");
    let proposer = status(&out.now, "item-poll");
    assert!(proposer.followup && proposer.status == "rejected");
    assert!(!repo.root().join(RAW).exists());
    assert_eq!(out.pr_actions.len(), 1);
    let action = &out.pr_actions[0];
    assert!(!action.close && !action.needs_commit);
    // Every note and comment names what differs and holds no value from either entry: the
    // routines settle it by re-reading the notice.
    for text in [&rejected.note, &proposer.note, &action.comment] {
        assert!(text.contains("sample_size differs"), "{text}");
        assert!(text.contains("scenario S1 (check: S1): anna, bruno differ"), "{text}");
        for value in ["1500", "1501", "40", "41", "35", "34", "25"] {
            assert!(!text.contains(value), "`{value}` leaks into: {text}");
        }
    }
    assert!(action.comment.contains("- sample_size differs\n"));
}

#[test]
fn a_check_with_nothing_to_check_is_rejected() {
    let (_dir, repo) = scratch();
    let out = run(
        &repo,
        &[item("item-check", "poll_check", poll("checks-2026-10-15T08:30Z"))],
        &[],
    );
    assert!(
        status(&out.now, "item-check")
            .note
            .starts_with("nothing to check: no open proposal")
    );

    let mut old = poll("checks-2026-10-15T08:30Z");
    old["poll_id"] = json!("ifop-2026-09-20");
    let out = run(&repo, &[item("item-check", "poll_check", old)], &[]);
    assert!(status(&out.now, "item-check").note.contains("already on main"));
}

#[test]
fn a_check_from_the_proposals_run_is_rejected() {
    let (_dir, repo) = scratch();
    let same = "polls-2026-10-15T07:30Z";
    let out = run(
        &repo,
        &[
            item("item-poll", "poll", poll(same)),
            item("item-check", "poll_check", poll(same)),
        ],
        &[],
    );
    assert!(status(&out.now, "item-check").note.contains("same routine run"));
    assert_eq!(status(&out.after_push, "item-poll").status, "proposed");
    assert!(!repo.root().join(RAW).exists());

    let mut anonymous = poll(same);
    anonymous.as_object_mut().unwrap().remove("agent");
    let out = run(&repo, &[item("item-check", "poll_check", anonymous.clone())], &[]);
    assert!(status(&out.now, "item-check").note.contains("agent"));
    let out = run(&repo, &[item("item-poll", "poll", anonymous)], &[]);
    assert!(status(&out.now, "item-poll").note.contains("must carry agent"));
}

#[test]
fn a_newer_poll_replaces_the_open_proposal() {
    let (_dir, repo) = scratch();
    let open = [open_proposal(7, "polls-2026-10-15T07:30Z")];
    let out = run(
        &repo,
        &[item("item-new", "poll", poll("polls-2026-10-15T17:30Z"))],
        &open,
    );
    assert_eq!(status(&out.after_push, "item-new").status, "proposed");
    let old = status(&out.now, "item-poll");
    assert!(old.followup && old.status == "rejected" && old.note.contains("replaced"));
    assert!(out.pr_actions[0].close && !out.pr_actions[0].needs_commit);
    assert!(repo.proposals_dir().join("polls/elabe-2026-10-14-10320.json").exists());
}

#[test]
fn superseding_polls_must_name_a_poll_on_main() {
    let (_dir, repo) = scratch();
    let mut late = poll("checks-2026-10-15T08:30Z");
    late["supersedes"] = json!("ifop-2026-09-02");
    let out = run(&repo, &[item("item-late", "poll", late.clone())], &[]);
    assert!(status(&out.now, "item-late").note.contains("supersedes unknown poll"));
    late["supersedes"] = json!("ifop-2026-09-20");
    let out = run(&repo, &[item("item-late", "poll", late)], &[]);
    assert_eq!(status(&out.after_push, "item-late").status, "proposed");
}

fn field_change(operations: Value, event_ids: Value) -> Value {
    json!({
        "schema_version": "1.0",
        "change_id": "2026-10-16-chloe-declares",
        "event_ids": event_ids,
        "sources": [{"url": "https://www.lemonde.fr/x", "title": "Chloé declares"}],
        "summary": "Chloé declared her candidacy on 16 October.",
        "log": "Chloé announced her candidacy on national television; status possible to declared.",
        "agent": {"name": "Grok Bot fr2027", "skill_version": "checks-1", "run_id": "checks-2026-10-16T08:30Z"},
        "operations": operations
    })
}

#[test]
fn a_field_change_is_applied_and_logged_once() {
    let (_dir, repo) = scratch();
    let event = json!({
        "schema_version": "1.0", "event_id": "2026-10-16-chloe-declares", "date": "2026-10-16",
        "kind": "declaration", "candidate_ids": ["chloe"], "summary": {"fr": "Chloé a annoncé sa candidature.", "en": "Chloé declared her candidacy."},
        "sources": [{"url": "https://www.lemonde.fr/x"}], "reported_by": "Grok Bot fr2027",
        "reported_at": "2026-10-16T20:00:00Z"
    });
    let change = field_change(
        json!([{"op": "set_status", "candidate_id": "chloe", "status": "declared", "declared_on": "2026-10-16"}]),
        json!(["2026-10-16-chloe-declares"]),
    );
    // The event comes in the same pull, after the change: events are handled first.
    let out = run(
        &repo,
        &[
            item("item-change", "field_change", change.clone()),
            item("item-event", "event", event),
        ],
        &[],
    );
    assert!(out.now.is_empty(), "{:?}", out.now);
    assert_eq!(out.field_changes, ["2026-10-16-chloe-declares"]);
    let config = fs::read_to_string(repo.config_dir().join("candidates.yaml")).unwrap();
    assert_eq!(
        config,
        CANDIDATES.replace("as_of: 2026-10-01", "as_of: 2026-10-16").replace(
            "    status: possible\n",
            "    status: declared\n    declared_on: 2026-10-16\n"
        )
    );
    let log = fs::read_to_string(repo.root().join("research/LOG.md")).unwrap();
    assert!(log.starts_with("# Research log\n\n## 2026-10-16\n\n### Field: 2026-10-16-chloe-declares"));

    // Sent again (a run that could not resolve it): accepted, nothing written twice.
    let out = run(&repo, &[item("item-again", "field_change", change)], &[]);
    assert!(
        status(&out.after_push, "item-again")
            .note
            .starts_with("already applied")
    );
    assert!(out.field_changes.is_empty());
    assert_eq!(fs::read_to_string(repo.root().join("research/LOG.md")).unwrap(), log);
}

#[test]
fn an_invalid_field_change_writes_nothing() {
    let (_dir, repo) = scratch();
    let change = field_change(
        json!([{"op": "set_slot", "slot_id": "centre_right", "rationale": "Too much.",
                "options": [{"run": ["bruno"], "p": 0.8}, {"run": ["chloe"], "p": 0.3}]}]),
        json!(["2026-10-16-not-sent"]),
    );
    let out = run(&repo, &[item("item-change", "field_change", change)], &[]);
    let note = &status(&out.now, "item-change").note;
    assert!(
        note.contains("event `2026-10-16-not-sent` is not in data/raw/events/"),
        "{note}"
    );
    assert!(out.field_changes.is_empty());
    assert_eq!(
        fs::read_to_string(repo.config_dir().join("candidates.yaml")).unwrap(),
        CANDIDATES
    );
    assert_eq!(fs::read_to_string(repo.root().join("research/LOG.md")).unwrap(), LOG);
}

#[test]
fn a_field_change_adds_a_candidate_a_poll_can_use_in_the_same_pull() {
    let (_dir, repo) = scratch();
    let change = field_change(
        json!([{"op": "add_candidate",
                "candidate": {"id": "david", "name": "David", "party": "Other party", "bloc": "right",
                              "status": "declared", "sources": [{"url": "https://www.lemonde.fr/d"}]},
                "names": ["David"], "wikipedia": null,
                "slot": {"slot_id": "david", "p": 0.2, "rationale": "Declared on 16 October."}}]),
        json!([]),
    );
    let mut with_david = poll("polls-2026-10-16T07:30Z");
    with_david["scenarios"][0]["shares"] = json!({"anna": 40, "bruno": 30, "chloe": 20, "david": 10});
    let out = run(
        &repo,
        &[
            item("item-poll", "poll", with_david),
            item("item-change", "field_change", change),
        ],
        &[],
    );
    assert!(out.now.is_empty(), "{:?}", out.now);
    assert_eq!(status(&out.after_push, "item-poll").status, "proposed");
    let config = fs::read_to_string(repo.config_dir().join("candidates.yaml")).unwrap();
    assert!(config.contains("  # A comment between blocks, kept.\n"));
    assert!(config.starts_with("# Header comment, kept.\n"));
    let sources = fs::read_to_string(repo.config_dir().join("sources.yaml")).unwrap();
    assert!(sources.contains("  david: { wikipedia: null, names: [\"David\"] }\n"));
}
