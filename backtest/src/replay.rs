//! A replay: the model as it would have run on a past date, fitted only to the polls published by
//! then.

use std::collections::BTreeMap;

use fr2027_model::{
    Fit, ModelParams, Poll, day_number, fit,
    round1::{ShareScratch, round1_shares},
};
use jiff::civil::Date;

use crate::{Error, Result, config::BacktestConfig, election::Election};

/// The polls a replay may see.
#[derive(Clone, Debug)]
pub struct Selection {
    pub polls: Vec<Poll>,
    /// Polls published by the replay date within the window, before thinning.
    pub published: usize,
    /// Polls left out entirely because, in each round they asked about, the same firm's
    /// fieldwork overlapped a later poll that was kept.
    pub overlapping: usize,
}

/// The polls published on or before `as_of` whose fieldwork started at most `window_days` before
/// round 1, thinned so that, for each firm and round, no two kept polls have overlapping
/// fieldwork. Rolling polls (Ifop and OpinionWay published one a day from overlapping samples in
/// 2017 and 2022) would otherwise count the same respondents several times. Going back from the
/// firm's most recent poll, a poll is kept when its fieldwork ended before the last kept one's
/// started; a poll kept for one round only loses its other round's scenarios.
pub fn select_polls(election: &Election, as_of: Date, window_days: u32) -> Selection {
    let first_day = day_number(election.round1) - window_days as i32;
    let published: Vec<&Poll> = election
        .polls
        .iter()
        .filter(|p| p.published_at <= as_of && day_number(p.field_start) >= first_day)
        .collect();
    let mut keep: BTreeMap<(&str, u8), bool> = BTreeMap::new();
    for round in [1u8, 2] {
        let mut by_firm: BTreeMap<&str, Vec<&Poll>> = BTreeMap::new();
        for poll in published
            .iter()
            .filter(|p| p.scenarios.iter().any(|s| s.round == round))
        {
            by_firm.entry(poll.firm.as_str()).or_default().push(poll);
        }
        for polls in by_firm.values_mut() {
            polls.sort_by_key(|p| std::cmp::Reverse((p.field_end, p.field_start, p.poll_id.as_str())));
            let mut earliest_kept: Option<Date> = None;
            for poll in polls.iter() {
                let kept = earliest_kept.is_none_or(|start| poll.field_end < start);
                if kept {
                    earliest_kept = Some(poll.field_start);
                }
                keep.insert((poll.poll_id.as_str(), round), kept);
            }
        }
    }
    let mut polls: Vec<Poll> = published
        .iter()
        .filter_map(|poll| {
            let scenarios: Vec<_> = poll
                .scenarios
                .iter()
                .filter(|s| keep.get(&(poll.poll_id.as_str(), s.round)).copied().unwrap_or(false))
                .cloned()
                .collect();
            (!scenarios.is_empty()).then(|| Poll {
                scenarios,
                ..(*poll).clone()
            })
        })
        .collect();
    polls.sort_by(|a, b| (a.published_at, &a.poll_id).cmp(&(b.published_at, &b.poll_id)));
    Selection {
        overlapping: published.len() - polls.len(),
        published: published.len(),
        polls,
    }
}

/// The model parameters for a replay of `election`: config/model.yaml with the election's dates
/// and the backtest's seed and number of simulations.
pub fn replay_params(model: &ModelParams, config: &BacktestConfig, election: &Election) -> ModelParams {
    ModelParams {
        seed: config.seed,
        simulations: config.simulations,
        round1_date: election.round1,
        round2_date: election.round2,
        ..model.clone()
    }
}

/// The model fitted as of one replay date.
#[derive(Clone, Debug)]
pub struct Replay {
    pub year: i16,
    pub horizon_days: u32,
    pub as_of: Date,
    pub polls_published: usize,
    pub polls_overlapping: usize,
    pub polls_used: usize,
    pub fit: Fit,
    pub params: ModelParams,
    /// Runners whose share in the fitted mean state at round 1 reaches the contender threshold.
    pub contenders: Vec<usize>,
}

pub fn prepare(election: &Election, model: &ModelParams, config: &BacktestConfig, horizon_days: u32) -> Result<Replay> {
    let as_of = fr2027_model::date_from_day_number(day_number(election.round1) - horizon_days as i32);
    let selection = select_polls(election, as_of, config.poll_window_days);
    // Leakage check: nothing published after the replay date reaches the model.
    if let Some(late) = selection.polls.iter().find(|p| p.published_at > as_of) {
        return Err(Error::invalid(
            format!("data/backtest/{}", election.year),
            format!("poll `{}` published after the replay date {as_of}", late.poll_id),
        ));
    }
    let params = replay_params(model, config, election);
    let fit = fit(&election.field, &selection.polls, &params, as_of)?;
    let contenders = contenders(election, &fit, config.contender_share);
    Ok(Replay {
        year: election.year,
        horizon_days,
        as_of,
        polls_published: selection.published,
        polls_overlapping: selection.overlapping,
        polls_used: fit.round1.polls_used,
        fit,
        params,
        contenders,
    })
}

/// Runners (who had been polled by the replay date) whose share, in the fitted state's mean
/// projected to round 1 and before any election-day error, reaches `threshold`. Depends on the
/// polls only, so every error setting scores the same candidates.
fn contenders(election: &Election, fit: &Fit, threshold: f64) -> Vec<usize> {
    let round1 = &fit.round1;
    let state = round1.vote_state_at(election.round1);
    let present: Vec<bool> = (0..election.field.len())
        .map(|c| election.ran[c] && round1.first_polled[c].is_some())
        .collect();
    let mut shares = vec![0.0; present.len()];
    let mut scratch = ShareScratch::new(&round1.layout);
    let mean: Vec<f64> = state.mean.iter().copied().collect();
    round1_shares(&round1.layout, &mean, &present, None, &mut scratch, &mut shares);
    (0..present.len())
        .filter(|c| present[*c] && shares[*c] >= threshold)
        .collect()
}
