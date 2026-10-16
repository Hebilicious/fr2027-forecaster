//! Runs the whole backtest on a small fictional repository: it completes, is deterministic, and
//! the fit picks a setting from the grid by its rule.

use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use fr2027_collectors::Repo;

const CANDIDATES: &str = r#"
schema_version: "1.0"
as_of: 2001-04-22
blocs: [far_left, left, centre, right, far_right, other]
candidates:
  - { id: anna, name: "Anna A", party: L, bloc: left, status: declared, sources: [] }
  - { id: bruno, name: "Bruno B", party: C, bloc: centre, status: declared, sources: [] }
  - { id: chloe, name: "Chloé C", party: R, bloc: right, status: declared, sources: [] }
  - { id: david, name: "David D", party: FR, bloc: far_right, status: declared, sources: [] }
  - { id: emma, name: "Emma E", party: R, bloc: right, status: withdrawn, sources: [] }
slots:
  - { id: anna, options: [{ run: [anna], p: 1 }] }
  - { id: bruno, options: [{ run: [bruno], p: 1 }] }
  - { id: chloe, options: [{ run: [chloe], p: 1 }] }
  - { id: david, options: [{ run: [david], p: 1 }] }
  - { id: emma, options: [{ run: [emma], p: 0 }] }
"#;

fn backtest_yaml() -> String {
    r#"
seed: 7
simulations: 600
horizons_days: [60, 7]
poll_window_days: 365
contender_share: 0.10
probability_floor: 0.001
elections:
  - { year: 2001, dir: data/backtest/2001, round1: 2001-04-22, round2: 2001-05-06 }
  - { year: 2006, dir: data/backtest/2006, round1: 2006-04-23, round2: 2006-05-07 }
results_file: research/sources/results.csv
final_polls:
  round1_file: research/sources/round1.csv
  round2_file: research/sources/round2.csv
  years: [1995]
  blocs: { "Anna A": left, "Bruno B": centre, "Chloé C": right, "David D": far_right }
grid:
  bloc_sd: [0.06, 0.12, 0.2]
  candidate_sd: [0.05, 0.1]
  round2_error_sd: [0.04, 0.08, 0.16]
"#
    .to_string()
}

const HEADER: &str = "poll_id,firm,sponsor,field_start,field_end,published_at,sample_size,method,population,round,scenario_id,candidate_id,share,source_url,notice_url,index_url,entry,retrieved_at,content_hash\n";

/// Twelve polls in the two months before round 1, one every five days, from three firms; the
/// later ones are published after the 60-day replay date and must not reach it.
fn polls_csv(year: i32) -> String {
    let mut out = HEADER.to_string();
    for i in 0..12 {
        let day = 1 + i * 5;
        let (month, day) = if day <= 26 { (2, day) } else { (3, day - 28) };
        let start = format!("{year}-{month:02}-{day:02}");
        let end = format!("{year}-{month:02}-{:02}", day + 1);
        let published = format!("{year}-{month:02}-{:02}", day + 2);
        let firm = ["ifop", "elabe", "odoxa"][i as usize % 3];
        let id = format!("{firm}-{end}");
        let wobble = f64::from(i % 3) - 1.0;
        let rows = [
            (1, "S1", "anna", 20.0 + wobble),
            (1, "S1", "bruno", 27.0),
            (1, "S1", "chloe", 23.0 - wobble),
            (1, "S1", "david", 30.0),
            (1, "S2", "anna", 21.0),
            (1, "S2", "bruno", 27.0),
            (1, "S2", "emma", 21.0),
            (1, "S2", "david", 31.0),
            (2, "H1", "bruno", 53.0 + wobble),
            (2, "H1", "david", 47.0 - wobble),
        ];
        for (round, scenario, candidate, share) in rows {
            let _ = writeln!(
                out,
                "{id},{firm},,{start},{end},{published},1000,,,{round},{scenario},{candidate},{share},https://example.org/{id}.pdf,,https://en.wikipedia.org/w/index.php?oldid=1,index,2026-10-16T12:00Z,sha256:00"
            );
        }
    }
    out
}

