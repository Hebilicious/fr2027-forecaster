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
//!
//! The spec's optional **momentum** term tilts each simulated round 1 vote by how a candidate's
//! share of attention moved: over the last [`SignalParams::momentum_days`] against the
//! [`SignalParams::baseline_days`] before, on the log scale, averaged over the sources, and
//! decayed with the half-life over the days left to the election:
//!
//! ```text
//! p_c ∝ p_c · exp(momentum_weight · m_c · 2^(−days_to_election / momentum_half_life_days))
//! ```
//!
//! Attention says nothing about its sign (a scandal draws readers too), so the weight may be
//! negative; it is 0 unless the backtest shows a gain.
//!
//! **Tone momentum** gives the tilt a direction from the words of the coverage: GDELT scores each
//! article's tone (positive minus negative words, in percent), and a candidate's tone on a day is
//! read against the mean tone of every French article that day, so a dark news day for everyone
//! moves no one. The tilt adds `tone_momentum_weight` times the change in that relative tone,
//! article-weighted, between the same two windows:
//!
//! ```text
//! T_c(window) = Σ_days (tone_c − tone_all) · articles_c / Σ_days articles_c
//! p_c ∝ p_c · exp(tone_momentum_weight · (T_c(recent) − T_c(baseline)) · 2^(−days_to_election / momentum_half_life_days))
//! ```
//!
//! A candidate with fewer than `tone_min_articles` articles in either window gets no tone tilt:
//! the tone of a handful of articles is noise.
//!
//! **Stance momentum** reads the same articles for what they say about the candidate rather than
//! their words' mood: Jev (TypeSafe's classifier) judges each article favourable, neutral or
//! unfavourable to each candidate it names, leaving aside the opinions of the people it quotes.
//! The tilt adds `stance_momentum_weight` times the change in the candidate's net stance, the
//! favourable share minus the unfavourable share of the articles read, between the same windows;
//! fewer than `stance_min_articles` read articles in either window give none:
//!
//! ```text
//! S_c(window) = Σ_days (favourable_c − unfavourable_c) / Σ_days read_c
//! p_c ∝ p_c · exp(stance_momentum_weight · (S_c(recent) − S_c(baseline)) · 2^(−days_to_election / momentum_half_life_days))
//! ```

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
    /// Share of sampled X posts supporting the candidate (Grok Bot drops), for X momentum only.
    XSupport,
    /// The day's GDELT tone of the candidate's articles against every French article's, times their
    /// number of articles (`(tone_c − tone_all) · articles_c`), for tone momentum only.
    NewsTone,
    /// The day's articles naming the candidate that Jev read and found about them, for stance
    /// momentum only.
    StanceRead,
    /// Their favourable probabilities summed minus their unfavourable ones, for stance momentum only.
    StanceNet,
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
    /// Weight of the momentum term; 0 turns it off.
    pub momentum_weight: f64,
    /// Days of recent attention the momentum compares with the baseline before them.
    pub momentum_days: u32,
    /// Weight of X momentum: the change in the X support share over the same windows.
    pub x_momentum_weight: f64,
    /// Weight of tone momentum: the change in the candidate's GDELT tone against the day's, over the
    /// same windows, in points of tone.
    pub tone_momentum_weight: f64,
    /// Fewest articles in each window for a candidate's tone to count.
    pub tone_min_articles: f64,
    /// Weight of stance momentum: the change in the candidate's net stance (favourable minus
    /// unfavourable share of the articles Jev read) over the same windows.
    pub stance_momentum_weight: f64,
    /// Fewest articles read in each window for a candidate's stance to count.
    pub stance_min_articles: f64,
    /// How fast a momentum tilt fades over the days left to the election: it halves every this
    /// many days, so far from the vote it counts for little.
    pub momentum_half_life_days: f64,
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
            momentum_weight: 0.0,
            momentum_days: 7,
            x_momentum_weight: 0.0,
            tone_momentum_weight: 0.0,
            tone_min_articles: 30.0,
            stance_momentum_weight: 0.0,
            stance_min_articles: 20.0,
            momentum_half_life_days: 10.0,
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
        if !(self.half_life_days.is_finite() && self.half_life_days > 0.0)
            || !(self.momentum_half_life_days.is_finite() && self.momentum_half_life_days > 0.0)
        {
            return invalid("half_life_days and momentum_half_life_days must be positive");
        }
        if self.min_baseline_days == 0 || self.min_baseline_days > self.baseline_days {
            return invalid("min_baseline_days must be between 1 and baseline_days");
        }
        if !(self.attention_floor >= 0.0 && self.news_floor >= 0.0) {
            return invalid("floors must be non-negative");
        }
        if !self.momentum_weight.is_finite()
            || !self.x_momentum_weight.is_finite()
            || !self.tone_momentum_weight.is_finite()
            || !self.stance_momentum_weight.is_finite()
            || self.momentum_days == 0
        {
            return invalid("the momentum weights must be numbers and momentum_days positive");
        }
        if !(self.tone_min_articles.is_finite() && self.tone_min_articles >= 1.0)
            || !(self.stance_min_articles.is_finite() && self.stance_min_articles >= 1.0)
        {
            return invalid("tone_min_articles and stance_min_articles must be at least 1");
        }
        Ok(())
    }

    fn floor(&self, source: SignalSource) -> f64 {
        match source {
            SignalSource::Attention => self.attention_floor,
            SignalSource::News => self.news_floor,
            SignalSource::XSupport | SignalSource::NewsTone | SignalSource::StanceRead | SignalSource::StanceNet => 0.0,
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

/// Each candidate's momentum tilt on the log scale (see the module notes), from values dated
/// before `as_of`: `momentum_weight` times the log change of their share of attention, plus
/// `x_momentum_weight` times the change in their X support share, plus `tone_momentum_weight`
/// times the change in their relative news tone, plus `stance_momentum_weight` times the change in
/// their net news stance, decayed with `momentum_half_life_days` over the days left to `election`.
/// 0 for a candidate without data.
pub fn momentum(signals: &Signals, field: &Field, params: &SignalParams, as_of: Date, election: Date) -> Vec<f64> {
    let n = field.len();
    let last = day_number(as_of) - 1;
    let recent_start = last - params.momentum_days as i32 + 1;
    let baseline_start = recent_start - params.baseline_days as i32;
    let series = |c: usize, source: SignalSource| signals.series.get(&(field.candidates[c].id.clone(), source));

    // Attention: the log change of each candidate's share of the page views and articles.
    // Floor on a share, so a candidate nobody reads does not swing from nothing.
    const SHARE_FLOOR: f64 = 0.001;
    let mut sums = vec![0.0; n];
    let mut counts = vec![0usize; n];
    let volume = params.sources.iter().filter(|s| {
        !matches!(
            s,
            SignalSource::XSupport | SignalSource::NewsTone | SignalSource::StanceRead | SignalSource::StanceNet
        )
    });
    for &source in volume.filter(|_| params.momentum_weight != 0.0) {
        let totals = |from: i32, to: i32| -> Vec<f64> {
            (0..n)
                .map(|c| series(c, source).map_or(0.0, |s| s.range(from..=to).map(|(_, v)| v).sum()))
                .collect()
        };
        let recent = totals(recent_start, last);
        let baseline = totals(baseline_start, recent_start - 1);
        let (recent_all, baseline_all): (f64, f64) = (recent.iter().sum(), baseline.iter().sum());
        if recent_all <= 0.0 || baseline_all <= 0.0 {
            continue;
        }
        for c in 0..n {
            if recent[c] > 0.0 || baseline[c] > 0.0 {
                let share = |x: f64, total: f64| x / total + SHARE_FLOOR;
                sums[c] += (share(recent[c], recent_all) / share(baseline[c], baseline_all)).ln();
                counts[c] += 1;
            }
        }
    }

    // X: the change in the mean support share between the two windows.
    let x_shift = |c: usize| -> f64 {
        if params.x_momentum_weight == 0.0 {
            return 0.0;
        }
        let Some(s) = series(c, SignalSource::XSupport) else {
            return 0.0;
        };
        let mean = |from: i32, to: i32| {
            let values: Vec<f64> = s.range(from..=to).map(|(_, v)| *v).collect();
            (!values.is_empty()).then(|| values.iter().sum::<f64>() / values.len() as f64)
        };
        match (mean(recent_start, last), mean(baseline_start, recent_start - 1)) {
            (Some(recent), Some(baseline)) => params.x_momentum_weight * (recent - baseline),
            _ => 0.0,
        }
    };

    // Tone: the change in the article-weighted tone against the day's, when both windows hold enough
    // articles.
    let tone_shift = |c: usize| -> f64 {
        if params.tone_momentum_weight == 0.0 {
            return 0.0;
        }
        let (Some(tone), Some(articles)) = (series(c, SignalSource::NewsTone), series(c, SignalSource::News)) else {
            return 0.0;
        };
        let mean = |from: i32, to: i32| {
            // Only days that carry a tone count, articles included.
            let days: Vec<i32> = tone.range(from..=to).map(|(d, _)| *d).collect();
            let n: f64 = days.iter().filter_map(|d| articles.get(d)).sum();
            let sum: f64 = days.iter().filter_map(|d| tone.get(d)).sum();
            (n >= params.tone_min_articles).then(|| sum / n)
        };
        match (mean(recent_start, last), mean(baseline_start, recent_start - 1)) {
            (Some(recent), Some(baseline)) => params.tone_momentum_weight * (recent - baseline),
            _ => 0.0,
        }
    };

    // Stance: the change in the net stance of the articles Jev read, when both windows hold enough.
    let stance_shift = |c: usize| -> f64 {
        if params.stance_momentum_weight == 0.0 {
            return 0.0;
        }
        let (Some(net), Some(read)) = (series(c, SignalSource::StanceNet), series(c, SignalSource::StanceRead)) else {
            return 0.0;
        };
        let share = |from: i32, to: i32| {
            let n: f64 = read.range(from..=to).map(|(_, v)| v).sum();
            let sum: f64 = net.range(from..=to).map(|(_, v)| v).sum();
            (n >= params.stance_min_articles).then(|| sum / n)
        };
        match (share(recent_start, last), share(baseline_start, recent_start - 1)) {
            (Some(recent), Some(baseline)) => params.stance_momentum_weight * (recent - baseline),
            _ => 0.0,
        }
    };

    let days = f64::from((day_number(election) - day_number(as_of)).max(0));
    let decay = (-std::f64::consts::LN_2 * days / params.momentum_half_life_days).exp();
    (0..n)
        .map(|c| {
            let attention = if counts[c] == 0 {
                0.0
            } else {
                params.momentum_weight * sums[c] / counts[c] as f64
            };
            (attention + x_shift(c) + tone_shift(c) + stance_shift(c)) * decay
        })
        .collect()
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
    fn momentum_follows_a_rising_share_and_decays() {
        // l1 has 1,000 views a day, r1 1,000 until 23 September and 3,000 from the 24th.
        let mut s = Signals::new();
        let start = day_number(date(2026, 8, 1));
        for d in 0..61 {
            let day = crate::date_from_day_number(start + d);
            s.add("l1", SignalSource::Attention, day, 1000.0);
            let r1 = if day >= date(2026, 9, 24) { 3000.0 } else { 1000.0 };
            s.add("r1", SignalSource::Attention, day, r1);
        }
        let p = SignalParams {
            momentum_weight: 0.1,
            ..params()
        };
        let as_of = date(2026, 10, 1);
        let now = momentum(&s, &field(), &p, as_of, as_of);
        // Recent shares 0.25 / 0.75 against 0.5 / 0.5 before.
        assert!((now[1] - 0.1 * (0.751f64 / 0.501).ln()).abs() < 1e-9, "{now:?}");
        assert!(now[0] < 0.0);
        let later = momentum(&s, &field(), &p, as_of, date(2026, 10, 11));
        assert!((later[1] - now[1] / 2.0).abs() < 1e-9, "one half-life later");
        assert_eq!(momentum(&s, &field(), &params(), as_of, as_of), vec![0.0, 0.0]);
    }

    #[test]
    fn x_momentum_follows_the_support_share() {
        // r1's X support share: 0.30 through September, 0.40 from the 24th.
        let mut s = Signals::new();
        let start = day_number(date(2026, 8, 20));
        for d in 0..42 {
            let day = crate::date_from_day_number(start + d);
            s.add(
                "r1",
                SignalSource::XSupport,
                day,
                if day >= date(2026, 9, 24) { 0.4 } else { 0.3 },
            );
        }
        let p = SignalParams {
            x_momentum_weight: 0.5,
            ..params()
        };
        let as_of = date(2026, 10, 1);
        let tilt = momentum(&s, &field(), &p, as_of, as_of);
        assert_eq!(tilt[0], 0.0, "no X data for l1");
        assert!((tilt[1] - 0.5 * 0.1).abs() < 1e-9, "{tilt:?}");
    }

    #[test]
    fn tone_momentum_follows_the_tone_against_the_days() {
        // r1: 20 articles a day; their tone 1 point above the day's through September, 1 below
        // from the 24th. l1: 2 articles a day, too few to count.
        let mut s = Signals::new();
        let start = day_number(date(2026, 8, 20));
        for d in 0..42 {
            let day = crate::date_from_day_number(start + d);
            let relative = if day >= date(2026, 9, 24) { -1.0 } else { 1.0 };
            s.add("r1", SignalSource::News, day, 20.0);
            s.add("r1", SignalSource::NewsTone, day, relative * 20.0);
            s.add("l1", SignalSource::News, day, 2.0);
            s.add("l1", SignalSource::NewsTone, day, -40.0);
        }
        let p = SignalParams {
            tone_momentum_weight: 0.05,
            ..params()
        };
        let as_of = date(2026, 10, 1);
        let tilt = momentum(&s, &field(), &p, as_of, as_of);
        assert_eq!(tilt[0], 0.0, "14 articles a week is under tone_min_articles");
        assert!((tilt[1] - 0.05 * (-1.0 - 1.0)).abs() < 1e-9, "{tilt:?}");
        assert_eq!(
            momentum(&s, &field(), &params(), as_of, as_of),
            vec![0.0, 0.0],
            "off at weight 0"
        );
    }

    #[test]
    fn stance_momentum_follows_the_net_stance_of_the_articles_read() {
        // r1: 10 articles read a day; net stance +0.2 per article through September, −0.3 from the 24th.
        let mut s = Signals::new();
        let start = day_number(date(2026, 8, 20));
        for d in 0..42 {
            let day = crate::date_from_day_number(start + d);
            let net = if day >= date(2026, 9, 24) { -0.3 } else { 0.2 };
            s.add("r1", SignalSource::StanceRead, day, 10.0);
            s.add("r1", SignalSource::StanceNet, day, net * 10.0);
            s.add("l1", SignalSource::StanceRead, day, 1.0);
            s.add("l1", SignalSource::StanceNet, day, 1.0);
        }
        let p = SignalParams {
            stance_momentum_weight: 0.5,
            ..params()
        };
        let as_of = date(2026, 10, 1);
        let tilt = momentum(&s, &field(), &p, as_of, as_of);
        assert_eq!(tilt[0], 0.0, "7 read in a week is under stance_min_articles");
        assert!((tilt[1] - 0.5 * (-0.3 - 0.2)).abs() < 1e-9, "{tilt:?}");
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
