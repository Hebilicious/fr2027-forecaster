//! The field model: who runs. Every candidate belongs to exactly one slot; a slot draws at
//! most one of its options, and an option names the candidates who run together when it is
//! drawn. A lone candidate is a slot with one option; a party nominee is a slot whose options
//! are its possible nominees; a coalition outcome is a slot whose options are sets of
//! candidates. Slots are drawn independently.

use serde::{Deserialize, Serialize};

use crate::{ModelError, rng::Rng};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Candidate {
    pub id: String,
    pub name: String,
    pub bloc: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SlotOption {
    /// Candidate ids who run when this option is drawn.
    pub run: Vec<String>,
    pub p: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Slot {
    pub id: String,
    /// Mutually exclusive outcomes; `1 - sum(p)` is the probability nobody from the slot runs.
    pub options: Vec<SlotOption>,
}

/// The validated field with ids resolved to indexes.
#[derive(Clone, Debug)]
pub struct Field {
    pub blocs: Vec<String>,
    pub candidates: Vec<Candidate>,
    pub bloc_of: Vec<usize>,
    slots: Vec<ResolvedSlot>,
    slot_of: Vec<usize>,
}

#[derive(Clone, Debug)]
struct ResolvedSlot {
    id: String,
    probabilities: Vec<f64>,
    runners: Vec<Vec<usize>>,
}

const PROBABILITY_TOLERANCE: f64 = 1e-9;

impl Field {
    pub fn new(blocs: Vec<String>, candidates: Vec<Candidate>, slots: &[Slot]) -> Result<Self, ModelError> {
        let invalid = |message: String| Err(ModelError::InvalidField(message));
        let mut bloc_of = Vec::with_capacity(candidates.len());
        for (i, candidate) in candidates.iter().enumerate() {
            if candidates[..i].iter().any(|c| c.id == candidate.id) {
                return invalid(format!("candidate `{}` is listed twice", candidate.id));
            }
            match blocs.iter().position(|b| *b == candidate.bloc) {
                Some(b) => bloc_of.push(b),
                None => {
                    return invalid(format!(
                        "candidate `{}` has unknown bloc `{}`",
                        candidate.id, candidate.bloc
                    ));
                }
            }
        }
        let mut slot_of = vec![usize::MAX; candidates.len()];
        let mut resolved = Vec::with_capacity(slots.len());
        for (s, slot) in slots.iter().enumerate() {
            let mut probabilities = Vec::new();
            let mut runners = Vec::new();
            for option in &slot.options {
                if !(0.0..=1.0).contains(&option.p) {
                    return invalid(format!(
                        "slot `{}` has probability {} outside [0, 1]",
                        slot.id, option.p
                    ));
                }
                let mut ids = Vec::new();
                for id in &option.run {
                    let Some(c) = candidates.iter().position(|c| c.id == *id) else {
                        return invalid(format!("slot `{}` names unknown candidate `{id}`", slot.id));
                    };
                    if slot_of[c] != usize::MAX && slot_of[c] != s {
                        return invalid(format!("candidate `{id}` appears in more than one slot"));
                    }
                    slot_of[c] = s;
                    ids.push(c);
                }
                probabilities.push(option.p);
                runners.push(ids);
            }
            let total: f64 = probabilities.iter().sum();
            if total > 1.0 + PROBABILITY_TOLERANCE {
                return invalid(format!("slot `{}` probabilities sum to {total} > 1", slot.id));
            }
            resolved.push(ResolvedSlot {
                id: slot.id.clone(),
                probabilities,
                runners,
            });
        }
        if let Some(c) = slot_of.iter().position(|s| *s == usize::MAX) {
            return invalid(format!("candidate `{}` belongs to no slot", candidates[c].id));
        }
        Ok(Self {
            blocs,
            candidates,
            bloc_of,
            slots: resolved,
            slot_of,
        })
    }

    pub fn len(&self) -> usize {
        self.candidates.len()
    }

    pub fn is_empty(&self) -> bool {
        self.candidates.is_empty()
    }

    pub fn index_of(&self, id: &str) -> Option<usize> {
        self.candidates.iter().position(|c| c.id == id)
    }

    /// The probability each candidate runs.
    pub fn p_run(&self) -> Vec<f64> {
        let mut p = vec![0.0; self.len()];
        for slot in &self.slots {
            for (probability, runners) in slot.probabilities.iter().zip(&slot.runners) {
                for c in runners {
                    p[*c] += probability;
                }
            }
        }
        p
    }

    /// Draws who runs.
    pub fn draw(&self, rng: &mut Rng, present: &mut [bool]) {
        present.fill(false);
        for slot in &self.slots {
            if let Some(option) = rng.categorical(&slot.probabilities) {
                for c in &slot.runners[option] {
                    present[*c] = true;
                }
            }
        }
    }

    /// The single most likely field: each slot at its most likely outcome (possibly nobody).
    pub fn modal(&self) -> Vec<bool> {
        let mut present = vec![false; self.len()];
        for slot in &self.slots {
            if let Some(option) = modal_option(slot) {
                for c in &slot.runners[option] {
                    present[*c] = true;
                }
            }
        }
        present
    }

    /// The modal field with `candidate`'s slot set to its most likely option that includes them.
    pub fn modal_with(&self, candidate: usize) -> Vec<bool> {
        let mut present = self.modal();
        let slot = &self.slots[self.slot_of[candidate]];
        for runners in &slot.runners {
            for c in runners {
                present[*c] = false;
            }
        }
        let best = slot
            .runners
            .iter()
            .enumerate()
            .filter(|(_, runners)| runners.contains(&candidate))
            .max_by(|(a, _), (b, _)| slot.probabilities[*a].total_cmp(&slot.probabilities[*b]))
            .map(|(option, _)| option);
        match best {
            Some(option) => {
                for c in &slot.runners[option] {
                    present[*c] = true;
                }
            }
            None => present[candidate] = true,
        }
        present
    }

    pub fn slot_id(&self, candidate: usize) -> &str {
        &self.slots[self.slot_of[candidate]].id
    }

    /// The field with each candidate's chance of running moved to `targets[c]` where given.
    /// Each option's probability is scaled by the geometric mean of its runners' factors
    /// (`target / p_run`), so an option with one runner lands exactly on the target; a slot whose
    /// options then sum above 1 is scaled back to 1. A candidate the field gives no chance keeps
    /// none: a hand-set 0 is a fact (a lost primary), not an estimate.
    pub fn with_run_targets(&self, targets: &[Option<f64>]) -> Field {
        let p_run = self.p_run();
        let factor = |c: usize| match targets.get(c).copied().flatten() {
            Some(t) if p_run[c] > 0.0 => t.clamp(0.0, 1.0) / p_run[c],
            _ => 1.0,
        };
        let mut field = self.clone();
        for slot in &mut field.slots {
            for (probability, runners) in slot.probabilities.iter_mut().zip(&slot.runners) {
                if runners.is_empty() {
                    continue;
                }
                let log_mean = runners.iter().map(|c| factor(*c).ln()).sum::<f64>() / runners.len() as f64;
                *probability *= log_mean.exp();
            }
            let total: f64 = slot.probabilities.iter().sum();
            if total > 1.0 {
                for probability in &mut slot.probabilities {
                    *probability /= total;
                }
            }
        }
        field
    }
}

fn modal_option(slot: &ResolvedSlot) -> Option<usize> {
    let none = 1.0 - slot.probabilities.iter().sum::<f64>();
    let (best, p) = slot
        .probabilities
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.total_cmp(b.1))?;
    (*p >= none).then_some(best)
}

#[cfg(test)]
pub(crate) mod tests {
    #[test]
    fn run_targets_move_single_runner_options_exactly() {
        let field = super::Field::new(
            vec!["left".into(), "right".into()],
            vec![candidate("a", "left"), candidate("b", "left"), candidate("c", "right")],
            &[
                slot(
                    "ab",
                    vec![option(&["a"], 0.6), option(&["b"], 0.2), option(&["a", "b"], 0.1)],
                ),
                slot("c", vec![option(&["c"], 0.0)]),
            ],
        )
        .unwrap();
        // a runs 0.7 by hand, b 0.3; c is ruled out.
        let moved = field.with_run_targets(&[Some(0.35), Some(0.6), Some(0.9)]);
        let p = moved.p_run();
        // a's own option halves, b's doubles, the joint option by their geometric mean (1 here),
        // so a candidate in a joint option lands near its target, not on it.
        let joint = 0.1 * (0.5f64 * 2.0).sqrt();
        assert!((p[0] - (0.3 + joint)).abs() < 1e-12, "{p:?}");
        assert!((p[1] - (0.4 + joint)).abs() < 1e-12, "{p:?}");
        assert_eq!(p[2], 0.0, "a hand-set 0 stays");
        // Over 1: scaled back.
        let over = field.with_run_targets(&[Some(0.9), Some(0.9), None]);
        let total: f64 = over.slots[0].probabilities.iter().sum();
        assert!((total - 1.0).abs() < 1e-12);
    }

    use super::*;

    pub(crate) fn candidate(id: &str, bloc: &str) -> Candidate {
        Candidate {
            id: id.into(),
            name: id.into(),
            bloc: bloc.into(),
        }
    }

    pub(crate) fn option(run: &[&str], p: f64) -> SlotOption {
        SlotOption {
            run: run.iter().map(|s| s.to_string()).collect(),
            p,
        }
    }

    pub(crate) fn slot(id: &str, options: Vec<SlotOption>) -> Slot {
        Slot { id: id.into(), options }
    }

    fn sample_field() -> Field {
        Field::new(
            vec!["left".into(), "right".into()],
            vec![
                candidate("a", "left"),
                candidate("b", "left"),
                candidate("c", "right"),
                candidate("d", "right"),
            ],
            &[
                slot("left", vec![option(&["a", "b"], 0.3), option(&["a"], 0.6)]),
                slot("c", vec![option(&["c"], 0.9)]),
                slot("d", vec![option(&["d"], 0.2)]),
            ],
        )
        .unwrap()
    }

    #[test]
    fn p_run_sums_options() {
        let p = sample_field().p_run();
        assert!((p[0] - 0.9).abs() < 1e-12);
        assert!((p[1] - 0.3).abs() < 1e-12);
        assert!((p[2] - 0.9).abs() < 1e-12);
        assert!((p[3] - 0.2).abs() < 1e-12);
    }

    #[test]
    fn draws_match_p_run() {
        let field = sample_field();
        let mut rng = Rng::new(8);
        let n = 100_000;
        let mut counts = [0usize; 4];
        let mut present = [false; 4];
        for _ in 0..n {
            field.draw(&mut rng, &mut present);
            for (c, p) in present.iter().enumerate() {
                counts[c] += usize::from(*p);
            }
            // b never runs without a.
            assert!(!present[1] || present[0]);
        }
        for (c, expected) in field.p_run().iter().enumerate() {
            assert!((counts[c] as f64 / n as f64 - expected).abs() < 0.01);
        }
    }

    #[test]
    fn modal_field_and_substitution() {
        let field = sample_field();
        assert_eq!(field.modal(), vec![true, false, true, false]);
        assert_eq!(field.modal_with(1), vec![true, true, true, false]);
        assert_eq!(field.modal_with(3), vec![true, false, true, true]);
    }

    #[test]
    fn rejects_bad_configuration() {
        let blocs = vec!["left".to_string()];
        let two = vec![candidate("a", "left"), candidate("b", "left")];
        assert!(Field::new(blocs.clone(), two.clone(), &[slot("a", vec![option(&["a"], 1.0)])]).is_err());
        assert!(
            Field::new(
                blocs.clone(),
                two.clone(),
                &[slot("x", vec![option(&["a"], 0.7), option(&["b"], 0.5)])]
            )
            .is_err()
        );
        assert!(
            Field::new(
                blocs.clone(),
                two,
                &[
                    slot("a", vec![option(&["a"], 0.5)]),
                    slot("b", vec![option(&["a", "b"], 0.5)])
                ]
            )
            .is_err()
        );
        assert!(Field::new(blocs, vec![candidate("a", "centre")], &[]).is_err());
    }
}