fn scratch_repo() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    fs::create_dir_all(root.join("schemas")).unwrap();
    for entry in fs::read_dir(source.join("schemas")).unwrap() {
        let entry = entry.unwrap();
        fs::copy(entry.path(), root.join("schemas").join(entry.file_name())).unwrap();
    }
    fs::create_dir_all(root.join("config")).unwrap();
    fs::copy(source.join("config/model.yaml"), root.join("config/model.yaml")).unwrap();
    fs::write(root.join("config/backtest.yaml"), backtest_yaml()).unwrap();
    for (year, round1) in [(2001, "2001-04-22"), (2006, "2006-04-23")] {
        let dir = root.join(format!("data/backtest/{year}"));
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("candidates.yaml"), CANDIDATES.replace("2001-04-22", round1)).unwrap();
        fs::write(dir.join("polls.csv"), polls_csv(year)).unwrap();
    }
    let sources = root.join("research/sources");
    fs::create_dir_all(&sources).unwrap();
    let mut results = "year,round,candidate_name,party,share_expressed,source_url,notes\n".to_string();
    for year in [2001, 2006] {
        for (round, name, share) in [
            (1, "Anna A", 19.0),
            (1, "Bruno B", 29.5),
            (1, "Chloé C", 20.5),
            (1, "David D", 31.0),
            (2, "Bruno B", 55.0),
            (2, "David D", 45.0),
        ] {
            let _ = writeln!(results, "{year},{round},{name},X,{share},https://example.org,");
        }
        let _ = writeln!(results, "{year},1,_turnout,,75.0,https://example.org,");
    }
    fs::write(sources.join("results.csv"), results).unwrap();
    fs::write(
        sources.join("round1.csv"),
        "year,candidate_name,result,final_poll_avg\n1995,Anna A,22,20\n1995,Bruno B,26,27\n1995,Chloé C,20,22\n1995,David D,32,31\n",
    )
    .unwrap();
    fs::write(
        sources.join("round2.csv"),
        "year,winner_name,winner_result,loser_name,loser_result,final_poll_avg_winner\n1995,Bruno B,54,David D,46,52.5\n",
    )
    .unwrap();
    dir
}

#[test]
fn runs_deterministically_and_fits_on_the_grid() {
    let dir = scratch_repo();
    let repo = Repo::new(dir.path());
    let mut lines = Vec::new();
    let first = fr2027_backtest::run(&repo, None, &mut |l| lines.push(l.to_string())).unwrap();
    let second = fr2027_backtest::run(&repo, None, &mut |_| {}).unwrap();
    assert_eq!(
        serde_json::to_string(&first).unwrap(),
        serde_json::to_string(&second).unwrap(),
        "same seed, same report"
    );
    assert_eq!(first.replays.len(), 4);
    assert!(lines.iter().any(|l| l.starts_with("fitted 2001")));
    // The 60-day replay sees fewer polls than the file holds: the others are published later.
    let late = first
        .replays
        .iter()
        .find(|r| r.year == 2001 && r.horizon_days == 60)
        .unwrap();
    assert!(late.polls_published < 12, "{}", late.polls_published);
    assert_eq!(late.polls_used + late.polls_overlapping, late.polls_published);

    let grid = &first.config.grid;
    assert!(grid.bloc_sd.contains(&first.fitted.bloc_sd) || first.fitted.bloc_sd == first.current.bloc_sd);
    assert!(
        grid.round2_error_sd.contains(&first.fitted.round2_error_sd)
            || first.fitted.round2_error_sd == first.current.round2_error_sd
    );
    // The chosen point never scores worse than the current setting on any source.
    if let Some(point) = first.round1_grid.iter().find(|p| p.chosen) {
        assert!(point.beats_current);
        assert!(point.evidence.total(None) >= first.round1_reference.total(None));
    }
    assert!(first.round1_grid.iter().filter(|p| p.chosen).count() <= 1);
    assert_eq!(first.leave_one_out.len(), 2);

    let report = first.markdown();
    for heading in [
        "## Scores",
        "## Reliability",
        "## Fit",
        "## Leave one election out",
        "## Replays",
    ] {
        assert!(report.contains(heading), "{heading}");
    }
    // Probabilities stay coherent in every replay.
    for replay in &first.fitted_scores.replays {
        let qualify: f64 = replay.candidates.iter().map(|c| c.p_qualify).sum();
        let win: f64 = replay.candidates.iter().map(|c| c.p_win).sum();
        assert!((qualify - 2.0).abs() < 1e-9 && (win - 1.0).abs() < 1e-9);
    }
}
