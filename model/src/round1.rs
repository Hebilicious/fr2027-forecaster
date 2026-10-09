//! Round 1 poll aggregation.
//!
//! The latent state, on the log scale, holds for each bloc `b` a strength `beta_b`, for each
//! candidate `c` a within-bloc support `alpha_c` and a presence effect `kappa_c`, and for each
//! pollster `f` and bloc `b` a house effect `h_fb`. In a field `S` (a poll's scenario or a
//! simulated ballot):
//!
//! ```text
//! bloc total    P_b  ∝ exp(beta_b + Σ_{c ∈ b∩S} kappa_c)        over blocs with a candidate in S
//! within bloc   q_c  ∝ exp(alpha_c)                              over c ∈ b∩S
//! share         p_c  = P_b(c) · q_c
//! ```
//!
//! So a candidate who drops out passes their support to the rest of their bloc first, and the
//! presence effects learn, from scenario polls that include and exclude a candidate, how much
//! of a bloc's total that candidate brings. Every quantity a poll reports is linear in the
//! state once written as a log-ratio (bloc totals against the largest bloc, candidates against
//! the largest candidate of their bloc), so a Kalman filter fits it exactly. `beta` and
//! `alpha` follow daily random walks; presence and house effects are static.
//!
//! A poll's sampling covariance is the multinomial one (delta method) times a design effect.
//! The scenarios of one poll ask the same respondents, so their sampling errors are mostly
//! shared: all of a poll's scenarios enter as one observation whose error is a common draw on
//! the bloc and candidate levels (the multinomial variance, `1 / (n p)` per level) plus a small
//! scenario-specific part (`scenario_noise_share` of the multinomial covariance). A poll then
//! counts once for the levels, while the contrasts between its scenarios, which is where the
//! presence effects come from, stay as precise as the respondents make them.

use nalgebra::{DMatrix, DVector};

use crate::{
    AggregationParams, ModelError, Poll, date_from_day_number, day_number,
    field::Field,
    kalman::{Gaussian, Sampler},
    quantile,
    rng::Rng,
};

/// Where each quantity sits in the state vector. The vote-relevant quantities (`beta`,
/// `alpha`, `kappa`) come first so they can be marginalised as a leading block.
#[derive(Clone, Debug)]
pub struct Layout {
    pub n_blocs: usize,
    pub n_candidates: usize,
    pub firms: Vec<String>,
    pub bloc_of: Vec<usize>,
}

impl Layout {
    pub fn beta(&self, bloc: usize) -> usize {
        bloc
    }
    pub fn alpha(&self, candidate: usize) -> usize {
        self.n_blocs + candidate
    }
    pub fn kappa(&self, candidate: usize) -> usize {
        self.n_blocs + self.n_candidates + candidate
    }
    pub fn house(&self, firm: usize, bloc: usize) -> usize {
        self.vote_dim() + firm * self.n_blocs + bloc
    }
    pub fn vote_dim(&self) -> usize {
        self.n_blocs + 2 * self.n_candidates
    }
    pub fn dim(&self) -> usize {
        self.vote_dim() + self.firms.len() * self.n_blocs
    }
}

/// Reusable buffers for [`round1_shares`].
#[derive(Clone, Debug)]
pub struct ShareScratch {
    strength: Vec<f64>,
    has: Vec<bool>,
    within_max: Vec<f64>,
    within_sum: Vec<f64>,
}

impl ShareScratch {
    pub fn new(layout: &Layout) -> Self {
        Self {
            strength: vec![0.0; layout.n_blocs],
            has: vec![false; layout.n_blocs],
            within_max: vec![f64::NEG_INFINITY; layout.n_blocs],
            within_sum: vec![0.0; layout.n_blocs],
        }
    }
}

/// Election-day polling errors added to bloc strengths and within-bloc supports.
pub struct Errors<'a> {
    pub bloc: &'a [f64],
    pub candidate: &'a [f64],
}

