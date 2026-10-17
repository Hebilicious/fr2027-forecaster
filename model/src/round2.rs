//! Round 2.
//!
//! A matchup that pollsters have tested is aggregated directly: a local-level Kalman filter on
//! the logit of one finalist's share of the two, projected to election day. A matchup nobody
//! has tested is anchored, when it can be, on a tested one with a candidate of the same bloc in
//! their place, shifted by the gap between the two candidates: measured where polls tested both
//! against a common opponent, otherwise taken from history. Any matchup can also be built from
//! the simulated round 1 vote with a bloc transfer table (each round 1 electorate splits between
//! the two finalists and abstention by bloc affinity). The polled or anchored estimate and the
//! transfer estimate are combined by precision; without either, the transfer estimate stands
//! alone.

use std::collections::BTreeMap;

use crate::{ModelError, Poll, Round2Params, day_number, field::Field};

#[derive(Clone, Debug)]
pub struct PairFit {
    /// Lower candidate index of the pair; `mean` is the logit of `a`'s share.
    pub a: usize,
    pub b: usize,
    pub mean: f64,
    pub var: f64,
    pub polls: usize,
    pub last_field_end: jiff::civil::Date,
}

/// How much better `a` does than `b`, a candidate of the same bloc, against a common opponent:
/// the mean of logit(`a` vs C) − logit(`b` vs C) over polls that tested both against the same C.
#[derive(Clone, Debug)]
pub struct GapFit {
    pub a: usize,
    pub b: usize,
    pub mean: f64,
    /// Variance of `mean` from the polls' sampling error.
    pub var: f64,
    pub polls: usize,
}

/// An estimate for an untested matchup, from a tested one.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Anchor {
    /// Logit of the first finalist's share, and its variance.
    pub mean: f64,
    pub var: f64,
    /// The tested matchup it stands on, as (stand-in or first finalist, opponent) in the
    /// untested matchup's order.
    pub via: (usize, usize),
}

#[derive(Clone, Debug, Default)]
pub struct Round2Fit {
    pub pairs: Vec<PairFit>,
    pub gaps: Vec<GapFit>,
    /// Round 2 scenarios that tested each candidate, by candidate index.
    pub tested: Vec<usize>,
}

impl Round2Fit {
    /// The polled estimate for `a` against `b`, as (logit of `a`'s share, variance).
    pub fn lookup(&self, a: usize, b: usize) -> Option<(f64, f64)> {
        let (low, high) = if a < b { (a, b) } else { (b, a) };
        let pair = self.pairs.iter().find(|p| p.a == low && p.b == high)?;
        Some(if a == low {
            (pair.mean, pair.var)
        } else {
            (-pair.mean, pair.var)
        })
    }

    /// The gap between `a` and `b`, two candidates of the same bloc, as (mean, variance) on the
    /// logit scale: the prior from `params` (the candidate tested in more runoff polls is the
    /// stronger by `anchor_gap_mean`, give or take `anchor_gap_sd`), updated by the gaps polls
    /// measured.
    pub fn gap(&self, a: usize, b: usize, params: &Round2Params) -> (f64, f64) {
        let prior_mean = match self.tested[a].cmp(&self.tested[b]) {
            std::cmp::Ordering::Greater => params.anchor_gap_mean,
            std::cmp::Ordering::Less => -params.anchor_gap_mean,
            std::cmp::Ordering::Equal => 0.0,
        };
        let prior_var = params.anchor_gap_sd.powi(2);
        let measured = self.gaps.iter().find_map(|g| {
            if (g.a, g.b) == (a, b) {
                Some((g.mean, g.var))
            } else if (g.a, g.b) == (b, a) {
                Some((-g.mean, g.var))
            } else {
                None
            }
        });
        match measured {
            Some((mean, var)) => {
                let precision = 1.0 / prior_var + 1.0 / var;
                ((prior_mean / prior_var + mean / var) / precision, 1.0 / precision)
            }
            None => (prior_mean, prior_var),
        }
    }

