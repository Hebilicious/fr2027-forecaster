//! The forward log: signal flags recorded when they are seen, so they can be scored later against
//! what the polls did, without hindsight. Few past elections have X data, so the spec validates
//! the social layer forward (SPEC.md, Backtesting).
//!
//! `fr2027 forward` writes `data/forward/YYYY-MM-DDTHH.json` with the flags not logged before,
//! and never edits a file: a flag's `detected_at` is when it was first seen. Two kinds:
//!
//! - `spike`: a news and attention spike ([`fr2027_model::signals::detect`]), on its day.
//! - `x_shift`: the share of sampled X posts supporting a candidate moved between two drops by
//!   more than both 95% intervals allow (the intervals do not overlap), with both drops'
//!   double-labelling kappa at least [`MIN_KAPPA`]; on the later drop's window end.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::{Rejection, Repo, Result, SchemaKind, Schemas, grok::GrokRow, read_json_documents};

/// Below this agreement between two blind labelling passes, a drop's stance shares are too
/// noisy to flag a shift. A threshold chosen for this log, not one the owner or the spec set.
pub const MIN_KAPPA: f64 = 0.4;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlagFile {
    pub schema_version: String,
    pub detected_at: String,
    pub flags: Vec<Flag>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FlagKind {
    Spike,
    XShift,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Flag {
    pub kind: FlagKind,
    pub candidate_id: String,
    /// The day the flag is about.
    pub date: String,
    /// Spike: mean log ratio to the baseline. X shift: the change in the support share.
    pub score: f64,
    pub detail: String,
}

impl Flag {
    pub fn key(&self) -> (FlagKind, String, String) {
        (self.kind.clone(), self.candidate_id.clone(), self.date.clone())
    }
}

/// A flag and when it was first logged.
#[derive(Clone, Debug, PartialEq)]
pub struct Logged {
    pub detected_at: String,
    pub flag: Flag,
}

/// Every logged flag, oldest file first, and the files that failed their schema.
pub fn read(repo: &Repo, schemas: &Schemas) -> Result<(Vec<Logged>, Vec<Rejection>)> {
    let (accepted, rejections) =
        read_json_documents::<FlagFile>(&repo.forward_dir(), schemas, SchemaKind::ForwardFlags, |stem, file| {
            let expected = file.detected_at.get(..13).unwrap_or_default();
            if stem == expected {
                Vec::new()
            } else {
                vec![format!(
                    "file name `{stem}` must be the hour it was written, `{expected}`"
                )]
            }
        })?;
    let logged = accepted
        .into_iter()
        .flat_map(|a| {
            let detected_at = a.document.detected_at.clone();
            a.document.flags.into_iter().map(move |flag| Logged {
                detected_at: detected_at.clone(),
                flag,
            })
        })
        .collect();
    Ok((logged, rejections))
}

/// The keys already logged, so a flag is written once.
pub fn logged_keys(logged: &[Logged]) -> BTreeSet<(FlagKind, String, String)> {
    logged.iter().map(|l| l.flag.key()).collect()
}

/// X stance shifts between consecutive drops of each candidate (see the module notes).
pub fn x_shifts(rows: &[GrokRow]) -> Vec<Flag> {
    let mut by_candidate: BTreeMap<&str, Vec<&GrokRow>> = BTreeMap::new();
    for row in rows.iter().filter(|r| r.support_share.is_some()) {
        by_candidate.entry(row.candidate_id.as_str()).or_default().push(row);
    }
    let mut flags = Vec::new();
    for (candidate, mut drops) in by_candidate {
        drops.sort_by(|a, b| a.window_end.cmp(&b.window_end));
        for pair in drops.windows(2) {
            let (before, after) = (pair[0], pair[1]);
            let reliable = |r: &GrokRow| r.drop_kappa.is_some_and(|k| k >= MIN_KAPPA);
            if !(reliable(before) && reliable(after)) {
                continue;
            }
            let (Some(b), Some(b_lo), Some(b_hi), Some(a), Some(a_lo), Some(a_hi)) = (
                before.support_share,
                before.support_low,
                before.support_high,
                after.support_share,
                after.support_low,
                after.support_high,
            ) else {
                continue;
            };
            if a_lo > b_hi || a_hi < b_lo {
                flags.push(Flag {
                    kind: FlagKind::XShift,
                    candidate_id: candidate.to_string(),
                    date: after.window_end.get(..10).unwrap_or_default().to_string(),
                    score: ((a - b) * 10_000.0).round() / 10_000.0,
                    detail: format!(
                        "support {b:.2} [{b_lo:.2}, {b_hi:.2}] in the drop to {} → {a:.2} [{a_lo:.2}, {a_hi:.2}] in the drop to {}",
                        before.window_end, after.window_end
                    ),
                });
            }
        }
    }
    flags
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drop(end: &str, share: f64, lo: f64, hi: f64, kappa: f64) -> GrokRow {
        let row = serde_json::json!({
            "window_start": "", "window_end": end, "collected_at": end, "candidate_id": "le_pen",
            "support_share": share, "support_low": lo, "support_high": hi, "drop_kappa": kappa,
            "source_file": "", "content_hash": "",
        });
        serde_json::from_value(row).unwrap()
    }

    #[test]
    fn flags_shifts_beyond_both_intervals_only() {
        let rows = vec![
            drop("2026-10-16T12:00:00Z", 0.30, 0.25, 0.35, 0.7),
            drop("2026-10-17T00:00:00Z", 0.33, 0.28, 0.38, 0.7),
            drop("2026-10-17T12:00:00Z", 0.45, 0.40, 0.50, 0.7),
            drop("2026-10-18T00:00:00Z", 0.30, 0.25, 0.35, 0.2),
        ];
        let flags = x_shifts(&rows);
        assert_eq!(flags.len(), 1, "{flags:?}");
        assert_eq!((flags[0].date.as_str(), flags[0].score), ("2026-10-17", 0.12));
    }
}
