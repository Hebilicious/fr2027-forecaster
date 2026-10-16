//! Ingest of the live sources (markets, news, attention, events) and config/sources.yaml,
//! against a scratch repository holding the real schemas.

use std::fs;
use std::path::Path;

use fr2027_collectors::{
    Repo, Schemas, attention, config::CandidatesConfig, events, markets, news, sources::SourcesConfig,
};

const CANDIDATES: &str = r#"
schema_version: "1.0"
as_of: 2026-10-09
blocs: [left, right]
candidates:
  - { id: alice, name: Alice Martin, party: A, bloc: left, status: declared, sources: [] }
  - { id: bob, name: Bob Durand, party: B, bloc: right, status: likely, sources: [] }
slots:
  - { id: alice, options: [{ run: [alice], p: 0.9 }] }
  - { id: bob, options: [{ run: [bob], p: 0.6 }] }
"#;

const SOURCES: &str = r#"
schema_version: "1.0"
user_agent: "fr2027-forecaster tests (+https://example.org)"
markets:
  - { id: pm-win, venue: polymarket, question: win, event: some-event, url: "https://polymarket.com/event/some-event" }
feeds:
  - { id: paper, outlet: "Le Papier", url: "https://example.org/rss.xml" }
wikipedia: { project: fr.wikipedia.org }
candidates:
  alice: { wikipedia: Alice_Martin, names: ["Alice Martin"] }
  bob: { wikipedia: null, names: ["Bob Durand", "Durand"] }
publication_blackouts:
  - { from: "2027-04-17T00:00:00+02:00", until: "2027-04-18T20:00:00+02:00" }
"#;

fn scratch_repo() -> (tempfile::TempDir, Repo, Schemas, CandidatesConfig) {
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
    fs::write(root.join("config/sources.yaml"), SOURCES).unwrap();
    let repo = Repo::new(root);
    let schemas = Schemas::load(&repo).unwrap();
    let config = CandidatesConfig::load(&repo, &schemas).unwrap();
    (dir, repo, schemas, config)
}

fn market_snapshot(fetched_at: &str, price: f64, candidate: &str) -> String {
    format!(
        r#"{{"schema_version": "1.0", "fetched_at": "{fetched_at}", "markets": [{{
          "id": "pm-win", "venue": "polymarket", "question": "win", "event": "some-event",
          "url": "https://polymarket.com/event/some-event", "status": "ok", "error": null,
          "contracts": [
            {{"label": "Alice Martin", "candidate_id": "{candidate}", "price": {price}, "bid": null, "ask": null, "last": {price}, "volume": 10}},
            {{"label": "Someone Else", "candidate_id": null, "price": 0.2, "bid": 0.19, "ask": 0.21, "last": 0.2, "volume": 5}}
          ]}}]}}"#
    )
}

#[test]
fn sources_config_needs_every_candidate_and_valid_windows() {
    let (_dir, repo, schemas, config) = scratch_repo();
    let sources = SourcesConfig::load(&repo, &schemas, &config).unwrap();
    assert!(sources.blackout_at("2027-04-17T12:00:00Z".parse().unwrap()).is_some());
    assert!(sources.blackout_at("2027-04-18T18:00:00Z".parse().unwrap()).is_none());

    fs::write(
        repo.config_dir().join("sources.yaml"),
        SOURCES.replace("  bob: { wikipedia: null, names: [\"Bob Durand\", \"Durand\"] }\n", ""),
    )
    .unwrap();
    let error = SourcesConfig::load(&repo, &schemas, &config).unwrap_err().to_string();
    assert!(error.contains("no entry for candidate `bob`"), "{error}");
}

#[test]
fn market_history_keeps_a_week_of_hours_and_one_close_per_older_day() {
    let (_dir, repo, schemas, config) = scratch_repo();
    let dir = repo.raw_markets_dir();
    fs::create_dir_all(&dir).unwrap();
    for (stem, at, price) in [
        ("2026-09-29T08", "2026-09-29T08:10:00Z", 0.30),
        ("2026-09-29T20", "2026-09-29T20:10:00Z", 0.31),
        ("2026-10-09T09", "2026-10-09T09:10:00Z", 0.35),
        ("2026-10-09T10", "2026-10-09T10:10:00Z", 0.36),
    ] {
        fs::write(dir.join(format!("{stem}.json")), market_snapshot(at, price, "alice")).unwrap();
    }
    // Misnamed, and naming an unknown candidate.
    fs::write(
        dir.join("2026-10-09T11.json"),
        market_snapshot("2026-10-09T12:00:00Z", 0.4, "zoe"),
    )
    .unwrap();

    let ingest = markets::ingest(&repo, &schemas, &config).unwrap();
    assert_eq!(ingest.accepted, 4);
    assert_eq!(ingest.rejections.len(), 1);
    let errors = &ingest.rejections[0].errors;
    assert!(errors.iter().any(|e| e.contains("fetch hour")), "{errors:?}");
    assert!(
        errors.iter().any(|e| e.contains("unknown candidate `zoe`")),
        "{errors:?}"
    );
    let kept: Vec<&str> = ingest.rows.iter().map(|r| r.fetched_at.as_str()).collect();
    assert_eq!(
        kept,
        ["2026-09-29T20:10:00Z", "2026-10-09T09:10:00Z", "2026-10-09T10:10:00Z"],
        "unmatched contracts are left out; the morning of an old day is dropped"
    );
    assert_eq!(ingest.latest.as_ref().unwrap().fetched_at, "2026-10-09T10:10:00Z");

    markets::write_clean(&repo, &ingest).unwrap();
    assert_eq!(markets::read_clean_rows(&repo).unwrap(), ingest.rows);
}

