//! The signal layers' ablation: every replay fitted again with event detection on, under each
//! variant in `config/backtest.yaml` (`signals_ablation`), simulated at the election-day errors in
//! `config/model.yaml`, and scored against the same replays with the polls only.
//!
//! A variant may be adopted only if it scores at least as well as the polls-only model on every
//! election, on each of: the contenders' round 1 share log score, the runoff log score, and the
//! log-loss of who qualifies and who wins; and if it changes something (the polls chose a
//! `shock_sd` above zero, or the momentum tilted a vote, in at least one replay). That is the backtest's rule for the error
//! settings (each body of evidence must agree), with the spec's log-loss added.

use std::collections::BTreeMap;

use fr2027_model::{ModelParams, SignalParams, SignalSource};
use serde::{Deserialize, Serialize};

use crate::{
    Result,
    config::BacktestConfig,
    election::Election,
    replay::{self, Replay},
    score::{self, ReplayScore, Summary},
};

/// `signals_ablation` in `config/backtest.yaml`.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AblationConfig {
    /// The `shock_sd` values each variant's marginal likelihood chooses from.
    pub shock_sd_grid: Vec<f64>,
    pub variants: Vec<Variant>,
}

/// One way of detecting spikes: the model's `signals` settings with these fields replaced.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Variant {
    pub name: String,
    pub sources: Vec<SignalSource>,
    pub spike_ratio: Option<f64>,
    pub half_life_days: Option<f64>,
    pub attention_floor: Option<f64>,
    pub news_floor: Option<f64>,
    /// The momentum weight (default: none).
    pub momentum_weight: Option<f64>,
    /// Whether spikes may add variance (default: yes); off to score momentum alone.
    pub widening: Option<bool>,
}