/// Shares of the vote (fractions summing to 1 over `present`) implied by vote state `x`.
pub fn round1_shares(
    layout: &Layout,
    x: &[f64],
    present: &[bool],
    errors: Option<&Errors>,
    scratch: &mut ShareScratch,
    out: &mut [f64],
) {
    let nb = layout.n_blocs;
    for b in 0..nb {
        scratch.strength[b] = x[layout.beta(b)] + errors.map_or(0.0, |e| e.bloc[b]);
        scratch.has[b] = false;
        scratch.within_max[b] = f64::NEG_INFINITY;
        scratch.within_sum[b] = 0.0;
    }
    for c in 0..layout.n_candidates {
        out[c] = 0.0;
        if !present[c] {
            continue;
        }
        let b = layout.bloc_of[c];
        scratch.has[b] = true;
        scratch.strength[b] += x[layout.kappa(c)];
        let support = x[layout.alpha(c)] + errors.map_or(0.0, |e| e.candidate[c]);
        scratch.within_max[b] = scratch.within_max[b].max(support);
    }
    let bloc_max = (0..nb)
        .filter(|b| scratch.has[*b])
        .map(|b| scratch.strength[b])
        .fold(f64::NEG_INFINITY, f64::max);
    if bloc_max == f64::NEG_INFINITY {
        return;
    }
    let mut bloc_total = 0.0;
    for b in 0..nb {
        if scratch.has[b] {
            scratch.strength[b] = (scratch.strength[b] - bloc_max).exp();
            bloc_total += scratch.strength[b];
        }
    }
    for c in 0..layout.n_candidates {
        if present[c] {
            let b = layout.bloc_of[c];
            let support = x[layout.alpha(c)] + errors.map_or(0.0, |e| e.candidate[c]);
            out[c] = (support - scratch.within_max[b]).exp();
            scratch.within_sum[b] += out[c];
        }
    }
    for c in 0..layout.n_candidates {
        if present[c] {
            let b = layout.bloc_of[c];
            out[c] *= scratch.strength[b] / bloc_total / scratch.within_sum[b];
        }
    }
}

#[derive(Clone, Debug)]
pub struct SeriesPoint {
    pub date: jiff::civil::Date,
    pub candidate: usize,
    pub mean: f64,
    pub lo80: f64,
    pub hi80: f64,
}

/// The noise hyperparameters, chosen together by marginal likelihood.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Noise {
    pub random_walk_sd: f64,
    pub design_effect: f64,
    pub scenario_noise_share: f64,
}

#[derive(Clone, Debug)]
pub struct GridPoint {
    pub noise: Noise,
    pub log_likelihood: f64,
}

#[derive(Clone, Debug)]
pub struct HouseEffect {
    pub firm: String,
    pub bloc: usize,
    pub mean: f64,
    pub sd: f64,
}

#[derive(Clone, Debug)]
pub struct Round1Fit {
    pub layout: Layout,
    pub noise: Noise,
    pub log_likelihood: f64,
    pub grid: Vec<GridPoint>,
    pub as_of: jiff::civil::Date,
    /// Filtered state at `as_of`.
    pub state: Gaussian,
    pub first_polled: Vec<Option<jiff::civil::Date>>,
    /// Latent share history: each candidate's share in the most likely field (with that
    /// candidate in it), from their first poll on.
    pub series: Vec<SeriesPoint>,
    pub polls_used: usize,
    pub scenarios_used: usize,
    pub house_effects: Vec<HouseEffect>,
}

impl Round1Fit {
    /// The vote state projected to `date` by the random walk.
    pub fn vote_state_at(&self, date: jiff::civil::Date) -> Gaussian {
        let mut state = self.state.leading(self.layout.vote_dim());
        let days = f64::from(day_number(date) - day_number(self.as_of));
        state.predict_random_walk(
            &daily_variance(&self.layout, self.noise.random_walk_sd, self.layout.vote_dim()),
            days,
        );
        state
    }
}

fn daily_variance(layout: &Layout, random_walk_sd: f64, dim: usize) -> Vec<f64> {
    let mut variance = vec![0.0; dim];
    let v = random_walk_sd * random_walk_sd;
    for b in 0..layout.n_blocs {
        variance[layout.beta(b)] = v;
    }
    for c in 0..layout.n_candidates {
        variance[layout.alpha(c)] = v;
    }
    variance
}

