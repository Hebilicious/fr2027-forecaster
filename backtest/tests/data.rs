//! Checks on the real backtest data: every file loads, every row carries its provenance, and no
//! replay can see a poll published after its date.

use std::path::Path;

use fr2027_backtest::{
    config::BacktestConfig,
    election::{self, Election},
    final_polls,
    replay::select_polls,
};
use fr2027_collectors::{Repo, Schemas, polls};
use fr2027_model::day_number;

fn repo() -> Repo {
    Repo::new(Path::new(env!("CARGO_MANIFEST_DIR")).join(".."))
}

fn elections(repo: &Repo) -> (BacktestConfig, Vec<Election>) {
    let config = BacktestConfig::load(repo).unwrap();
    let schemas = Schemas::load(repo).unwrap();
    let elections = config
        .elections
        .iter()
        .map(|entry| election::load(repo, &schemas, entry, &config.results_file).unwrap())
        .collect();
    (config, elections)
}

#[test]
fn elections_load_with_their_results() {
    let repo = repo();
    let (_, elections) = elections(&repo);
    assert_eq!(elections.len(), 2);
    for election in &elections {
        let (a, b) = election.finalists;
        assert_eq!(
            [election.id(a), election.id(b)]
                .iter()
                .filter(|id| ["macron", "le_pen"].contains(id))
                .count(),
            2,
            "{}",
            election.year
        );
        assert_eq!(election.id(election.winner()), "macron");
        assert!(
            election.polls.len() > 300,
            "{}: {} polls",
            election.year,
            election.polls.len()
        );
        let runners = election.ran.iter().filter(|r| **r).count();
        assert!(runners >= 11, "{}: {runners} runners", election.year);
    }
}

#[test]
fn every_poll_row_records_its_provenance() {
    let repo = repo();
    let (config, _) = elections(&repo);
    for entry in &config.elections {
        let rows = polls::read_rows(&repo.root().join(&entry.dir).join("polls.csv")).unwrap();
        for row in &rows {
            assert!(row.source_url.starts_with("http"), "{}", row.poll_id);
            let index = row.index_url.as_deref().unwrap_or_default();
            assert!(
                index.contains("wikipedia.org") && index.contains("oldid="),
                "{}",
                row.poll_id
            );
            assert!(row.entry == "index" || row.entry == "primary", "{}", row.poll_id);
            assert!(!row.retrieved_at.is_empty() && row.content_hash.starts_with("sha256:"));
            assert!(row.field_start <= row.field_end && row.field_end < row.published_at);
        }
    }
}

#[test]
fn replays_never_see_later_polls_and_thin_overlaps() {
    let repo = repo();
    let (config, elections) = elections(&repo);
    for election in &elections {
        for &horizon in &config.horizons_days {
            let as_of = fr2027_model::date_from_day_number(day_number(election.round1) - horizon as i32);
            let selection = select_polls(election, as_of, config.poll_window_days);
            assert!(!selection.polls.is_empty());
            for poll in &selection.polls {
                assert!(poll.published_at <= as_of, "{} after {as_of}", poll.poll_id);
                assert!(day_number(poll.field_start) >= day_number(election.round1) - config.poll_window_days as i32);
            }
            for round in [1u8, 2] {
                let asked: Vec<_> = selection
                    .polls
                    .iter()
                    .filter(|p| p.scenarios.iter().any(|s| s.round == round))
                    .collect();
                for (i, a) in asked.iter().enumerate() {
                    for b in &asked[i + 1..] {
                        let overlap = a.firm == b.firm && a.field_start <= b.field_end && b.field_start <= a.field_end;
                        assert!(!overlap, "{} and {} overlap in round {round}", a.poll_id, b.poll_id);
                    }
                }
            }
        }
    }
}

#[test]
fn final_poll_files_load_for_every_configured_year() {
    let repo = repo();
    let (config, elections) = elections(&repo);
    let round1 = final_polls::load_round1(&repo, &config, &elections[0].field.blocs).unwrap();
    let round2 = final_polls::load_round2(&repo, &config).unwrap();
    for year in &config.final_polls.years {
        assert!(round1.iter().any(|e| e.year == *year), "round 1 {year}");
        assert!(round2.iter().any(|e| e.year == *year), "round 2 {year}");
    }
}
