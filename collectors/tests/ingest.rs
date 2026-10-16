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

/// A routine 1 drop with stance counts (rubric stance-1), counted by hand below.
const STANCE_DROP: &str = r#"{
  "schema_version": "1.0",
  "collected_at": "2026-10-17T00:25:00Z",
  "window": {"start": "2026-10-16T12:00:00Z", "end": "2026-10-17T00:00:00Z"},
  "query_method": "Grok Bot native X search, one query per candidate, lang:fr, routine v3",
  "agent": {"name": "Grok Bot fr2027", "skill_version": "x-activity-3", "run_id": "x-activity-2026-10-17T00:20Z"},
  "rubric_version": "stance-1",
  "sampling": {"method": "random_spread", "posts_per_candidate": 200, "read_all_below": 100,
               "max_posts_per_author": 2, "reposts_excluded": true, "automated_excluded": true,
               "relabel_share": 0.2},
  "double_labels": {"posts": 46, "counts": {
    "support":   {"support": 12, "oppose": 1,  "neutral": 2,  "off_topic": 0},
    "oppose":    {"support": 0,  "oppose": 11, "neutral": 2,  "off_topic": 0},
    "neutral":   {"support": 1,  "oppose": 1,  "neutral": 10, "off_topic": 1},
    "off_topic": {"support": 0,  "oppose": 0,  "neutral": 1,  "off_topic": 4}}},
  "candidates": [
    {"candidate_id": "alice", "query": "\"Alice\" lang:fr", "mentions": 5000,
     "mentions_method": "x_counts_endpoint", "sample_size": 200, "sentiment": null,
     "stance": {"read_all": false, "posts_drawn": 200, "off_topic": 20, "support": 72, "oppose": 63, "neutral": 45},
     "top_topics": ["retraites"], "bot_share_estimate": 0.05},
    {"candidate_id": "bob", "query": "\"Bob\" lang:fr", "mentions": 40,
     "mentions_method": "x_counts_endpoint", "sample_size": 30, "sentiment": null,
     "stance": {"read_all": true, "posts_drawn": 30, "off_topic": 5, "support": 10, "oppose": 5, "neutral": 10},
     "top_topics": [], "bot_share_estimate": null},
    {"candidate_id": "carol", "mentions": 0, "mentions_method": "x_counts_endpoint", "sample_size": 0,
     "sentiment": null,
     "stance": {"read_all": true, "posts_drawn": 0, "off_topic": 0, "support": 0, "oppose": 0, "neutral": 0}}
  ]
}"#;

fn ingest_one(repo: &Repo, stem: &str, drop: &str) -> grok::GrokIngest {
    fs::write(repo.raw_grok_dir().join(format!("{stem}.json")), drop).unwrap();
    let schemas = Schemas::load(repo).unwrap();
    let config = CandidatesConfig::load(repo, &schemas).unwrap();
    grok::ingest(repo, &schemas, &config).unwrap()
}

