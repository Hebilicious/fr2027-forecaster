//! The latest-poll average, the simplest forecast the model has to beat. For each firm, take its
//! latest round 1 poll published by the replay date (fieldwork ended within the last 30 days) and,
//! in it, the scenario closest to the field that ran; average each candidate over the firms that
//! tested them. The two leaders qualify (probability 1, everyone else 0), and the finalist ahead in
//! the average of the firms' latest head-to-heads wins (or, with no head-to-head, the one ahead in
//! round 1). It has no uncertainty, so its log-loss is set by the probability floor.

use std::collections::BTreeMap;

use fr2027_model::{Poll, Scenario, day_number};
use jiff::civil::Date;

use crate::{
    election::Election,
    replay::Selection,
    score::{CandidateScore, Event, EventKind, ReplayScore, Round2Score},
};

const RECENT_DAYS: i32 = 30;

/// Each firm's latest poll (by fieldwork end, then id) among `polls` that has a scenario
/// accepted by `pick`, with that scenario.
fn latest_by_firm<'a>(
    polls: &'a [Poll],
    pick: impl Fn(&'a Poll) -> Option<&'a Scenario>,
) -> Vec<(&'a Poll, &'a Scenario)> {
    let mut latest: BTreeMap<&str, (&Poll, &Scenario)> = BTreeMap::new();
    for poll in polls {
        let Some(scenario) = pick(poll) else { continue };
        let newer = latest
            .get(poll.firm.as_str())
            .is_none_or(|(best, _)| (poll.field_end, &poll.poll_id) > (best.field_end, &best.poll_id));
        if newer {
            latest.insert(poll.firm.as_str(), (poll, scenario));
        }
    }
    latest.into_values().collect()
}

/// The round 1 scenario of `poll` closest to the field that ran: most runners, fewest
/// non-runners, then the first by id.
fn closest_scenario<'a>(election: &Election, poll: &'a Poll) -> Option<&'a Scenario> {
    let fit = |scenario: &Scenario| -> i64 {
        scenario
            .shares
            .iter()
            .map(|s| match election.field.index_of(&s.candidate_id) {
                Some(c) if election.ran[c] => 1,
                _ => -1,
            })
            .sum()
    };
    poll.scenarios
        .iter()
        .filter(|s| s.round == 1)
        .max_by_key(|s| (fit(s), std::cmp::Reverse(s.scenario_id.clone())))
}

/// The head-to-head of candidates `a` and `b` in `poll`, if it has one.
fn head_to_head<'a>(election: &Election, poll: &'a Poll, a: usize, b: usize) -> Option<&'a Scenario> {
    poll.scenarios.iter().find(|s| {
        let mut pair: Vec<Option<usize>> = s
            .shares
            .iter()
            .map(|x| election.field.index_of(&x.candidate_id))
            .collect();
        pair.sort_unstable();
        s.round == 2 && pair == [Some(a.min(b)), Some(a.max(b))]
    })
}

pub fn evaluate(election: &Election, selection: &Selection, horizon_days: u32, as_of: Date) -> ReplayScore {
    let field = &election.field;
    let n = field.len();
    let recent: Vec<Poll> = selection
        .polls
        .iter()
        .filter(|p| day_number(p.field_end) >= day_number(as_of) - RECENT_DAYS)
        .cloned()
        .collect();
    let mut latest = latest_by_firm(&recent, |p| closest_scenario(election, p));
    if latest.is_empty() {
        latest = latest_by_firm(&selection.polls, |p| closest_scenario(election, p));
    }

    let mut totals = vec![(0.0, 0usize); n];
    for (_, scenario) in &latest {
        for share in &scenario.shares {
            if let Some(c) = field.index_of(&share.candidate_id) {
                totals[c].0 += share.share;
                totals[c].1 += 1;
            }
        }
    }
    let average: Vec<Option<f64>> = (0..n)
        .map(|c| (election.ran[c] && totals[c].1 > 0).then(|| totals[c].0 / totals[c].1 as f64))
        .collect();
    let sum: f64 = average.iter().flatten().sum();
    let share = |c: usize| average[c].map(|a| a / sum);

    let mut ranked: Vec<usize> = (0..n).filter(|c| average[*c].is_some()).collect();
    ranked.sort_by(|a, b| {
        average[*b]
            .unwrap_or(0.0)
            .total_cmp(&average[*a].unwrap_or(0.0))
            .then(a.cmp(b))
    });
    let predicted = (ranked.len() >= 2).then(|| (ranked[0].min(ranked[1]), ranked[0].max(ranked[1])));
    let predicted_winner = predicted.map(|(a, b)| {
        let polls = latest_by_firm(&selection.polls, |p| head_to_head(election, p, a, b));
        let a_share: Vec<f64> = polls
            .iter()
            .map(|(_, s)| {
                let of = |c: usize| {
                    s.shares
                        .iter()
                        .find(|x| field.index_of(&x.candidate_id) == Some(c))
                        .map_or(0.0, |x| x.share)
                };
                of(a) / (of(a) + of(b))
            })
            .collect();
        let a_ahead = if a_share.is_empty() {
            average[a] >= average[b]
        } else {
            a_share.iter().sum::<f64>() / a_share.len() as f64 > 0.5
        };
        if a_ahead { a } else { b }
    });

    let (first, second) = election.finalists;
    let winner = election.winner();
    let mut candidates = Vec::new();
    let mut events = Vec::new();
    for c in (0..n).filter(|c| election.ran[*c]) {
        let qualifies = predicted.is_some_and(|(a, b)| c == a || c == b);
        let p_qualify = if qualifies { 1.0 } else { 0.0 };
        let p_win = if predicted_winner == Some(c) { 1.0 } else { 0.0 };
        let qualified = c == first || c == second;
        events.push(Event {
            kind: EventKind::Qualify,
            p: p_qualify,
            happened: qualified,
        });
        events.push(Event {
            kind: EventKind::Win,
            p: p_win,
            happened: c == winner,
        });
        candidates.push(CandidateScore {
            candidate_id: election.id(c).to_string(),
            p_qualify,
            p_win,
            share_mean: share(c),
            share_lo80: None,
            share_hi80: None,
            result_share: election.round1_result[c],
            qualified,
            won: c == winner,
            share_log_score: None,
        });
    }
    candidates.sort_by(|a, b| b.result_share.total_cmp(&a.result_share));
    let matchup = predicted == Some((first, second));
    ReplayScore {
        year: election.year,
        horizon_days,
        as_of,
        candidates,
        round2: Round2Score {
            p_matchup: if matchup { 1.0 } else { 0.0 },
            p_winner_given_matchup: matchup.then(|| if predicted_winner == Some(winner) { 1.0 } else { 0.0 }),
            log_score: None,
        },
        events,
    }
}