/// The round 1 scenarios of one poll, ready for the filter.
#[derive(Clone, Debug)]
struct Observation {
    day: i32,
    firm: usize,
    /// Per scenario: candidate index and share (fraction), floored and renormalised.
    scenarios: Vec<Vec<(usize, f64)>>,
    sample_size: f64,
}

/// Fits the aggregation to every round 1 scenario published on or before `as_of`, choosing
/// the random-walk scale and design effect by marginal likelihood.
pub fn fit_round1(
    field: &Field,
    polls: &[Poll],
    params: &AggregationParams,
    as_of: jiff::civil::Date,
    seed: u64,
) -> Result<Round1Fit, ModelError> {
    if params.random_walk_sd_grid.is_empty()
        || params.design_effect_grid.is_empty()
        || params.scenario_noise_share_grid.is_empty()
    {
        return Err(ModelError::InvalidParameters("empty hyperparameter grid".into()));
    }
    let (observations, firms, polls_used) = observations(field, polls, params, as_of)?;
    if observations.is_empty() {
        return Err(ModelError::NotEnoughData(format!(
            "no round 1 poll published by {as_of}"
        )));
    }
    let layout = Layout {
        n_blocs: field.blocs.len(),
        n_candidates: field.len(),
        firms,
        bloc_of: field.bloc_of.clone(),
    };
    let end_day = day_number(as_of);

    let mut grid = Vec::new();
    for &random_walk_sd in &params.random_walk_sd_grid {
        for &design_effect in &params.design_effect_grid {
            for &scenario_noise_share in &params.scenario_noise_share_grid {
                let noise = Noise {
                    random_walk_sd,
                    design_effect,
                    scenario_noise_share,
                };
                let (_, log_likelihood) = run_filter(&layout, &observations, params, noise, end_day, None)?;
                grid.push(GridPoint { noise, log_likelihood });
            }
        }
    }
    let best = grid
        .iter()
        .max_by(|a, b| a.log_likelihood.total_cmp(&b.log_likelihood))
        .expect("non-empty grid")
        .clone();

    let mut recorder = SeriesRecorder::new(field, &layout, &observations, params, end_day, seed);
    let (state, _) = run_filter(&layout, &observations, params, best.noise, end_day, Some(&mut recorder))?;

    let mut house_effects = Vec::new();
    for (f, firm) in layout.firms.iter().enumerate() {
        for b in 0..layout.n_blocs {
            let i = layout.house(f, b);
            house_effects.push(HouseEffect {
                firm: firm.clone(),
                bloc: b,
                mean: state.mean[i],
                sd: state.cov[(i, i)].sqrt(),
            });
        }
    }
    let first_polled = recorder
        .first_polled
        .iter()
        .map(|d| d.map(date_from_day_number))
        .collect();
    Ok(Round1Fit {
        noise: best.noise,
        log_likelihood: best.log_likelihood,
        grid,
        as_of,
        state,
        first_polled,
        series: recorder.series,
        polls_used,
        scenarios_used: observations.iter().map(|o| o.scenarios.len()).sum(),
        house_effects,
        layout,
    })
}

fn observations(
    field: &Field,
    polls: &[Poll],
    params: &AggregationParams,
    as_of: jiff::civil::Date,
) -> Result<(Vec<Observation>, Vec<String>, usize), ModelError> {
    let mut firms: Vec<String> = Vec::new();
    let mut result = Vec::new();
    let mut polls_used = 0;
    for poll in polls.iter().filter(|p| p.published_at <= as_of) {
        let invalid = |message: String| ModelError::InvalidPoll {
            poll: poll.poll_id.clone(),
            message,
        };
        if poll.sample_size == 0 {
            return Err(invalid("sample size is zero".into()));
        }
        let scenarios: Vec<_> = poll.scenarios.iter().filter(|s| s.round == 1).collect();
        if scenarios.is_empty() {
            continue;
        }
        let mut cells = Vec::with_capacity(scenarios.len());
        for scenario in &scenarios {
            let mut shares = Vec::with_capacity(scenario.shares.len());
            for share in &scenario.shares {
                let Some(c) = field.index_of(&share.candidate_id) else {
                    return Err(invalid(format!("unknown candidate `{}`", share.candidate_id)));
                };
                if shares.iter().any(|(existing, _)| *existing == c) {
                    return Err(invalid(format!("candidate `{}` listed twice", share.candidate_id)));
                }
                if !(share.share.is_finite() && share.share >= 0.0) {
                    return Err(invalid(format!("share {} is not a percentage", share.share)));
                }
                shares.push((c, share.share.max(params.share_floor)));
            }
            if shares.len() < 2 {
                continue;
            }
            let total: f64 = shares.iter().map(|(_, s)| s).sum();
            for (_, s) in &mut shares {
                *s /= total;
            }
            cells.push(shares);
        }
        if cells.is_empty() {
            continue;
        }
        polls_used += 1;
        let firm = match firms.iter().position(|f| *f == poll.firm) {
            Some(f) => f,
            None => {
                firms.push(poll.firm.clone());
                firms.len() - 1
            }
        };
        result.push(Observation {
            day: poll.midpoint(),
            firm,
            scenarios: cells,
            sample_size: f64::from(poll.sample_size),
        });
    }
    result.sort_by_key(|o| o.day);
    Ok((result, firms, polls_used))
}

