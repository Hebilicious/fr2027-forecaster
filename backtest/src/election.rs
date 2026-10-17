//! One past election: the field (`<dir>/candidates.yaml`), the polls (`<dir>/polls.csv`, in the
//! clean poll file's columns), the page views and news volume ([`crate::signals`]) and the
//! official result.

use std::collections::BTreeSet;
use std::path::Path;

use fr2027_collectors::{Repo, Schemas, config::CandidatesConfig, polls};
use fr2027_model::{Field, Poll};
use jiff::civil::Date;
use serde::Deserialize;

use crate::{Error, Result, config::ElectionEntry, signals::ElectionSignals};

#[derive(Clone, Debug)]
pub struct Election {
    pub year: i16,
    pub round1: Date,
    pub round2: Date,
    pub config: CandidatesConfig,
    pub field: Field,
    pub polls: Vec<Poll>,
    pub signals: ElectionSignals,
    /// Prediction-market prices (`<dir>/markets.csv`), when the election has any.
    pub markets: Vec<crate::blend::MarketRow>,
    /// Who was on the round 1 ballot, by candidate index.
    pub ran: Vec<bool>,
    /// Round 1 result as fractions of expressed votes, by candidate index (0 for non-runners).
    pub round1_result: Vec<f64>,
    /// The two finalists, lower candidate index first.
    pub finalists: (usize, usize),
    /// The first finalist's share of the round 2 expressed votes (a fraction).
    pub round2_share: f64,
}

impl Election {
    pub fn winner(&self) -> usize {
        if self.round2_share > 0.5 {
            self.finalists.0
        } else {
            self.finalists.1
        }
    }

    pub fn id(&self, candidate: usize) -> &str {
        &self.config.candidates[candidate].id
    }
}

/// A row of `research/sources/results-2002-2022.csv`.
#[derive(Debug, Deserialize)]
struct ResultRow {
    year: i16,
    round: u8,
    candidate_name: String,
    share_expressed: f64,
}

pub fn load(repo: &Repo, schemas: &Schemas, entry: &ElectionEntry, results_file: &Path) -> Result<Election> {
    let dir = repo.root().join(&entry.dir);
    let config = CandidatesConfig::load_from(&dir.join("candidates.yaml"), schemas)?;
    let field = config.field()?;
    let polls_path = dir.join("polls.csv");
    let polls = polls::to_polls(&polls::read_rows(&polls_path)?);
    let known: BTreeSet<&str> = config.candidates.iter().map(|c| c.id.as_str()).collect();
    for poll in &polls {
        for scenario in &poll.scenarios {
            if let Some(unknown) = scenario
                .shares
                .iter()
                .find(|s| !known.contains(s.candidate_id.as_str()))
            {
                return Err(Error::invalid(
                    &polls_path,
                    format!(
                        "poll `{}` names unknown candidate `{}`",
                        poll.poll_id, unknown.candidate_id
                    ),
                ));
            }
        }
    }

    let signals = crate::signals::load(&dir, &|id| known.contains(id))?;
    let markets = crate::blend::load(&dir, &|id| known.contains(id))?;

    let results_path = repo.root().join(results_file);
    let bytes = std::fs::read(&results_path).map_err(|e| Error::invalid(&results_path, e.to_string()))?;
    let mut reader = csv::Reader::from_reader(bytes.as_slice());
    let rows: Vec<ResultRow> = reader
        .deserialize()
        .collect::<Result<_, _>>()
        .map_err(|e| Error::invalid(&results_path, e.to_string()))?;
    let invalid = |message: String| Error::invalid(&results_path, format!("{}: {message}", entry.year));
    let index_of = |name: &str| {
        config
            .candidates
            .iter()
            .position(|c| c.name == name)
            .ok_or_else(|| invalid(format!("`{name}` is not in {}", dir.join("candidates.yaml").display())))
    };
    let n = config.candidates.len();
    let mut ran = vec![false; n];
    let mut round1_result = vec![0.0; n];
    let mut round2 = Vec::new();
    for row in rows
        .iter()
        .filter(|r| r.year == entry.year && !r.candidate_name.starts_with('_'))
    {
        let c = index_of(&row.candidate_name)?;
        match row.round {
            1 => {
                ran[c] = true;
                round1_result[c] = row.share_expressed / 100.0;
            }
            2 => round2.push((c, row.share_expressed / 100.0)),
            other => return Err(invalid(format!("round {other}"))),
        }
    }
    let total: f64 = round1_result.iter().sum();
    if (total - 1.0).abs() > 0.002 {
        return Err(invalid(format!("round 1 shares sum to {:.2}%", total * 100.0)));
    }
    let [(i, share_i), (j, share_j)] = round2[..] else {
        return Err(invalid("round 2 needs exactly two candidates".into()));
    };
    let mut by_share: Vec<usize> = (0..n).filter(|c| ran[*c]).collect();
    by_share.sort_by(|a, b| round1_result[*b].total_cmp(&round1_result[*a]));
    let mut top_two = [by_share[0], by_share[1]];
    let mut pair = [i, j];
    top_two.sort_unstable();
    pair.sort_unstable();
    if top_two != pair {
        return Err(invalid("the round 2 candidates are not round 1's top two".into()));
    }
    let round2_share = if i < j {
        share_i / (share_i + share_j)
    } else {
        share_j / (share_i + share_j)
    };
    // The field must be exactly the runners: the replays condition on who actually ran.
    let p_run = field.p_run();
    if let Some(c) = (0..n).find(|c| (p_run[*c] > 0.5) != ran[*c]) {
        return Err(invalid(format!(
            "candidates.yaml gives `{}` p_run {} but the results say ran = {}",
            config.candidates[c].id, p_run[c], ran[c]
        )));
    }
    Ok(Election {
        year: entry.year,
        round1: entry.round1,
        round2: entry.round2,
        config,
        field,
        polls,
        signals,
        markets,
        ran,
        round1_result,
        finalists: (i.min(j), i.max(j)),
        round2_share,
    })
}
