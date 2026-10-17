//! The backtest (milestone 3): replays the model on past elections as of several dates before
//! each one, with only the polls published by then; scores the replays against the results and
//! against a latest-poll-average baseline; and fits the error settings (`round1_error` and the
//! round 2 errors in `config/model.yaml`, election-day and horizon) by repeated grid searches on
//! the log score of the realised result.
//!
//! - [`config`]: the settings in `config/backtest.yaml`.
//! - [`election`]: one past election: its field, polls and result.
//! - [`replay`]: the polls a replay may see, and the model fitted to them.
//! - [`score`]: Brier score, log-loss, log scores of realised shares, reliability.
//! - [`baseline`]: the latest-poll average.
//! - [`final_polls`]: final-week poll errors of earlier elections, as extra evidence.
//! - [`report`]: the whole run: scores before and after the fit, its stages, the cross-checks.

pub mod baseline;
pub mod config;
pub mod election;
pub mod final_polls;
pub mod replay;
pub mod report;
pub mod score;

use std::path::PathBuf;

pub use report::{Report, run};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Collectors(#[from] fr2027_collectors::Error),
    #[error(transparent)]
    Model(#[from] fr2027_model::ModelError),
    #[error("{path}: {message}")]
    Invalid { path: PathBuf, message: String },
}

impl Error {
    pub fn invalid(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        Error::Invalid {
            path: path.into(),
            message: message.into(),
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