    /// An estimate for an untested matchup `a` vs `b`: the tested matchup with a same-bloc
    /// candidate in place of `a` or of `b` that gives the smallest variance, shifted by the gap
    /// between the two. None when no such matchup exists.
    pub fn anchored(&self, field: &Field, a: usize, b: usize, params: &Round2Params) -> Option<Anchor> {
        let mut best: Option<Anchor> = None;
        let mut consider = |estimate: Anchor| {
            if best.is_none_or(|b| estimate.var < b.var) {
                best = Some(estimate);
            }
        };
        for other in 0..field.len() {
            if other == a || other == b {
                continue;
            }
            // `other` stands in for `a`: logit(a vs b) = logit(other vs b) + gap(a, other).
            if field.bloc_of[other] == field.bloc_of[a]
                && let Some((mean, var)) = self.lookup(other, b)
            {
                let (gap, gap_var) = self.gap(a, other, params);
                consider(Anchor {
                    mean: mean + gap,
                    var: var + gap_var,
                    via: (other, b),
                });
            }
            // `other` stands in for `b`: logit(a vs b) = logit(a vs other) - gap(b, other).
            if field.bloc_of[other] == field.bloc_of[b]
                && let Some((mean, var)) = self.lookup(a, other)
            {
                let (gap, gap_var) = self.gap(b, other, params);
                consider(Anchor {
                    mean: mean - gap,
                    var: var + gap_var,
                    via: (a, other),
                });
            }
        }
        best
    }
}

struct Observation {
    day: i32,
    logit: f64,
    var: f64,
    field_end: jiff::civil::Date,
}

pub fn fit_round2(
    field: &Field,
    polls: &[Poll],
    params: &Round2Params,
    share_floor: f64,
    as_of: jiff::civil::Date,
    election: jiff::civil::Date,
) -> Result<Round2Fit, ModelError> {
    let mut by_pair: BTreeMap<(usize, usize), Vec<Observation>> = BTreeMap::new();
    let mut tested = vec![0usize; field.len()];
    // Per same-bloc pair (lower index first): the gaps polls measured, with their variances.
    let mut gap_observations: BTreeMap<(usize, usize), Vec<(f64, f64)>> = BTreeMap::new();
    for poll in polls.iter().filter(|p| p.published_at <= as_of) {
        let invalid = |message: String| ModelError::InvalidPoll {
            poll: poll.poll_id.clone(),
            message,
        };
        let scenarios: Vec<_> = poll.scenarios.iter().filter(|s| s.round == 2).collect();
        // This poll's matchups, each candidate against each opponent: (logit, variance).
        let mut matchups: BTreeMap<(usize, usize), (f64, f64)> = BTreeMap::new();
        for scenario in &scenarios {
            let [first, second] = scenario.shares.as_slice() else {
                return Err(invalid(format!(
                    "round 2 scenario `{}` must have exactly two candidates",
                    scenario.scenario_id
                )));
            };
            let resolve = |id: &str| {
                field
                    .index_of(id)
                    .ok_or_else(|| invalid(format!("unknown candidate `{id}`")))
            };
            let (i, j) = (resolve(&first.candidate_id)?, resolve(&second.candidate_id)?);
            if i == j {
                return Err(invalid("a round 2 scenario pits a candidate against themself".into()));
            }
            let (a, share_a, share_b) = if i < j {
                (i, first.share, second.share)
            } else {
                (j, second.share, first.share)
            };
            let b = i.max(j);
            let (share_a, share_b) = (share_a.max(share_floor), share_b.max(share_floor));
            let p = share_a / (share_a + share_b);
            let var = params.design_effect * scenarios.len() as f64 / (f64::from(poll.sample_size) * p * (1.0 - p));
            let logit = (p / (1.0 - p)).ln();
            by_pair.entry((a, b)).or_default().push(Observation {
                day: poll.midpoint(),
                logit,
                var,
                field_end: poll.field_end,
            });
            tested[a] += 1;
            tested[b] += 1;
            matchups.insert((a, b), (logit, var));
            matchups.insert((b, a), (-logit, var));
        }
        for (&(x, opponent), &(logit_x, var_x)) in &matchups {
            for (&(y, other_opponent), &(logit_y, var_y)) in &matchups {
                if x < y && opponent == other_opponent && field.bloc_of[x] == field.bloc_of[y] {
                    gap_observations
                        .entry((x, y))
                        .or_default()
                        .push((logit_x - logit_y, var_x + var_y));
                }
            }
        }
    }
    let gaps = gap_observations
        .into_iter()
        .map(|((a, b), observations)| {
            let precision: f64 = observations.iter().map(|(_, var)| 1.0 / var).sum();
            GapFit {
                a,
                b,
                mean: observations.iter().map(|(gap, var)| gap / var).sum::<f64>() / precision,
                var: 1.0 / precision,
                polls: observations.len(),
            }
        })
        .collect();
    let daily = params.random_walk_sd * params.random_walk_sd;
    let pairs = by_pair
        .into_iter()
        .map(|((a, b), mut observations)| {
            observations.sort_by_key(|o| o.day);
            let mut mean = 0.0;
            let mut var = params.prior_sd * params.prior_sd;
            let mut day = observations[0].day;
            for o in &observations {
                var += daily * f64::from(o.day - day);
                day = o.day;
                let gain = var / (var + o.var);
                mean += gain * (o.logit - mean);
                var *= 1.0 - gain;
            }
            var += daily * f64::from(day_number(election) - day);
            PairFit {
                a,
                b,
                mean,
                var,
                polls: observations.len(),
                last_field_end: observations.iter().map(|o| o.field_end).max().expect("non-empty"),
            }
        })
        .collect();
    Ok(Round2Fit { pairs, gaps, tested })
}

