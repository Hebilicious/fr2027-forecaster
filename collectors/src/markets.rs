//! Prediction-market snapshots: `data/raw/markets/YYYY-MM-DDTHH.json` → `data/clean/markets.csv`.
//!
//! The clean file keeps every snapshot of the last seven days and the last snapshot of each
//! earlier day, and only contracts matched to a candidate, so it stays small enough to diff.

use std::collections::BTreeMap;

use jiff::{Timestamp, ToSpan};
use serde::{Deserialize, Serialize};

use crate::{
    Rejection, Repo, Result, SchemaKind, Schemas, config::CandidatesConfig, csv_bytes, read_csv, read_json_documents,
    write, write_quarantine,
};

pub const CLEAN_FILE: &str = "markets.csv";
const HEADER: &str = "fetched_at,market_id,venue,question,candidate_id,label,price,bid,ask,last,volume,source_file";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Snapshot {
    pub schema_version: String,
    pub fetched_at: String,
    pub markets: Vec<Market>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Market {
    pub id: String,
    pub venue: String,
    pub question: String,
    pub event: String,
    pub url: String,
    pub status: String,
    pub error: Option<String>,
    pub contracts: Vec<Contract>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Contract {
    pub label: String,
    pub candidate_id: Option<String>,
    pub price: Option<f64>,
    pub bid: Option<f64>,
    pub ask: Option<f64>,
    pub last: Option<f64>,
    pub volume: Option<f64>,
}

/// The probability a contract implies: the bid–ask midpoint when the book is tight enough to
/// mean something (both sides present, at most 0.10 apart), otherwise the last trade.
pub fn implied_price(bid: Option<f64>, ask: Option<f64>, last: Option<f64>) -> Option<f64> {
    match (bid, ask) {
        (Some(b), Some(a)) if a >= b && a - b <= 0.10 + 1e-9 => Some(((a + b) / 2.0 * 10_000.0).round() / 10_000.0),
        _ => last,
    }
}

/// `2026-10-09T20:17:03Z` → `2026-10-09T20`, the file stem of a snapshot.
pub fn file_stem(fetched_at: &str) -> String {
    fetched_at.get(..13).unwrap_or(fetched_at).to_string()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketRow {
    pub fetched_at: String,
    pub market_id: String,
    pub venue: String,
    pub question: String,
    pub candidate_id: String,
    pub label: String,
    pub price: Option<f64>,
    pub bid: Option<f64>,
    pub ask: Option<f64>,
    pub last: Option<f64>,
    pub volume: Option<f64>,
    pub source_file: String,
}

#[derive(Debug, Default)]
pub struct MarketsIngest {
    pub rows: Vec<MarketRow>,
    pub accepted: usize,
    pub rejections: Vec<Rejection>,
    /// The newest accepted snapshot, in full.
    pub latest: Option<Snapshot>,
}

/// Errors in a snapshot beyond its schema.
pub fn check(stem: Option<&str>, snapshot: &Snapshot, config: &CandidatesConfig) -> Vec<String> {
    let mut errors = Vec::new();
    if let Some(stem) = stem {
        let expected = file_stem(&snapshot.fetched_at);
        if stem != expected {
            errors.push(format!("file name `{stem}` must be the fetch hour `{expected}`"));
        }
    }
    for market in &snapshot.markets {
        for contract in &market.contracts {
            if let Some(id) = &contract.candidate_id
                && config.candidate(id).is_none()
            {
                errors.push(format!("{}: unknown candidate `{id}`", market.id));
            }
        }
    }
    errors
}

pub fn ingest(repo: &Repo, schemas: &Schemas, config: &CandidatesConfig) -> Result<MarketsIngest> {
    let (accepted, rejections) = read_json_documents::<Snapshot>(
        &repo.raw_markets_dir(),
        schemas,
        SchemaKind::Markets,
        |stem, snapshot| check(Some(stem), snapshot, config),
    )?;
    let newest = accepted.iter().map(|a| a.document.fetched_at.clone()).max();
    let cutoff = newest
        .as_deref()
        .and_then(|n| n.parse::<Timestamp>().ok())
        .and_then(|n| n.checked_sub(168.hours()).ok())
        .map(|c| c.strftime("%Y-%m-%dT%H:%M:%SZ").to_string());
    // The last snapshot of each day.
    let mut last_of_day: BTreeMap<String, String> = BTreeMap::new();
    for a in &accepted {
        let day = a.document.fetched_at[..10].to_string();
        let entry = last_of_day.entry(day).or_default();
        if a.document.fetched_at > *entry {
            entry.clone_from(&a.document.fetched_at);
        }
    }
    let mut ingest = MarketsIngest {
        accepted: accepted.len(),
        rejections,
        ..Default::default()
    };
    for a in &accepted {
        let snapshot = &a.document;
        let recent = cutoff.as_deref().is_some_and(|c| snapshot.fetched_at.as_str() >= c);
        let closing = last_of_day.get(&snapshot.fetched_at[..10]) == Some(&snapshot.fetched_at);
        if !recent && !closing {
            continue;
        }
        let source_file = repo.relative(&a.path);
        for market in &snapshot.markets {
            for contract in &market.contracts {
                let Some(candidate_id) = &contract.candidate_id else {
                    continue;
                };
                ingest.rows.push(MarketRow {
                    fetched_at: snapshot.fetched_at.clone(),
                    market_id: market.id.clone(),
                    venue: market.venue.clone(),
                    question: market.question.clone(),
                    candidate_id: candidate_id.clone(),
                    label: contract.label.clone(),
                    price: contract.price,
                    bid: contract.bid,
                    ask: contract.ask,
                    last: contract.last,
                    volume: contract.volume,
                    source_file: source_file.clone(),
                });
            }
        }
    }
    ingest.rows.sort_by(|a, b| {
        (&a.fetched_at, &a.market_id, &a.candidate_id).cmp(&(&b.fetched_at, &b.market_id, &b.candidate_id))
    });
    ingest.latest = accepted.into_iter().last().map(|a| a.document);
    Ok(ingest)
}

pub fn clean_bytes(ingest: &MarketsIngest) -> Result<Vec<u8>> {
    csv_bytes(&ingest.rows, HEADER)
}

pub fn write_clean(repo: &Repo, ingest: &MarketsIngest) -> Result<()> {
    write(&repo.clean_dir().join(CLEAN_FILE), &clean_bytes(ingest)?)?;
    write_quarantine(repo, "markets", &ingest.rejections)
}

pub fn read_clean_rows(repo: &Repo) -> Result<Vec<MarketRow>> {
    read_csv(&repo.clean_dir().join(CLEAN_FILE))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn price_is_the_midpoint_of_a_tight_book() {
        assert_eq!(implied_price(Some(0.41), Some(0.411), Some(0.4)), Some(0.4105));
        assert_eq!(implied_price(Some(0.10), Some(0.30), Some(0.2)), Some(0.2));
        assert_eq!(implied_price(None, Some(0.3), Some(0.25)), Some(0.25));
        assert_eq!(implied_price(None, None, None), None);
    }

    #[test]
    fn stems_are_the_fetch_hour() {
        assert_eq!(file_stem("2026-10-09T20:17:03Z"), "2026-10-09T20");
    }
}