/// One log-ratio: its state coefficients, observed value, and Jacobian over the cells.
type Row = (Vec<(usize, f64)>, f64, Vec<f64>);

struct Linearised {
    h: DMatrix<f64>,
    y: DVector<f64>,
    r: DMatrix<f64>,
}

/// Writes one scenario as log-ratios (bloc totals against the largest bloc, then candidates
/// against the largest candidate of their bloc), with each row's Jacobian over the cells.
fn scenario_rows(layout: &Layout, firm: usize, cells: &[(usize, f64)]) -> Vec<Row> {
    let k = cells.len();
    let mut blocs: Vec<usize> = cells.iter().map(|(c, _)| layout.bloc_of[*c]).collect();
    blocs.sort_unstable();
    blocs.dedup();
    let bloc_total = |b: usize| -> f64 {
        cells
            .iter()
            .filter(|(c, _)| layout.bloc_of[*c] == b)
            .map(|(_, s)| s)
            .sum()
    };
    let reference_bloc = *blocs
        .iter()
        .max_by(|a, b| bloc_total(**a).total_cmp(&bloc_total(**b)))
        .expect("at least one bloc");

    let mut rows: Vec<Row> = Vec::new();
    for &b in blocs.iter().filter(|b| **b != reference_bloc) {
        let mut coefficients = vec![
            (layout.beta(b), 1.0),
            (layout.house(firm, b), 1.0),
            (layout.beta(reference_bloc), -1.0),
            (layout.house(firm, reference_bloc), -1.0),
        ];
        let (total_b, total_r) = (bloc_total(b), bloc_total(reference_bloc));
        let mut jacobian = vec![0.0; k];
        for (i, (c, _)) in cells.iter().enumerate() {
            let cb = layout.bloc_of[*c];
            if cb == b {
                coefficients.push((layout.kappa(*c), 1.0));
                jacobian[i] = 1.0 / total_b;
            } else if cb == reference_bloc {
                coefficients.push((layout.kappa(*c), -1.0));
                jacobian[i] = -1.0 / total_r;
            }
        }
        rows.push((coefficients, (total_b / total_r).ln(), jacobian));
    }
    for &b in &blocs {
        let members: Vec<usize> = (0..k).filter(|i| layout.bloc_of[cells[*i].0] == b).collect();
        if members.len() < 2 {
            continue;
        }
        let reference = *members
            .iter()
            .max_by(|x, y| cells[**x].1.total_cmp(&cells[**y].1))
            .expect("non-empty bloc");
        for &i in members.iter().filter(|i| **i != reference) {
            let mut jacobian = vec![0.0; k];
            jacobian[i] = 1.0 / cells[i].1;
            jacobian[reference] = -1.0 / cells[reference].1;
            rows.push((
                vec![
                    (layout.alpha(cells[i].0), 1.0),
                    (layout.alpha(cells[reference].0), -1.0),
                ],
                (cells[i].1 / cells[reference].1).ln(),
                jacobian,
            ));
        }
    }
    rows
}