#[test]
#[allow(clippy::approx_constant, reason = "0.318 is a Wilson bound, not 1/π")]
fn grok_stance_drop_computes_shares_intervals_and_kappa() {
    let (_dir, repo) = scratch_repo();
    let ingest = ingest_one(&repo, "2026-10-17T00-00", STANCE_DROP);
    assert!(ingest.rejections.is_empty(), "{:?}", ingest.rejections);
    let row = |id: &str| ingest.rows.iter().find(|r| r.candidate_id == id).unwrap();

    // alice: 180 relevant posts (200 drawn, 20 off topic). Support 72/180 = 0.4. Wilson, z = 1.96:
    // z²/n = 0.021341, centre = (0.4 + 0.010671) / 1.021341 = 0.402089,
    // half-width = 1.96 / 1.021341 · √(0.4 · 0.6 / 180 + 3.841459 / 129600) = 0.070846.
    let alice = row("alice");
    assert_eq!(alice.rubric_version.as_deref(), Some("stance-1"));
    assert_eq!(alice.stance_posts_drawn, Some(200));
    assert_eq!(alice.stance_read_all, Some(false));
    assert_eq!(
        (alice.support_share, alice.support_low, alice.support_high),
        (Some(0.4), Some(0.3312), Some(0.4729))
    );
    assert_eq!(
        (alice.oppose_share, alice.oppose_low, alice.oppose_high),
        (Some(0.35), Some(0.2841), Some(0.4222))
    );
    assert_eq!(
        (alice.neutral_share, alice.neutral_low, alice.neutral_high),
        (Some(0.25), Some(0.1924), Some(0.318))
    );
    // Sentiment is derived from stance, so the Signals view keeps working.
    assert_eq!(
        (alice.sentiment_pos, alice.sentiment_neg, alice.sentiment_neu),
        (Some(0.4), Some(0.35), Some(0.25))
    );
    assert!(
        alice
            .sentiment_method
            .as_deref()
            .unwrap()
            .contains("rubric stance-1: support, oppose, neutral among 180 relevant of 200 drawn posts")
    );

    // bob: every post read, 25 relevant; 0.4 [0.234, 0.5926] and 0.2 [0.0886, 0.3913].
    let bob = row("bob");
    assert_eq!(bob.stance_read_all, Some(true));
    assert_eq!(
        (bob.support_share, bob.support_low, bob.support_high),
        (Some(0.4), Some(0.234), Some(0.5926))
    );
    assert_eq!(
        (bob.oppose_share, bob.oppose_low, bob.oppose_high),
        (Some(0.2), Some(0.0886), Some(0.3913))
    );

    // carol: nothing to read, so no shares and no sentiment, but the counts are kept.
    let carol = row("carol");
    assert_eq!(carol.stance_posts_drawn, Some(0));
    assert_eq!((carol.support_share, carol.sentiment_pos), (None, None));

    // Kappa, the same on every row of the drop. 46 posts, 37 on the diagonal: p_o = 37/46.
    // First-pass totals 15, 13, 13, 5; second-pass totals 13, 13, 15, 5:
    // p_e = (15·13 + 13·13 + 13·15 + 5·5) / 46² = 584/2116.
    // κ = (1702 − 584) / (2116 − 584) = 1118/1532 = 0.72977.
    for r in &ingest.rows {
        assert_eq!(r.drop_double_labelled, Some(46));
        assert_eq!(r.drop_agreement, Some(0.8043));
        assert_eq!(r.drop_kappa, Some(0.7298));
    }

    grok::write_clean(&repo, &ingest).unwrap();
    assert_eq!(grok::read_clean_rows(&repo).unwrap(), ingest.rows);
}

#[test]
fn grok_drop_without_stance_keeps_its_sentiment() {
    // The first real drop (rubric-less, sentiment from 25-post readings) still ingests unchanged.
    let (_dir, repo) = scratch_repo();
    let real = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    fs::copy(
        real.join("config/candidates.yaml"),
        repo.config_dir().join("candidates.yaml"),
    )
    .unwrap();
    let drop = fs::read_to_string(real.join("data/raw/grok/2026-10-16T12-00.json")).unwrap();
    let ingest = ingest_one(&repo, "2026-10-16T12-00", &drop);
    assert!(ingest.rejections.is_empty(), "{:?}", ingest.rejections);
    let melenchon = ingest.rows.iter().find(|r| r.candidate_id == "melenchon").unwrap();
    assert_eq!(melenchon.sentiment_pos, Some(0.36));
    assert_eq!(melenchon.rubric_version, None);
    assert_eq!(melenchon.support_share, None);
    assert_eq!(melenchon.drop_kappa, None);
}

