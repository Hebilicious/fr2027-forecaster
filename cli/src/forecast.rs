//! `fr2027 forecast`: run the model on the clean data and write the forecast file and the
//! latent-share history, both checked against their schemas before they are written.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use fr2027_collectors::{
    Repo, SchemaKind, Schemas, config::CandidatesConfig, config::load_model_params, content_hash, markets, polls,
};
use fr2027_model::{Forecast, ModelParams, SignalSource, run_forecast, signals::detect};
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
    /// The settings of config/model.yaml this run simulated with, so the page explains the model
    /// that produced the numbers it shows.
    parameters: Parameters,
    signals: SignalsOut,
    ballot: BallotOut,
    /// The chances at several strengths of the news and social signals, for the UI's slider.
    signal_levels: Vec<LevelOut>,
    changes: Changes,
    warnings: Vec<String>,
}

#[derive(Serialize)]
struct Parameters {
    round1_error: fr2027_model::ErrorParams,
    round2: fr2027_model::Round2Params,
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
    /// The config slot the candidate runs from, shared with the others who can only run instead of them.
    slot: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    anchor: Option<AnchorOut>,
}

/// The tested matchup an untested one was anchored on.
#[derive(Serialize)]
struct AnchorOut {
    a: String,
    b: String,
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

/// The spikes of the last [`SPIKE_DAYS`] days and what they add.
#[derive(Serialize)]
struct SignalsOut {
    sources: Vec<SignalSource>,
    shock_sd: f64,
    half_life_days: f64,
    momentum_weight: f64,
    x_momentum_weight: f64,
    tone_momentum_weight: f64,
    spikes: Vec<SpikeOut>,
    /// Candidates whose round 1 vote the momentum tilts, and by how much (log scale).
    momentum: Vec<TiltOut>,
}

#[derive(Serialize)]
struct TiltOut {
    candidate_id: String,
    tilt: f64,
}

/// The forecast with every news and social signal weight (spike widening, attention, X and tone
/// momentum) multiplied by `scale`: 0 is the polls alone, 1 the published weights.
#[derive(Serialize)]
struct LevelOut {
    scale: f64,
    candidates: Vec<LevelCandidate>,
}

#[derive(Serialize)]
struct LevelCandidate {
    candidate_id: String,
    p_qualify_r1: f64,
    p_win: f64,
}

/// The strengths of the news and social signals the forecast is also run at, as multiples of the
/// published weights.
const SIGNAL_LEVELS: [f64; 5] = [0.0, 0.5, 1.0, 1.5, 2.0];

/// The signal weights multiplied by `scale`.
fn scaled_signals(signals: &fr2027_model::SignalParams, scale: f64) -> fr2027_model::SignalParams {
    fr2027_model::SignalParams {
        shock_sd_grid: signals.shock_sd_grid.iter().map(|s| s * scale).collect(),
        momentum_weight: signals.momentum_weight * scale,
        x_momentum_weight: signals.x_momentum_weight * scale,
        tone_momentum_weight: signals.tone_momentum_weight * scale,
        ..signals.clone()
    }
}

/// Who runs: the hand-set chances and the "on the ballot" prices they were pooled with.
#[derive(Serialize)]
struct BallotOut {
    hand_weight: f64,
    market_fetched_at: Option<String>,
    candidates: Vec<BallotCandidate>,
}

#[derive(Serialize)]
struct BallotCandidate {
    candidate_id: String,
    p_run_hand: f64,
    p_market: f64,
}

#[derive(Serialize)]
struct SpikeOut {
    candidate_id: String,
    date: Date,
    score: f64,
    ratios: BTreeMap<SignalSource, f64>,
}

/// Spikes older than this are left out of the forecast file: by then a spike with a 10-day
/// half-life has accrued 98% of its variance.
const SPIKE_DAYS: i32 = 60;

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
    /// Spikes this run detected that the previous forecast had not.
    new_spikes: Vec<NewSpike>,
    p_win_delta: Vec<Delta>,
}

