//! The two-round Monte Carlo. Each simulated election draws who runs, the true round 1 vote
//! (the aggregation's state projected to election day, plus a fat-tailed national polling
//! error correlated within blocs), the two qualifiers, and the round 2 result.

use crate::{
    ModelError, ModelParams, Poll, day_number,
    field::Field,
    quantile,
    rng::Rng,
    round1::{Errors, Round1Fit, ShareScratch, fit_round1, round1_shares},
    round2::{Round2Fit, Transfers, fit_round2},
};

#[derive(Clone, Debug)]
pub struct Interval {
    pub mean: f64,
    pub lo80: f64,
    pub hi80: f64,
}

#[derive(Clone, Debug)]
pub struct CandidateForecast {
    pub candidate: usize,
    pub p_run: f64,
    /// False when no poll has tested the candidate: they are left out of every simulated field.
    pub simulated: bool,
    pub p_qualify_r1: f64,
    pub p_win: f64,
    /// Round 1 share when the candidate runs.
    pub r1_share: Option<Interval>,
}

#[derive(Clone, Debug)]
pub struct PairForecast {
    pub a: usize,
    pub b: usize,
    pub p_matchup: f64,
    /// Probability `a` wins when this matchup happens.
    pub p_a_wins: f64,
    /// Whether direct round 2 polls informed the matchup.
    pub polled: bool,
}

#[derive(Clone, Debug)]
pub struct Forecast {
    pub as_of: jiff::civil::Date,
    pub simulations: u32,
    pub seed: u64,
    pub candidates: Vec<CandidateForecast>,
    /// Matchups that occurred in at least one simulation, most likely first.
    pub pairs: Vec<PairForecast>,
    pub round1: Round1Fit,
    pub round2: Round2Fit,
    pub warnings: Vec<String>,
}

const MAX_FIELD_DRAWS: usize = 10_000;

