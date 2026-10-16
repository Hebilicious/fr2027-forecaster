//! The market blend's weight, fitted on the replays of elections with market history
//! (`<dir>/markets.csv`): each replay's model probabilities, pooled with the last market prices
//! published by its date ([`fr2027_model::blend`]) at every weight on the grid, scored by the
//! log-loss of who qualifies and who wins.
//!
//! The rule is the backtest's: among the weights that do at least as well as the model alone
//! (weight 1) on every election with market history, take the one with the lowest total log-loss;
//! with none, keep 1.

use std::collections::BTreeMap;
use std::path::Path;

use fr2027_collectors::read_csv;
use fr2027_model::blend::{BlendParams, WinPrices, pool_binary, pool_win};
use jiff::civil::Date;
use serde::{Deserialize, Serialize};

use crate::{
    Error, Result,
    election::Election,
    score::{self, ReplayScore},
};

pub const MARKETS_FILE: &str = "markets.csv";

/// A row of `<dir>/markets.csv`: one contract's price on one day.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MarketRow {
    pub date: Date,
    pub venue: String,
    /// `win` or `qualify`.
    pub question: String,
    /// The candidate, or empty for a contract on someone outside the backtest's field.
    pub candidate_id: Option<String>,
    pub label: String,
    pub price: f64,
    pub source_url: String,
}

/// Reads `<dir>/markets.csv`; none when the election has no market history.
pub fn load(dir: &Path, known: &dyn Fn(&str) -> bool) -> Result<Vec<MarketRow>> {
    let path = dir.join(MARKETS_FILE);
    if !path.exists() {
        return Ok(Vec::new());
    }
    let rows: Vec<MarketRow> = read_csv(&path)?;
    for row in &rows {
        if let Some(id) = &row.candidate_id
            && !known(id)
        {
            return Err(Error::invalid(&path, format!("unknown candidate `{id}`")));
        }
        if !matches!(row.question.as_str(), "win" | "qualify") || !(0.0..=1.0).contains(&row.price) {
            return Err(Error::invalid(
                &path,
                format!("{} {}: bad question or price", row.date, row.label),
            ));
        }
    }
    Ok(rows)
}

/// Prices older than this, on a replay date, are not used.
const MAX_AGE_DAYS: i32 = 7;

/// The market's view on `as_of`: for each venue and contract its last price within
/// [`MAX_AGE_DAYS`]; for who wins, each venue's prices scaled to sum to 1 with everyone outside
/// the field, then averaged over venues; for reaching round 2, averaged as they are.
fn market_on(rows: &[MarketRow], election: &Election, as_of: Date) -> Option<(WinPrices, Vec<Option<f64>>)> {
    let n = election.field.len();
    let first_day = fr2027_model::day_number(as_of) - MAX_AGE_DAYS;
    let mut last: BTreeMap<(&str, &str, &str), &MarketRow> = BTreeMap::new();
    for row in rows
        .iter()
        .filter(|r| r.date <= as_of && fr2027_model::day_number(r.date) >= first_day)
    {
        let key = (row.venue.as_str(), row.question.as_str(), row.label.as_str());
        if last.get(&key).is_none_or(|r| r.date <= row.date) {
            last.insert(key, row);
        }
    }
    if last.is_empty() {
        return None;
    }
    let index = |row: &MarketRow| row.candidate_id.as_deref().and_then(|id| election.field.index_of(id));
    let venues: Vec<&str> = {
        let mut v: Vec<&str> = last.keys().map(|(venue, _, _)| *venue).collect();
        v.dedup();
        v
    };
    let mut win_sum = vec![(0.0, 0usize); n];
    let (mut others, mut win_venues) = (0.0, 0usize);
    let mut qualify_sum = vec![(0.0, 0usize); n];
    for venue in venues {
        let of = |question: &str| -> Vec<&MarketRow> {
            last.iter()
                .filter(|((v, q, _), _)| *v == venue && *q == question)
                .map(|(_, r)| *r)
                .collect()
        };
        let win = of("win");
        let total: f64 = win.iter().map(|r| r.price).sum();
        if total > 0.0 {
            win_venues += 1;
            for row in win {
                match index(row) {
                    Some(c) => {
                        win_sum[c].0 += row.price / total;
                        win_sum[c].1 += 1;
                    }
                    None => others += row.price / total,
                }
            }
        }
        for row in of("qualify") {
            if let Some(c) = index(row) {
                qualify_sum[c].0 += row.price;
                qualify_sum[c].1 += 1;
            }
        }
    }
    let average = |sums: &[(f64, usize)]| -> Vec<Option<f64>> {
        sums.iter().map(|(s, k)| (*k > 0).then(|| s / *k as f64)).collect()
    };
    Some((
        WinPrices {
            prices: average(&win_sum),
            others: if win_venues > 0 {
                others / win_venues as f64
            } else {
                0.0
            },
        },
        average(&qualify_sum),
    ))
}

