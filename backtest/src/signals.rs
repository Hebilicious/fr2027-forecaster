//! A past election's page views and news volume: `<dir>/signals.yaml` says where they come from,
//! and `fr2027 backtest-signals <year>` writes `<dir>/attention.csv` (daily user views of each
//! candidate's French Wikipedia article) and `<dir>/gdelt.csv` (daily GDELT counts of French
//! articles naming them), the same series the live collectors record.

use std::collections::BTreeMap;
use std::path::Path;

use fr2027_collectors::{read_csv, sources::CandidateSources};
use fr2027_model::{SignalSource, Signals};
use jiff::civil::Date;
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

pub const SOURCES_FILE: &str = "signals.yaml";
pub const ATTENTION_FILE: &str = "attention.csv";
pub const NEWS_FILE: &str = "gdelt.csv";

/// `<dir>/signals.yaml`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignalSources {
    /// First and last day fetched.
    pub start: Date,
    pub end: Date,
    pub candidates: BTreeMap<String, CandidateSources>,
}

/// A row of `<dir>/attention.csv`.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AttentionRow {
    pub date: Date,
    pub candidate_id: String,
    pub article: String,
    pub views: u64,
    pub retrieved_at: String,
}

/// A row of `<dir>/gdelt.csv`.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NewsRow {
    pub date: Date,
    pub candidate_id: String,
    pub query: String,
    pub articles: u64,
    pub total_articles: u64,
    pub retrieved_at: String,
}

pub const ATTENTION_HEADER: &str = "date,candidate_id,article,views,retrieved_at";
pub const NEWS_HEADER: &str = "date,candidate_id,query,articles,total_articles,retrieved_at";

/// The signals of one election, as fetched. Empty when the files are not there yet.
#[derive(Clone, Debug, Default)]
pub struct ElectionSignals {
    pub attention: Vec<AttentionRow>,
    pub news: Vec<NewsRow>,
}

impl ElectionSignals {
    /// The rows as the model reads them: page views and article counts by candidate and day.
    pub fn to_model(&self) -> Signals {
        let mut signals = Signals::new();
        for row in &self.attention {
            signals.add(&row.candidate_id, SignalSource::Attention, row.date, row.views as f64);
        }
        for row in &self.news {
            signals.add(&row.candidate_id, SignalSource::News, row.date, row.articles as f64);
        }
        signals
    }
}

impl SignalSources {
    pub fn load(dir: &Path) -> Result<Self> {
        let path = dir.join(SOURCES_FILE);
        let text = std::fs::read_to_string(&path).map_err(|e| Error::invalid(&path, e.to_string()))?;
        let sources: Self = serde_saphyr::from_str(&text).map_err(|e| Error::invalid(&path, e.to_string()))?;
        if sources.start > sources.end {
            return Err(Error::invalid(&path, "start is after end"));
        }
        if let Some((id, _)) = sources.candidates.iter().find(|(_, c)| c.names.is_empty()) {
            return Err(Error::invalid(&path, format!("`{id}` has no names")));
        }
        Ok(sources)
    }
}

/// Reads `<dir>/attention.csv` and `<dir>/gdelt.csv`, when they exist, and checks that every row
/// names a known candidate and falls inside the fetched window. An election without
/// `signals.yaml` has no signals.
pub fn load(dir: &Path, known: &dyn Fn(&str) -> bool) -> Result<ElectionSignals> {
    if !dir.join(SOURCES_FILE).exists() {
        return Ok(ElectionSignals::default());
    }
    let sources = SignalSources::load(dir)?;
    if let Some(id) = sources.candidates.keys().find(|id| !known(id)) {
        return Err(Error::invalid(
            dir.join(SOURCES_FILE),
            format!("unknown candidate `{id}`"),
        ));
    }
    let read = |file: &str| -> Result<Option<std::path::PathBuf>> {
        let path = dir.join(file);
        Ok(path.exists().then_some(path))
    };
    let mut signals = ElectionSignals::default();
    if let Some(path) = read(ATTENTION_FILE)? {
        signals.attention = read_csv(&path)?;
        check_rows(
            &path,
            &sources,
            known,
            signals.attention.iter().map(|r| (r.date, r.candidate_id.as_str())),
        )?;
    }
    if let Some(path) = read(NEWS_FILE)? {
        signals.news = read_csv(&path)?;
        check_rows(
            &path,
            &sources,
            known,
            signals.news.iter().map(|r| (r.date, r.candidate_id.as_str())),
        )?;
    }
    Ok(signals)
}

fn check_rows<'a>(
    path: &Path,
    sources: &SignalSources,
    known: &dyn Fn(&str) -> bool,
    rows: impl Iterator<Item = (Date, &'a str)>,
) -> Result<()> {
    for (date, candidate) in rows {
        if !known(candidate) {
            return Err(Error::invalid(path, format!("unknown candidate `{candidate}`")));
        }
        if date < sources.start || date > sources.end {
            return Err(Error::invalid(
                path,
                format!("{date} is outside {}–{}", sources.start, sources.end),
            ));
        }
    }
    Ok(())
}