/// The bloc transfer table, resolved against the field's blocs.
#[derive(Clone, Debug)]
pub struct Transfers {
    affinity: Vec<Vec<f64>>,
    abstain: Vec<f64>,
    loyalty: f64,
}

impl Transfers {
    pub fn new(blocs: &[String], params: &Round2Params) -> Result<Self, ModelError> {
        let invalid = |message: String| Err(ModelError::InvalidParameters(message));
        if !(0.0..=1.0).contains(&params.loyalty) {
            return invalid(format!("loyalty {} is outside [0, 1]", params.loyalty));
        }
        let mut affinity = Vec::with_capacity(blocs.len());
        let mut abstain = Vec::with_capacity(blocs.len());
        for voter in blocs {
            let Some(row) = params.transfers.get(voter) else {
                return invalid(format!("transfers has no row for bloc `{voter}`"));
            };
            let mut weights = Vec::with_capacity(blocs.len());
            for finalist in blocs {
                match row.get(finalist) {
                    Some(w) if *w >= 0.0 => weights.push(*w),
                    _ => return invalid(format!("transfers `{voter}` → `{finalist}` is missing or negative")),
                }
            }
            match row.get("abstain") {
                Some(w) if *w >= 0.0 => abstain.push(*w),
                _ => return invalid(format!("transfers `{voter}` has no non-negative `abstain` weight")),
            }
            if let Some(extra) = row.keys().find(|k| *k != "abstain" && !blocs.contains(k)) {
                return invalid(format!("transfers `{voter}` names unknown bloc `{extra}`"));
            }
            affinity.push(weights);
        }
        if let Some(extra) = params.transfers.keys().find(|k| !blocs.contains(k)) {
            return invalid(format!("transfers has a row for unknown bloc `{extra}`"));
        }
        Ok(Self {
            affinity,
            abstain,
            loyalty: params.loyalty,
        })
    }