/// Fits both rounds to the polls published by `as_of` and simulates the election.
pub fn run_forecast(
    field: &Field,
    polls: &[Poll],
    params: &ModelParams,
    as_of: jiff::civil::Date,
) -> Result<Forecast, ModelError> {
    if params.simulations == 0 {
        return Err(ModelError::InvalidParameters("simulations must be positive".into()));
    }
    if params.round1_error.student_t_df <= 2.0 {
        return Err(ModelError::InvalidParameters(
            "round1_error.student_t_df must exceed 2".into(),
        ));
    }
    if day_number(params.round2_date) <= day_number(params.round1_date) {
        return Err(ModelError::InvalidParameters("round 2 must follow round 1".into()));
    }
    let round1 = fit_round1(field, polls, &params.aggregation, as_of, params.seed)?;
    let round2 = fit_round2(
        field,
        polls,
        &params.round2,
        params.aggregation.share_floor,
        as_of,
        params.round2_date,
    )?;
    let transfers = Transfers::new(&field.blocs, &params.round2)?;

    let layout = &round1.layout;
    let n = field.len();
    let simulated: Vec<bool> = round1.first_polled.iter().map(Option::is_some).collect();
    let p_run = field.p_run();
    let mut warnings = Vec::new();
    for c in 0..n {
        if !simulated[c] && p_run[c] > 0.0 {
            warnings.push(format!(
                "{} has p_run {:.2} but no round 1 poll has tested them; left out of the simulation",
                field.candidates[c].id, p_run[c]
            ));
        }
    }

    let election = round1.vote_state_at(params.round1_date);
    let sampler = election.sampler()?;
    let mut rng_field = Rng::derive(params.seed, "field");
    let mut rng_state = Rng::derive(params.seed, "state");
    let mut rng_error = Rng::derive(params.seed, "round1-error");
    let mut rng_round2 = Rng::derive(params.seed, "round2");

    let dim = layout.vote_dim();
    let mut x = vec![0.0; dim];
    let mut normals = vec![0.0; dim];
    let mut present = vec![false; n];
    let mut bloc_error = vec![0.0; layout.n_blocs];
    let mut candidate_error = vec![0.0; n];
    let mut shares = vec![0.0; n];
    let mut scratch = ShareScratch::new(layout);

    let mut qualify = vec![0u32; n];
    let mut wins = vec![0u32; n];
    let mut ran_shares: Vec<Vec<f64>> = vec![Vec::new(); n];
    let mut matchups = vec![0u32; n * n];
    let mut matchup_wins = vec![0u32; n * n];
    let error = &params.round1_error;

    for _ in 0..params.simulations {
        let mut attempts = 0;
        loop {
            field.draw(&mut rng_field, &mut present);
            for c in 0..n {
                present[c] &= simulated[c];
            }
            if present.iter().filter(|p| **p).count() >= 2 {
                break;
            }
            attempts += 1;
            if attempts == MAX_FIELD_DRAWS {
                return Err(ModelError::NotEnoughData(
                    "the field rarely has two polled candidates; check config/candidates.yaml".into(),
                ));
            }
        }
        sampler.draw(&mut rng_state, &mut normals, &mut x);
        for e in bloc_error.iter_mut() {
            *e = error.bloc_sd * rng_error.student_t_unit(error.student_t_df);
        }
        for e in candidate_error.iter_mut() {
            *e = error.candidate_sd * rng_error.student_t_unit(error.student_t_df);
        }
        let errors = Errors {
            bloc: &bloc_error,
            candidate: &candidate_error,
        };
        round1_shares(layout, &x, &present, Some(&errors), &mut scratch, &mut shares);

        let (first, second) = top_two(&shares, &present);
        qualify[first] += 1;
        qualify[second] += 1;
        for c in 0..n {
            if present[c] {
                ran_shares[c].push(shares[c]);
            }
        }

        let (a, b) = (first.min(second), first.max(second));
        let transfer = transfers.share(field, a, b, &shares).clamp(1e-6, 1.0 - 1e-6);
        let transfer_logit = (transfer / (1.0 - transfer)).ln();
        let transfer_var = params.round2.transfer_sd.powi(2);
        let (mean, var) = match round2.lookup(a, b) {
            Some((polled, polled_var)) => {
                let precision = 1.0 / transfer_var + 1.0 / polled_var;
                (
                    (transfer_logit / transfer_var + polled / polled_var) / precision,
                    1.0 / precision,
                )
            }
            None => (transfer_logit, transfer_var),
        };
        let sd = (var + params.round2.error_sd.powi(2)).sqrt();
        let a_wins = mean + sd * rng_round2.student_t_unit(error.student_t_df) > 0.0;
        let winner = if a_wins { a } else { b };
        wins[winner] += 1;
        matchups[a * n + b] += 1;
        if a_wins {
            matchup_wins[a * n + b] += 1;
        }
    }

    let total = f64::from(params.simulations);
    let candidates = (0..n)
        .map(|c| {
            let values = &mut ran_shares[c];
            values.sort_by(f64::total_cmp);
            let r1_share = (!values.is_empty()).then(|| Interval {
                mean: values.iter().sum::<f64>() / values.len() as f64,
                lo80: quantile(values, 0.1),
                hi80: quantile(values, 0.9),
            });
            CandidateForecast {
                candidate: c,
                p_run: p_run[c],
                simulated: simulated[c],
                p_qualify_r1: f64::from(qualify[c]) / total,
                p_win: f64::from(wins[c]) / total,
                r1_share,
            }
        })
        .collect();
    let mut pairs: Vec<PairForecast> = (0..n)
        .flat_map(|a| ((a + 1)..n).map(move |b| (a, b)))
        .filter(|(a, b)| matchups[a * n + b] > 0)
        .map(|(a, b)| PairForecast {
            a,
            b,
            p_matchup: f64::from(matchups[a * n + b]) / total,
            p_a_wins: f64::from(matchup_wins[a * n + b]) / f64::from(matchups[a * n + b]),
            polled: round2.lookup(a, b).is_some(),
        })
        .collect();
    pairs.sort_by(|x, y| y.p_matchup.total_cmp(&x.p_matchup).then((x.a, x.b).cmp(&(y.a, y.b))));

    Ok(Forecast {
        as_of,
        simulations: params.simulations,
        seed: params.seed,
        candidates,
        pairs,
        round1,
        round2,
        warnings,
    })
}

