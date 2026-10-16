//! `config/backtest.yaml`: which elections are replayed, when, and which error settings the fit
//! tries.

use std::collections::BTreeMap;
use std::path::PathBuf;

use fr2027_collectors::Repo;
use jiff::civil::Date;
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

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
    pub elections: Vec<ElectionEntry>,
    /// Official results (`research/sources/results-2017-2022.csv`).
    pub results_file: PathBuf,
    pub final_polls: FinalPollsConfig,
    pub grid: Grid,
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
    pub round2_error_sd: Vec<f64>,
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
        if self.simulations == 0 {
            return Err("simulations must be positive".into());
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
        Ok(())
    }
}
