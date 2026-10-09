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
