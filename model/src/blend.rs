//! The market blend (spec, Modeling step 7): the model's probabilities pooled with the prediction
//! markets' on the log scale,
//!
//! ```text
//! p_blend ∝ exp(w · ln p_model + (1 − w) · ln p_market)
//! ```
//!
//! For who wins, the outcomes are the candidates both sides price plus "everyone else", so the
//! pooled probabilities sum to 1; the "everyone else" share is then split over the remaining
//! candidates as the model splits it. Reaching round 2 and being on the ballot are pooled one
//! candidate at a time, as yes/no events.
//!
//! Market prices are taken as given: a long shot's price is known to run above its chance
//! (the favourite–long-shot bias), and the markets' fees are not removed. The weight `w` on the
//! model is fitted by the backtest where market history exists; 1, the model alone, until then.

use serde::{Deserialize, Serialize};

use crate::ModelError;

/// `market_blend` in `config/model.yaml`.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct BlendParams {
    /// `w`: 1 is the model alone, 0 the markets alone.
    pub model_weight: f64,
    /// Probabilities are clipped to `[floor, 1 - floor]` before taking logs.
    pub probability_floor: f64,
    /// A market snapshot older than this, against the time the blend is computed, is not used.
    pub max_age_hours: f64,
}

impl BlendParams {
    pub fn check(&self) -> Result<(), ModelError> {
        if !(0.0..=1.0).contains(&self.model_weight) {
            return Err(ModelError::InvalidParameters(
                "market_blend.model_weight must be in [0, 1]".into(),
            ));
        }
        if !(self.probability_floor > 0.0 && self.probability_floor < 0.5) {
            return Err(ModelError::InvalidParameters(
                "market_blend.probability_floor must be in (0, 0.5)".into(),
            ));
        }
        if self.max_age_hours.is_nan() || self.max_age_hours <= 0.0 {
            return Err(ModelError::InvalidParameters(
                "market_blend.max_age_hours must be positive".into(),
            ));
        }
        Ok(())
    }
}

/// The markets' view of who wins: each candidate's price (None when no market lists them) and
/// the total price of contracts on people outside the field.
#[derive(Clone, Debug, PartialEq)]
pub struct WinPrices {
    pub prices: Vec<Option<f64>>,
    pub others: f64,
}

impl WinPrices {
    /// The prices scaled to sum to 1 with the others: each candidate's market probability, and
    /// the others' total.
    pub fn normalised(&self) -> (Vec<Option<f64>>, f64) {
        let total: f64 = self.prices.iter().flatten().sum::<f64>() + self.others;
        if total <= 0.0 {
            return (vec![None; self.prices.len()], 0.0);
        }
        (
            self.prices.iter().map(|p| p.map(|p| p / total)).collect(),
            self.others / total,
        )
    }
}

fn clip(p: f64, floor: f64) -> f64 {
    p.clamp(floor, 1.0 - floor)
}

/// Pools one yes/no probability: `logit p = w logit p_model + (1 − w) logit p_market`.
pub fn pool_binary(p_model: f64, p_market: Option<f64>, params: &BlendParams) -> f64 {
    let w = params.model_weight;
    let Some(p_market) = p_market else { return p_model };
    if w >= 1.0 {
        return p_model;
    }
    let logit = |p: f64| {
        let p = clip(p, params.probability_floor);
        (p / (1.0 - p)).ln()
    };
    let x = w * logit(p_model) + (1.0 - w) * logit(p_market);
    1.0 / (1.0 + (-x).exp())
}

/// Pools who wins over the candidates both sides price and "everyone else"; see the module notes.
pub fn pool_win(p_model: &[f64], market: &WinPrices, params: &BlendParams) -> Vec<f64> {
    let w = params.model_weight;
    let (m, _) = market.normalised();
    let both: Vec<usize> = (0..p_model.len()).filter(|c| m[*c].is_some()).collect();
    if w >= 1.0 || both.is_empty() {
        return p_model.to_vec();
    }
    let model_rest = (1.0 - both.iter().map(|c| p_model[*c]).sum::<f64>()).max(0.0);
    let market_rest = (1.0 - both.iter().map(|c| m[*c].unwrap_or(0.0)).sum::<f64>()).max(0.0);
    let floor = params.probability_floor;
    let pooled = |a: f64, b: f64| (w * clip(a, floor).ln() + (1.0 - w) * clip(b, floor).ln()).exp();
    let mut weights: Vec<f64> = both.iter().map(|c| pooled(p_model[*c], m[*c].unwrap_or(0.0))).collect();
    // "Everyone else" is an outcome only if the model gives it some chance: its share is split
    // over the remaining candidates as the model splits it.
    if model_rest > 0.0 {
        weights.push(pooled(model_rest, market_rest));
    }
    let total: f64 = weights.iter().sum();
    let rest = weights.get(both.len()).map_or(0.0, |r| r / total);
    let mut out: Vec<f64> = p_model
        .iter()
        .map(|p| if model_rest > 0.0 { rest * p / model_rest } else { 0.0 })
        .collect();
    for (i, c) in both.iter().enumerate() {
        out[*c] = weights[i] / total;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params(model_weight: f64) -> BlendParams {
        BlendParams {
            model_weight,
            probability_floor: 0.001,
            max_age_hours: 6.0,
        }
    }

    #[test]
    fn weight_one_is_the_model() {
        let market = WinPrices {
            prices: vec![Some(0.4), Some(0.3), None],
            others: 0.3,
        };
        let model = [0.8, 0.15, 0.05];
        assert_eq!(pool_win(&model, &market, &params(1.0)), model);
        assert_eq!(pool_binary(0.9, Some(0.5), &params(1.0)), 0.9);
    }

    #[test]
    fn weight_zero_is_the_market_where_it_prices() {
        // Market prices sum to 1.1 (overround): normalised to 0.4/1.1 and 0.5/1.1, others 0.2/1.1.
        let market = WinPrices {
            prices: vec![Some(0.4), Some(0.5), None],
            others: 0.2,
        };
        let model = [0.7, 0.2, 0.1];
        let blend = pool_win(&model, &market, &params(0.0));
        assert!((blend[0] - 0.4 / 1.1).abs() < 1e-9);
        assert!((blend[1] - 0.5 / 1.1).abs() < 1e-9);
        // Candidate 2 is the model's whole "rest", so it gets the market's rest.
        assert!((blend[2] - 0.2 / 1.1).abs() < 1e-9);
        assert!((blend.iter().sum::<f64>() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn halfway_is_the_normalised_geometric_mean() {
        let market = WinPrices {
            prices: vec![Some(0.5), Some(0.5)],
            others: 0.0,
        };
        let blend = pool_win(&[0.9, 0.1], &market, &params(0.5));
        let (a, b) = ((0.9f64 * 0.5).sqrt(), (0.1f64 * 0.5).sqrt());
        assert!((blend[0] - a / (a + b)).abs() < 1e-3, "{blend:?}");
        assert!((blend.iter().sum::<f64>() - 1.0).abs() < 1e-9);
        let p = pool_binary(0.9, Some(0.5), &params(0.5));
        assert!((p - 0.75).abs() < 1e-9, "logit average of 0.9 and 0.5 is 0.75: {p}");
    }
}