/// All of a poll's scenarios as one observation. The sampling error is a draw shared by every
/// scenario on the bloc and candidate levels (variance `1 / (n p)` for each, `p` the level's
/// mean share across the scenarios), plus a scenario-specific multinomial part; the two are
/// weighted so a lone scenario gets exactly the multinomial covariance.
fn linearise(layout: &Layout, observation: &Observation, design_effect: f64, scenario_noise_share: f64) -> Linearised {
    let mut level_share: Vec<(usize, f64, usize)> = Vec::new();
    let mut add_level = |index: usize, share: f64| match level_share.iter_mut().find(|(i, _, _)| *i == index) {
        Some(entry) => {
            entry.1 += share;
            entry.2 += 1;
        }
        None => level_share.push((index, share, 1)),
    };
    for cells in &observation.scenarios {
        let mut totals = vec![0.0; layout.n_blocs];
        for (c, share) in cells {
            add_level(layout.alpha(*c), *share);
            totals[layout.bloc_of[*c]] += share;
        }
        for (b, total) in totals.into_iter().enumerate() {
            if total > 0.0 {
                add_level(layout.beta(b), total);
            }
        }
    }

    let per_scenario: Vec<(Vec<Row>, usize)> = observation
        .scenarios
        .iter()
        .map(|cells| (scenario_rows(layout, observation.firm, cells), cells.len()))
        .collect();
    let m: usize = per_scenario.iter().map(|(rows, _)| rows.len()).sum();
    let mut h = DMatrix::zeros(m, layout.dim());
    let mut y = DVector::zeros(m);
    let mut r = DMatrix::zeros(m, m);
    let mut offset = 0;
    for ((rows, k), cells) in per_scenario.iter().zip(&observation.scenarios) {
        let mut jacobian = DMatrix::zeros(rows.len(), *k);
        for (row, (coefficients, value, jac)) in rows.iter().enumerate() {
            for (index, coefficient) in coefficients {
                h[(offset + row, *index)] += coefficient;
            }
            y[offset + row] = *value;
            for (i, v) in jac.iter().enumerate() {
                jacobian[(row, i)] = *v;
            }
        }
        let p = DVector::from_iterator(*k, cells.iter().map(|(_, s)| *s));
        let multinomial = DMatrix::from_diagonal(&p) - &p * p.transpose();
        let own = &jacobian * multinomial * jacobian.transpose() * scenario_noise_share;
        let mut block = r.view_mut((offset, offset), (rows.len(), rows.len()));
        block += own;
        offset += rows.len();
    }
    // The shared part: G diag(1 / p) G^T, with G the rows' coefficients on bloc and candidate levels.
    let mut levels = DMatrix::zeros(m, level_share.len());
    for (j, (index, _, _)) in level_share.iter().enumerate() {
        for row in 0..m {
            levels[(row, j)] = h[(row, *index)];
        }
    }
    let inverse_share = DVector::from_iterator(
        level_share.len(),
        level_share.iter().map(|(_, total, count)| *count as f64 / total),
    );
    r += &levels * DMatrix::from_diagonal(&inverse_share) * levels.transpose() * (1.0 - scenario_noise_share);
    r *= design_effect / observation.sample_size;
    Linearised { h, y, r }
}

fn prior(layout: &Layout, params: &AggregationParams) -> Gaussian {
    let n = layout.dim();
    let mut cov = DMatrix::zeros(n, n);
    for b in 0..layout.n_blocs {
        cov[(layout.beta(b), layout.beta(b))] = params.prior_sd.powi(2);
    }
    for c in 0..layout.n_candidates {
        cov[(layout.alpha(c), layout.alpha(c))] = params.prior_sd.powi(2);
        cov[(layout.kappa(c), layout.kappa(c))] = params.presence_sd.powi(2);
    }
    for f in 0..layout.firms.len() {
        for b in 0..layout.n_blocs {
            cov[(layout.house(f, b), layout.house(f, b))] = params.house_sd.powi(2);
        }
    }
    Gaussian::new(DVector::zeros(n), cov)
}

