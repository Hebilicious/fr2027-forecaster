//! `fr2027 forecast`: run the model on the clean data and write the forecast file and the
//! latent-share history, both checked against their schemas before they are written.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use fr2027_collectors::{
    Repo, SchemaKind, Schemas, config::CandidatesConfig, config::load_model_params, content_hash, polls,
};
use fr2027_model::{Forecast, ModelParams, run_forecast};
use jiff::{Timestamp, civil::Date, tz::TimeZone};
use serde::Serialize;
use serde_json::Value;

#[derive(clap::Args)]
pub struct Args {
    /// Use only polls published on or before this date (default: today, UTC).
    #[arg(long)]
    as_of: Option<Date>,
    /// Override the number of simulations set in config/model.yaml.
    #[arg(long)]
    simulations: Option<u32>,
    /// Time recorded as generated_at, which also names the file (default: now).
    #[arg(long)]
    now: Option<Timestamp>,
    /// Write here instead of data/forecasts/.
    #[arg(long)]
    out_dir: Option<PathBuf>,
    /// Do nothing when the newest forecast already used the same inputs (polls, config and
    /// as-of date), so an hourly schedule refits once a day or when something changed.
    #[arg(long)]
    if_changed: bool,
}

pub const SERIES_FILE: &str = "series.json";

#[derive(Serialize)]
struct ForecastFile {
    schema_version: &'static str,
    model_version: String,
    generated_at: String,
    as_of: Date,
    inputs_hash: String,
    seed: u64,
    simulations: u32,
    election: Election,
    candidates: Vec<CandidateOut>,
    pairs: Vec<PairOut>,
    aggregation: AggregationOut,
    changes: Changes,
    warnings: Vec<String>,
}

#[derive(Serialize)]
struct Election {
    round1: Date,
    round2: Date,
}

#[derive(Serialize)]
struct CandidateOut {
    candidate_id: String,
    name: String,
    party: String,
    bloc: String,
    status: String,
    p_run: f64,
    simulated: bool,
    p_qualify_r1: f64,
    p_win: f64,
    r1_share: Option<IntervalOut>,
}

#[derive(Serialize)]
struct IntervalOut {
    mean: f64,
    lo80: f64,
    hi80: f64,
}

#[derive(Serialize)]
struct PairOut {
    a: String,
    b: String,
    p_matchup: f64,
    p_win_given_matchup: BTreeMap<String, f64>,
    polled: bool,
}

#[derive(Serialize)]
struct AggregationOut {
    random_walk_sd: f64,
    design_effect: f64,
    scenario_noise_share: f64,
    log_likelihood: f64,
    polls_used: usize,
    scenarios_used: usize,
    round2_pairs_polled: usize,
    last_field_end: Option<Date>,
    poll_ids: Vec<String>,
    house_effects: Vec<HouseOut>,
}

#[derive(Serialize)]
struct HouseOut {
    firm: String,
    bloc: String,
    mean: f64,
    sd: f64,
}

#[derive(Serialize)]
struct Changes {
    previous: Option<String>,
    new_polls: Vec<String>,
    p_win_delta: Vec<Delta>,
}

#[derive(Serialize)]
struct Delta {
    candidate_id: String,
    delta: f64,
}

#[derive(Serialize)]
struct SeriesFile {
    schema_version: &'static str,
    forecast: String,
    points: Vec<SeriesOut>,
}

#[derive(Serialize)]
struct SeriesOut {
    date: Date,
    candidate_id: String,
    mean: f64,
    lo80: f64,
    hi80: f64,
}

/// Probabilities and shares are written to four decimals: finer digits are Monte Carlo noise
/// and would only churn the diffs.
fn round4(x: f64) -> f64 {
    (x * 10_000.0).round() / 10_000.0
}

