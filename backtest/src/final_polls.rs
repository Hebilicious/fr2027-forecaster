//! Final-week poll averages of earlier elections (2002–2012), as evidence for the election-day
//! error beyond the two replayed elections. Each average is treated as the model's election-day
//! state with no uncertainty left but the election-day error itself, which is drawn exactly as
//! the simulation draws it (Student-t on each bloc's log strength and on each candidate's
//! within-bloc support); the realised result is then scored like a replay's.

use std::collections::BTreeMap;
use std::path::Path;

use fr2027_collectors::Repo;
use fr2027_model::{
    ErrorParams,
    rng::Rng,
    round1::{Errors, Layout, ShareScratch, round1_shares},
};
use serde::{Deserialize, Serialize};

use crate::{
    Error, Result,
    config::BacktestConfig,
    score::{kde_log_density, logit, student_t_log_pdf},
};

/// One election's round 1: the candidates listed in the file, their bloc, final-week poll
/// average and result (percentages).
#[derive(Clone, Debug)]
pub struct Round1Election {
    pub year: i16,
    pub names: Vec<String>,
    pub bloc_of: Vec<usize>,
    pub poll_average: Vec<f64>,
    pub result: Vec<f64>,
}

/// One runoff: the winner's share of the two in the final-week poll average and in the result.
#[derive(Clone, Debug, Serialize)]
pub struct Round2Election {
    pub year: i16,
    pub winner: String,
    pub loser: String,
    pub poll_average: f64,
    pub result: f64,
}

#[derive(Debug, Deserialize)]
struct Round1Row {
    year: i16,
    candidate_name: String,
    result: f64,
    final_poll_avg: f64,
}

#[derive(Debug, Deserialize)]
struct Round2Row {
    year: i16,
    winner_name: String,
    winner_result: f64,
    loser_name: String,
    loser_result: f64,
    final_poll_avg_winner: f64,
}

fn read_csv<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<Vec<T>> {
    let bytes = std::fs::read(path).map_err(|e| Error::invalid(path, e.to_string()))?;
    csv::Reader::from_reader(bytes.as_slice())
        .deserialize()
        .collect::<Result<Vec<T>, _>>()
        .map_err(|e| Error::invalid(path, e.to_string()))
}

pub fn load_round1(repo: &Repo, config: &BacktestConfig, blocs: &[String]) -> Result<Vec<Round1Election>> {
    let path = repo.root().join(&config.final_polls.round1_file);
    let rows: Vec<Round1Row> = read_csv(&path)?;
    let mut by_year: BTreeMap<i16, Round1Election> = BTreeMap::new();
    for row in rows.iter().filter(|r| config.final_polls.years.contains(&r.year)) {
        let bloc = config
            .final_polls
            .blocs
            .get(&row.candidate_name)
            .and_then(|b| blocs.iter().position(|x| x == b))
            .ok_or_else(|| {
                Error::invalid(
                    &path,
                    format!("no known bloc for `{}` in config/backtest.yaml", row.candidate_name),
                )
            })?;
        let election = by_year.entry(row.year).or_insert_with(|| Round1Election {
            year: row.year,
            names: Vec::new(),
            bloc_of: Vec::new(),
            poll_average: Vec::new(),
            result: Vec::new(),
        });
        election.names.push(row.candidate_name.clone());
        election.bloc_of.push(bloc);
        election.poll_average.push(row.final_poll_avg);
        election.result.push(row.result);
    }
    Ok(by_year.into_values().collect())
}

pub fn load_round2(repo: &Repo, config: &BacktestConfig) -> Result<Vec<Round2Election>> {
    let path = repo.root().join(&config.final_polls.round2_file);
    let rows: Vec<Round2Row> = read_csv(&path)?;
    Ok(rows
        .into_iter()
        .filter(|r| config.final_polls.years.contains(&r.year))
        .map(|r| Round2Election {
            year: r.year,
            winner: r.winner_name,
            loser: r.loser_name,
            poll_average: r.final_poll_avg_winner / 100.0,
            result: r.winner_result / (r.winner_result + r.loser_result),
        })
        .collect())
}

