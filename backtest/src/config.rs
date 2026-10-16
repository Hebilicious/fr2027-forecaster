//! `config/backtest.yaml`: which elections are replayed, when, and which error settings the fit
//! tries.

use std::collections::BTreeMap;
use std::path::PathBuf;

use fr2027_collectors::Repo;
use jiff::civil::Date;
use serde::{Deserialize, Serialize};

use crate::{Error, Result, ablation::AblationConfig, report::Param};

pub const CONFIG_FILE: &str = "backtest.yaml";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct BacktestConfig {
    pub seed: u64,
    /// Simulations per replay and per error setting.
    pub simulations: u32,
    /// Replay dates, in days before round 1.
    pub horizons_days: Vec<u32>,
    /// Polls whose fieldwork started more than this many days before round 1 are left out.
    pub poll_window_days: u32,
    /// Runners whose fitted share reaches this fraction have their round 1 share scored.
    pub contender_share: f64,
    /// Probabilities are clipped to `[floor, 1 - floor]` before taking logs.
    pub probability_floor: f64,
    /// The fit repeats its stages until a pass changes nothing, at most this many times.
    pub max_passes: usize,
    pub elections: Vec<ElectionEntry>,
    /// Official results (`research/sources/results-2002-2022.csv`).
    pub results_file: PathBuf,
    pub final_polls: FinalPollsConfig,
    pub grid: Grid,
    /// The signal layers to score against the polls-only model; none skips the ablation.
    #[serde(default)]
    pub signals_ablation: Option<AblationConfig>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ElectionEntry {
    pub year: i16,
    /// Holds `candidates.yaml` (the field) and `polls.csv` (the polls, clean-file columns).
    pub dir: PathBuf,
    pub round1: Date,
    pub round2: Date,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct FinalPollsConfig {
    pub round1_file: PathBuf,
    pub round2_file: PathBuf,
    pub years: Vec<i16>,
    /// Bloc of each candidate named in `round1_file` for `years`.
    pub blocs: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Grid {
    pub bloc_sd: Vec<f64>,
    pub candidate_sd: Vec<f64>,
    pub horizon_bloc_sd: Vec<f64>,
    pub horizon_candidate_sd: Vec<f64>,
    pub student_t_df: Vec<f64>,
    pub round2_error_sd: Vec<f64>,
    pub round2_horizon_error_sd: Vec<f64>,
}

impl Grid {
    /// The values the fit tries for `param`.
    pub fn values(&self, param: Param) -> &[f64] {
        match param {
            Param::BlocSd => &self.bloc_sd,
            Param::CandidateSd => &self.candidate_sd,
            Param::HorizonBlocSd => &self.horizon_bloc_sd,
            Param::HorizonCandidateSd => &self.horizon_candidate_sd,
            Param::StudentTDf => &self.student_t_df,
            Param::Round2ErrorSd => &self.round2_error_sd,
            Param::Round2HorizonErrorSd => &self.round2_horizon_error_sd,
        }
    }
}

impl BacktestConfig {
    pub fn load(repo: &Repo) -> Result<Self> {
        let path = repo.config_dir().join(CONFIG_FILE);
        let text = std::fs::read_to_string(&path).map_err(|e| Error::invalid(&path, e.to_string()))?;
        let config: Self = serde_saphyr::from_str(&text).map_err(|e| Error::invalid(&path, e.to_string()))?;
        config.check().map_err(|message| Error::invalid(&path, message))?;
        Ok(config)
    }

    fn check(&self) -> Result<(), String> {
        if self.simulations == 0 || self.max_passes == 0 {
            return Err("simulations and max_passes must be positive".into());
        }
        if self.horizons_days.is_empty() || self.elections.is_empty() {
            return Err("needs at least one election and one horizon".into());
        }
        if !(self.probability_floor > 0.0 && self.probability_floor < 0.5) {
            return Err("probability_floor must be in (0, 0.5)".into());
        }
        let grid = &self.grid;
        for (name, values) in [
            ("bloc_sd", &grid.bloc_sd),
            ("candidate_sd", &grid.candidate_sd),
            ("round2_error_sd", &grid.round2_error_sd),
        ] {
            if values.is_empty() || values.iter().any(|v| !(v.is_finite() && *v > 0.0)) {
                return Err(format!("grid.{name} must be a non-empty list of positive numbers"));
            }
        }
        for (name, values) in [
            ("horizon_bloc_sd", &grid.horizon_bloc_sd),
            ("horizon_candidate_sd", &grid.horizon_candidate_sd),
            ("round2_horizon_error_sd", &grid.round2_horizon_error_sd),
        ] {
            if values.is_empty() || values.iter().any(|v| !(v.is_finite() && *v >= 0.0)) {
                return Err(format!("grid.{name} must be a non-empty list of non-negative numbers"));
            }
        }
        if grid.student_t_df.is_empty() || grid.student_t_df.iter().any(|v| !(v.is_finite() && *v > 2.0)) {
            return Err("grid.student_t_df must be a non-empty list of numbers above 2".into());
        }
        if let Some(ablation) = &self.signals_ablation
            && (ablation.shock_sd_grid.is_empty()
                || ablation.shock_sd_grid.iter().any(|v| !(v.is_finite() && *v >= 0.0)))
        {
            return Err("signals_ablation.shock_sd_grid must be a non-empty list of non-negative numbers".into());
        }
        Ok(())
    }
}
