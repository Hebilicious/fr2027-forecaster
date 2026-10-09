//! Runs the real `fr2027` binary on a scratch repository: ingest, forecast, validate. The
//! candidates and polls are fictional; the schemas and model parameters are the real ones.

use std::fs;
use std::path::Path;
use std::process::Command;

const CANDIDATES: &str = r#"
schema_version: "1.0"
as_of: 2026-10-01
blocs: [far_left, left, centre, right, far_right, other]
candidates:
  - { id: anna, name: Anna, party: Left party, bloc: left, status: declared, sources: [] }
  - { id: bruno, name: Bruno, party: Green party, bloc: left, status: likely, sources: [] }
  - { id: chloe, name: Chloé, party: Centre party, bloc: centre, status: declared, sources: [] }
  - { id: david, name: David, party: Right party, bloc: right, status: possible, sources: [] }
  - { id: emma, name: Emma, party: Far-right party, bloc: far_right, status: likely, sources: [] }
  - { id: felix, name: Félix, party: Far-right party, bloc: far_right, status: possible, sources: [] }
slots:
  - { id: anna, options: [{ run: [anna], p: 0.95 }] }
  - { id: bruno, options: [{ run: [bruno], p: 0.6 }] }
  - { id: chloe, options: [{ run: [chloe], p: 0.9 }] }
  - { id: david, options: [{ run: [david], p: 0.8 }] }
  - { id: far_right_nominee, options: [{ run: [emma], p: 0.7 }, { run: [felix], p: 0.3 }] }
"#;

fn poll(id: &str, firm: &str, day: u32, emma: f64) -> String {
    format!(
        r#"schema_version: "1.0"
poll_id: {id}
firm: {firm}
sponsor: null
field_start: 2026-09-{day:02}
field_end: 2026-09-{end:02}
published_at: 2026-09-{published:02}
sample_size: 1200
method: online
population: registered
source_url: https://example.org/{id}.pdf
retrieved_at: 2026-10-01T00:00:00Z
entry: primary
scenarios:
  - scenario_id: A
    round: 1
    shares: {{ anna: 15, bruno: 6, chloe: 22, david: 12, emma: {emma} }}
  - scenario_id: B
    round: 1
    shares: {{ anna: 19, chloe: 23, david: 13, felix: {felix} }}
  - scenario_id: R2
    round: 2
    shares: {{ emma: 51, chloe: 49 }}
"#,
        end = day + 1,
        published = day + 2,
        felix = emma - 5.0,
    )
}

fn scratch_repo() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    for sub in ["schemas", "config"] {
        fs::create_dir_all(root.join(sub)).unwrap();
    }
    for entry in fs::read_dir(source.join("schemas")).unwrap() {
        let entry = entry.unwrap();
        fs::copy(entry.path(), root.join("schemas").join(entry.file_name())).unwrap();
    }
    fs::copy(source.join("config/model.yaml"), root.join("config/model.yaml")).unwrap();
    fs::write(root.join("config/candidates.yaml"), CANDIDATES).unwrap();
    let polls = root.join("data/raw/polls");
    fs::create_dir_all(&polls).unwrap();
    for (i, firm) in ["ifop", "elabe", "odoxa", "ifop", "elabe", "odoxa"].iter().enumerate() {
        let id = format!("{firm}-2026-09-{:02}", 1 + i * 4);
        let emma = 44.0 + (i % 3) as f64;
        fs::write(
            polls.join(format!("{id}.yaml")),
            poll(&id, firm, 1 + i as u32 * 4, emma),
        )
        .unwrap();
    }
    dir
}

fn fr2027(root: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_fr2027"))
        .arg("--repo")
        .arg(root)
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn ingest_forecast_validate() {
    let repo = scratch_repo();
    let root = repo.path();

    // Before ingest, validate reports the missing clean data.
    let before = fr2027(root, &["validate"]);
    assert!(!before.status.success());
    assert!(String::from_utf8_lossy(&before.stderr).contains("out of date"));

    let ingest = fr2027(root, &["ingest"]);
    assert!(ingest.status.success(), "{}", String::from_utf8_lossy(&ingest.stderr));
    assert!(String::from_utf8_lossy(&ingest.stdout).contains("polls: 6 accepted"));

    let args = [
        "forecast",
        "--as-of",
        "2026-10-01",
        "--now",
        "2026-10-01T12:00:00Z",
        "--simulations",
        "4000",
    ];
    let forecast = fr2027(root, &args);
    assert!(
        forecast.status.success(),
        "{}",
        String::from_utf8_lossy(&forecast.stderr)
    );

    let file = root.join("data/forecasts/2026-10-01T12.json");
    let value: serde_json::Value = serde_json::from_slice(&fs::read(&file).unwrap()).unwrap();
    let candidates = value["candidates"].as_array().unwrap();
    let total = |key: &str| candidates.iter().map(|c| c[key].as_f64().unwrap()).sum::<f64>();
    assert!((total("p_win") - 1.0).abs() < 0.002, "p_win sums to {}", total("p_win"));
    assert!((total("p_qualify_r1") - 2.0).abs() < 0.002);
    assert_eq!(value["aggregation"]["polls_used"], 6);
    assert_eq!(value["changes"]["previous"], serde_json::Value::Null);
    assert!(root.join("data/forecasts/series.json").exists());

    // Same inputs, same seed: the same numbers.
    let again = fr2027(
        root,
        &[
            "forecast",
            "--as-of",
            "2026-10-01",
            "--now",
            "2026-10-01T13:00:00Z",
            "--simulations",
            "4000",
        ],
    );
    assert!(again.status.success());
    let second: serde_json::Value =
        serde_json::from_slice(&fs::read(root.join("data/forecasts/2026-10-01T13.json")).unwrap()).unwrap();
    assert_eq!(second["candidates"], value["candidates"]);
    assert_eq!(second["changes"]["previous"], "2026-10-01T12.json");
    assert_eq!(second["changes"]["new_polls"], serde_json::json!([]));

    let validate = fr2027(root, &["validate"]);
    assert!(
        validate.status.success(),
        "{}",
        String::from_utf8_lossy(&validate.stderr)
    );
}

#[test]
fn leakage_guard_drops_later_polls() {
    let repo = scratch_repo();
    let root = repo.path();
    assert!(fr2027(root, &["ingest"]).status.success());
    let early = fr2027(
        root,
        &[
            "forecast",
            "--as-of",
            "2026-09-12",
            "--now",
            "2026-09-12T00:00:00Z",
            "--simulations",
            "2000",
        ],
    );
    assert!(early.status.success(), "{}", String::from_utf8_lossy(&early.stderr));
    let value: serde_json::Value =
        serde_json::from_slice(&fs::read(root.join("data/forecasts/2026-09-12T00.json")).unwrap()).unwrap();
    // Polls published on 09-03, 09-07 and 09-11 only.
    assert_eq!(value["aggregation"]["polls_used"], 3);
    for id in value["aggregation"]["poll_ids"].as_array().unwrap() {
        let id = id.as_str().unwrap();
        let day: u32 = id[id.len() - 2..].parse().unwrap();
        assert!(day + 2 <= 12, "{id} was published after the as-of date");
    }
}