impl Variant {
    fn params(&self, base: &SignalParams, shock_sd_grid: &[f64]) -> SignalParams {
        SignalParams {
            sources: self.sources.clone(),
            spike_ratio: self.spike_ratio.unwrap_or(base.spike_ratio),
            half_life_days: self.half_life_days.unwrap_or(base.half_life_days),
            attention_floor: self.attention_floor.unwrap_or(base.attention_floor),
            news_floor: self.news_floor.unwrap_or(base.news_floor),
            shock_sd_grid: if self.widening.unwrap_or(true) {
                shock_sd_grid.to_vec()
            } else {
                vec![0.0]
            },
            momentum_weight: self.momentum_weight.unwrap_or(0.0),
            ..base.clone()
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Ablation {
    pub shock_sd_grid: Vec<f64>,
    pub polls_only: Scores,
    pub variants: Vec<VariantResult>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Scores {
    pub overall: Summary,
    pub by_year: BTreeMap<i16, Summary>,
    pub by_horizon: BTreeMap<u32, Summary>,
}

#[derive(Clone, Debug, Serialize)]
pub struct VariantResult {
    pub variant: Variant,
    pub signals: SignalParams,
    pub scores: Scores,
    /// Per replay: spikes detected by its date, and the `shock_sd` its polls chose.
    pub replays: Vec<VariantReplay>,
    /// The polls chose a `shock_sd` above zero, or the momentum tilted a vote, in at least one
    /// replay.
    pub has_effect: bool,
    /// Has an effect, and is at least as good as the polls-only model on every election and
    /// every score.
    pub adoptable: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct VariantReplay {
    pub year: i16,
    pub horizon_days: u32,
    pub spikes: usize,
    pub shock_sd: f64,
}

fn scores(replays: &[ReplayScore], floor: f64) -> Scores {
    let mut by_year: BTreeMap<i16, Vec<&ReplayScore>> = BTreeMap::new();
    let mut by_horizon: BTreeMap<u32, Vec<&ReplayScore>> = BTreeMap::new();
    for s in replays {
        by_year.entry(s.year).or_default().push(s);
        by_horizon.entry(s.horizon_days).or_default().push(s);
    }
    Scores {
        overall: score::summarise(replays, floor),
        by_year: by_year
            .into_iter()
            .map(|(k, v)| (k, score::summarise(v, floor)))
            .collect(),
        by_horizon: by_horizon
            .into_iter()
            .map(|(k, v)| (k, score::summarise(v, floor)))
            .collect(),
    }
}

/// Whether `candidate` is at least as good as `reference` on every election.
fn at_least(candidate: &Scores, reference: &Scores) -> bool {
    reference.by_year.iter().all(|(year, r)| {
        let c = &candidate.by_year[year];
        c.share_log_score >= r.share_log_score
            && c.round2_log_score >= r.round2_log_score
            && c.qualify_log_loss <= r.qualify_log_loss
            && c.win_log_loss <= r.win_log_loss
    })
}

/// Scores each replay at the model's own settings, one thread per replay. `contenders` (from the
/// polls-only replays, by year and horizon) fixes who is scored, so every variant is scored on
/// the same candidates.
fn evaluate(
    elections: &[Election],
    replays: &mut [Replay],
    contenders: &BTreeMap<(i16, u32), Vec<usize>>,
) -> Result<Vec<ReplayScore>> {
    for replay in replays.iter_mut() {
        if let Some(c) = contenders.get(&(replay.year, replay.horizon_days)) {
            replay.contenders = c.clone();
        }
    }
    std::thread::scope(|scope| {
        let handles: Vec<_> = replays
            .iter()
            .map(|replay| {
                let election = elections
                    .iter()
                    .find(|e| e.year == replay.year)
                    .expect("replay of a loaded election");
                scope.spawn(move || score::evaluate(election, replay, &replay.params))
            })
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().expect("replay thread panicked"))
            .collect()
    })
}

/// Fits every replay of every election under `model`, one thread per replay.
fn prepare_all(elections: &[Election], model: &ModelParams, config: &BacktestConfig) -> Result<Vec<Replay>> {
    std::thread::scope(|scope| {
        let handles: Vec<_> = elections
            .iter()
            .flat_map(|election| {
                config
                    .horizons_days
                    .iter()
                    .map(move |&horizon| scope.spawn(move || replay::prepare(election, model, config, horizon)))
            })
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().expect("replay thread panicked"))
            .collect()
    })
}

/// Runs the ablation. `polls_only` are the replays the rest of the backtest fitted, with the
/// signals layer off.
pub fn run(
    elections: &[Election],
    polls_only: &[Replay],
    published: &[Replay],
    model: &ModelParams,
    config: &BacktestConfig,
    ablation: &AblationConfig,
    progress: &mut dyn FnMut(&str),
) -> Result<Ablation> {
    let floor = config.probability_floor;
    let contenders: BTreeMap<(i16, u32), Vec<usize>> = polls_only
        .iter()
        .map(|r| ((r.year, r.horizon_days), r.contenders.clone()))
        .collect();
    let mut reference = polls_only.to_vec();
    let reference_scores = scores(&evaluate(elections, &mut reference, &contenders)?, floor);
    // The published settings first: the replays the rest of the backtest scored, with
    // config/model.yaml's signal layers.
    let mut published_replays = published.to_vec();
    let published_scores = scores(&evaluate(elections, &mut published_replays, &contenders)?, floor);
    progress(&format!(
        "signals ablation, published settings: share log score {:.1} (polls only {:.1})",
        published_scores.overall.share_log_score, reference_scores.overall.share_log_score
    ));
    let mut variants = vec![VariantResult {
        variant: Variant {
            name: "published settings (config/model.yaml)".into(),
            sources: model.signals.sources.clone(),
            spike_ratio: None,
            half_life_days: None,
            attention_floor: None,
            news_floor: None,
            momentum_weight: None,
            widening: None,
        },
        signals: published
            .first()
            .map_or_else(|| model.signals.clone(), |r| r.params.signals.clone()),
        replays: published_replays
            .iter()
            .map(|r| VariantReplay {
                year: r.year,
                horizon_days: r.horizon_days,
                spikes: r.fit.round1.spikes.len(),
                shock_sd: r.fit.round1.noise.shock_sd,
            })
            .collect(),
        has_effect: true,
        adoptable: at_least(&published_scores, &reference_scores),
        scores: published_scores,
    }];
    for variant in &ablation.variants {
        let mut params = model.clone();
        params.signals = variant.params(&model.signals, &ablation.shock_sd_grid);
        let mut replays = prepare_all(elections, &params, config)?;
        let replay_scores = scores(&evaluate(elections, &mut replays, &contenders)?, floor);
        let has_effect = replays
            .iter()
            .any(|r| r.fit.round1.noise.shock_sd > 0.0 || r.fit.momentum.iter().any(|m| *m != 0.0));
        let adoptable = has_effect && at_least(&replay_scores, &reference_scores);
        progress(&format!(
            "signals ablation, {}: share log score {:.1} (polls only {:.1}){}",
            variant.name,
            replay_scores.overall.share_log_score,
            reference_scores.overall.share_log_score,
            if adoptable {
                ", no worse"
            } else if has_effect {
                ""
            } else {
                ", no effect"
            }
        ));
        variants.push(VariantResult {
            variant: variant.clone(),
            signals: params.signals,
            scores: replay_scores,
            replays: replays
                .iter()
                .map(|r| VariantReplay {
                    year: r.year,
                    horizon_days: r.horizon_days,
                    spikes: r.fit.round1.spikes.len(),
                    shock_sd: r.fit.round1.noise.shock_sd,
                })
                .collect(),
            has_effect,
            adoptable,
        });
    }
    Ok(Ablation {
        shock_sd_grid: ablation.shock_sd_grid.clone(),
        polls_only: reference_scores,
        variants,
    })
}