    /// `a`'s share of the two finalists' round 2 vote, given round 1 shares.
    pub fn share(&self, field: &Field, a: usize, b: usize, shares: &[f64]) -> f64 {
        let (bloc_a, bloc_b) = (field.bloc_of[a], field.bloc_of[b]);
        let mut votes_a = shares[a] * self.loyalty;
        let mut votes_b = shares[b] * self.loyalty;
        for (c, share) in shares.iter().enumerate() {
            if c == a || c == b || *share <= 0.0 {
                continue;
            }
            let row = &self.affinity[field.bloc_of[c]];
            let (to_a, to_b) = if bloc_a == bloc_b {
                (row[bloc_a] / 2.0, row[bloc_b] / 2.0)
            } else {
                (row[bloc_a], row[bloc_b])
            };
            let total = to_a + to_b + self.abstain[field.bloc_of[c]];
            if total > 0.0 {
                votes_a += share * to_a / total;
                votes_b += share * to_b / total;
            }
        }
        if votes_a + votes_b > 0.0 {
            votes_a / (votes_a + votes_b)
        } else {
            0.5
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Scenario, Share,
        field::tests::{candidate, option, slot},
    };
    use jiff::civil::date;

    fn field() -> Field {
        Field::new(
            vec!["left".into(), "right".into()],
            vec![
                candidate("l", "left"),
                candidate("r", "right"),
                candidate("r2", "right"),
            ],
            &[
                slot("l", vec![option(&["l"], 1.0)]),
                slot("r", vec![option(&["r"], 1.0)]),
                slot("r2", vec![option(&["r2"], 1.0)]),
            ],
        )
        .unwrap()
    }

    fn params() -> Round2Params {
        let row = |left: f64, right: f64, abstain: f64| {
            BTreeMap::from([
                ("left".to_string(), left),
                ("right".to_string(), right),
                ("abstain".to_string(), abstain),
            ])
        };
        Round2Params {
            random_walk_sd: 0.01,
            design_effect: 1.5,
            prior_sd: 1.5,
            error_sd: 0.08,
            horizon_error_sd: 0.0,
            student_t_df: 5.0,
            transfer_sd: 0.35,
            anchor_gap_mean: 0.1,
            anchor_gap_sd: 0.2,
            loyalty: 1.0,
            transfers: BTreeMap::from([
                ("left".to_string(), row(1.0, 0.0, 0.0)),
                ("right".to_string(), row(0.25, 0.5, 0.25)),
            ]),
        }
    }

    #[test]
    fn transfers_follow_affinity() {
        let field = field();
        let transfers = Transfers::new(&field.blocs, &params()).unwrap();
        // l 40, r 35, r2 25: r2's voters split 1/4 l, 1/2 r, 1/4 abstain.
        let share = transfers.share(&field, 0, 1, &[0.40, 0.35, 0.25]);
        let expected = (0.40 + 0.25 * 0.25) / (0.40 + 0.25 * 0.25 + 0.35 + 0.25 * 0.5);
        assert!((share - expected).abs() < 1e-12);
    }

    #[test]
    fn transfers_need_every_bloc() {
        let mut p = params();
        p.transfers.remove("right");
        assert!(Transfers::new(&field().blocs, &p).is_err());
    }

    fn duel(id: &str, day: i8, scenarios: &[(&str, f64, &str, f64)]) -> Poll {
        Poll {
            poll_id: id.into(),
            firm: "a".into(),
            field_start: date(2026, 9, day),
            field_end: date(2026, 9, day),
            published_at: date(2026, 9, day + 1),
            sample_size: 1000,
            scenarios: scenarios
                .iter()
                .enumerate()
                .map(|(i, (x, sx, y, sy))| Scenario {
                    scenario_id: format!("h{i}"),
                    round: 2,
                    shares: vec![
                        Share {
                            candidate_id: (*x).into(),
                            share: *sx,
                        },
                        Share {
                            candidate_id: (*y).into(),
                            share: *sy,
                        },
                    ],
                })
                .collect(),
        }
    }

    #[test]
    fn untested_matchup_is_anchored_on_a_same_bloc_candidate() {
        // l, r and r2 as indices 0, 1, 2; only l vs r is ever tested.
        let polls = vec![
            duel("a", 1, &[("l", 45.0, "r", 55.0)]),
            duel("b", 10, &[("l", 44.0, "r", 56.0)]),
        ];
        let params = params();
        let fit = fit_round2(&field(), &polls, &params, 0.5, date(2026, 10, 1), date(2027, 5, 2)).unwrap();
        let (polled, polled_var) = fit.lookup(0, 1).unwrap();
        // r2 was never tested, so r (more often tested) is stronger by the prior gap.
        let anchor = fit.anchored(&field(), 0, 2, &params).unwrap();
        assert!((anchor.mean - (polled + params.anchor_gap_mean)).abs() < 1e-12);
        assert!((anchor.var - (polled_var + params.anchor_gap_sd.powi(2))).abs() < 1e-12);
        assert_eq!(anchor.via, (0, 1));
        // And the other way round.
        let reverse = fit.anchored(&field(), 2, 0, &params).unwrap();
        assert!((reverse.mean + anchor.mean).abs() < 1e-12);
        assert_eq!(reverse.via, (1, 0));
        // A tested matchup is not anchored on itself, and two same-bloc finalists have no anchor.
        assert!(fit.anchored(&field(), 1, 2, &params).is_none());
    }

    #[test]
    fn a_measured_gap_outweighs_the_prior() {
        // One poll tests l against both r and r2: r2 does 10 points worse than r.
        let polls = vec![
            duel("a", 1, &[("l", 45.0, "r", 55.0), ("l", 55.0, "r2", 45.0)]),
            duel("b", 10, &[("l", 44.0, "r", 56.0), ("l", 54.0, "r2", 46.0)]),
        ];
        let fit = fit_round2(&field(), &polls, &params(), 0.5, date(2026, 10, 1), date(2027, 5, 2)).unwrap();
        assert_eq!(fit.gaps.len(), 1);
        let (gap, gap_var) = fit.gap(1, 2, &params());
        // logit(r vs l) - logit(r2 vs l) is about 0.4; the prior (0.1, equal counts so 0) pulls it
        // only a little.
        assert!(gap > 0.3 && gap < 0.41, "{gap}");
        assert!(gap_var < params().anchor_gap_sd.powi(2));
        assert_eq!(fit.gap(2, 1, &params()).0, -gap);
    }

    #[test]
    fn polled_pair_tracks_polls_and_orientation() {
        let poll = |id: &str, day: i8, l: f64, r: f64| Poll {
            poll_id: id.into(),
            firm: "a".into(),
            field_start: date(2026, 9, day),
            field_end: date(2026, 9, day),
            published_at: date(2026, 9, day + 1),
            sample_size: 1000,
            scenarios: vec![Scenario {
                scenario_id: "r2".into(),
                round: 2,
                shares: vec![
                    Share {
                        candidate_id: "r".into(),
                        share: r,
                    },
                    Share {
                        candidate_id: "l".into(),
                        share: l,
                    },
                ],
            }],
        };
        let polls = vec![
            poll("a", 1, 45.0, 55.0),
            poll("b", 10, 44.0, 56.0),
            poll("c", 20, 46.0, 54.0),
        ];
        let fit = fit_round2(&field(), &polls, &params(), 0.5, date(2026, 10, 1), date(2027, 5, 2)).unwrap();
        let (mean, var) = fit.lookup(0, 1).unwrap();
        let share = 1.0 / (1.0 + (-mean).exp());
        assert!((share - 0.45).abs() < 0.01, "{share}");
        assert!(var > 0.0);
        let (flipped, _) = fit.lookup(1, 0).unwrap();
        assert_eq!(flipped, -mean);
        assert!(fit.lookup(0, 2).is_none());
    }
}
