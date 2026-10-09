//! Ingest against a scratch repository holding the real schemas.

use std::fs;
use std::path::Path;

use fr2027_collectors::{Repo, Schemas, config::CandidatesConfig, grok, polls};

const CANDIDATES: &str = r#"
schema_version: "1.0"
as_of: 2026-10-09
blocs: [left, right]
candidates:
  - { id: alice, name: Alice, party: A, bloc: left, status: declared, sources: [] }
  - { id: bob, name: Bob, party: B, bloc: right, status: likely, sources: [] }
  - { id: carol, name: Carol, party: C, bloc: right, status: possible, sources: [] }
slots:
  - { id: alice, options: [{ run: [alice], p: 0.9 }] }
  - { id: right, options: [{ run: [bob], p: 0.6 }, { run: [carol], p: 0.3 }] }
"#;

const POLL: &str = r#"
schema_version: "1.0"
poll_id: ifop-2026-09-12
firm: ifop
sponsor: Example
field_start: 2026-09-10
field_end: 2026-09-11
published_at: 2026-09-12
sample_size: 1200
method: online
population: registered
source_url: https://example.org/poll.pdf
retrieved_at: 2026-10-09T12:00:00Z
entry: primary
scenarios:
  - scenario_id: A
    round: 1
    shares: { alice: 45, bob: 55 }
  - scenario_id: B
    round: 1
    shares: { alice: 48, carol: 52 }
  - scenario_id: H1
    round: 2
    shares: { alice: 47, bob: 53 }
"#;

fn scratch_repo() -> (tempfile::TempDir, Repo) {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    let schemas = Path::new(env!("CARGO_MANIFEST_DIR")).join("../schemas");
    fs::create_dir_all(root.join("schemas")).unwrap();
    for entry in fs::read_dir(schemas).unwrap() {
        let entry = entry.unwrap();
        fs::copy(entry.path(), root.join("schemas").join(entry.file_name())).unwrap();
    }
    fs::create_dir_all(root.join("config")).unwrap();
    fs::write(root.join("config/candidates.yaml"), CANDIDATES).unwrap();
    fs::create_dir_all(root.join("data/raw/polls")).unwrap();
    fs::create_dir_all(root.join("data/raw/grok")).unwrap();
    let repo = Repo::new(root);
    (dir, repo)
}

#[test]
fn valid_poll_becomes_clean_rows_and_round_trips() {
    let (_dir, repo) = scratch_repo();
    fs::write(repo.raw_polls_dir().join("ifop-2026-09-12.yaml"), POLL).unwrap();
    let schemas = Schemas::load(&repo).unwrap();
    let config = CandidatesConfig::load(&repo, &schemas).unwrap();
    let ingest = polls::ingest(&repo, &schemas, &config).unwrap();
    assert!(ingest.rejections.is_empty(), "{:?}", ingest.rejections);
    assert_eq!(ingest.rows.len(), 6);
    assert!(ingest.rows.iter().all(|r| r.content_hash.starts_with("sha256:")));
    polls::write_clean(&repo, &ingest).unwrap();
    let rows = polls::read_clean_rows(&repo).unwrap();
    assert_eq!(rows, ingest.rows);
    let polls = polls::to_polls(&rows);
    assert_eq!(polls.len(), 1);
    assert_eq!(polls[0].scenarios.len(), 3);
}

#[test]
fn invalid_poll_is_quarantined_with_reasons() {
    let (_dir, repo) = scratch_repo();
    let bad = POLL
        .replace("alice: 48, carol: 52", "alice: 48, dave: 52")
        .replace("published_at: 2026-09-12", "published_at: 2026-09-01");
    fs::write(repo.raw_polls_dir().join("ifop-2026-09-12.yaml"), bad).unwrap();
    fs::write(repo.raw_polls_dir().join("broken.yaml"), "poll_id: [").unwrap();
    let schemas = Schemas::load(&repo).unwrap();
    let config = CandidatesConfig::load(&repo, &schemas).unwrap();
    let ingest = polls::ingest(&repo, &schemas, &config).unwrap();
    assert!(ingest.rows.is_empty());
    assert_eq!(ingest.rejections.len(), 2);
    let poll = ingest
        .rejections
        .iter()
        .find(|r| r.path.ends_with("ifop-2026-09-12.yaml"))
        .unwrap();
    assert!(
        poll.errors.iter().any(|e| e.contains("unknown candidate `dave`")),
        "{:?}",
        poll.errors
    );
    assert!(
        poll.errors
            .iter()
            .any(|e| e.contains("published_at is before field_end"))
    );
    polls::write_clean(&repo, &ingest).unwrap();
    let note = fs::read_to_string(repo.quarantine_dir().join("polls/ifop-2026-09-12.yaml.error.txt")).unwrap();
    assert!(note.contains("dave"));
    // The raw file is left as it was.
    assert!(repo.raw_polls_dir().join("broken.yaml").exists());
}