fn run_filter(
    layout: &Layout,
    observations: &[Observation],
    params: &AggregationParams,
    noise: Noise,
    end_day: i32,
    mut recorder: Option<&mut SeriesRecorder>,
) -> Result<(Gaussian, f64), ModelError> {
    let variance = daily_variance(layout, noise.random_walk_sd, layout.dim());
    let mut state = prior(layout, params);
    let mut day = observations[0].day;
    let mut log_likelihood = 0.0;
    for observation in observations {
        if let Some(recorder) = recorder.as_deref_mut() {
            while let Some(grid_day) = recorder.next_day().filter(|d| *d < observation.day) {
                state.predict_random_walk(&variance, f64::from(grid_day - day));
                day = grid_day;
                recorder.record(layout, &state)?;
            }
            recorder.saw(observation);
        }
        state.predict_random_walk(&variance, f64::from(observation.day - day));
        day = observation.day;
        let linearised = linearise(layout, observation, noise.design_effect, noise.scenario_noise_share);
        log_likelihood += state.update(&linearised.h, &linearised.y, &linearised.r)?;
    }
    if let Some(recorder) = recorder {
        while let Some(grid_day) = recorder.next_day() {
            state.predict_random_walk(&variance, f64::from(grid_day - day));
            day = grid_day;
            recorder.record(layout, &state)?;
        }
    }
    state.predict_random_walk(&variance, f64::from(end_day - day));
    Ok((state, log_likelihood))
}

struct SeriesRecorder {
    days: Vec<i32>,
    next: usize,
    fields: Vec<Vec<bool>>,
    first_polled: Vec<Option<i32>>,
    draws: usize,
    seed: u64,
    series: Vec<SeriesPoint>,
}

impl SeriesRecorder {
    fn new(
        field: &Field,
        layout: &Layout,
        observations: &[Observation],
        params: &AggregationParams,
        end_day: i32,
        seed: u64,
    ) -> Self {
        let start = observations[0].day;
        let step = params.series_step_days.max(1) as i32;
        let mut days: Vec<i32> = (0..).map(|i| end_day - i * step).take_while(|d| *d >= start).collect();
        days.reverse();
        if days.is_empty() {
            days.push(end_day);
        }
        Self {
            days,
            next: 0,
            fields: (0..layout.n_candidates).map(|c| field.modal_with(c)).collect(),
            first_polled: vec![None; layout.n_candidates],
            draws: params.series_draws.max(2) as usize,
            seed,
            series: Vec::new(),
        }
    }

    fn next_day(&self) -> Option<i32> {
        self.days.get(self.next).copied()
    }

    fn saw(&mut self, observation: &Observation) {
        for (c, _) in observation.scenarios.iter().flatten() {
            self.first_polled[*c].get_or_insert(observation.day);
        }
    }