#[derive(Clone, Debug, Serialize)]
pub struct BlendFit {
    /// Weights on the model tried.
    pub grid: Vec<WeightScore>,
    /// The weight the rule picks (1 when none beats the model alone).
    pub fitted: f64,
    /// Elections with market history, and their replays with a price by their date.
    pub replays_by_year: BTreeMap<i16, usize>,
}

#[derive(Clone, Debug, Serialize)]
pub struct WeightScore {
    pub model_weight: f64,
    /// Mean log-loss of who qualifies and who wins, by election.
    pub qualify_log_loss: BTreeMap<i16, f64>,
    pub win_log_loss: BTreeMap<i16, f64>,
    pub total: f64,
}

/// One replay with a market price by its date: the model's scores, the markets' view of who wins,
/// and their prices of reaching round 2.
type Priced<'a> = (&'a ReplayScore, WinPrices, Vec<Option<f64>>);

/// Fits the blend weight on `replays` (the model at its current settings), against the market
/// history of each election.
pub fn fit(elections: &[Election], replays: &[ReplayScore], weights: &[f64], floor: f64) -> Option<BlendFit> {
    // Per election: each replay's (model scores, market view).
    let mut by_year: BTreeMap<i16, Vec<Priced>> = BTreeMap::new();
    for replay in replays {
        let election = elections.iter().find(|e| e.year == replay.year)?;
        if let Some((win, qualify)) = market_on(&election.markets, election, replay.as_of) {
            by_year.entry(replay.year).or_default().push((replay, win, qualify));
        }
    }
    if by_year.is_empty() {
        return None;
    }
    let mut grid: Vec<WeightScore> = weights
        .iter()
        .map(|&w| {
            let params = BlendParams {
                model_weight: w,
                probability_floor: floor,
                max_age_hours: f64::from(MAX_AGE_DAYS * 24),
            };
            let mut qualify_log_loss = BTreeMap::new();
            let mut win_log_loss = BTreeMap::new();
            for (year, items) in &by_year {
                let election = elections.iter().find(|e| e.year == *year).expect("loaded");
                let (mut q, mut v) = (Vec::new(), Vec::new());
                for (replay, win, qualify) in items {
                    // The model's probabilities over the whole field (0 for non-runners).
                    let mut p_win = vec![0.0; election.field.len()];
                    for c in &replay.candidates {
                        if let Some(i) = election.field.index_of(&c.candidate_id) {
                            p_win[i] = c.p_win;
                        }
                    }
                    let pooled = pool_win(&p_win, win, &params);
                    for c in &replay.candidates {
                        let i = election.field.index_of(&c.candidate_id).expect("runner in field");
                        v.push(score::log_loss(pooled[i], c.won, floor));
                        q.push(score::log_loss(
                            pool_binary(c.p_qualify, qualify[i], &params),
                            c.qualified,
                            floor,
                        ));
                    }
                }
                let mean = |x: &[f64]| x.iter().sum::<f64>() / x.len().max(1) as f64;
                qualify_log_loss.insert(*year, mean(&q));
                win_log_loss.insert(*year, mean(&v));
            }
            let total = qualify_log_loss.values().sum::<f64>() + win_log_loss.values().sum::<f64>();
            WeightScore {
                model_weight: w,
                qualify_log_loss,
                win_log_loss,
                total,
            }
        })
        .collect();
    grid.sort_by(|a, b| a.model_weight.total_cmp(&b.model_weight));
    let model_alone = grid.iter().find(|g| g.model_weight == 1.0)?.clone();
    let at_least = |g: &WeightScore| {
        g.qualify_log_loss
            .iter()
            .all(|(y, l)| *l <= model_alone.qualify_log_loss[y])
            && g.win_log_loss.iter().all(|(y, l)| *l <= model_alone.win_log_loss[y])
    };
    let fitted = grid
        .iter()
        .filter(|g| at_least(g))
        .min_by(|a, b| a.total.total_cmp(&b.total))
        .map_or(1.0, |g| g.model_weight);
    Some(BlendFit {
        grid,
        fitted,
        replays_by_year: by_year.iter().map(|(y, items)| (*y, items.len())).collect(),
    })
}