#[test]
fn superseding_poll_replaces_the_original() {
    let (_dir, repo) = scratch_repo();
    fs::write(repo.raw_polls_dir().join("ifop-2026-09-12.yaml"), POLL).unwrap();
    let fixed = POLL
        .replace(
            "poll_id: ifop-2026-09-12",
            "poll_id: ifop-2026-09-12-r1\nsupersedes: ifop-2026-09-12",
        )
        .replace("alice: 45, bob: 55", "alice: 46, bob: 54");
    fs::write(repo.raw_polls_dir().join("ifop-2026-09-12-r1.yaml"), fixed).unwrap();
    let schemas = Schemas::load(&repo).unwrap();
    let config = CandidatesConfig::load(&repo, &schemas).unwrap();
    let ingest = polls::ingest(&repo, &schemas, &config).unwrap();
    assert_eq!(ingest.accepted, vec!["ifop-2026-09-12-r1".to_string()]);
    assert!(ingest.rows.iter().any(|r| r.candidate_id == "alice" && r.share == 46.0));
}

#[test]
fn grok_drop_validation() {
    let (_dir, repo) = scratch_repo();
    let drop = r#"{
      "schema_version": "1.0",
      "collected_at": "2026-10-09T14:05:00Z",
      "window": {"start": "2026-10-09T13:00:00Z", "end": "2026-10-09T14:00:00Z"},
      "query_method": "keyword search",
      "candidates": [
        {"candidate_id": "alice", "mentions": 100, "unique_authors": 80, "engagement": 1000,
         "sentiment": {"pos": 0.3, "neg": 0.5, "neu": 0.2, "method": "grok-classifier"},
         "top_topics": ["retraites"], "bot_share_estimate": 0.1, "sample_post_ids": ["1"]},
        {"candidate_id": "bob", "mentions": null, "sentiment": null}
      ]
    }"#;
    fs::write(repo.raw_grok_dir().join("2026-10-09T14-00.json"), drop).unwrap();
    fs::write(repo.raw_grok_dir().join("2026-10-09T15-00.json"), drop).unwrap();
    fs::write(repo.raw_grok_dir().join("README.md"), "not a drop").unwrap();
    let schemas = Schemas::load(&repo).unwrap();
    let config = CandidatesConfig::load(&repo, &schemas).unwrap();
    let ingest = grok::ingest(&repo, &schemas, &config).unwrap();
    assert_eq!(ingest.accepted, 1);
    assert_eq!(ingest.rows.len(), 2);
    assert_eq!(ingest.rejections.len(), 1);
    assert!(ingest.rejections[0].errors[0].contains("window end"));
    grok::write_clean(&repo, &ingest).unwrap();
    assert!(
        repo.quarantine_dir()
            .join("grok/2026-10-09T15-00.json.error.txt")
            .exists()
    );
}

#[test]
fn config_rejects_overfull_slot() {
    let (_dir, repo) = scratch_repo();
    fs::write(
        repo.config_dir().join("candidates.yaml"),
        CANDIDATES.replace("{ run: [carol], p: 0.3 }", "{ run: [carol], p: 0.5 }"),
    )
    .unwrap();
    let schemas = Schemas::load(&repo).unwrap();
    let error = CandidatesConfig::load(&repo, &schemas).unwrap_err().to_string();
    assert!(error.contains("sum to"), "{error}");
}
