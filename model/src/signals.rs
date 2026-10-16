//! Event detection: news and attention spikes that widen a candidate's uncertainty.
//!
//! The spec's news and social layer is not a vote-share predictor: a spike in how much a
//! candidate is read about flags a possible shock (a scandal, a primary, a withdrawal), and for a
//! while after it the latent support of that candidate, and the strength of their bloc, may move
//! faster than the everyday random walk allows. Nothing here says in which direction.
//!
//! A **spike** on day `d` is when the day's value, against the median of the
//! [`SignalParams::baseline_days`] before it, reaches [`SignalParams::spike_ratio`]:
//!
//! ```text
//! score_d = mean over sources of ln((x_d + floor) / (median(x_{d-28..d-1}) + floor))
//! spike   = score_d ≥ ln(spike_ratio), and no spike for this candidate in the
//!           refractory_days before (a scandal that runs for a week is one event)
//! ```
//!
//! Each spike adds `shock_sd²` of variance to the candidate's within-bloc support and to their
//! bloc's strength, spread over the following days with a half-life: the extra variance accrued
//! between days `a` and `b` after a spike on day `e` is `shock_sd² (e^{-λ(a-e)} - e^{-λ(b-e)})`,
//! `λ = ln 2 / half_life_days`. The part of a recent spike's variance not yet accrued by the
//! forecast date is added when the state is projected to election day.
//!
//! `shock_sd` is chosen with the other noise settings by marginal likelihood on the polls, from
//! [`SignalParams::shock_sd_grid`]; a grid of `[0.0]` turns the layer off. The backtest decides
//! whether the grid may hold anything else.

use std::collections::BTreeMap;

use jiff::civil::Date;
use serde::{Deserialize, Serialize};

use crate::{ModelError, day_number, field::Field};

/// Where a daily signal comes from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalSource {
    /// Daily user views of the candidate's French Wikipedia article.
    Attention,
    /// Daily count of French articles naming the candidate (GDELT).
    News,
}

/// Event detection settings, read from `config/model.yaml` (`signals`).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SignalParams {
    /// The sources whose spikes count; empty turns event detection off.
    pub sources: Vec<SignalSource>,
    /// Days before a day whose median is that day's baseline.
    pub baseline_days: u32,
    /// Fewest days with data in the baseline window for a day to be scored.
    pub min_baseline_days: u32,
    /// A day at least this many times its baseline is a spike.
    pub spike_ratio: f64,
    /// Added to a day's value and to its baseline before the ratio, so that a small count
    /// doubling (2 articles instead of 1) is not a spike: views, then articles.
    pub attention_floor: f64,
    pub news_floor: f64,
    /// Days after a spike during which another spike of the same candidate is not a new event.
    pub refractory_days: u32,
    /// Half-life of a spike's extra variance, in days.
    pub half_life_days: f64,
    /// Candidate values of the total extra standard deviation (log scale) a spike lets a
    /// candidate's support and bloc strength move; chosen by marginal likelihood.
    pub shock_sd_grid: Vec<f64>,
}

impl SignalParams {
    /// Settings that detect nothing: the polls-only model.
    pub fn off() -> Self {
        SignalParams {
            sources: Vec::new(),
            baseline_days: 28,
            min_baseline_days: 21,
            spike_ratio: 3.0,
            attention_floor: 100.0,
            news_floor: 5.0,
            refractory_days: 14,
            half_life_days: 10.0,
            shock_sd_grid: vec![0.0],
        }
    }

    pub fn check(&self) -> Result<(), ModelError> {
        let invalid = |message: &str| Err(ModelError::InvalidParameters(format!("signals: {message}")));
        if self.shock_sd_grid.is_empty() || self.shock_sd_grid.iter().any(|v| !(v.is_finite() && *v >= 0.0)) {
            return invalid("shock_sd_grid must be a non-empty list of non-negative numbers");
        }
        if !(self.spike_ratio.is_finite() && self.spike_ratio > 1.0) {
            return invalid("spike_ratio must be above 1");
        }
        if !(self.half_life_days.is_finite() && self.half_life_days > 0.0) {
            return invalid("half_life_days must be positive");
        }
        if self.min_baseline_days == 0 || self.min_baseline_days > self.baseline_days {
            return invalid("min_baseline_days must be between 1 and baseline_days");
        }
        if !(self.attention_floor >= 0.0 && self.news_floor >= 0.0) {
            return invalid("floors must be non-negative");
        }
        Ok(())
    }

