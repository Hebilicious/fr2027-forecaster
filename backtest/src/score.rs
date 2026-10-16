//! Scores. Binary events (a candidate qualifies, a candidate wins) get the Brier score and the
//! log-loss; the realised round 1 shares and round 2 share get a log score, the log of the
//! predictive density at the result, which is what the error settings are fitted on.

use fr2027_model::{Draw, ModelParams};
use serde::Serialize;

use crate::{Result, election::Election, replay::Replay};

pub fn brier(p: f64, happened: bool) -> f64 {
    (p - if happened { 1.0 } else { 0.0 }).powi(2)
}

/// Log-loss with `p` clipped to `[floor, 1 - floor]`.
pub fn log_loss(p: f64, happened: bool, floor: f64) -> f64 {
    let p = p.clamp(floor, 1.0 - floor);
    -(if happened { p } else { 1.0 - p }).ln()
}

/// Log of a Gaussian kernel density estimate at `y`, with Silverman's rule-of-thumb bandwidth
/// (0.9 · min(sd, IQR / 1.34) · n^(-1/5)).
pub fn kde_log_density(samples: &mut [f64], y: f64) -> f64 {
    let n = samples.len();
    if n < 2 {
        return f64::NEG_INFINITY;
    }
    samples.sort_by(f64::total_cmp);
    let mean = samples.iter().sum::<f64>() / n as f64;
    let sd = (samples.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1) as f64).sqrt();
    let quartile = |p: f64| samples[((n - 1) as f64 * p).round() as usize];
    let iqr = quartile(0.75) - quartile(0.25);
    let spread = if iqr > 0.0 { sd.min(iqr / 1.34) } else { sd };
    let h = (0.9 * spread * (n as f64).powf(-0.2)).max(1e-6);
    let density: f64 = samples
        .iter()
        .map(|x| (-0.5 * ((y - x) / h).powi(2)).exp())
        .sum::<f64>()
        / (n as f64 * h * (2.0 * std::f64::consts::PI).sqrt());
    density.max(f64::MIN_POSITIVE).ln()
}

/// Log density at `y` of the model's Student-t with `df` degrees of freedom, centred on
/// `location` and scaled to standard deviation `sd` (the model draws unit-variance t errors).
pub fn student_t_log_pdf(y: f64, location: f64, sd: f64, df: f64) -> f64 {
    let scale = sd * ((df - 2.0) / df).sqrt();
    let z = (y - location) / scale;
    ln_gamma((df + 1.0) / 2.0)
        - ln_gamma(df / 2.0)
        - 0.5 * (df * std::f64::consts::PI).ln()
        - scale.ln()
        - (df + 1.0) / 2.0 * (1.0 + z * z / df).ln()
}

/// Lanczos approximation (g = 7, n = 9), accurate to about 15 digits for positive arguments.
pub fn ln_gamma(x: f64) -> f64 {
    const G: f64 = 7.0;
    const COEFFICIENTS: [f64; 9] = [
        0.999_999_999_999_809_9,
        676.520_368_121_885_1,
        -1_259.139_216_722_402_8,
        771.323_428_777_653_1,
        -176.615_029_162_140_6,
        12.507_343_278_686_905,
        -0.138_571_095_265_720_12,
        9.984_369_578_019_572e-6,
        1.505_632_735_149_311_6e-7,
    ];
    if x < 0.5 {
        let pi = std::f64::consts::PI;
        return (pi / (pi * x).sin()).ln() - ln_gamma(1.0 - x);
    }
    let x = x - 1.0;
    let mut sum = COEFFICIENTS[0];
    for (i, c) in COEFFICIENTS.iter().enumerate().skip(1) {
        sum += c / (x + i as f64);
    }
    let t = x + G + 0.5;
    0.5 * (2.0 * std::f64::consts::PI).ln() + (x + 0.5) * t.ln() - t + sum.ln()
}

pub fn logit(p: f64) -> f64 {
    (p / (1.0 - p)).ln()
}