/// Sum, over the candidates whose poll average reaches `contender_share`, of the log density of
/// their realised log share (both renormalised over the listed candidates), with the election-day
/// error drawn `simulations` times. The random numbers depend on the seed and the year only, so
/// every error setting sees the same draws.
pub fn round1_log_score(
    election: &Round1Election,
    blocs: usize,
    error: &ErrorParams,
    simulations: u32,
    seed: u64,
    contender_share: f64,
) -> f64 {
    let k = election.names.len();
    let layout = Layout {
        n_blocs: blocs,
        n_candidates: k,
        firms: Vec::new(),
        bloc_of: election.bloc_of.clone(),
    };
    // The state whose shares are the poll average: bloc strengths are log bloc totals, supports
    // are log shares, presence effects are zero.
    let mut x = vec![0.0; layout.vote_dim()];
    for b in 0..blocs {
        let total: f64 = (0..k)
            .filter(|c| election.bloc_of[*c] == b)
            .map(|c| election.poll_average[c])
            .sum();
        x[layout.beta(b)] = if total > 0.0 { total.ln() } else { 0.0 };
    }
    for c in 0..k {
        x[layout.alpha(c)] = election.poll_average[c].ln();
    }
    let present = vec![true; k];
    let result_total: f64 = election.result.iter().sum();
    let contenders: Vec<usize> = (0..k)
        .filter(|c| election.poll_average[*c] / 100.0 >= contender_share)
        .collect();
    let mut rng = Rng::derive(seed, &format!("final-polls:{}", election.year));
    let mut bloc_error = vec![0.0; blocs];
    let mut candidate_error = vec![0.0; k];
    let mut shares = vec![0.0; k];
    let mut scratch = ShareScratch::new(&layout);
    let mut samples: Vec<Vec<f64>> = vec![Vec::with_capacity(simulations as usize); k];
    for _ in 0..simulations {
        for e in bloc_error.iter_mut() {
            *e = error.bloc_sd * rng.student_t_unit(error.student_t_df);
        }
        for e in candidate_error.iter_mut() {
            *e = error.candidate_sd * rng.student_t_unit(error.student_t_df);
        }
        let errors = Errors {
            bloc: &bloc_error,
            candidate: &candidate_error,
        };
        round1_shares(&layout, &x, &present, Some(&errors), &mut scratch, &mut shares);
        for &c in &contenders {
            samples[c].push(shares[c].ln());
        }
    }
    contenders
        .iter()
        .map(|&c| kde_log_density(&mut samples[c], (election.result[c] / result_total).ln()))
        .sum()
}

/// Log density of the realised logit share of the winner, under the model's Student-t
/// election-day error centred on the final-week average.
pub fn round2_log_score(election: &Round2Election, error_sd: f64, df: f64) -> f64 {
    student_t_log_pdf(logit(election.result), logit(election.poll_average), error_sd, df)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn election() -> Round1Election {
        Round1Election {
            year: 2000,
            names: vec!["a".into(), "b".into(), "c".into(), "d".into()],
            bloc_of: vec![0, 0, 1, 2],
            poll_average: vec![30.0, 10.0, 25.0, 35.0],
            result: vec![30.0, 10.0, 25.0, 35.0],
        }
    }

    #[test]
    fn exact_polls_favour_small_errors_and_misses_favour_larger_ones() {
        let error = |sd: f64| ErrorParams {
            student_t_df: 5.0,
            bloc_sd: sd,
            candidate_sd: sd,
        };
        let exact = election();
        let small = round1_log_score(&exact, 3, &error(0.03), 5000, 1, 0.05);
        let large = round1_log_score(&exact, 3, &error(0.2), 5000, 1, 0.05);
        assert!(small > large, "{small} vs {large}");

        let mut missed = election();
        missed.result = vec![24.0, 12.0, 26.0, 38.0];
        let small = round1_log_score(&missed, 3, &error(0.03), 5000, 1, 0.05);
        let large = round1_log_score(&missed, 3, &error(0.2), 5000, 1, 0.05);
        assert!(large > small, "{small} vs {large}");
    }

    #[test]
    fn round2_score_peaks_near_the_miss() {
        let election = Round2Election {
            year: 2000,
            winner: "a".into(),
            loser: "b".into(),
            poll_average: 0.55,
            result: 0.60,
        };
        let miss = logit(0.60) - logit(0.55);
        let scores: Vec<f64> = [0.05, miss, 0.5]
            .iter()
            .map(|sd| round2_log_score(&election, *sd, 5.0))
            .collect();
        assert!(scores[1] > scores[0] && scores[1] > scores[2], "{scores:?}");
    }
}