    fn floor(&self, source: SignalSource) -> f64 {
        match source {
            SignalSource::Attention => self.attention_floor,
            SignalSource::News => self.news_floor,
        }
    }
}

/// Daily values of each candidate's signals, as recorded. A missing day is unknown, not zero.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Signals {
    series: BTreeMap<(String, SignalSource), BTreeMap<i32, f64>>,
}

impl Signals {
    pub fn new() -> Self {
        Self::default()
    }

    /// Records one day's value; a later value for the same day replaces an earlier one.
    pub fn add(&mut self, candidate_id: &str, source: SignalSource, date: Date, value: f64) {
        self.series
            .entry((candidate_id.to_string(), source))
            .or_default()
            .insert(day_number(date), value);
    }

    pub fn is_empty(&self) -> bool {
        self.series.is_empty()
    }

    /// The last day with any value.
    pub fn last_day(&self) -> Option<Date> {
        self.series
            .values()
            .filter_map(|days| days.keys().next_back())
            .max()
            .map(|d| crate::date_from_day_number(*d))
    }
}

/// One detected event.
#[derive(Clone, Debug, PartialEq)]
pub struct Spike {
    pub candidate: usize,
    pub date: Date,
    /// Mean log ratio of the day's values to their baselines.
    pub score: f64,
    /// Each source's ratio of the day's value to its baseline (with the floor added to both).
    pub ratios: Vec<(SignalSource, f64)>,
}

/// The spikes of every candidate in `field`, using only values dated before `as_of` (a day's
/// page views and articles are complete the day after).
pub fn detect(signals: &Signals, field: &Field, params: &SignalParams, as_of: Date) -> Vec<Spike> {
    if params.sources.is_empty() {
        return Vec::new();
    }
    let last = day_number(as_of) - 1;
    let threshold = params.spike_ratio.ln();
    let mut spikes = Vec::new();
    for (c, candidate) in field.candidates.iter().enumerate() {
        // Each day's ratio per source, then the mean over the sources that have one.
        let mut by_day: BTreeMap<i32, Vec<(SignalSource, f64)>> = BTreeMap::new();
        for &source in &params.sources {
            let Some(series) = signals.series.get(&(candidate.id.clone(), source)) else {
                continue;
            };
            for (&day, &value) in series.range(..=last) {
                let window: Vec<f64> = series
                    .range(day - params.baseline_days as i32..day)
                    .map(|(_, v)| *v)
                    .collect();
                if window.len() < params.min_baseline_days as usize {
                    continue;
                }
                let floor = params.floor(source);
                let ratio = (value + floor) / (median(window) + floor);
                by_day.entry(day).or_default().push((source, ratio));
            }
        }
        let mut last_spike: Option<i32> = None;
        for (day, ratios) in by_day {
            let score = ratios.iter().map(|(_, r)| r.ln()).sum::<f64>() / ratios.len() as f64;
            let refractory = last_spike.is_some_and(|s| day - s <= params.refractory_days as i32);
            if score >= threshold && !refractory {
                last_spike = Some(day);
                spikes.push(Spike {
                    candidate: c,
                    date: crate::date_from_day_number(day),
                    score,
                    ratios,
                });
            }
        }
    }
    spikes.sort_by_key(|s| (s.date, s.candidate));
    spikes
}

fn median(mut values: Vec<f64>) -> f64 {
    values.sort_by(f64::total_cmp);
    let n = values.len();
    if n % 2 == 1 {
        values[n / 2]
    } else {
        (values[n / 2 - 1] + values[n / 2]) / 2.0
    }
}

/// The extra variance spikes add to the state, as the filter consumes it.
#[derive(Clone, Debug, PartialEq)]
pub struct ShockVariance {
    /// Per spike: the state indices it widens (support, bloc strength) and its day.
    shocks: Vec<([usize; 2], i32)>,
    /// `shock_sd²`.
    variance: f64,
    /// Decay rate, `ln 2 / half_life_days`.
    rate: f64,
}