/// One binary forecast and what happened.
#[derive(Clone, Debug, Serialize)]
pub struct Event {
    pub kind: EventKind,
    pub p: f64,
    pub happened: bool,
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    Qualify,
    Win,
}

#[derive(Clone, Debug, Serialize)]
pub struct CandidateScore {
    pub candidate_id: String,
    pub p_qualify: f64,
    pub p_win: f64,
    /// Round 1 share when running, from the simulations: mean and 80% interval. None when no poll
    /// had tested the candidate by the replay date.
    pub share_mean: Option<f64>,
    pub share_lo80: Option<f64>,
    pub share_hi80: Option<f64>,
    pub result_share: f64,
    pub qualified: bool,
    pub won: bool,
    /// Log density of the realised log share (contenders only).
    pub share_log_score: Option<f64>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Round2Score {
    /// Probability that the actual finalists meet.
    pub p_matchup: f64,
    /// Probability the actual winner wins, given that matchup.
    pub p_winner_given_matchup: Option<f64>,
    /// Log density of the realised logit share of the first finalist, given the matchup.
    pub log_score: Option<f64>,
}

/// One replay scored under one parameter setting.
#[derive(Clone, Debug, Serialize)]
pub struct ReplayScore {
    pub year: i16,
    pub horizon_days: u32,
    pub as_of: jiff::civil::Date,
    pub candidates: Vec<CandidateScore>,
    pub round2: Round2Score,
    #[serde(skip)]
    pub events: Vec<Event>,
}

impl ReplayScore {
    /// Sum of the contenders' share log scores.
    pub fn share_log_score(&self) -> f64 {
        self.candidates.iter().filter_map(|c| c.share_log_score).sum()
    }
}

/// Simulates `replay` under `params` and scores it against the result.
pub fn evaluate(election: &Election, replay: &Replay, params: &ModelParams) -> Result<ReplayScore> {
    let n = election.field.len();
    let (first, second) = election.finalists;
    let realised_logit = logit(election.round2_share);
    let df = params.round1_error.student_t_df;
    let mut log_shares: Vec<Vec<f64>> = vec![Vec::new(); n];
    let mut matchup_density = 0.0;
    let mut matchup_draws = 0u32;
    let mut observe = |draw: &Draw| {
        for &c in &replay.contenders {
            if draw.present[c] {
                log_shares[c].push(draw.shares[c].ln());
            }
        }
        if draw.finalists == (first, second) {
            matchup_draws += 1;
            matchup_density += student_t_log_pdf(realised_logit, draw.round2_mean, draw.round2_sd, df).exp();
        }
    };
    let simulation = fr2027_model::simulate(&election.field, &replay.fit, params, &mut observe)?;

    let winner = election.winner();
    let mut candidates = Vec::new();
    let mut events = Vec::new();
    for forecast in simulation.candidates.iter().filter(|c| election.ran[c.candidate]) {
        let c = forecast.candidate;
        let qualified = c == first || c == second;
        let share_log_score = replay
            .contenders
            .contains(&c)
            .then(|| kde_log_density(&mut log_shares[c], election.round1_result[c].ln()));
        events.push(Event {
            kind: EventKind::Qualify,
            p: forecast.p_qualify_r1,
            happened: qualified,
        });
        events.push(Event {
            kind: EventKind::Win,
            p: forecast.p_win,
            happened: c == winner,
        });
        candidates.push(CandidateScore {
            candidate_id: election.id(c).to_string(),
            p_qualify: forecast.p_qualify_r1,
            p_win: forecast.p_win,
            share_mean: forecast.r1_share.as_ref().map(|s| s.mean),
            share_lo80: forecast.r1_share.as_ref().map(|s| s.lo80),
            share_hi80: forecast.r1_share.as_ref().map(|s| s.hi80),
            result_share: election.round1_result[c],
            qualified,
            won: c == winner,
            share_log_score,
        });
    }
    candidates.sort_by(|a, b| b.result_share.total_cmp(&a.result_share));
    let pair = simulation.pairs.iter().find(|p| (p.a, p.b) == (first, second));
    let round2 = Round2Score {
        p_matchup: pair.map_or(0.0, |p| p.p_matchup),
        p_winner_given_matchup: pair.map(|p| if winner == first { p.p_a_wins } else { 1.0 - p.p_a_wins }),
        log_score: (matchup_draws > 0).then(|| (matchup_density / f64::from(matchup_draws)).ln()),
    };
    Ok(ReplayScore {
        year: replay.year,
        horizon_days: replay.horizon_days,
        as_of: replay.as_of,
        candidates,
        round2,
        events,
    })
}

/// Scores of a set of replays under one setting.
#[derive(Clone, Debug, Default, Serialize)]
pub struct Summary {
    pub replays: usize,
    pub qualify_brier: f64,
    pub qualify_log_loss: f64,
    pub win_brier: f64,
    pub win_log_loss: f64,
    /// Mean absolute error of round 1 shares, in points, over runners the polls had tested.
    pub share_mae_points: f64,
    /// Share of those runners whose result fell inside the 80% interval.
    pub coverage80: f64,
    /// Sum over replays of the contenders' share log scores.
    pub share_log_score: f64,
    /// Sum over replays of the round 2 log score (replays where the matchup was simulated).
    pub round2_log_score: f64,
    /// Mean probability given to the actual winner winning the actual matchup.
    pub p_winner_given_matchup: f64,
}

/// Averages the event scores (mean per event) and sums the log scores over `scores`.
pub fn summarise<'a>(scores: impl IntoIterator<Item = &'a ReplayScore>, floor: f64) -> Summary {
    let scores: Vec<&ReplayScore> = scores.into_iter().collect();
    let mut summary = Summary {
        replays: scores.len(),
        ..Default::default()
    };
    let mean = |values: Vec<f64>| {
        if values.is_empty() {
            f64::NAN
        } else {
            values.iter().sum::<f64>() / values.len() as f64
        }
    };
    let events = |kind: EventKind| -> Vec<&Event> {
        scores
            .iter()
            .flat_map(|s| s.events.iter().filter(move |e| e.kind == kind))
            .collect()
    };
    let qualify = events(EventKind::Qualify);
    let win = events(EventKind::Win);
    summary.qualify_brier = mean(qualify.iter().map(|e| brier(e.p, e.happened)).collect());
    summary.qualify_log_loss = mean(qualify.iter().map(|e| log_loss(e.p, e.happened, floor)).collect());
    summary.win_brier = mean(win.iter().map(|e| brier(e.p, e.happened)).collect());
    summary.win_log_loss = mean(win.iter().map(|e| log_loss(e.p, e.happened, floor)).collect());
    let tested: Vec<&CandidateScore> = scores
        .iter()
        .flat_map(|s| s.candidates.iter())
        .filter(|c| c.share_mean.is_some())
        .collect();
    summary.share_mae_points = mean(
        tested
            .iter()
            .map(|c| (c.share_mean.unwrap_or(0.0) - c.result_share).abs() * 100.0)
            .collect(),
    );
    summary.coverage80 = mean(
        tested
            .iter()
            .filter_map(|c| Some((c.share_lo80?, c.share_hi80?, c.result_share)))
            .map(|(lo, hi, result)| if lo <= result && result <= hi { 1.0 } else { 0.0 })
            .collect(),
    );
    let total = |values: Vec<f64>| {
        if values.is_empty() {
            f64::NAN
        } else {
            values.iter().sum()
        }
    };
    summary.share_log_score = total(
        scores
            .iter()
            .flat_map(|s| s.candidates.iter().filter_map(|c| c.share_log_score))
            .collect(),
    );
    summary.round2_log_score = total(scores.iter().filter_map(|s| s.round2.log_score).collect());
    summary.p_winner_given_matchup = mean(scores.iter().filter_map(|s| s.round2.p_winner_given_matchup).collect());
    summary
}

