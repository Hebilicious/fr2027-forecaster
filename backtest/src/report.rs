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
    /// The steps of the fit, in order, over every pass; each starts from the setting the
    /// previous one chose.
    pub stages: Vec<Stage>,
    /// Passes of the three stages run until one changed nothing (or `max_passes`).
    pub passes: usize,
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

/// The error settings the backtest fits: `round1_error` and the round 2 errors of
/// config/model.yaml.
#[derive(Clone, Copy, Debug, Serialize, PartialEq)]
pub struct Setting {
    pub bloc_sd: f64,
    pub candidate_sd: f64,
    pub horizon_bloc_sd: f64,
    pub horizon_candidate_sd: f64,
    pub student_t_df: f64,
    pub round2_error_sd: f64,
    pub round2_horizon_error_sd: f64,
}

/// A parameter of [`Setting`] that a stage of the fit varies.
#[derive(Clone, Copy, Debug, Serialize, PartialEq)]
pub enum Param {
    BlocSd,
    CandidateSd,
    HorizonBlocSd,
    HorizonCandidateSd,
    StudentTDf,
    Round2ErrorSd,
    Round2HorizonErrorSd,
}

impl Param {
    /// The parameter's name in config/model.yaml.
    pub fn name(self) -> &'static str {
        match self {
            Param::BlocSd => "round1_error.bloc_sd",
            Param::CandidateSd => "round1_error.candidate_sd",
            Param::HorizonBlocSd => "round1_error.horizon_bloc_sd",
            Param::HorizonCandidateSd => "round1_error.horizon_candidate_sd",
            Param::StudentTDf => "round1_error.student_t_df",
            Param::Round2ErrorSd => "round2.error_sd",
            Param::Round2HorizonErrorSd => "round2.horizon_error_sd",
        }
    }

    pub fn get(self, s: &Setting) -> f64 {
        match self {
            Param::BlocSd => s.bloc_sd,
            Param::CandidateSd => s.candidate_sd,
            Param::HorizonBlocSd => s.horizon_bloc_sd,
            Param::HorizonCandidateSd => s.horizon_candidate_sd,
            Param::StudentTDf => s.student_t_df,
            Param::Round2ErrorSd => s.round2_error_sd,
            Param::Round2HorizonErrorSd => s.round2_horizon_error_sd,
        }
    }

    fn set(self, s: &mut Setting, value: f64) {
        let field = match self {
            Param::BlocSd => &mut s.bloc_sd,
            Param::CandidateSd => &mut s.candidate_sd,
            Param::HorizonBlocSd => &mut s.horizon_bloc_sd,
            Param::HorizonCandidateSd => &mut s.horizon_candidate_sd,
            Param::StudentTDf => &mut s.student_t_df,
            Param::Round2ErrorSd => &mut s.round2_error_sd,
            Param::Round2HorizonErrorSd => &mut s.round2_horizon_error_sd,
        };
        *field = value;
    }

    pub const ALL: [Param; 7] = [
        Param::BlocSd,
        Param::CandidateSd,
        Param::HorizonBlocSd,
        Param::HorizonCandidateSd,
        Param::StudentTDf,
        Param::Round2ErrorSd,
        Param::Round2HorizonErrorSd,
    ];
}

#[derive(Clone, Debug, Serialize)]
pub struct SettingScores {
    pub overall: Summary,
    pub by_year: BTreeMap<i16, Summary>,
    pub by_horizon: BTreeMap<u32, Summary>,
    /// Log scores of the final-week poll errors of `final_polls.years` (none for the baseline).
    pub final_polls_round1: Option<f64>,
    pub final_polls_round2: Option<f64>,
    pub reliability: Vec<ReliabilityBin>,
    pub replays: Vec<ReplayScore>,
}

