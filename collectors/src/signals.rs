//! The model's view of the live signals: Wikipedia page views (`data/clean/attention.csv`), GDELT
//! article counts and their tone (`data/clean/gdelt.csv`) and the share of sampled X posts supporting each
//! candidate (`data/clean/grok.csv`), by candidate and day.

use std::collections::BTreeMap;

use fr2027_model::{SignalSource, Signals};

use crate::{Error, Repo, Result, attention, forward::MIN_KAPPA, gdelt, grok};

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
            // The day's tone against every French article's, times the articles (fetches since
            // 2026-10-18 carry it).
            if let (Some(tone), Some(all)) = (row.tone, row.total_tone)
                && row.articles > 0
            {
                signals.add(
                    &row.candidate_id,
                    SignalSource::NewsTone,
                    date,
                    (tone - all) * row.articles as f64,
                );
            }
            // How Jev read the day's articles (fetches since 2026-10-19).
            if let Some(stance) = row.stance() {
                signals.add(&row.candidate_id, SignalSource::StanceRead, date, stance.read as f64);
                signals.add(
                    &row.candidate_id,
                    SignalSource::StanceNet,
                    date,
                    stance.favourable - stance.unfavourable,
                );
            }
        }
    }
    if present(grok::CLEAN_FILE) {
        // A day's support share is the mean over its drops (one every 12 hours) whose two blind
        // labelling passes agreed well enough.
        let mut by_day: BTreeMap<(String, String), Vec<f64>> = BTreeMap::new();
        for row in grok::read_clean_rows(repo)? {
            let (Some(share), Some(kappa)) = (row.support_share, row.drop_kappa) else {
                continue;
            };
            if kappa >= MIN_KAPPA {
                let day = row.window_end.get(..10).unwrap_or_default().to_string();
                by_day.entry((row.candidate_id, day)).or_default().push(share);
            }
        }
        for ((candidate_id, day), shares) in by_day {
            let date = parse_date(repo, grok::CLEAN_FILE, &day)?;
            let mean = shares.iter().sum::<f64>() / shares.len() as f64;
            signals.add(&candidate_id, SignalSource::XSupport, date, mean);
        }
    }
    Ok(signals)
}

fn parse_date(repo: &Repo, file: &str, text: &str) -> Result<jiff::civil::Date> {
    text.parse()
        .map_err(|_| Error::invalid(&repo.clean_dir().join(file), format!("`{text}` is not a date")))
}