impl ShockVariance {
    pub fn new(shocks: Vec<([usize; 2], i32)>, shock_sd: f64, half_life_days: f64) -> Self {
        Self {
            shocks,
            variance: shock_sd * shock_sd,
            rate: std::f64::consts::LN_2 / half_life_days,
        }
    }

    pub fn none() -> Self {
        Self::new(Vec::new(), 0.0, 1.0)
    }

    pub fn is_none(&self) -> bool {
        self.shocks.is_empty() || self.variance == 0.0
    }

    /// Adds to `diagonal` the variance every spike accrues between days `from` and `to`.
    pub fn accrue(&self, diagonal: &mut dyn FnMut(usize, f64), from: i32, to: i32) {
        if self.is_none() || to <= from {
            return;
        }
        for (indices, day) in &self.shocks {
            let elapsed = |t: i32| f64::from((t - day).max(0));
            let accrued = self.variance * ((-self.rate * elapsed(from)).exp() - (-self.rate * elapsed(to)).exp());
            if accrued > 0.0 {
                for &i in indices {
                    diagonal(i, accrued);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::tests::{candidate, option, slot};
    use jiff::civil::date;

    fn field() -> Field {
        Field::new(
            vec!["left".into(), "right".into()],
            vec![candidate("l1", "left"), candidate("r1", "right")],
            &[
                slot("l1", vec![option(&["l1"], 1.0)]),
                slot("r1", vec![option(&["r1"], 1.0)]),
            ],
        )
        .unwrap()
    }

    fn params() -> SignalParams {
        SignalParams {
            sources: vec![SignalSource::Attention],
            ..SignalParams::off()
        }
    }

    /// 60 days of 1,000 views from 1 August, with `extra` on some days.
    fn signals(extra: &[(i8, i8, f64)]) -> Signals {
        let mut signals = Signals::new();
        let start = day_number(date(2026, 8, 1));
        for d in 0..60 {
            signals.add(
                "l1",
                SignalSource::Attention,
                crate::date_from_day_number(start + d),
                1000.0,
            );
        }
        for &(month, day, value) in extra {
            signals.add("l1", SignalSource::Attention, date(2026, month, day), value);
        }
        signals
    }

    #[test]
    fn finds_a_spike_and_ignores_its_aftermath() {
        let s = signals(&[(9, 10, 5000.0), (9, 11, 4000.0), (9, 20, 4000.0)]);
        let spikes = detect(&s, &field(), &params(), date(2026, 10, 1));
        assert_eq!(spikes.len(), 1, "{spikes:?}");
        assert_eq!((spikes[0].candidate, spikes[0].date), (0, date(2026, 9, 10)));
        assert!((spikes[0].score - (5100.0f64 / 1100.0).ln()).abs() < 1e-12);
    }

    #[test]
    fn needs_a_baseline_and_ignores_the_forecast_day() {
        // A spike on the first days has no baseline; one on as_of is not complete yet.
        let s = signals(&[(8, 3, 9000.0), (9, 25, 9000.0)]);
        assert!(detect(&s, &field(), &params(), date(2026, 9, 25)).is_empty());
        assert_eq!(detect(&s, &field(), &params(), date(2026, 9, 26)).len(), 1);
    }

    #[test]
    fn no_sources_detects_nothing() {
        let s = signals(&[(9, 10, 50_000.0)]);
        assert!(detect(&s, &field(), &SignalParams::off(), date(2026, 10, 1)).is_empty());
    }

    #[test]
    fn a_spike_accrues_its_whole_variance_over_time() {
        let shocks = ShockVariance::new(vec![([0, 1], 10)], 0.2, 10.0);
        let mut total = [0.0; 2];
        shocks.accrue(&mut |i, v| total[i] += v, 0, 10);
        assert_eq!(total, [0.0; 2], "nothing before the spike");
        shocks.accrue(&mut |i, v| total[i] += v, 10, 20);
        assert!((total[0] - 0.04 * 0.5).abs() < 1e-12, "half after one half-life");
        shocks.accrue(&mut |i, v| total[i] += v, 20, 10_000);
        assert!((total[0] - 0.04).abs() < 1e-12 && total[0] == total[1]);
    }
}