pub fn run(repo: &Repo, args: &Args) -> Result<()> {
    let schemas = Schemas::load(repo)?;
    let config = CandidatesConfig::load(repo, &schemas)?;
    let mut params = load_model_params(repo)?;
    if let Some(simulations) = args.simulations {
        params.simulations = simulations;
    }
    let rows = polls::read_clean_rows(repo).context("reading data/clean/polls.csv (run `moon run cli:ingest`)")?;
    let all_polls = polls::to_polls(&rows);
    let now = args.now.unwrap_or_else(Timestamp::now);
    let as_of = args.as_of.unwrap_or_else(|| now.to_zoned(TimeZone::UTC).date());
    let out_dir = args.out_dir.clone().unwrap_or_else(|| repo.forecasts_dir());
    let inputs_hash = inputs_hash(repo, &params, as_of)?;
    if args.if_changed
        && let Some((name, latest)) = previous_forecast(&out_dir, "")?
        && latest["inputs_hash"].as_str() == Some(inputs_hash.as_str())
    {
        println!("inputs unchanged since {name}; no new forecast");
        return Ok(());
    }
    let field = config.field()?;
    let forecast = run_forecast(&field, &all_polls, &params, as_of)?;

    let file_name = format!("{}.json", now.strftime("%Y-%m-%dT%H"));
    let previous = previous_forecast(&out_dir, &file_name)?;
    let poll_ids: Vec<String> = {
        let mut ids: Vec<String> = all_polls
            .iter()
            .filter(|p| p.published_at <= as_of)
            .map(|p| p.poll_id.clone())
            .collect();
        ids.sort();
        ids
    };
    let output = build(
        &config,
        &params,
        &forecast,
        &all_polls,
        now,
        inputs_hash,
        poll_ids,
        previous,
    );
    let value = serde_json::to_value(&output)?;
    let violations = schemas.violations(SchemaKind::Forecast, &value);
    if !violations.is_empty() {
        bail!(
            "forecast output violates schemas/forecast.schema.json: {}",
            violations.join("; ")
        );
    }
    let series = SeriesFile {
        schema_version: "1.0",
        forecast: file_name.clone(),
        points: forecast
            .round1
            .series
            .iter()
            .map(|p| SeriesOut {
                date: p.date,
                candidate_id: config.candidates[p.candidate].id.clone(),
                mean: round4(p.mean),
                lo80: round4(p.lo80),
                hi80: round4(p.hi80),
            })
            .collect(),
    };
    let series_value = serde_json::to_value(&series)?;
    let violations = schemas.violations(SchemaKind::Series, &series_value);
    if !violations.is_empty() {
        bail!(
            "series output violates schemas/series.schema.json: {}",
            violations.join("; ")
        );
    }
    std::fs::create_dir_all(&out_dir)?;
    let path = out_dir.join(&file_name);
    std::fs::write(&path, pretty(&value)?)?;
    std::fs::write(out_dir.join(SERIES_FILE), pretty(&series_value)?)?;

    println!(
        "wrote {} (as of {as_of}; {} polls, {} scenarios; random walk sd {}, design effect {}, scenario noise share {})",
        repo.relative(&path),
        forecast.round1.polls_used,
        forecast.round1.scenarios_used,
        forecast.round1.noise.random_walk_sd,
        forecast.round1.noise.design_effect,
        forecast.round1.noise.scenario_noise_share,
    );
    for candidate in output.candidates.iter().take(8) {
        println!(
            "  {:<28} win {:>5.1}%  qualify {:>5.1}%",
            candidate.name,
            candidate.p_win * 100.0,
            candidate.p_qualify_r1 * 100.0
        );
    }
    for warning in &output.warnings {
        eprintln!("warning: {warning}");
    }
    Ok(())
}

fn pretty(value: &Value) -> Result<Vec<u8>> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}

/// The newest forecast file other than `current`, as (file name, parsed JSON).
fn previous_forecast(dir: &std::path::Path, current: &str) -> Result<Option<(String, Value)>> {
    let mut names: Vec<String> = match std::fs::read_dir(dir) {
        Ok(entries) => entries
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|n| n.ends_with(".json") && n != SERIES_FILE && n != current)
            .collect(),
        Err(_) => return Ok(None),
    };
    names.sort();
    let Some(name) = names.pop() else {
        return Ok(None);
    };
    let value = serde_json::from_slice(&std::fs::read(dir.join(&name))?)
        .with_context(|| format!("reading previous forecast {name}"))?;
    Ok(Some((name, value)))
}

fn inputs_hash(repo: &Repo, params: &ModelParams, as_of: Date) -> Result<String> {
    let mut bytes = Vec::new();
    for path in [
        repo.clean_dir().join(polls::CLEAN_FILE),
        repo.config_dir().join("candidates.yaml"),
        repo.config_dir().join("model.yaml"),
    ] {
        bytes.extend(std::fs::read(&path).with_context(|| format!("reading {}", path.display()))?);
        bytes.push(0);
    }
    bytes.extend(format!("as_of={as_of};simulations={}", params.simulations).bytes());
    Ok(content_hash(&bytes))
}

