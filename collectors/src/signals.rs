//! The model's view of the live signals: Wikipedia page views (`data/clean/attention.csv`) and
//! GDELT article counts (`data/clean/gdelt.csv`), by candidate and day.

use fr2027_model::{SignalSource, Signals};

use crate::{Error, Repo, Result, attention, gdelt};

/// Reads both clean files. A missing file is no signal, so a repository without either still
/// forecasts from its polls.
pub fn read_clean(repo: &Repo) -> Result<Signals> {
    let mut signals = Signals::new();
    let present = |file: &str| repo.clean_dir().join(file).exists();
    if present(attention::CLEAN_FILE) {
        for row in attention::read_clean_rows(repo)? {
            let date = parse_date(repo, attention::CLEAN_FILE, &row.date)?;
            signals.add(&row.candidate_id, SignalSource::Attention, date, row.views as f64);
        }
    }
    if present(gdelt::CLEAN_FILE) {
        for row in gdelt::read_clean_rows(repo)? {
            let date = parse_date(repo, gdelt::CLEAN_FILE, &row.date)?;
            signals.add(&row.candidate_id, SignalSource::News, date, row.articles as f64);
        }
    }
    Ok(signals)
}

fn parse_date(repo: &Repo, file: &str, text: &str) -> Result<jiff::civil::Date> {
    text.parse()
        .map_err(|_| Error::invalid(&repo.clean_dir().join(file), format!("`{text}` is not a date")))
}