#[test]
fn news_rows_are_unique_links_with_outlets() {
    let (_dir, repo, schemas, config) = scratch_repo();
    let sources = SourcesConfig::load(&repo, &schemas, &config).unwrap();
    let dir = repo.raw_news_dir();
    fs::create_dir_all(&dir).unwrap();
    let snapshot = |at: &str, url: &str| {
        format!(
            r#"{{"schema_version": "1.0", "fetched_at": "{at}",
              "feeds": [{{"id": "paper", "status": "ok", "items": 20, "error": null}}],
              "items": [{{"feed": "paper", "title": "Durand répond", "url": "{url}", "published_at": null, "candidate_ids": ["bob"]}}]}}"#
        )
    };
    fs::write(
        dir.join("2026-10-09T09.json"),
        snapshot("2026-10-09T09:00:00Z", "https://example.org/a"),
    )
    .unwrap();
    fs::write(
        dir.join("2026-10-09T10.json"),
        snapshot("2026-10-09T10:00:00Z", "https://example.org/a"),
    )
    .unwrap();
    fs::write(
        dir.join("2026-10-09T11.json"),
        snapshot("2026-10-09T11:00:00Z", "https://example.org/b"),
    )
    .unwrap();
    let ingest = news::ingest(&repo, &schemas, &config, &sources).unwrap();
    assert!(ingest.rejections.is_empty(), "{:?}", ingest.rejections);
    assert_eq!(ingest.rows.len(), 2);
    assert_eq!(ingest.rows[0].outlet, "Le Papier");
    assert_eq!(
        ingest.rows[0].published_at, "2026-10-09T09:00:00Z",
        "fetch time stands in"
    );
    assert!(ingest.seen_urls.contains("https://example.org/b"));
    news::write_clean(&repo, &ingest).unwrap();
    assert_eq!(news::read_clean_rows(&repo).unwrap(), ingest.rows);
}

#[test]
fn later_attention_fetch_wins() {
    let (_dir, repo, schemas, config) = scratch_repo();
    let dir = repo.raw_attention_dir();
    fs::create_dir_all(&dir).unwrap();
    let fetch = |at: &str, start: &str, end: &str, days: &str| {
        format!(
            r#"{{"schema_version": "1.0", "fetched_at": "{at}", "project": "fr.wikipedia.org", "start": "{start}", "end": "{end}",
              "articles": [{{"candidate_id": "alice", "article": "Alice_Martin", "status": "ok", "error": null, "days": [{days}]}}]}}"#
        )
    };
    fs::write(
        dir.join("2026-10-08.json"),
        fetch(
            "2026-10-08T06:00:00Z",
            "2026-10-06",
            "2026-10-07",
            r#"{"date": "2026-10-06", "views": 10}, {"date": "2026-10-07", "views": 5}"#,
        ),
    )
    .unwrap();
    fs::write(
        dir.join("2026-10-09.json"),
        fetch(
            "2026-10-09T06:00:00Z",
            "2026-10-07",
            "2026-10-08",
            r#"{"date": "2026-10-07", "views": 12}, {"date": "2026-10-08", "views": 20}"#,
        ),
    )
    .unwrap();
    // A day outside the declared range.
    fs::write(
        dir.join("2026-10-10.json"),
        fetch(
            "2026-10-10T06:00:00Z",
            "2026-10-09",
            "2026-10-09",
            r#"{"date": "2026-10-01", "views": 1}"#,
        ),
    )
    .unwrap();
    let ingest = attention::ingest(&repo, &schemas, &config).unwrap();
    assert_eq!(ingest.rejections.len(), 1);
    let views: Vec<(&str, u64)> = ingest.rows.iter().map(|r| (r.date.as_str(), r.views)).collect();
    assert_eq!(views, [("2026-10-06", 10), ("2026-10-07", 12), ("2026-10-08", 20)]);
}

#[test]
fn events_need_matching_ids_and_known_candidates() {
    let (_dir, repo, schemas, config) = scratch_repo();
    let dir = repo.raw_events_dir();
    fs::create_dir_all(&dir).unwrap();
    let event = |id: &str, date: &str, candidate: &str| {
        format!(
            r#"{{"schema_version": "1.0", "event_id": "{id}", "date": "{date}", "kind": "declaration",
              "candidate_ids": ["{candidate}"], "summary": "Alice Martin declares her candidacy on the evening news.",
              "sources": [{{"url": "https://example.org/news", "title": "Alice Martin candidate"}}],
              "proposed_change": "Raise alice p_run to 0.95.", "reported_by": "grok-bot", "reported_at": "2026-10-09T20:00:00Z"}}"#
        )
    };
    fs::write(
        dir.join("2026-10-09-alice-declares.json"),
        event("2026-10-09-alice-declares", "2026-10-09", "alice"),
    )
    .unwrap();
    fs::write(
        dir.join("2026-10-08-zoe-declares.json"),
        event("2026-10-08-zoe-declares", "2026-10-09", "zoe"),
    )
    .unwrap();
    let ingest = events::ingest(&repo, &schemas, &config).unwrap();
    assert_eq!(ingest.rows.len(), 1);
    assert_eq!(ingest.rows[0].source_title.as_deref(), Some("Alice Martin candidate"));
    let errors = &ingest.rejections[0].errors;
    assert!(
        errors.iter().any(|e| e.contains("must start with its date")),
        "{errors:?}"
    );
    assert!(
        errors.iter().any(|e| e.contains("unknown candidate `zoe`")),
        "{errors:?}"
    );
}