/// The log scores a setting earns from each independent body of evidence: each election's
/// replays, and the final-week polls of the elections in `final_polls.years`.
#[derive(Clone, Debug, Serialize)]
pub struct Evidence {
    /// Summed over each election's replays.
    pub by_year: BTreeMap<i16, f64>,
    /// Summed over every election, by horizon (shown, not used by the fit).
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

/// The fit's rule: among the grid points that score at least as well as the stage's starting
/// setting on every source of evidence, the one with the highest total. None when no point
/// does, which keeps the starting setting. A value that one election favours and another
/// disfavours is not adopted.
fn choose<'a>(grid: &'a [GridPoint], reference: &Evidence, without: Option<i16>) -> Option<&'a GridPoint> {
    grid.iter()
        .filter(|p| p.evidence.at_least(reference, without))
        .max_by(|a, b| a.evidence.total(without).total_cmp(&b.evidence.total(without)))
}

/// One step of the fit: every combination of a few parameters' grids, the others held at the
/// step's starting setting, scored on one round's evidence.
#[derive(Clone, Debug, Serialize)]
pub struct Stage {
    pub title: &'static str,
    pub pass: usize,
    /// 1 for round 1 shares, 2 for the realised runoff share.
    pub round: u8,
    pub params: Vec<Param>,
    /// The setting the step starts from, and its evidence: a point must match it on every source.
    pub start: Setting,
    pub reference: Evidence,
    pub grid: Vec<GridPoint>,
    /// The chosen point's setting, or the start if no point qualifies.
    pub chosen: Setting,
}

#[derive(Clone, Debug, Serialize)]
pub struct GridPoint {
    pub setting: Setting,
    pub evidence: Evidence,
    pub total: f64,
    /// Scores at least as well as the stage's start on every source.
    pub beats_start: bool,
    pub chosen: bool,
    pub qualify_log_loss: f64,
    pub p_winner_given_matchup: f64,
}

/// The whole fit rerun without one election's replays, then that election's replays scored at
/// the setting it gives and at the current one.
#[derive(Clone, Debug, Serialize)]
pub struct LeaveOneOut {
    pub held_out: i16,
    pub chosen: Setting,
    pub passes: usize,
    /// The held-out election's share log score at the setting chosen without it, and at the
    /// current setting.
    pub round1_held_out: f64,
    pub round1_held_out_current: f64,
    /// The same for its round 2 log score.
    pub round2_held_out: f64,
    pub round2_held_out_current: f64,
}

fn with_errors(params: &ModelParams, setting: Setting) -> ModelParams {
    let mut params = params.clone();
    params.round1_error = error_params(setting);
    params.round2.error_sd = setting.round2_error_sd;
    params.round2.horizon_error_sd = setting.round2_horizon_error_sd;
    params
}

fn error_params(setting: Setting) -> ErrorParams {
    ErrorParams {
        student_t_df: setting.student_t_df,
        bloc_sd: setting.bloc_sd,
        candidate_sd: setting.candidate_sd,
        horizon_bloc_sd: setting.horizon_bloc_sd,
        horizon_candidate_sd: setting.horizon_candidate_sd,
    }
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

fn argmax<T>(items: &[T], key: impl Fn(&T) -> f64) -> &T {
    items
        .iter()
        .max_by(|a, b| key(a).total_cmp(&key(b)))
        .expect("non-empty grid")
}

/// The three stages of a pass: round 1's horizon error and tails, then its election-day error,
/// then round 2's two errors.
const STAGES: [(&str, u8, [Option<Param>; 3]); 3] = [
    (
        "Round 1 horizon error and tails",
        1,
        [
            Some(Param::HorizonBlocSd),
            Some(Param::HorizonCandidateSd),
            Some(Param::StudentTDf),
        ],
    ),
    (
        "Round 1 election-day error",
        1,
        [Some(Param::BlocSd), Some(Param::CandidateSd), None],
    ),
    (
        "Round 2 errors",
        2,
        [Some(Param::Round2ErrorSd), Some(Param::Round2HorizonErrorSd), None],
    ),
];

/// Scores settings against every source of evidence, remembering each setting's replay scores
/// so that later passes and the leave-one-out fits reuse them.
struct Fitter<'a> {
    elections: &'a [Election],
    replays: &'a [Replay],
    years: Vec<i16>,
    final_round1: Vec<Round1Election>,
    final_round2: Vec<Round2Election>,
    blocs: usize,
    /// Round 2's Student-t degrees of freedom, which the fit does not vary.
    round2_df: f64,
    config: &'a BacktestConfig,
    scored: BTreeMap<[u64; 7], Vec<ReplayScore>>,
}