#[allow(clippy::too_many_arguments)]
fn build(
    config: &CandidatesConfig,
    params: &ModelParams,
    forecast: &Forecast,
    all_polls: &[fr2027_model::Poll],
    now: Timestamp,
    inputs_hash: String,
    poll_ids: Vec<String>,
    previous: Option<(String, Value)>,
) -> ForecastFile {
    let id = |c: usize| config.candidates[c].id.clone();
    let mut candidates: Vec<CandidateOut> = forecast
        .candidates
        .iter()
        .map(|c| {
            let entry = &config.candidates[c.candidate];
            CandidateOut {
                candidate_id: entry.id.clone(),
                name: entry.name.clone(),
                party: entry.party.clone(),
                bloc: entry.bloc.clone(),
                status: entry.status.clone(),
                p_run: round4(c.p_run),
                simulated: c.simulated,
                p_qualify_r1: round4(c.p_qualify_r1),
                p_win: round4(c.p_win),
                r1_share: c.r1_share.as_ref().map(|s| IntervalOut {
                    mean: round4(s.mean),
                    lo80: round4(s.lo80),
                    hi80: round4(s.hi80),
                }),
            }
        })
        .collect();
    candidates.sort_by(|a, b| {
        b.p_win
            .total_cmp(&a.p_win)
            .then(b.p_qualify_r1.total_cmp(&a.p_qualify_r1))
            .then(b.p_run.total_cmp(&a.p_run))
            .then(a.candidate_id.cmp(&b.candidate_id))
    });
    let pairs = forecast
        .pairs
        .iter()
        .filter(|p| p.p_matchup >= 0.001)
        .map(|p| PairOut {
            a: id(p.a),
            b: id(p.b),
            p_matchup: round4(p.p_matchup),
            p_win_given_matchup: BTreeMap::from([(id(p.a), round4(p.p_a_wins)), (id(p.b), round4(1.0 - p.p_a_wins))]),
            polled: p.polled,
        })
        .collect();
    let blocs = &config.blocs;
    let house_effects = forecast
        .round1
        .house_effects
        .iter()
        .map(|h| HouseOut {
            firm: h.firm.clone(),
            bloc: blocs[h.bloc].clone(),
            mean: round4(h.mean),
            sd: round4(h.sd),
        })
        .collect();
    let last_field_end = all_polls
        .iter()
        .filter(|p| p.published_at <= forecast.as_of)
        .map(|p| p.field_end)
        .max();

    let (previous_name, new_polls, p_win_delta) = match previous {
        None => (None, poll_ids.clone(), Vec::new()),
        Some((name, value)) => {
            let before: BTreeSet<String> = value["aggregation"]["poll_ids"]
                .as_array()
                .map(|ids| ids.iter().filter_map(|i| i.as_str().map(String::from)).collect())
                .unwrap_or_default();
            let new_polls = poll_ids.iter().filter(|id| !before.contains(*id)).cloned().collect();
            let mut deltas: Vec<Delta> = candidates
                .iter()
                .filter_map(|c| {
                    let old = value["candidates"]
                        .as_array()?
                        .iter()
                        .find(|o| o["candidate_id"] == c.candidate_id.as_str())?["p_win"]
                        .as_f64()?;
                    Some(Delta {
                        candidate_id: c.candidate_id.clone(),
                        delta: round4(c.p_win - old),
                    })
                })
                .filter(|d| d.delta != 0.0)
                .collect();
            deltas.sort_by(|a, b| b.delta.abs().total_cmp(&a.delta.abs()));
            (Some(name), new_polls, deltas)
        }
    };

    ForecastFile {
        schema_version: "1.0",
        model_version: params.model_version.clone(),
        generated_at: now.strftime("%Y-%m-%dT%H:%M:%SZ").to_string(),
        as_of: forecast.as_of,
        inputs_hash,
        seed: forecast.seed,
        simulations: forecast.simulations,
        election: Election {
            round1: params.round1_date,
            round2: params.round2_date,
        },
        candidates,
        pairs,
        aggregation: AggregationOut {
            random_walk_sd: forecast.round1.noise.random_walk_sd,
            design_effect: forecast.round1.noise.design_effect,
            scenario_noise_share: forecast.round1.noise.scenario_noise_share,
            log_likelihood: (forecast.round1.log_likelihood * 100.0).round() / 100.0,
            polls_used: forecast.round1.polls_used,
            scenarios_used: forecast.round1.scenarios_used,
            round2_pairs_polled: forecast.round2.pairs.len(),
            last_field_end,
            poll_ids,
            house_effects,
        },
        changes: Changes {
            previous: previous_name,
            new_polls,
            p_win_delta,
        },
        warnings: forecast.warnings.clone(),
    }
}
