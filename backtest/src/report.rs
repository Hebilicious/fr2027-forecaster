//! The whole backtest run: replays, scores under the current settings, the grid search, the
//! leave-one-election-out check, and the report written to `research/backtest/`.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use fr2027_collectors::{Repo, Schemas, config::load_model_params};
use fr2027_model::{ErrorParams, ModelParams};
use serde::Serialize;

use crate::{
    Result, baseline,
    config::BacktestConfig,
    election::{self, Election},
    final_polls::{self, Round1Election, Round2Election},
    replay::{self, Replay},
    score::{self, ReliabilityBin, ReplayScore, Summary},
};

#[derive(Clone, Debug, Serialize)]
pub struct Report {
    pub schema_version: &'static str,
    pub model_version: String,
    pub config: BacktestConfig,
    pub elections: Vec<ElectionInfo>,
    pub replays: Vec<ReplayInfo>,
    /// The current settings (config/model.yaml) and the fitted ones.
    pub current: Setting,
    pub fitted: Setting,
    /// The latest-poll average.
    pub baseline: SettingScores,
    pub current_scores: SettingScores,
    pub fitted_scores: SettingScores,
    /// The current setting's evidence, which a grid point must match on every source.
    pub round1_reference: Evidence,
    pub round1_grid: Vec<Round1GridPoint>,
    /// The current round 2 error, with round 1 at its fitted values.
    pub round2_reference: Evidence,
    pub round2_grid: Vec<Round2GridPoint>,
    pub leave_one_out: Vec<LeaveOneOut>,
    pub final_polls_round2: Vec<Round2Election>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ElectionInfo {
    pub year: i16,
    pub round1: jiff::civil::Date,
    pub round2: jiff::civil::Date,
    pub polls_in_file: usize,
    pub finalists: [String; 2],
    pub winner: String,
    pub winner_round2_share: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct ReplayInfo {
    pub year: i16,
    pub horizon_days: u32,
    pub as_of: jiff::civil::Date,
    pub polls_published: usize,
    pub polls_overlapping: usize,
    pub polls_used: usize,
    pub scenarios_used: usize,
    pub round2_pairs_polled: usize,
    pub random_walk_sd: f64,
    pub design_effect: f64,
    pub scenario_noise_share: f64,
    pub contenders: Vec<String>,
    pub not_yet_polled: Vec<String>,
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq)]
pub struct Setting {
    pub bloc_sd: f64,
    pub candidate_sd: f64,
    pub round2_error_sd: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct SettingScores {
    pub overall: Summary,
    pub by_year: BTreeMap<i16, Summary>,
    pub by_horizon: BTreeMap<u32, Summary>,
    /// Log scores of the 2002–2012 final-week poll errors (none for the baseline).
    pub final_polls_round1: Option<f64>,
    pub final_polls_round2: Option<f64>,
    pub reliability: Vec<ReliabilityBin>,
    pub replays: Vec<ReplayScore>,
}

/// The log scores a setting earns from each independent body of evidence: each election's
/// replays, and the 2002–2012 final-week polls.
#[derive(Clone, Debug, Serialize)]
pub struct Evidence {
    /// Summed over each election's replays.
    pub by_year: BTreeMap<i16, f64>,
    /// Summed over both elections, by horizon (shown, not used by the fit).
    pub by_horizon: BTreeMap<u32, f64>,
    pub final_polls: f64,
}

impl Evidence {
    /// The sum of every source, leaving out one election's replays if asked.
    pub fn total(&self, without: Option<i16>) -> f64 {
        self.by_year
            .iter()
            .filter(|(year, _)| Some(**year) != without)
            .map(|(_, score)| score)
            .sum::<f64>()
            + self.final_polls
    }