fn key(setting: &Setting) -> [u64; 7] {
    Param::ALL.map(|p| p.get(setting).to_bits())
}

impl Fitter<'_> {
    /// The final-week polls are taken as election-eve polls: they carry no horizon error.
    fn final_polls(&self, round: u8, setting: Setting) -> f64 {
        let c = self.config;
        if round == 1 {
            let error = error_params(setting);
            self.final_round1
                .iter()
                .map(|e| final_polls::round1_log_score(e, self.blocs, &error, c.simulations, c.seed, c.contender_share))
                .sum()
        } else {
            self.final_round2
                .iter()
                .map(|e| final_polls::round2_log_score(e, setting.round2_error_sd, self.round2_df))
                .sum()
        }
    }

    /// Scores the settings not scored yet, in parallel.
    fn score(&mut self, settings: &[Setting]) -> Result<()> {
        let mut missing: Vec<Setting> = Vec::new();
        for setting in settings {
            if !self.scored.contains_key(&key(setting)) && !missing.iter().any(|m| key(m) == key(setting)) {
                missing.push(*setting);
            }
        }
        if !missing.is_empty() {
            let scores = evaluate_all(self.elections, self.replays, &missing)?;
            for (setting, scores) in missing.iter().zip(scores) {
                self.scored.insert(key(setting), scores);
            }
        }
        Ok(())
    }

    fn replay_scores(&self, setting: &Setting) -> &[ReplayScore] {
        &self.scored[&key(setting)]
    }

    fn evidence(&self, round: u8, setting: Setting) -> Evidence {
        let mut by_year: BTreeMap<i16, f64> = self.years.iter().map(|y| (*y, 0.0)).collect();
        let mut by_horizon = BTreeMap::new();
        for s in self.replay_scores(&setting) {
            let value = if round == 1 {
                s.share_log_score()
            } else {
                s.round2.log_score.unwrap_or(0.0)
            };
            *by_year.entry(s.year).or_insert(0.0) += value;
            *by_horizon.entry(s.horizon_days).or_insert(0.0) += value;
        }
        Evidence {
            by_year,
            by_horizon,
            final_polls: self.final_polls(round, setting),
        }
    }

    /// Every combination of the stage's parameter grids around `start`, scored, and the rule
    /// applied without the replays of `without`.
    fn stage(
        &mut self,
        (title, round, params): (&'static str, u8, [Option<Param>; 3]),
        pass: usize,
        start: Setting,
        without: Option<i16>,
    ) -> Result<Stage> {
        let params: Vec<Param> = params.into_iter().flatten().collect();
        let mut settings = vec![start];
        for param in &params {
            let values = self.config.grid.values(*param);
            settings = settings
                .iter()
                .flat_map(|s| {
                    values.iter().map(move |v| {
                        let mut s = *s;
                        param.set(&mut s, *v);
                        s
                    })
                })
                .collect();
        }
        let mut all = settings.clone();
        all.push(start);
        self.score(&all)?;
        let reference = self.evidence(round, start);
        let floor = self.config.probability_floor;
        let mut grid: Vec<GridPoint> = settings
            .iter()
            .map(|setting| {
                let evidence = self.evidence(round, *setting);
                let summary = score::summarise(self.replay_scores(setting), floor);
                GridPoint {
                    setting: *setting,
                    total: evidence.total(without),
                    beats_start: evidence.at_least(&reference, without),
                    chosen: false,
                    qualify_log_loss: summary.qualify_log_loss,
                    p_winner_given_matchup: summary.p_winner_given_matchup,
                    evidence,
                }
            })
            .collect();
        let chosen = match choose(&grid, &reference, without).map(|p| p.setting) {
            Some(setting) => {
                for point in &mut grid {
                    point.chosen = point.setting == setting;
                }
                setting
            }
            None => start,
        };
        Ok(Stage {
            title,
            pass,
            round,
            params,
            start,
            reference,
            grid,
            chosen,
        })
    }

    /// Passes of the three stages from `start` until a pass changes nothing, or `max_passes`.
    fn fit(
        &mut self,
        start: Setting,
        without: Option<i16>,
        progress: &mut dyn FnMut(&str),
    ) -> Result<(Vec<Stage>, usize)> {
        let mut stages = Vec::new();
        let mut setting = start;
        for pass in 1..=self.config.max_passes {
            let before = setting;
            for definition in STAGES {
                let stage = self.stage(definition, pass, setting, without)?;
                setting = stage.chosen;
                stages.push(stage);
            }
            progress(&format!(
                "fit{}: pass {pass} done, {} settings scored so far",
                without.map_or(String::new(), |y| format!(" without {y}")),
                self.scored.len()
            ));
            if setting == before {
                return Ok((stages, pass));
            }
        }
        Ok((stages, self.config.max_passes))
    }
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

    let mut elections = Vec::new();
    for entry in &config.elections {
        elections.push(election::load(repo, &schemas, entry, &config.results_file)?);
    }
    let blocs = elections[0].field.blocs.clone();

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
        horizon_bloc_sd: model.round1_error.horizon_bloc_sd,
        horizon_candidate_sd: model.round1_error.horizon_candidate_sd,
        student_t_df: model.round1_error.student_t_df,
        round2_error_sd: model.round2.error_sd,
        round2_horizon_error_sd: model.round2.horizon_error_sd,
    };
    let mut fitter = Fitter {
        elections: &elections,
        replays: &replays,
        years: elections.iter().map(|e| e.year).collect(),
        final_round1: final_polls::load_round1(repo, &config, &blocs)?,
        final_round2: final_polls::load_round2(repo, &config)?,
        blocs: blocs.len(),
        round2_df: model.round2.student_t_df,
        config: &config,
        scored: BTreeMap::new(),
    };

    // Coordinate ascent: each stage varies a few parameters with the others held where the
    // previous stage left them, and every stage keeps or improves every source of evidence, so
    // the fitted setting scores at least as well as the current one on all of them.
    let (stages, passes) = fitter.fit(current, None, progress)?;
    let fitted = stages.last().map_or(current, |s| s.chosen);

    let current_scores = setting_scores(
        fitter.replay_scores(&current).to_vec(),
        floor,
        Some(fitter.final_polls(1, current)),
        Some(fitter.final_polls(2, current)),
    );
    let fitted_scores = setting_scores(
        fitter.replay_scores(&fitted).to_vec(),
        floor,
        Some(fitter.final_polls(1, fitted)),
        Some(fitter.final_polls(2, fitted)),
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

    // Leave one election out: the whole fit again without that election's replays, then that
    // election's replays scored at the setting it gives.
    let mut leave_one_out = Vec::new();
    for year in fitter.years.clone() {
        let (held_stages, held_passes) = fitter.fit(current, Some(year), progress)?;
        let chosen = held_stages.last().map_or(current, |s| s.chosen);
        let score = |round: u8, setting: Setting| fitter.evidence(round, setting).by_year[&year];
        leave_one_out.push(LeaveOneOut {
            held_out: year,
            chosen,
            passes: held_passes,
            round1_held_out: score(1, chosen),
            round1_held_out_current: score(1, current),
            round2_held_out: score(2, chosen),
            round2_held_out_current: score(2, current),
        });
    }

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
    let final_polls_round2 = fitter.final_round2.clone();

    Ok(Report {
        schema_version: "2.0",
        model_version: model.model_version.clone(),
        config,
        elections: election_info,
        replays: replay_info,
        current,
        fitted,
        baseline: baseline_scores,
        current_scores,
        fitted_scores,
        stages,
        passes,
        leave_one_out,
        final_polls_round2,
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
        let replayed = year_list(&self.elections.iter().map(|e| e.year).collect::<Vec<_>>());
        let final_years = year_list(&c.final_polls.years);
        let _ = writeln!(out, "# Backtest: {replayed} replays\n");
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
            "Horizon errors are per square root of 30 days before the round; they add in variance to the election-day error.\n"
        );
        let _ = writeln!(out, "| | Current (config/model.yaml) | Fitted |\n|---|---|---|");
        for param in Param::ALL {
            let _ = writeln!(
                out,
                "| {} | {} | {} |",
                param.name(),
                param.get(&self.current),
                param.get(&self.fitted)
            );
        }

        let _ = writeln!(out, "## Scores\n\nAll replays (every election, every horizon):\n");
        let _ = writeln!(out, "{SUMMARY_HEADER}");
        let _ = writeln!(out, "{}", summary_row("Latest-poll average", &self.baseline.overall));
        let _ = writeln!(out, "{}", summary_row("Model, current", &self.current_scores.overall));
        let _ = writeln!(out, "{}", summary_row("Model, fitted", &self.fitted_scores.overall));
        if !c.final_polls.years.is_empty() {
            let _ = writeln!(
                out,
                "\nFinal-week poll errors {final_years} (log score, round 1 / round 2): current {} / {}, fitted {} / {}.",
                num(self.current_scores.final_polls_round1.unwrap_or(f64::NAN), 1),
                num(self.current_scores.final_polls_round2.unwrap_or(f64::NAN), 2),
                num(self.fitted_scores.final_polls_round1.unwrap_or(f64::NAN), 1),
                num(self.fitted_scores.final_polls_round2.unwrap_or(f64::NAN), 2),
            );
        }
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
            "\n## Fit\n\nThe rule, in each stage: among the grid points that score at least as well as the stage's starting setting on every source of evidence (each election's replays, and the final-week polls of other elections: {final_years}), take the one with the highest total log score; if none does, keep the starting setting. The first stage starts from config/model.yaml and each later one from the setting the previous one chose; the three stages repeat until a pass changes nothing ({} passes here, at most {}). Every stage keeps or improves every source, so the fitted setting scores at least as well as the current one on all of them. Round 1 stages score contenders' shares; the round 2 stage scores the realised runoff share given the actual matchup. Final-week polls are taken as election-eve polls, with no horizon error.\n",
            self.passes, c.max_passes
        );
        for stage in &self.stages {
            self.stage_markdown(&mut out, stage, &final_years);
        }
        if !self.final_polls_round2.is_empty() {
            let _ = writeln!(
                out,
                "\nFinal-week runoff polls used:\n\n| Year | Winner | Final-week average | Result (share of the two) |\n|---|---|---|---|"
            );
        }
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
            "\n## Leave one election out\n\nThe whole fit rerun without one election's replays, then that election's replays scored at the setting it gives, and at the current one.\n\n| Held out | Passes | {} | Round 1 log score (chosen without / current) | Round 2 log score (chosen without / current) |\n|---|---|{}---|---|",
            Param::ALL.map(short_name).join(" | "),
            "---|".repeat(Param::ALL.len())
        );
        for l in &self.leave_one_out {
            let _ = writeln!(
                out,
                "| {} | {} | {} | {} / {} | {} / {} |",
                l.held_out,
                l.passes,
                Param::ALL.map(|p| p.get(&l.chosen).to_string()).join(" | "),
                num(l.round1_held_out, 2),
                num(l.round1_held_out_current, 2),
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

impl Report {
    fn stage_markdown(&self, out: &mut String, stage: &Stage, final_years: &str) {
        let c = &self.config;
        let what = if stage.round == 1 {
            "share log score of the contenders"
        } else {
            "log score of the realised runoff share"
        };
        let start = stage
            .params
            .iter()
            .map(|p| format!("{} {}", short_name(*p), p.get(&stage.start)))
            .collect::<Vec<_>>()
            .join(", ");
        let _ = writeln!(
            out,
            "\n### Pass {}: {}\n\nEvidence: {what}. Starts from {start}.\n",
            stage.pass, stage.title
        );
        let _ = writeln!(
            out,
            "| Evidence | Start | Chosen | Best on this source alone |\n|---|---|---|---|"
        );
        let chosen = stage.grid.iter().find(|p| p.chosen);
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
                format!("{h}-day replays, every election (not used)"),
                Box::new(move |v: &Evidence| v.by_horizon[&h]),
            ));
        }
        if !c.final_polls.years.is_empty() {
            sources.push((
                format!("Final-week polls ({final_years})"),
                Box::new(|v: &Evidence| v.final_polls),
            ));
        }
        sources.push(("Total".into(), Box::new(|v: &Evidence| v.total(None))));
        for (label, key) in &sources {
            let best = argmax(&stage.grid, |p| key(&p.evidence));
            let best_at = stage
                .params
                .iter()
                .map(|p| format!("{} {}", short_name(*p), p.get(&best.setting)))
                .collect::<Vec<_>>()
                .join(", ");
            let _ = writeln!(
                out,
                "| {label} | {} | {} | {} ({best_at}) |",
                num(key(&stage.reference), 2),
                chosen.map_or("–".into(), |p| num(key(&p.evidence), 2)),
                num(key(&best.evidence), 2),
            );
        }

        // The grid as tables of total log score: the first parameter in rows, the second in
        // columns, one table per value of the third.
        let (rows, cols) = (stage.params[0], stage.params[1]);
        let values = |param: Param| -> Vec<f64> {
            let mut values: Vec<f64> = Vec::new();
            for point in &stage.grid {
                let v = param.get(&point.setting);
                if !values.contains(&v) {
                    values.push(v);
                }
            }
            values
        };
        let tables: Vec<Option<f64>> = match stage.params.get(2) {
            Some(third) => values(*third).into_iter().map(Some).collect(),
            None => vec![None],
        };
        let _ = writeln!(
            out,
            "\nTotal log score by `{}` (rows) and `{}` (columns). `*` marks the points that score at least as well as the start on every source; bold is the chosen one.",
            short_name(rows),
            short_name(cols)
        );
        for table in tables {
            if let (Some(value), Some(third)) = (table, stage.params.get(2)) {
                let _ = writeln!(out, "\n{} = {value}:", short_name(*third));
            }
            let _ = write!(out, "\n| {} \\ {} |", short_name(rows), short_name(cols));
            let col_values = values(cols);
            for v in &col_values {
                let _ = write!(out, " {v} |");
            }
            let _ = writeln!(out, "\n|---|{}", "---|".repeat(col_values.len()));
            for row in values(rows) {
                let _ = write!(out, "| {row} |");
                for point in stage.grid.iter().filter(|p| {
                    rows.get(&p.setting) == row && table.is_none_or(|t| stage.params[2].get(&p.setting) == t)
                }) {
                    let value = format!("{}{}", num(point.total, 1), if point.beats_start { "*" } else { "" });
                    let _ = write!(out, " {} |", if point.chosen { format!("**{value}**") } else { value });
                }
                let _ = writeln!(out);
            }
        }
    }
}

/// A parameter's name without its section, for tables.
fn short_name(param: Param) -> &'static str {
    param.name().rsplit('.').next().expect("a name")
}

/// "2002, 2007 and 2012".
fn year_list(years: &[i16]) -> String {
    let names: Vec<String> = years.iter().map(i16::to_string).collect();
    match names.split_last() {
        None => "none".into(),
        Some((last, [])) => last.clone(),
        Some((last, rest)) => format!("{} and {last}", rest.join(", ")),
    }
}