#[test]
fn grok_stance_inconsistencies_are_rejected() {
    let cases = [
        (
            STANCE_DROP.replace(r#""off_topic": 20, "support": 72"#, r#""off_topic": 21, "support": 72"#),
            "support + oppose + neutral + off_topic = 201, but posts_drawn is 200",
        ),
        (
            STANCE_DROP.replace(r#""posts": 46"#, r#""posts": 45"#),
            "double_labels: the counts add up to 46, but posts is 45",
        ),
        (
            STANCE_DROP
                .replace(
                    r#""off_topic": {"support": 0,  "oppose": 0,  "neutral": 1,  "off_topic": 4}"#,
                    r#""off_topic": {"support": 0,  "oppose": 0,  "neutral": 1,  "off_topic": 30}"#,
                )
                .replace(r#""posts": 46"#, r#""posts": 72"#),
            "31 posts first labelled off_topic, more than the 25 off_topic posts",
        ),
        (
            STANCE_DROP.replace(
                r#""sample_size": 30, "sentiment": null"#,
                r#""sample_size": 30, "sentiment": {"pos": 0.4, "neg": 0.2, "neu": 0.4, "method": "m"}"#,
            ),
            "`bob` has both stance and sentiment",
        ),
        (
            STANCE_DROP.replace(r#""sample_size": 200"#, r#""sample_size": 25"#),
            "sample_size 25 must equal stance.posts_drawn 200",
        ),
        (
            STANCE_DROP.replace(r#""mentions": 40"#, r#""mentions": 20"#),
            "30 posts drawn but only 20 mentions",
        ),
        (
            STANCE_DROP.replace(r#""posts_per_candidate": 200"#, r#""posts_per_candidate": 150"#),
            "more than sampling.posts_per_candidate 150",
        ),
    ];
    for (drop, expected) in cases {
        let (_dir, repo) = scratch_repo();
        let ingest = ingest_one(&repo, "2026-10-17T00-00", &drop);
        assert_eq!(ingest.accepted, 0, "{expected}");
        let errors = &ingest.rejections[0].errors;
        assert!(errors.iter().any(|e| e.contains(expected)), "{expected}: {errors:?}");
    }

    // Stance without the rubric and sampling that produced it.
    let (_dir, repo) = scratch_repo();
    let mut value: serde_json::Value = serde_json::from_str(STANCE_DROP).unwrap();
    let object = value.as_object_mut().unwrap();
    for key in ["rubric_version", "sampling", "double_labels"] {
        object.remove(key);
    }
    let ingest = ingest_one(&repo, "2026-10-17T00-00", &value.to_string());
    assert!(
        ingest.rejections[0].errors[0].contains("needs rubric_version and sampling"),
        "{:?}",
        ingest.rejections
    );

    // The schema refuses a negative count and a table without every cell.
    for drop in [
        STANCE_DROP.replace(r#""neutral": 45"#, r#""neutral": -45"#),
        STANCE_DROP.replace(
            r#""support": 12, "oppose": 1,  "neutral": 2,  "off_topic": 0"#,
            r#""support": 12"#,
        ),
    ] {
        let (_dir, repo) = scratch_repo();
        assert_eq!(ingest_one(&repo, "2026-10-17T00-00", &drop).accepted, 0);
    }
}

#[test]
fn wilson_and_kappa_edge_cases() {
    assert_eq!(grok::wilson(0, 0), None);
    // 0 of 10: the interval still reaches 0.2775 (Wilson, not the zero-width normal interval).
    let none = grok::wilson(0, 10).unwrap();
    assert_eq!((none.share, none.low, none.high), (0.0, 0.0, 0.2775));
    // No posts labelled twice: nothing to measure.
    let empty = grok::Reliability::of(&[[0; 4]; 4]);
    assert_eq!((empty.posts, empty.agreement, empty.kappa), (0, None, None));
    // Both passes said "support" every time: agreement 1, kappa undefined (chance agreement is 1).
    let mut same = [[0; 4]; 4];
    same[0][0] = 9;
    let same = grok::Reliability::of(&same);
    assert_eq!((same.agreement, same.kappa), (Some(1.0), None));
    // Perfect agreement over two labels: kappa 1. Systematic swap: kappa −1.
    let perfect = grok::Reliability::of(&[[5, 0, 0, 0], [0, 5, 0, 0], [0; 4], [0; 4]]);
    assert_eq!(perfect.kappa, Some(1.0));
    let swapped = grok::Reliability::of(&[[0, 5, 0, 0], [5, 0, 0, 0], [0; 4], [0; 4]]);
    assert_eq!(swapped.kappa, Some(-1.0));
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