    fn record(&mut self, layout: &Layout, state: &Gaussian) -> Result<(), ModelError> {
        let day = self.days[self.next];
        self.next += 1;
        let candidates: Vec<usize> = (0..layout.n_candidates)
            .filter(|c| self.first_polled[*c].is_some_and(|first| first <= day))
            .collect();
        if candidates.is_empty() {
            return Ok(());
        }
        let vote = state.leading(layout.vote_dim());
        let sampler: Sampler = vote.sampler()?;
        let mut rng = Rng::derive(self.seed, &format!("series:{day}"));
        let n = layout.vote_dim();
        let mut scratch = vec![0.0; n];
        let draws: Vec<Vec<f64>> = (0..self.draws)
            .map(|_| {
                let mut x = vec![0.0; n];
                sampler.draw(&mut rng, &mut scratch, &mut x);
                x
            })
            .collect();
        let mut shares = vec![0.0; layout.n_candidates];
        let mut share_scratch = ShareScratch::new(layout);
        for c in candidates {
            // A candidate no poll had tested by this date has only a prior; leave them out of the
            // field rather than let that prior take a random slice of their bloc.
            let field: Vec<bool> = self.fields[c]
                .iter()
                .enumerate()
                .map(|(k, present)| *present && self.first_polled[k].is_some_and(|first| first <= day))
                .collect();
            let mut values: Vec<f64> = draws
                .iter()
                .map(|x| {
                    round1_shares(layout, x, &field, None, &mut share_scratch, &mut shares);
                    shares[c]
                })
                .collect();
            values.sort_by(f64::total_cmp);
            self.series.push(SeriesPoint {
                date: date_from_day_number(day),
                candidate: c,
                mean: values.iter().sum::<f64>() / values.len() as f64,
                lo80: quantile(&values, 0.1),
                hi80: quantile(&values, 0.9),
            });
        }
        Ok(())
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

    pub(crate) fn params() -> AggregationParams {
        AggregationParams {
            random_walk_sd_grid: vec![0.005, 0.02],
            design_effect_grid: vec![1.0, 2.0],
            prior_sd: 2.0,
            presence_sd: 0.3,
            house_sd: 0.08,
            share_floor: 0.5,
            scenario_noise_share_grid: vec![0.2],
            series_step_days: 7,
            series_draws: 200,
        }
    }

    fn field() -> Field {
        Field::new(
            vec!["left".into(), "centre".into(), "right".into()],
            vec![
                candidate("l1", "left"),
                candidate("l2", "left"),
                candidate("c1", "centre"),
                candidate("r1", "right"),
            ],
            &[
                slot("left", vec![option(&["l1", "l2"], 0.5), option(&["l1"], 0.5)]),
                slot("c1", vec![option(&["c1"], 1.0)]),
                slot("r1", vec![option(&["r1"], 1.0)]),
            ],
        )
        .unwrap()
    }

    fn poll(id: &str, firm: &str, day: i8, scenarios: Vec<Vec<(&str, f64)>>) -> Poll {
        Poll {
            poll_id: id.into(),
            firm: firm.into(),
            field_start: date(2026, 9, day),
            field_end: date(2026, 9, day + 1),
            published_at: date(2026, 9, day + 2),
            sample_size: 1500,
            scenarios: scenarios
                .into_iter()
                .enumerate()
                .map(|(i, shares)| Scenario {
                    scenario_id: format!("{i}"),
                    round: 1,
                    shares: shares
                        .into_iter()
                        .map(|(c, s)| Share {
                            candidate_id: c.into(),
                            share: s,
                        })
                        .collect(),
                })
                .collect(),
        }
    }

    #[test]
    fn shares_sum_to_one_and_respect_blocs() {
        let field = field();
        let layout = Layout {
            n_blocs: 3,
            n_candidates: 4,
            firms: vec![],
            bloc_of: field.bloc_of.clone(),
        };
        // Equal bloc strengths, l1 twice as popular as l2 within the left.
        let mut x = vec![0.0; layout.vote_dim()];
        x[layout.alpha(0)] = 2f64.ln();
        let mut out = vec![0.0; 4];
        let mut scratch = ShareScratch::new(&layout);
        round1_shares(&layout, &x, &[true, true, true, true], None, &mut scratch, &mut out);
        assert!((out.iter().sum::<f64>() - 1.0).abs() < 1e-12);
        assert!((out[0] - 2.0 / 9.0).abs() < 1e-12);
        assert!((out[1] - 1.0 / 9.0).abs() < 1e-12);
        // Without l2, the whole left bloc goes to l1.
        round1_shares(&layout, &x, &[true, false, true, true], None, &mut scratch, &mut out);
        assert!((out[0] - 1.0 / 3.0).abs() < 1e-12);
        assert_eq!(out[1], 0.0);
    }

    #[test]
    fn recovers_scenario_structure() {
        // Truth: bloc totals 40/25/35 when the left is split, l1:l2 = 3:1 inside it; the left
        // sheds 3 points to the others when l1 stands alone.
        let split = vec![("l1", 30.0), ("l2", 10.0), ("c1", 25.0), ("r1", 35.0)];
        let alone = vec![("l1", 37.0), ("c1", 26.25), ("r1", 36.75)];
        let polls: Vec<Poll> = (0..12)
            .map(|i| {
                poll(
                    &format!("p{i}"),
                    if i % 2 == 0 { "a" } else { "b" },
                    1 + i as i8 * 2,
                    vec![split.clone(), alone.clone()],
                )
            })
            .collect();
        let fit = fit_round1(&field(), &polls, &params(), date(2026, 10, 1), 1).unwrap();
        let x: Vec<f64> = fit.state.mean.iter().copied().take(fit.layout.vote_dim()).collect();
        let mut out = vec![0.0; 4];
        let mut scratch = ShareScratch::new(&fit.layout);
        round1_shares(&fit.layout, &x, &[true, true, true, true], None, &mut scratch, &mut out);
        assert!((out[0] - 0.30).abs() < 0.01, "{out:?}");
        assert!((out[1] - 0.10).abs() < 0.01, "{out:?}");
        round1_shares(
            &fit.layout,
            &x,
            &[true, false, true, true],
            None,
            &mut scratch,
            &mut out,
        );
        assert!((out[0] - 0.37).abs() < 0.01, "{out:?}");
        assert_eq!(fit.polls_used, 12);
        assert_eq!(fit.scenarios_used, 24);
        assert!(!fit.series.is_empty());
        assert!(fit.series.iter().all(|p| p.lo80 <= p.mean && p.mean <= p.hi80));
    }

    #[test]
    fn ignores_polls_published_after_as_of() {
        let early = poll("early", "a", 1, vec![vec![("l1", 30.0), ("c1", 30.0), ("r1", 40.0)]]);
        let mut late = poll("late", "a", 20, vec![vec![("l1", 60.0), ("c1", 20.0), ("r1", 20.0)]]);
        late.published_at = date(2026, 9, 25);
        let fit = fit_round1(&field(), &[early.clone(), late], &params(), date(2026, 9, 24), 1).unwrap();
        let only_early = fit_round1(&field(), &[early], &params(), date(2026, 9, 24), 1).unwrap();
        assert_eq!(fit.polls_used, 1);
        assert_eq!(fit.state.mean, only_early.state.mean);
    }

    #[test]
    fn rejects_unknown_candidates() {
        let bad = poll("bad", "a", 1, vec![vec![("zz", 30.0), ("c1", 70.0)]]);
        assert!(matches!(
            fit_round1(&field(), &[bad], &params(), date(2026, 10, 1), 1),
            Err(ModelError::InvalidPoll { .. })
        ));
    }

    fn test_layout() -> Layout {
        Layout {
            n_blocs: 3,
            n_candidates: 4,
            firms: vec!["a".into()],
            bloc_of: field().bloc_of.clone(),
        }
    }

    #[test]
    fn lone_scenario_gets_the_multinomial_covariance() {
        let layout = test_layout();
        let cells = vec![(0, 0.3), (1, 0.1), (2, 0.25), (3, 0.35)];
        let observation = Observation {
            day: 0,
            firm: 0,
            scenarios: vec![cells.clone()],
            sample_size: 1000.0,
        };
        // Delta-method covariance of the rows, computed directly.
        let rows = scenario_rows(&layout, 0, &cells);
        let jacobian = DMatrix::from_fn(rows.len(), cells.len(), |r, c| rows[r].2[c]);
        let p = DVector::from_iterator(cells.len(), cells.iter().map(|(_, s)| *s));
        let expected =
            &jacobian * (DMatrix::from_diagonal(&p) - &p * p.transpose()) * jacobian.transpose() * 2.0 / 1000.0;
        for share in [0.1, 0.4, 0.9] {
            let linearised = linearise(&layout, &observation, 2.0, share);
            assert!(
                (linearised.r - &expected).abs().max() < 1e-12,
                "scenario noise share {share}"
            );
        }
    }

    #[test]
    fn scenarios_of_one_poll_share_their_sampling_error() {
        let layout = test_layout();
        let observation = Observation {
            day: 0,
            firm: 0,
            scenarios: vec![
                vec![(0, 0.3), (1, 0.1), (2, 0.25), (3, 0.35)],
                vec![(0, 0.37), (2, 0.26), (3, 0.37)],
            ],
            sample_size: 1000.0,
        };
        let linearised = linearise(&layout, &observation, 1.0, 0.2);
        let (first, second) = (rows_of(&layout, &observation, 0), rows_of(&layout, &observation, 1));
        assert_eq!(linearised.r.nrows(), first + second);
        let cross = linearised.r.view((0, first), (first, second));
        assert!(cross.abs().max() > 0.0, "scenarios of one poll must be correlated");
        // The covariance stays a valid covariance.
        assert!(linearised.r.clone().cholesky().is_some());
    }

    fn rows_of(layout: &Layout, observation: &Observation, scenario: usize) -> usize {
        scenario_rows(layout, observation.firm, &observation.scenarios[scenario]).len()
    }
}