    /// Whether this scores at least as well as `reference` on every source (but the one left out).
    pub fn at_least(&self, reference: &Evidence, without: Option<i16>) -> bool {
        self.final_polls >= reference.final_polls
            && self
                .by_year
                .iter()
                .filter(|(year, _)| Some(**year) != without)
                .all(|(year, score)| *score >= reference.by_year[year])
    }
}

/// The fit's rule: among the grid points that score at least as well as the current setting on
/// every source of evidence, the one with the highest total. None when no point does, which
/// keeps the current setting. A value that one election favours and another disfavours is not
/// adopted.
fn choose<'a, T>(
    grid: &'a [T],
    evidence: impl Fn(&T) -> &Evidence,
    reference: &Evidence,
    without: Option<i16>,
) -> Option<&'a T> {
    grid.iter()
        .filter(|p| evidence(p).at_least(reference, without))
        .max_by(|a, b| evidence(a).total(without).total_cmp(&evidence(b).total(without)))
}

#[derive(Clone, Debug, Serialize)]
pub struct Round1GridPoint {
    pub bloc_sd: f64,
    pub candidate_sd: f64,
    /// Contenders' share log scores.
    pub evidence: Evidence,
    pub total: f64,
    /// Scores at least as well as the current setting on every source.
    pub beats_current: bool,
    pub chosen: bool,
    pub qualify_log_loss: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct Round2GridPoint {
    pub error_sd: f64,
    /// Log scores of the realised runoff share.
    pub evidence: Evidence,
    pub total: f64,
    pub beats_current: bool,
    pub chosen: bool,
    pub p_winner_given_matchup: f64,
}

/// The fit's rule applied without one election's replays, then that election's replays scored.
#[derive(Clone, Debug, Serialize)]
pub struct LeaveOneOut {
    pub held_out: i16,
    pub bloc_sd: f64,
    pub candidate_sd: f64,
    pub round2_error_sd: f64,
    /// The held-out election's share log score at the setting chosen without it, and at the
    /// current setting.
    pub round1_held_out: f64,
    pub round1_held_out_current: f64,
    /// The same for the round 2 log score (round 1 at its fitted values in both).
    pub round2_held_out: f64,
    pub round2_held_out_current: f64,
}

fn with_errors(params: &ModelParams, setting: Setting) -> ModelParams {
    let mut params = params.clone();
    params.round1_error = ErrorParams {
        bloc_sd: setting.bloc_sd,
        candidate_sd: setting.candidate_sd,
        ..params.round1_error
    };
    params.round2.error_sd = setting.round2_error_sd;
    params
}

/// Scores every replay under each setting, one thread per replay.
fn evaluate_all(elections: &[Election], replays: &[Replay], settings: &[Setting]) -> Result<Vec<Vec<ReplayScore>>> {
    let per_replay: Vec<Result<Vec<ReplayScore>>> = std::thread::scope(|scope| {
        let handles: Vec<_> = replays
            .iter()
            .map(|replay| {
                let election = elections
                    .iter()
                    .find(|e| e.year == replay.year)
                    .expect("replay of a loaded election");
                scope.spawn(move || {
                    settings
                        .iter()
                        .map(|setting| score::evaluate(election, replay, &with_errors(&replay.params, *setting)))
                        .collect::<Result<Vec<_>>>()
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().expect("replay thread panicked"))
            .collect()
    });
    let per_replay: Vec<Vec<ReplayScore>> = per_replay.into_iter().collect::<Result<_>>()?;
    // Transpose to one list of replay scores per setting.
    Ok((0..settings.len())
        .map(|s| per_replay.iter().map(|scores| scores[s].clone()).collect())
        .collect())
}

fn setting_scores(
    scores: Vec<ReplayScore>,
    floor: f64,
    final_polls_round1: Option<f64>,
    final_polls_round2: Option<f64>,
) -> SettingScores {
    let mut by_year: BTreeMap<i16, Vec<&ReplayScore>> = BTreeMap::new();
    let mut by_horizon: BTreeMap<u32, Vec<&ReplayScore>> = BTreeMap::new();
    for score in &scores {
        by_year.entry(score.year).or_default().push(score);
        by_horizon.entry(score.horizon_days).or_default().push(score);
    }
    SettingScores {
        overall: score::summarise(&scores, floor),
        by_year: by_year
            .into_iter()
            .map(|(k, v)| (k, score::summarise(v, floor)))
            .collect(),
        by_horizon: by_horizon
            .into_iter()
            .map(|(k, v)| (k, score::summarise(v, floor)))
            .collect(),
        final_polls_round1,
        final_polls_round2,
        reliability: score::reliability(scores.iter().flat_map(|s| s.events.iter())),
        replays: scores,
    }
}

fn round1_final_polls(
    elections: &[Round1Election],
    blocs: usize,
    params: &ModelParams,
    setting: Setting,
    config: &BacktestConfig,
) -> f64 {
    let error = ErrorParams {
        bloc_sd: setting.bloc_sd,
        candidate_sd: setting.candidate_sd,
        ..params.round1_error
    };
    elections
        .iter()
        .map(|e| {
            final_polls::round1_log_score(
                e,
                blocs,
                &error,
                config.simulations,
                config.seed,
                config.contender_share,
            )
        })
        .sum()
}

fn round2_final_polls(elections: &[Round2Election], error_sd: f64, df: f64) -> f64 {
    elections
        .iter()
        .map(|e| final_polls::round2_log_score(e, error_sd, df))
        .sum()
}

fn argmax<T>(items: &[T], key: impl Fn(&T) -> f64) -> &T {
    items
        .iter()
        .max_by(|a, b| key(a).total_cmp(&key(b)))
        .expect("non-empty grid")
}

/// Runs the backtest on the repository's data and settings; `simulations` overrides the number
/// set in config/backtest.yaml.
pub fn run(repo: &Repo, simulations: Option<u32>, progress: &mut dyn FnMut(&str)) -> Result<Report> {
    let mut config = BacktestConfig::load(repo)?;
    if let Some(simulations) = simulations {
        config.simulations = simulations.max(1);
    }
    let model = load_model_params(repo)?;
    let schemas = Schemas::load(repo)?;
    let floor = config.probability_floor;
    let df = model.round1_error.student_t_df;

    let mut elections = Vec::new();
    for entry in &config.elections {
        elections.push(election::load(repo, &schemas, entry, &config.results_file)?);
    }
    let blocs = elections[0].field.blocs.clone();
    let final_round1 = final_polls::load_round1(repo, &config, &blocs)?;
    let final_round2 = final_polls::load_round2(repo, &config)?;

    let mut replays = Vec::new();
    for election in &elections {
        for &horizon in &config.horizons_days {
            let replay = replay::prepare(election, &model, &config, horizon)?;
            progress(&format!(
                "fitted {} as of {} ({} days out): {} polls ({} overlapping dropped)",
                election.year, replay.as_of, horizon, replay.polls_used, replay.polls_overlapping
            ));
            replays.push(replay);
        }
    }

    let current = Setting {
        bloc_sd: model.round1_error.bloc_sd,
        candidate_sd: model.round1_error.candidate_sd,
        round2_error_sd: model.round2.error_sd,
    };
    let years: Vec<i16> = elections.iter().map(|e| e.year).collect();

    // Round 1: every (bloc_sd, candidate_sd) on the grid with round 2 at its current error, and
    // the current setting itself as the reference.
    let mut round1_settings: Vec<Setting> = config
        .grid
        .bloc_sd
        .iter()
        .flat_map(|b| {
            config.grid.candidate_sd.iter().map(move |c| Setting {
                bloc_sd: *b,
                candidate_sd: *c,
                round2_error_sd: current.round2_error_sd,
            })
        })
        .collect();
    round1_settings.push(current);
    progress(&format!("round 1 grid: {} settings", round1_settings.len() - 1));
    let mut round1_scores = evaluate_all(&elections, &replays, &round1_settings)?;
    let current_replays = round1_scores.pop().expect("the reference setting");
    round1_settings.pop();
    let round1_evidence = |setting: Setting, scores: &[ReplayScore]| -> Evidence {
        let mut by_year: BTreeMap<i16, f64> = years.iter().map(|y| (*y, 0.0)).collect();
        let mut by_horizon = BTreeMap::new();
        for s in scores {
            *by_year.entry(s.year).or_insert(0.0) += s.share_log_score();
            *by_horizon.entry(s.horizon_days).or_insert(0.0) += s.share_log_score();
        }
        Evidence {
            by_year,
            by_horizon,
            final_polls: round1_final_polls(&final_round1, blocs.len(), &model, setting, &config),
        }
    };
    let round1_reference = round1_evidence(current, &current_replays);
    let mut round1_grid: Vec<Round1GridPoint> = round1_settings
        .iter()
        .zip(&round1_scores)
        .map(|(setting, scores)| {
            let evidence = round1_evidence(*setting, scores);
            Round1GridPoint {
                bloc_sd: setting.bloc_sd,
                candidate_sd: setting.candidate_sd,
                total: evidence.total(None),
                beats_current: evidence.at_least(&round1_reference, None),
                chosen: false,
                qualify_log_loss: score::summarise(scores, floor).qualify_log_loss,
                evidence,
            }
        })
        .collect();
    let chosen1 = choose(&round1_grid, |p| &p.evidence, &round1_reference, None);
    let (bloc_sd, candidate_sd) =
        chosen1.map_or((current.bloc_sd, current.candidate_sd), |p| (p.bloc_sd, p.candidate_sd));
    for point in &mut round1_grid {
        point.chosen = point.bloc_sd == bloc_sd && point.candidate_sd == candidate_sd;
    }

    // Round 2: the error_sd grid with round 1 at its chosen values, and the current round 2 error
    // (with the same round 1) as the reference.
    let mut round2_settings: Vec<Setting> = config
        .grid
        .round2_error_sd
        .iter()
        .map(|sd| Setting {
            bloc_sd,
            candidate_sd,
            round2_error_sd: *sd,
        })
        .collect();
    round2_settings.push(Setting {
        bloc_sd,
        candidate_sd,
        round2_error_sd: current.round2_error_sd,
    });
    progress(&format!("round 2 grid: {} settings", round2_settings.len() - 1));
    let mut round2_scores = evaluate_all(&elections, &replays, &round2_settings)?;
    let round2_reference_replays = round2_scores.pop().expect("the reference setting");
    round2_settings.pop();
    let round2_evidence = |error_sd: f64, scores: &[ReplayScore]| -> Evidence {
        let mut by_year: BTreeMap<i16, f64> = years.iter().map(|y| (*y, 0.0)).collect();
        let mut by_horizon = BTreeMap::new();
        for s in scores {
            let value = s.round2.log_score.unwrap_or(0.0);
            *by_year.entry(s.year).or_insert(0.0) += value;
            *by_horizon.entry(s.horizon_days).or_insert(0.0) += value;
        }
        Evidence {
            by_year,
            by_horizon,
            final_polls: round2_final_polls(&final_round2, error_sd, df),
        }
    };
    let round2_reference = round2_evidence(current.round2_error_sd, &round2_reference_replays);
    let mut round2_grid: Vec<Round2GridPoint> = round2_settings
        .iter()
        .zip(&round2_scores)
        .map(|(setting, scores)| {
            let evidence = round2_evidence(setting.round2_error_sd, scores);
            Round2GridPoint {
                error_sd: setting.round2_error_sd,
                total: evidence.total(None),
                beats_current: evidence.at_least(&round2_reference, None),
                p_winner_given_matchup: score::summarise(scores, floor).p_winner_given_matchup,
                evidence,
                chosen: false,
            }
        })
        .collect();
    let chosen2 = choose(&round2_grid, |p| &p.evidence, &round2_reference, None);
    let round2_error_sd = chosen2.map_or(current.round2_error_sd, |p| p.error_sd);
    for point in &mut round2_grid {
        point.chosen = point.error_sd == round2_error_sd;
    }
    let fitted = Setting {
        bloc_sd,
        candidate_sd,
        round2_error_sd,
    };

    // The fitted setting was scored in the round 2 grid unless nothing beat the current one.
    let fitted_replays = match round2_settings.iter().position(|s| *s == fitted) {
        Some(i) => round2_scores.swap_remove(i),
        None => round2_reference_replays,
    };
    let current_scores = setting_scores(
        current_replays,
        floor,
        Some(round1_reference.final_polls),
        Some(round2_final_polls(&final_round2, current.round2_error_sd, df)),
    );
    let fitted_scores = setting_scores(
        fitted_replays,
        floor,
        Some(round1_final_polls(&final_round1, blocs.len(), &model, fitted, &config)),
        Some(round2_final_polls(&final_round2, fitted.round2_error_sd, df)),
    );
    let baseline_scores = setting_scores(
        replays
            .iter()
            .map(|r| {
                let election = elections.iter().find(|e| e.year == r.year).expect("loaded");
                let selection = replay::select_polls(election, r.as_of, config.poll_window_days);
                baseline::evaluate(election, &selection, r.horizon_days, r.as_of)
            })
            .collect(),
        floor,
        None,
        None,
    );

    // Leave one election out: the same rule without that election's replays, then that
    // election's replays scored at the setting it gives.
    let leave_one_out = years
        .iter()
        .map(|&year| {
            let r1 = choose(&round1_grid, |p| &p.evidence, &round1_reference, Some(year));
            let r2 = choose(&round2_grid, |p| &p.evidence, &round2_reference, Some(year));
            LeaveOneOut {
                held_out: year,
                bloc_sd: r1.map_or(current.bloc_sd, |p| p.bloc_sd),
                candidate_sd: r1.map_or(current.candidate_sd, |p| p.candidate_sd),
                round2_error_sd: r2.map_or(current.round2_error_sd, |p| p.error_sd),
                round1_held_out: r1.map_or(round1_reference.by_year[&year], |p| p.evidence.by_year[&year]),
                round1_held_out_current: round1_reference.by_year[&year],
                round2_held_out: r2.map_or(round2_reference.by_year[&year], |p| p.evidence.by_year[&year]),
                round2_held_out_current: round2_reference.by_year[&year],
            }
        })
        .collect();

    let election_info = elections
        .iter()
        .map(|e| ElectionInfo {
            year: e.year,
            round1: e.round1,
            round2: e.round2,
            polls_in_file: e.polls.len(),
            finalists: [e.id(e.finalists.0).to_string(), e.id(e.finalists.1).to_string()],
            winner: e.id(e.winner()).to_string(),
            winner_round2_share: if e.winner() == e.finalists.0 {
                e.round2_share
            } else {
                1.0 - e.round2_share
            },
        })
        .collect();
    let replay_info = replays
        .iter()
        .map(|r| {
            let election = elections.iter().find(|e| e.year == r.year).expect("loaded");
            ReplayInfo {
                year: r.year,
                horizon_days: r.horizon_days,
                as_of: r.as_of,
                polls_published: r.polls_published,
                polls_overlapping: r.polls_overlapping,
                polls_used: r.polls_used,
                scenarios_used: r.fit.round1.scenarios_used,
                round2_pairs_polled: r.fit.round2.pairs.len(),
                random_walk_sd: r.fit.round1.noise.random_walk_sd,
                design_effect: r.fit.round1.noise.design_effect,
                scenario_noise_share: r.fit.round1.noise.scenario_noise_share,
                contenders: r.contenders.iter().map(|c| election.id(*c).to_string()).collect(),
                not_yet_polled: (0..election.field.len())
                    .filter(|c| election.ran[*c] && r.fit.round1.first_polled[*c].is_none())
                    .map(|c| election.id(c).to_string())
                    .collect(),
            }
        })
        .collect();

    Ok(Report {
        schema_version: "1.0",
        model_version: model.model_version.clone(),
        config,
        elections: election_info,
        replays: replay_info,
        current,
        fitted,
        baseline: baseline_scores,
        current_scores,
        fitted_scores,
        round1_reference,
        round1_grid,
        round2_reference,
        round2_grid,
        leave_one_out,
        final_polls_round2: final_round2,
    })
}

/// A labelled way to read one source of evidence, for the report.
type EvidenceSource = (String, Box<dyn Fn(&Evidence) -> f64>);

fn pct(x: f64) -> String {
    if x.is_nan() {
        "–".into()
    } else {
        format!("{:.1}%", x * 100.0)
    }
}

fn num(x: f64, digits: usize) -> String {
    if x.is_nan() {
        "–".into()
    } else {
        format!("{x:.digits$}")
    }
}

fn summary_row(label: &str, s: &Summary) -> String {
    format!(
        "| {label} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
        num(s.qualify_brier, 3),
        num(s.qualify_log_loss, 3),
        num(s.win_brier, 3),
        num(s.win_log_loss, 3),
        num(s.share_mae_points, 2),
        pct(s.coverage80),
        num(s.share_log_score, 1),
        num(s.round2_log_score, 2),
        pct(s.p_winner_given_matchup),
    )
}

const SUMMARY_HEADER: &str = "| | Qualify Brier | Qualify log-loss | Win Brier | Win log-loss | Share MAE (pts) | 80% coverage | Share log score | Round 2 log score | P(winner \\| matchup) |\n|---|---|---|---|---|---|---|---|---|---|";

impl Report {
    /// The report as Markdown, for `research/backtest/report.md`.
    pub fn markdown(&self) -> String {
        let mut out = String::new();
        let c = &self.config;
        let _ = writeln!(out, "# Backtest: 2017 and 2022 replays\n");
        let _ = writeln!(
            out,
            "Written by `fr2027 backtest` (moon task `cli:backtest`) from `config/backtest.yaml`, `config/model.yaml` (model {}) and `data/backtest/`; `results.json` beside this file has every number. Seed {}, {} simulations per replay and setting. How to read it and what was decided: `research/LOG.md`.\n",
            self.model_version, c.seed, c.simulations
        );
        let _ = writeln!(
            out,
            "Each replay fits the model to the polls published by its date (fieldwork started at most {} days before round 1; overlapping polls of one firm thinned), conditions on the field that actually ran, and simulates both rounds. Probabilities are clipped to [{}, {}] for log-loss. Share log scores are summed over contenders (fitted share ≥ {:.0}%) and replays; higher is better. Brier, log-loss and MAE: lower is better.\n",
            c.poll_window_days,
            c.probability_floor,
            1.0 - c.probability_floor,
            c.contender_share * 100.0
        );

        let _ = writeln!(out, "## Settings\n");
        let _ = writeln!(
            out,
            "| | round1_error.bloc_sd | round1_error.candidate_sd | round2.error_sd |\n|---|---|---|---|"
        );
        for (label, s) in [("Current (config/model.yaml)", self.current), ("Fitted", self.fitted)] {
            let _ = writeln!(
                out,
                "| {label} | {} | {} | {} |",
                s.bloc_sd, s.candidate_sd, s.round2_error_sd
            );
        }

        let _ = writeln!(out, "\n## Scores\n\nAll replays (both elections, every horizon):\n");
        let _ = writeln!(out, "{SUMMARY_HEADER}");
        let _ = writeln!(out, "{}", summary_row("Latest-poll average", &self.baseline.overall));
        let _ = writeln!(out, "{}", summary_row("Model, current", &self.current_scores.overall));
        let _ = writeln!(out, "{}", summary_row("Model, fitted", &self.fitted_scores.overall));
        let _ = writeln!(
            out,
            "\nFinal-week poll errors 2002–2012 (log score, round 1 / round 2): current {} / {}, fitted {} / {}.",
            num(self.current_scores.final_polls_round1.unwrap_or(f64::NAN), 1),
            num(self.current_scores.final_polls_round2.unwrap_or(f64::NAN), 2),
            num(self.fitted_scores.final_polls_round1.unwrap_or(f64::NAN), 1),
            num(self.fitted_scores.final_polls_round2.unwrap_or(f64::NAN), 2),
        );
        for (title, pick) in [("By election", true), ("By horizon (days before round 1)", false)] {
            let _ = writeln!(out, "\n{title}:\n\n{SUMMARY_HEADER}");
            let rows = |scores: &SettingScores| -> Vec<(String, Summary)> {
                if pick {
                    scores.by_year.iter().map(|(k, v)| (k.to_string(), v.clone())).collect()
                } else {
                    scores
                        .by_horizon
                        .iter()
                        .rev()
                        .map(|(k, v)| (k.to_string(), v.clone()))
                        .collect()
                }
            };
            for (label, scores) in [
                ("baseline", &self.baseline),
                ("current", &self.current_scores),
                ("fitted", &self.fitted_scores),
            ] {
                for (key, summary) in rows(scores) {
                    let _ = writeln!(out, "{}", summary_row(&format!("{key}, {label}"), &summary));
                }
            }
        }

        let _ = writeln!(
            out,
            "\n## Reliability\n\nQualification and win events pooled over every replay.\n"
        );
        let _ = writeln!(
            out,
            "| Forecast | Events (current) | Mean forecast | Observed | Events (fitted) | Mean forecast | Observed |\n|---|---|---|---|---|---|---|"
        );
        for (a, b) in self
            .current_scores
            .reliability
            .iter()
            .zip(&self.fitted_scores.reliability)
        {
            let opt = |x: Option<f64>| x.map_or("–".to_string(), pct);
            let _ = writeln!(
                out,
                "| {:.0}–{:.0}% | {} | {} | {} | {} | {} | {} |",
                a.lo * 100.0,
                a.hi * 100.0,
                a.events,
                opt(a.mean_forecast),
                opt(a.observed),
                b.events,
                opt(b.mean_forecast),
                opt(b.observed)
            );
        }

        let _ = writeln!(
            out,
            "\n## Fit\n\nThe rule: among the grid points that score at least as well as the current setting on every source of evidence (each election's replays, and the final-week polls of 2002–2012), take the one with the highest total log score; if none does, keep the current value. Round 1 first, with round 2 at its current error; then round 2, with round 1 at its fitted values.\n"
        );
        let _ = writeln!(
            out,
            "| Round 1 evidence (share log score) | Current | Fitted | Best on this source alone |\n|---|---|---|---|"
        );
        let fitted_point = self.round1_grid.iter().find(|p| p.chosen);
        let mut sources: Vec<EvidenceSource> = Vec::new();
        for e in &self.elections {
            let year = e.year;
            sources.push((
                format!("{year} replays"),
                Box::new(move |v: &Evidence| v.by_year[&year]),
            ));
        }
        for h in &c.horizons_days {
            let h = *h;
            sources.push((
                format!("{h}-day replays, both elections (not used)"),
                Box::new(move |v: &Evidence| v.by_horizon[&h]),
            ));
        }
        sources.push((
            "Final-week polls 2002–2012".into(),
            Box::new(|v: &Evidence| v.final_polls),
        ));
        sources.push(("Total".into(), Box::new(|v: &Evidence| v.total(None))));
        for (label, key) in &sources {
            let best = argmax(&self.round1_grid, |p| key(&p.evidence));
            let _ = writeln!(
                out,
                "| {label} | {} | {} | {} (bloc_sd {}, candidate_sd {}) |",
                num(key(&self.round1_reference), 1),
                fitted_point.map_or("–".into(), |p| num(key(&p.evidence), 1)),
                num(key(&best.evidence), 1),
                best.bloc_sd,
                best.candidate_sd
            );
        }

        let _ = writeln!(
            out,
            "\nRound 1 grid: total log score by `bloc_sd` (rows) and `candidate_sd` (columns). `*` marks the points that score at least as well as the current setting on every source; bold is the fitted one.\n"
        );
        let _ = write!(out, "| bloc_sd \\ candidate_sd |");
        for sd in &c.grid.candidate_sd {
            let _ = write!(out, " {sd} |");
        }
        let _ = writeln!(out, "\n|---|{}", "---|".repeat(c.grid.candidate_sd.len()));
        for bloc in &c.grid.bloc_sd {
            let _ = write!(out, "| {bloc} |");
            for point in self.round1_grid.iter().filter(|p| p.bloc_sd == *bloc) {
                let value = format!("{}{}", num(point.total, 1), if point.beats_current { "*" } else { "" });
                let _ = write!(out, " {} |", if point.chosen { format!("**{value}**") } else { value });
            }
            let _ = writeln!(out);
        }

        let _ = writeln!(
            out,
            "\nRound 2 grid, with round 1 at its fitted values: log score of the realised runoff share (the logit of the first finalist's share), given the actual matchup. Current error_sd {}: replays {}, final-week polls {}.\n",
            self.current.round2_error_sd,
            self.round2_reference
                .by_year
                .iter()
                .map(|(year, score)| format!("{year} {}", num(*score, 2)))
                .collect::<Vec<_>>()
                .join(", "),
            num(self.round2_reference.final_polls, 2)
        );
        let _ = write!(out, "| error_sd |");
        for e in &self.elections {
            let _ = write!(out, " {} replays |", e.year);
        }
        let _ = writeln!(out, " Final-week polls 2002–2012 | Total | P(winner \\| matchup) |");
        let _ = writeln!(out, "|---|{}---|---|---|", "---|".repeat(self.elections.len()));
        for point in &self.round2_grid {
            let mark = if point.chosen {
                " (fitted)"
            } else if point.beats_current {
                " *"
            } else {
                ""
            };
            let _ = write!(out, "| {}{mark} |", point.error_sd);
            for score in point.evidence.by_year.values() {
                let _ = write!(out, " {} |", num(*score, 2));
            }
            let _ = writeln!(
                out,
                " {} | {} | {} |",
                num(point.evidence.final_polls, 2),
                num(point.total, 2),
                pct(point.p_winner_given_matchup)
            );
        }
        let _ = writeln!(
            out,
            "\nFinal-week runoff polls used:\n\n| Year | Winner | Final-week average | Result (share of the two) |\n|---|---|---|---|"
        );
        for e in &self.final_polls_round2 {
            let _ = writeln!(
                out,
                "| {} | {} | {} | {} |",
                e.year,
                e.winner,
                pct(e.poll_average),
                pct(e.result)
            );
        }

        let _ = writeln!(
            out,
            "\n## Leave one election out\n\nThe same rule without one election's replays, then that election's replays scored at the setting it gives, and at the current one.\n\n| Held out | bloc_sd | candidate_sd | round2 error_sd | Round 1 log score (chosen without / current) | Round 2 log score (chosen without / current) |\n|---|---|---|---|---|---|"
        );
        for l in &self.leave_one_out {
            let _ = writeln!(
                out,
                "| {} | {} | {} | {} | {} / {} | {} / {} |",
                l.held_out,
                l.bloc_sd,
                l.candidate_sd,
                l.round2_error_sd,
                num(l.round1_held_out, 1),
                num(l.round1_held_out_current, 1),
                num(l.round2_held_out, 2),
                num(l.round2_held_out_current, 2)
            );
        }

        let _ = writeln!(
            out,
            "\n## Replays\n\n| Election | Days out | As of | Polls used | Overlapping dropped | Random walk sd | Design effect | Contenders | Runners not yet polled |\n|---|---|---|---|---|---|---|---|---|"
        );
        for r in &self.replays {
            let _ = writeln!(
                out,
                "| {} | {} | {} | {} | {} | {} | {} | {} | {} |",
                r.year,
                r.horizon_days,
                r.as_of,
                r.polls_used,
                r.polls_overlapping,
                r.random_walk_sd,
                r.design_effect,
                r.contenders.join(", "),
                if r.not_yet_polled.is_empty() {
                    "–".to_string()
                } else {
                    r.not_yet_polled.join(", ")
                }
            );
        }
        for (current, fitted) in self.current_scores.replays.iter().zip(&self.fitted_scores.replays) {
            let r2 = |s: &ReplayScore| {
                format!(
                    "{} (winner given matchup {})",
                    pct(s.round2.p_matchup),
                    s.round2.p_winner_given_matchup.map_or("–".into(), pct)
                )
            };
            let _ = writeln!(
                out,
                "\n### {}, {} days out ({})\n\nActual matchup: current {}, fitted {}.\n",
                current.year,
                current.horizon_days,
                current.as_of,
                r2(current),
                r2(fitted)
            );
            let _ = writeln!(
                out,
                "| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |\n|---|---|---|---|---|"
            );
            for (a, b) in current.candidates.iter().zip(&fitted.candidates) {
                let share = match (b.share_mean, b.share_lo80, b.share_hi80) {
                    (Some(m), Some(lo), Some(hi)) => format!("{} ({}–{})", pct(m), pct(lo), pct(hi)),
                    _ => "not polled".into(),
                };
                let mark = |happened: bool| if happened { " ✓" } else { "" };
                let _ = writeln!(
                    out,
                    "| {} | {} | {} | {} / {}{} | {} / {}{} |",
                    a.candidate_id,
                    pct(a.result_share),
                    share,
                    pct(a.p_qualify),
                    pct(b.p_qualify),
                    mark(a.qualified),
                    pct(a.p_win),
                    pct(b.p_win),
                    mark(a.won)
                );
            }
        }
        out
    }
}
