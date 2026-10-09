//! The forecasting model for the 2027 French presidential election.
//!
//! Pure computation: callers hand in the validated field, polls and parameters, and get back
//! the forecast. Nothing here reads files, the clock or the network, so a run is a function of
//! its inputs and its seed.
//!
//! - [`field`]: who runs (slots of mutually exclusive outcomes).
//! - [`round1`]: the poll aggregation, a linear-Gaussian state-space model of bloc strengths and
//!   within-bloc candidate support on the log-ratio scale, with pollster house effects.
//! - [`round2`]: head-to-head poll aggregation and the bloc-transfer fallback.
//! - [`simulate`]: the two-round Monte Carlo that turns both into probabilities.

pub mod field;
pub mod kalman;
pub mod rng;
pub mod round1;
pub mod round2;
pub mod simulate;

use std::collections::BTreeMap;

use jiff::civil::Date;
use serde::{Deserialize, Serialize};

pub use field::{Candidate, Field, Slot, SlotOption};
pub use simulate::{Forecast, run_forecast};

#[derive(Debug, thiserror::Error)]
pub enum ModelError {
    #[error("invalid field: {0}")]
    InvalidField(String),
    #[error("invalid poll `{poll}`: {message}")]
    InvalidPoll { poll: String, message: String },
    #[error("invalid parameters: {0}")]
    InvalidParameters(String),
    #[error("{0} is not positive definite")]
    NotPositiveDefinite(&'static str),
    #[error("not enough data: {0}")]
    NotEnoughData(String),
}

/// One published poll, as the collectors validated it. Shares are percentages of expressed votes.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Poll {
    pub poll_id: String,
    pub firm: String,
    pub field_start: Date,
    pub field_end: Date,
    pub published_at: Date,
    pub sample_size: u32,
    pub scenarios: Vec<Scenario>,
}

/// One ballot a poll tested: the exact set of candidates offered, and their shares.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Scenario {
    pub scenario_id: String,
    pub round: u8,
    pub shares: Vec<Share>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Share {
    pub candidate_id: String,
    pub share: f64,
}

impl Poll {
    /// The fieldwork midpoint, the date a poll's observation is placed at.
    pub fn midpoint(&self) -> i32 {
        let start = day_number(self.field_start);
        let end = day_number(self.field_end);
        start + (end - start) / 2
    }
}

/// Model parameters, read from `config/model.yaml`.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ModelParams {
    pub model_version: String,
    pub seed: u64,
    pub simulations: u32,
    pub round1_date: Date,
    pub round2_date: Date,
    pub aggregation: AggregationParams,
    pub round1_error: ErrorParams,
    pub round2: Round2Params,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AggregationParams {
    /// Candidate daily random-walk standard deviations on the log-ratio scale; the one with the
    /// highest marginal likelihood is used.
    pub random_walk_sd_grid: Vec<f64>,
    /// Multipliers of the multinomial sampling variance (online quota samples are not simple
    /// random samples); chosen jointly with the random-walk scale.
    pub design_effect_grid: Vec<f64>,
    /// Prior standard deviation of bloc and candidate levels before any poll.
    pub prior_sd: f64,
    /// Prior standard deviation of a candidate's presence effect on their bloc's total.
    pub presence_sd: f64,
    /// Prior standard deviation of a pollster's house effect on a bloc.
    pub house_sd: f64,
    /// Shares below this percentage are raised to it before taking logs.
    pub share_floor: f64,
    /// Fractions of a scenario's sampling variance that is its own rather than shared with the
    /// poll's other scenarios (same respondents); chosen with the two grids above.
    pub scenario_noise_share_grid: Vec<f64>,
    /// Spacing of the latent-share history, in days.
    pub series_step_days: u32,
    /// Draws per history point used for its interval.
    pub series_draws: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ErrorParams {
    pub student_t_df: f64,
    /// Election-day polling error of a bloc's log strength.
    pub bloc_sd: f64,
    /// Election-day polling error of a candidate within their bloc.
    pub candidate_sd: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Round2Params {
    pub random_walk_sd: f64,
    pub design_effect: f64,
    pub prior_sd: f64,
    /// Election-day polling error on the logit of a finalist's share.
    pub error_sd: f64,
    /// Uncertainty of a transfer-based estimate, on the logit scale.
    pub transfer_sd: f64,
    /// Share of a finalist's own first-round voters who vote for them again.
    pub loyalty: f64,
    /// Relative propensity of each voter bloc to vote for a finalist of each bloc, or to abstain
    /// (key `abstain`).
    pub transfers: BTreeMap<String, BTreeMap<String, f64>>,
}

/// Days since 1970-01-01.
pub fn day_number(date: Date) -> i32 {
    // Howard Hinnant's days_from_civil.
    let (y, m, d) = (i32::from(date.year()), i32::from(date.month()), i32::from(date.day()));
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// Inverse of [`day_number`].
pub fn date_from_day_number(days: i32) -> Date {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i32::from(m <= 2);
    Date::new(y as i16, m as i8, d as i8).expect("civil date from day number")
}

pub(crate) fn quantile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return f64::NAN;
    }
    let position = p * (sorted.len() - 1) as f64;
    let lower = position.floor() as usize;
    let upper = position.ceil() as usize;
    let weight = position - lower as f64;
    sorted[lower] * (1.0 - weight) + sorted[upper] * weight
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_numbers_round_trip() {
        let epoch = Date::new(1970, 1, 1).unwrap();
        assert_eq!(day_number(epoch), 0);
        let r1 = Date::new(2027, 4, 18).unwrap();
        let r2 = Date::new(2027, 5, 2).unwrap();
        assert_eq!(day_number(r2) - day_number(r1), 14);
        for days in [-1000, 0, 59, 60, 20_000, 20_744] {
            assert_eq!(day_number(date_from_day_number(days)), days);
        }
        let leap = Date::new(2028, 2, 29).unwrap();
        assert_eq!(date_from_day_number(day_number(leap)), leap);
    }

    #[test]
    fn quantile_interpolates() {
        let sorted = [0.0, 1.0, 2.0, 3.0, 4.0];
        assert_eq!(quantile(&sorted, 0.5), 2.0);
        assert_eq!(quantile(&sorted, 0.1), 0.4);
        assert_eq!(quantile(&sorted, 1.0), 4.0);
    }
}