fn top_two(shares: &[f64], present: &[bool]) -> (usize, usize) {
    let mut first = usize::MAX;
    let mut second = usize::MAX;
    for (c, share) in shares.iter().enumerate() {
        if !present[c] {
            continue;
        }
        if first == usize::MAX || *share > shares[first] {
            second = first;
            first = c;
        } else if second == usize::MAX || *share > shares[second] {
            second = c;
        }
    }
    (first, second)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::{
        AggregationParams, ErrorParams, Round2Params, Scenario, Share,
        field::tests::{candidate, option, slot},
    };
    use jiff::civil::date;

    fn field() -> Field {
        Field::new(
            vec!["left".into(), "centre".into(), "right".into()],
            vec![
                candidate("l1", "left"),
                candidate("l2", "left"),
                candidate("c1", "centre"),
                candidate("r1", "right"),
                candidate("r2", "right"),
                candidate("ghost", "centre"),
            ],
            &[
                slot("left", vec![option(&["l1", "l2"], 0.5), option(&["l1"], 0.4)]),
                slot("c1", vec![option(&["c1"], 0.95)]),
                slot("right", vec![option(&["r1"], 0.7), option(&["r2"], 0.3)]),
                slot("ghost", vec![option(&["ghost"], 0.5)]),
            ],
        )
        .unwrap()
    }

    fn params(simulations: u32, seed: u64) -> ModelParams {
        let row = |left: f64, centre: f64, right: f64, abstain: f64| {
            BTreeMap::from([
                ("left".to_string(), left),
                ("centre".to_string(), centre),
                ("right".to_string(), right),
                ("abstain".to_string(), abstain),
            ])
        };
        ModelParams {
            model_version: "test".into(),
            seed,
            simulations,
            round1_date: date(2027, 4, 18),
            round2_date: date(2027, 5, 2),
            aggregation: AggregationParams {
                random_walk_sd_grid: vec![0.005, 0.01],
                design_effect_grid: vec![1.5],
                prior_sd: 2.0,
                presence_sd: 0.3,
                house_sd: 0.08,
                share_floor: 0.5,
                series_step_days: 14,
                series_draws: 100,
            },
            round1_error: ErrorParams {
                student_t_df: 5.0,
                bloc_sd: 0.1,
                candidate_sd: 0.08,
            },
            round2: Round2Params {
                random_walk_sd: 0.01,
                design_effect: 1.5,
                prior_sd: 1.5,
                error_sd: 0.08,
                transfer_sd: 0.35,
                loyalty: 0.97,
                transfers: BTreeMap::from([
                    ("left".to_string(), row(1.0, 0.5, 0.1, 0.5)),
                    ("centre".to_string(), row(0.4, 1.0, 0.6, 0.3)),
                    ("right".to_string(), row(0.1, 0.5, 1.0, 0.4)),
                ]),
            },
        }
    }

    fn polls() -> Vec<Poll> {
        let scenario = |id: &str, round: u8, shares: &[(&str, f64)]| Scenario {
            scenario_id: id.into(),
            round,
            shares: shares
                .iter()
                .map(|(c, s)| Share {
                    candidate_id: (*c).into(),
                    share: *s,
                })
                .collect(),
        };
        (0..8)
            .map(|i| Poll {
                poll_id: format!("p{i}"),
                firm: ["a", "b"][i % 2].into(),
                field_start: date(2026, 6, 1 + i as i8 * 3),
                field_end: date(2026, 6, 2 + i as i8 * 3),
                published_at: date(2026, 6, 3 + i as i8 * 3),
                sample_size: 1200,
                scenarios: vec![
                    scenario("a", 1, &[("l1", 20.0), ("l2", 8.0), ("c1", 24.0), ("r1", 33.0)]),
                    scenario("b", 1, &[("l1", 26.0), ("c1", 26.0), ("r2", 31.0), ("r1", 17.0)]),
                    scenario("c", 2, &[("r1", 53.0), ("c1", 47.0)]),
                ],
            })
            .collect()
    }

    #[test]
    fn probabilities_are_coherent() {
        let forecast = run_forecast(&field(), &polls(), &params(4000, 7), date(2026, 7, 1)).unwrap();
        let p_win: f64 = forecast.candidates.iter().map(|c| c.p_win).sum();
        let p_qualify: f64 = forecast.candidates.iter().map(|c| c.p_qualify_r1).sum();
        let p_matchup: f64 = forecast.pairs.iter().map(|p| p.p_matchup).sum();
        assert!((p_win - 1.0).abs() < 1e-9);
        assert!((p_qualify - 2.0).abs() < 1e-9);
        assert!((p_matchup - 1.0).abs() < 1e-9);
        for c in &forecast.candidates {
            assert!(c.p_win <= c.p_qualify_r1 + 1e-12);
            assert!(c.p_qualify_r1 <= c.p_run + 0.02);
            if let Some(share) = &c.r1_share {
                assert!(share.lo80 <= share.mean && share.mean <= share.hi80);
            }
        }
        // The never-polled candidate is reported but never simulated.
        let ghost = &forecast.candidates[5];
        assert!(!ghost.simulated && ghost.p_qualify_r1 == 0.0 && ghost.r1_share.is_none());
        assert_eq!(forecast.warnings.len(), 1);
        // The polled matchup is marked as polled.
        let c1_r1 = forecast.pairs.iter().find(|p| p.a == 2 && p.b == 3).unwrap();
        assert!(c1_r1.polled);
    }

    #[test]
    fn deterministic_with_seed() {
        let a = run_forecast(&field(), &polls(), &params(2000, 11), date(2026, 7, 1)).unwrap();
        let b = run_forecast(&field(), &polls(), &params(2000, 11), date(2026, 7, 1)).unwrap();
        let c = run_forecast(&field(), &polls(), &params(2000, 12), date(2026, 7, 1)).unwrap();
        let wins = |f: &Forecast| f.candidates.iter().map(|c| c.p_win).collect::<Vec<_>>();
        assert_eq!(wins(&a), wins(&b));
        assert_ne!(wins(&a), wins(&c));
    }
}