/// One row of a reliability table: binary forecasts whose probability fell in `[lo, hi)`.
#[derive(Clone, Debug, Serialize)]
pub struct ReliabilityBin {
    pub lo: f64,
    pub hi: f64,
    pub events: usize,
    pub mean_forecast: Option<f64>,
    pub observed: Option<f64>,
}

/// Ten equal bins over [0, 1] (the last one includes 1).
pub fn reliability<'a>(events: impl IntoIterator<Item = &'a Event>) -> Vec<ReliabilityBin> {
    let mut sums = [(0usize, 0.0f64, 0usize); 10];
    for event in events {
        let bin = ((event.p * 10.0).floor() as usize).min(9);
        sums[bin].0 += 1;
        sums[bin].1 += event.p;
        sums[bin].2 += usize::from(event.happened);
    }
    sums.iter()
        .enumerate()
        .map(|(i, (count, p, happened))| ReliabilityBin {
            lo: i as f64 / 10.0,
            hi: (i + 1) as f64 / 10.0,
            events: *count,
            mean_forecast: (*count > 0).then(|| p / *count as f64),
            observed: (*count > 0).then(|| *happened as f64 / *count as f64),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brier_and_log_loss() {
        assert_eq!(brier(0.7, true), 0.09000000000000002);
        assert_eq!(brier(0.7, false), 0.48999999999999994);
        assert!((log_loss(0.5, true, 0.001) - 2f64.ln()).abs() < 1e-12);
        // Clipping keeps a confident miss finite.
        assert!((log_loss(0.0, true, 0.001) - (1000f64).ln()).abs() < 1e-9);
        assert!((log_loss(1.0, false, 0.001) - (1000f64).ln()).abs() < 1e-9);
    }

    #[test]
    fn ln_gamma_matches_known_values() {
        assert!(ln_gamma(1.0).abs() < 1e-12);
        assert!(ln_gamma(2.0).abs() < 1e-12);
        assert!((ln_gamma(5.0) - 24f64.ln()).abs() < 1e-12);
        assert!((ln_gamma(0.5) - std::f64::consts::PI.sqrt().ln()).abs() < 1e-12);
        assert!((ln_gamma(3.5) - (3.323_350_970_447_843f64).ln()).abs() < 1e-12);
    }

    #[test]
    fn student_t_density_integrates_to_one_with_the_right_variance() {
        let (df, sd, location) = (5.0, 0.3, 0.2);
        let step = 0.0005;
        let (mut mass, mut variance) = (0.0, 0.0);
        let mut y = location - 20.0;
        while y < location + 20.0 {
            let density = student_t_log_pdf(y, location, sd, df).exp();
            mass += density * step;
            variance += density * (y - location).powi(2) * step;
            y += step;
        }
        assert!((mass - 1.0).abs() < 1e-4, "{mass}");
        assert!((variance - sd * sd).abs() < 2e-3, "{variance}");
    }

    #[test]
    fn kde_recovers_a_normal_density() {
        let mut rng = fr2027_model::rng::Rng::new(4);
        let mut samples: Vec<f64> = (0..20_000).map(|_| 1.0 + 0.5 * rng.normal()).collect();
        let at_mean = kde_log_density(&mut samples, 1.0);
        let exact = -(0.5 * (2.0 * std::f64::consts::PI).sqrt()).ln();
        assert!((at_mean - exact).abs() < 0.03, "{at_mean} vs {exact}");
        // Far in the tail the density is tiny but the log stays finite.
        assert!(kde_log_density(&mut samples, 10.0).is_finite());
    }

    #[test]
    fn reliability_bins_events() {
        let events = [
            Event {
                kind: EventKind::Win,
                p: 0.05,
                happened: false,
            },
            Event {
                kind: EventKind::Win,
                p: 0.72,
                happened: true,
            },
            Event {
                kind: EventKind::Win,
                p: 0.78,
                happened: false,
            },
            Event {
                kind: EventKind::Win,
                p: 1.0,
                happened: true,
            },
        ];
        let bins = reliability(&events);
        assert_eq!(bins.len(), 10);
        assert_eq!(bins[0].events, 1);
        assert_eq!(bins[7].events, 2);
        assert_eq!(bins[7].observed, Some(0.5));
        assert!((bins[7].mean_forecast.unwrap() - 0.75).abs() < 1e-12);
        assert_eq!(bins[9].events, 1);
    }
}