#[derive(Serialize)]
struct NewSpike {
    candidate_id: String,
    date: Date,
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
    // Who runs: the hand-set chances pooled with the newest "on the ballot" prices, when fresh.
    let hand_field = config.field()?;
    let ids: Vec<&str> = config.candidates.iter().map(|c| c.id.as_str()).collect();
    let snapshot = markets::fresh_snapshot(repo, now, params.market_blend.max_age_hours)?;
    let ballot_prices = match &snapshot {
        Some(s) => markets::venue_average(s, "on_ballot", &ids).0,
        None => vec![None; ids.len()],
    };
    let hand_p_run = hand_field.p_run();
    let targets = fr2027_model::blend::run_targets(&hand_p_run, &ballot_prices, &params.market_blend);
    let field = hand_field.with_run_targets(&targets);
    let signals = fr2027_collectors::signals::read_clean(repo)?;
    // What the signals change goes into the hash, not the raw files: the spikes, the momentum and
    // the market-moved chances of running, rounded, so a day of page views or an hour of prices
    // that moves nothing the model computes does not refit it.
    let spikes = detect(&signals, &field, &params.signals, as_of);
    let momentum = fr2027_model::signals::momentum(&signals, &field, &params.signals, as_of, params.round1_date);
    let mut signal_key: Vec<String> = spikes
        .iter()
        .map(|s| format!("{}@{}", ids[s.candidate], s.date))
        .collect();
    signal_key.extend(
        momentum
            .iter()
            .zip(&ids)
            .filter(|(m, _)| m.abs() >= 0.005)
            .map(|(m, id)| format!("{id}~{:.2}", m)),
    );
    signal_key.extend(field.p_run().iter().zip(&ids).map(|(p, id)| format!("{id}:{:.2}", p)));
    let inputs_hash = inputs_hash(repo, &params, as_of, &signal_key)?;
    if args.if_changed
        && let Some((name, latest)) = previous_forecast(&out_dir, "")?
        && latest["inputs_hash"].as_str() == Some(inputs_hash.as_str())
    {
        println!("inputs unchanged since {name}; no new forecast");
        return Ok(());
    }
    let forecast = run_forecast(&field, &all_polls, &signals, &params, as_of)?;
    // The other strengths, one thread each; the published one is the forecast itself.
    let others: Vec<Result<(f64, Forecast)>> = std::thread::scope(|scope| {
        let handles: Vec<_> = SIGNAL_LEVELS
            .iter()
            .filter(|s| **s != 1.0)
            .map(|&scale| {
                let mut level = params.clone();
                level.signals = scaled_signals(&params.signals, scale);
                let (field, all_polls, signals) = (&field, &all_polls, &signals);
                scope.spawn(move || {
                    run_forecast(field, all_polls, signals, &level, as_of)
                        .map(|f| (scale, f))
                        .map_err(anyhow::Error::from)
                })
            })
            .collect();
        handles.into_iter().map(|h| h.join().expect("level thread")).collect()
    });
    let mut levels: Vec<(f64, &Forecast)> = vec![(1.0, &forecast)];
    let others: Vec<(f64, Forecast)> = others.into_iter().collect::<Result<_>>()?;
    levels.extend(others.iter().map(|(s, f)| (*s, f)));
    levels.sort_by(|a, b| a.0.total_cmp(&b.0));
    let signal_levels: Vec<LevelOut> = levels
        .iter()
        .map(|(scale, f)| LevelOut {
            scale: *scale,
            candidates: f
                .candidates
                .iter()
                .map(|c| LevelCandidate {
                    candidate_id: config.candidates[c.candidate].id.clone(),
                    p_qualify_r1: round4(c.p_qualify_r1),
                    p_win: round4(c.p_win),
                })
                .collect(),
        })
        .collect();

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
    let ballot = BallotOut {
        hand_weight: params.market_blend.ballot_hand_weight,
        market_fetched_at: snapshot.as_ref().map(|s| s.fetched_at.clone()),
        candidates: ids
            .iter()
            .zip(&hand_p_run)
            .zip(&ballot_prices)
            .filter_map(|((id, hand), market)| {
                market.map(|m| BallotCandidate {
                    candidate_id: id.to_string(),
                    p_run_hand: round4(*hand),
                    p_market: round4(m),
                })
            })
            .collect(),
    };
    let mut output = build(
        &config,
        &params,
        &forecast,
        &all_polls,
        now,
        inputs_hash,
        poll_ids,
        previous,
        ballot,
    );
    output.signal_levels = signal_levels;
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

/// Bumped whenever the forecast file gains, loses or changes a field, so that the next run refits rather than
/// keep a file the UI can no longer read in full.
const OUTPUT_VERSION: u32 = 5;

fn inputs_hash(repo: &Repo, params: &ModelParams, as_of: Date, signals: &[String]) -> Result<String> {
    let mut bytes = Vec::new();
    for path in [
        repo.clean_dir().join(polls::CLEAN_FILE),
        repo.config_dir().join("candidates.yaml"),
        repo.config_dir().join("model.yaml"),
    ] {
        bytes.extend(std::fs::read(&path).with_context(|| format!("reading {}", path.display()))?);
        bytes.push(0);
    }
    bytes.extend(
        format!(
            "output={OUTPUT_VERSION};as_of={as_of};simulations={}",
            params.simulations
        )
        .bytes(),
    );
    if !signals.is_empty() {
        bytes.extend(format!(";signals={}", signals.join(",")).bytes());
    }
    Ok(content_hash(&bytes))
}

/// The id of the slot whose options name the candidate; the field model rejects a config where none does.
fn slot_of(config: &CandidatesConfig, candidate: &str) -> String {
    config
        .slots
        .iter()
        .find(|s| s.options.iter().any(|o| o.run.iter().any(|id| id == candidate)))
        .map(|s| s.id.clone())
        .unwrap_or_default()
}

/// The forecast's spikes of the last [`SPIKE_DAYS`] days and the settings they were found with.
fn signals_out(params: &ModelParams, forecast: &Forecast, id: &dyn Fn(usize) -> String) -> SignalsOut {
    SignalsOut {
        sources: params.signals.sources.clone(),
        shock_sd: forecast.round1.noise.shock_sd,
        half_life_days: params.signals.half_life_days,
        momentum_weight: params.signals.momentum_weight,
        x_momentum_weight: params.signals.x_momentum_weight,
        tone_momentum_weight: params.signals.tone_momentum_weight,
        momentum: forecast
            .momentum
            .iter()
            .enumerate()
            .filter(|(_, m)| m.abs() >= 0.0005)
            .map(|(c, m)| TiltOut {
                candidate_id: id(c),
                tilt: round4(*m),
            })
            .collect(),
        spikes: forecast
            .round1
            .spikes
            .iter()
            .filter(|s| fr2027_model::day_number(forecast.as_of) - fr2027_model::day_number(s.date) <= SPIKE_DAYS)
            .map(|s| SpikeOut {
                candidate_id: id(s.candidate),
                date: s.date,
                score: round4(s.score),
                ratios: s.ratios.iter().map(|(source, r)| (*source, round4(*r))).collect(),
            })
            .collect(),
    }
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
    ballot: BallotOut,
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
                slot: slot_of(config, &entry.id),
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
            anchor: p.anchor.map(|(a, b)| AnchorOut { a: id(a), b: id(b) }),
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

    let spikes = signals_out(params, forecast, &id);
    let (previous_name, new_polls, new_spikes, p_win_delta) = match previous {
        None => (None, poll_ids.clone(), Vec::new(), Vec::new()),
        Some((name, value)) => {
            // A previous forecast written before event detection has no spikes to compare with.
            let new_spikes = match value["signals"]["spikes"].as_array() {
                None => Vec::new(),
                Some(before) => spikes
                    .spikes
                    .iter()
                    .filter(|s| {
                        !before.iter().any(|b| {
                            b["candidate_id"] == s.candidate_id.as_str() && b["date"] == s.date.to_string().as_str()
                        })
                    })
                    .map(|s| NewSpike {
                        candidate_id: s.candidate_id.clone(),
                        date: s.date,
                    })
                    .collect(),
            };
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
            (Some(name), new_polls, new_spikes, deltas)
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
        parameters: Parameters {
            round1_error: params.round1_error.clone(),
            round2: params.round2.clone(),
        },
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
        signals: spikes,
        ballot,
        signal_levels: Vec::new(),
        changes: Changes {
            previous: previous_name,
            new_polls,
            new_spikes,
            p_win_delta,
        },
        warnings: forecast.warnings.clone(),
    }
}
