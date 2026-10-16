//! The UI's data, one JSON document per name. `fr2027 export` writes them as static files
//! under `api/` for the public site; `fr2027 serve` computes them on each request. Both use
//! these functions, so the local app and the public page show the same thing.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use anyhow::{Context, Result, anyhow};
use fr2027_collectors::{
    Repo, attention, events, grok, list_files, markets, news, polls, read, sources::SourcesConfig,
};
use jiff::{Timestamp, ToSpan, tz::TimeZone};
use serde_json::{Value, json};

use crate::data::Inputs;
use crate::forecast::SERIES_FILE;

/// Every document the UI reads, as `api/<name>.json`.
pub const NAMES: [&str; 7] = [
    "forecast",
    "series",
    "polls",
    "history",
    "signals",
    "health",
    "candidates",
];

/// The document called `name`, or `None` when there is no such document.
pub fn document(repo: &Repo, name: &str, now: Timestamp) -> Result<Option<Value>> {
    Ok(Some(match name {
        "forecast" => {
            latest_forecast(repo)?
                .ok_or_else(|| anyhow!("no forecast yet: run `moon run cli:forecast`"))?
                .1
        }
        "series" => read_json(&repo.forecasts_dir().join(SERIES_FILE))?,
        "polls" => json!({ "rows": polls::read_clean_rows(repo)? }),
        "history" => history(repo)?,
        "signals" => signals(repo, now)?,
        "health" => health(repo, now)?,
        "candidates" => {
            let inputs = Inputs::load(repo)?;
            serde_json::to_value(&inputs.config)?
        }
        _ => return Ok(None),
    }))
}

fn read_json(path: &Path) -> Result<Value> {
    serde_json::from_slice(&read(path)?).with_context(|| format!("{} is not JSON", path.display()))
}

fn ts(t: Timestamp) -> String {
    t.strftime("%Y-%m-%dT%H:%M:%SZ").to_string()
}

pub fn is_forecast_name(name: &str) -> bool {
    // YYYY-MM-DDTHH.json
    let bytes = name.as_bytes();
    bytes.len() == 18
        && name.ends_with(".json")
        && bytes[..13].iter().enumerate().all(|(i, b)| match i {
            4 | 7 => *b == b'-',
            10 => *b == b'T',
            _ => b.is_ascii_digit(),
        })
}

pub fn forecast_names(repo: &Repo) -> Result<Vec<String>> {
    Ok(list_files(&repo.forecasts_dir(), "json")?
        .iter()
        .filter_map(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
        .filter(|n| is_forecast_name(n))
        .collect())
}

pub fn latest_forecast(repo: &Repo) -> Result<Option<(String, Value)>> {
    match forecast_names(repo)?.pop() {
        Some(name) => {
            let value = read_json(&repo.forecasts_dir().join(&name))?;
            Ok(Some((name, value)))
        }
        None => Ok(None),
    }
}

/// Every forecast's headline numbers, oldest first.
fn history(repo: &Repo) -> Result<Value> {
    let mut runs = Vec::new();
    for name in forecast_names(repo)? {
        let forecast = read_json(&repo.forecasts_dir().join(&name))?;
        let candidates: Vec<Value> = forecast["candidates"]
            .as_array()
            .map(|cs| {
                cs.iter()
                    .map(|c| {
                        json!({
                            "candidate_id": c["candidate_id"],
                            "p_win": c["p_win"],
                            "p_qualify_r1": c["p_qualify_r1"],
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();
        runs.push(json!({
            "file": name,
            "as_of": forecast["as_of"],
            "generated_at": forecast["generated_at"],
            "new_polls": forecast["changes"]["new_polls"],
            "candidates": candidates,
        }));
    }
    Ok(json!({ "runs": runs }))
}

/// The newest raw file in `dir`, parsed.
fn newest_raw(dir: &Path) -> Result<Option<Value>> {
    match list_files(dir, "json")?.pop() {
        Some(path) => Ok(Some(read_json(&path)?)),
        None => Ok(None),
    }
}

fn hours_since(now: Timestamp, when: Option<&str>) -> Option<i64> {
    let then = when?.parse::<Timestamp>().ok()?;
    Some(now.duration_since(then).as_secs() / 3600)
}

/// Freshness of every input and output, with plain-language warnings. Times come from the data
/// itself, not from file dates, so a fresh checkout reports the same thing.
fn health(repo: &Repo, now: Timestamp) -> Result<Value> {
    let mut warnings = Vec::new();
    let latest = match latest_forecast(repo)? {
        Some((name, forecast)) => {
            if let Some(hours) = hours_since(now, forecast["generated_at"].as_str())
                && hours > 36
            {
                warnings.push(format!("The latest forecast is {hours} hours old."));
            }
            if let Some(model_warnings) = forecast["warnings"].as_array() {
                warnings.extend(model_warnings.iter().filter_map(|w| w.as_str().map(String::from)));
            }
            json!({
                "file": name,
                "generated_at": forecast["generated_at"],
                "as_of": forecast["as_of"],
                "last_field_end": forecast["aggregation"]["last_field_end"],
                "polls_used": forecast["aggregation"]["polls_used"],
            })
        }
        None => {
            warnings.push("No forecast has been run yet.".into());
            Value::Null
        }
    };

    let last_poll = polls::read_clean_rows(repo)
        .ok()
        .and_then(|rows| rows.iter().map(|r| r.published_at).max())
        .map(|d| d.to_string());
    let markets = newest_raw(&repo.raw_markets_dir())?;
    let news = newest_raw(&repo.raw_news_dir())?;
    let attention = newest_raw(&repo.raw_attention_dir())?;
    let grok_drop = newest_raw(&repo.raw_grok_dir())?;
    let event = events::read_clean_rows(repo).ok().and_then(|rows| rows.last().cloned());
    let field = |doc: &Option<Value>, key: &str| doc.as_ref().and_then(|d| d[key].as_str().map(String::from));
    // name, last update, warn after this many hours (None: never), expected cadence
    let sources: [(&str, Option<String>, Option<i64>, &str); 6] = [
        ("polls", last_poll, None, "when a poll is published"),
        ("markets", field(&markets, "fetched_at"), Some(3), "hourly"),
        ("news", field(&news, "fetched_at"), Some(3), "hourly"),
        ("attention", field(&attention, "fetched_at"), Some(50), "daily"),
        (
            "x",
            field(&grok_drop, "collected_at"),
            Some(48),
            "Grok Bot, every 6 hours",
        ),
        ("events", event.map(|e| e.reported_at), None, "Grok Bot, daily"),
    ];
    let mut collectors = Vec::new();
    for (name, updated, stale_after, cadence) in sources {
        let age = hours_since(now, updated.as_deref());
        let stale = matches!((age, stale_after), (Some(a), Some(limit)) if a > limit);
        if stale {
            warnings.push(format!("No {name} update for {} hours.", age.unwrap_or_default()));
        }
        collectors.push(json!({ "name": name, "updated": updated, "cadence": cadence, "stale": stale }));
    }

    let mut quarantine = serde_json::Map::new();
    for area in ["polls", "grok", "markets", "news", "attention", "events"] {
        let files: Vec<String> = std::fs::read_dir(repo.quarantine_dir().join(area))
            .map(|entries| {
                let mut files: Vec<String> = entries
                    .filter_map(|e| e.ok())
                    .map(|e| e.file_name().to_string_lossy().into_owned())
                    .filter(|n| !n.ends_with(".error.txt"))
                    .collect();
                files.sort();
                files
            })
            .unwrap_or_default();
        if !files.is_empty() {
            warnings.push(format!(
                "{} quarantined {area} file(s) in data/quarantine/{area}/.",
                files.len()
            ));
        }
        quarantine.insert(area.into(), json!(files));
    }
    Ok(json!({
        "checked_at": ts(now),
        "latest_forecast": latest,
        "forecast_count": forecast_names(repo)?.len(),
        "collectors": collectors,
        "quarantine": quarantine,
        "warnings": warnings,
    }))
}

/// Markets, attention, news, X activity and events, trimmed to what the Signals view shows.
fn signals(repo: &Repo, now: Timestamp) -> Result<Value> {
    let inputs = Inputs::load(repo)?;
    let today = now.to_zoned(TimeZone::UTC).date();
    let since = |days: i32| -> String {
        today
            .checked_sub(days.days())
            .map(|d| d.to_string())
            .unwrap_or_default()
    };

    Ok(json!({
        "generated_at": ts(now),
        "markets": markets_signal(repo, &since(120))?,
        "attention": attention_signal(repo, &since(90))?,
        "news": news_signal(repo, &inputs.sources, &since(30))?,
        "x": x_signal(repo)?,
        "events": events_signal(repo)?,
    }))
}

fn markets_signal(repo: &Repo, since: &str) -> Result<Value> {
    let latest = newest_raw(&repo.raw_markets_dir())?;
    let Some(latest) = latest else {
        return Ok(json!({ "fetched_at": null, "sources": [], "latest": [], "unmatched": [], "history": [] }));
    };
    let snapshot: markets::Snapshot = serde_json::from_value(latest)?;
    let mut sources_out = Vec::new();
    let mut quotes = Vec::new();
    let mut unmatched = Vec::new();
    for market in &snapshot.markets {
        sources_out.push(json!({
            "id": market.id, "venue": market.venue, "question": market.question,
            "url": market.url, "status": market.status, "error": market.error,
        }));
        for contract in &market.contracts {
            match &contract.candidate_id {
                Some(id) => quotes.push(json!({
                    "candidate_id": id, "venue": market.venue, "question": market.question,
                    "price": contract.price, "bid": contract.bid, "ask": contract.ask, "volume": contract.volume,
                })),
                None if contract.price.unwrap_or(0.0) >= 0.01 => unmatched.push(json!({
                    "label": contract.label, "venue": market.venue, "question": market.question, "price": contract.price,
                })),
                None => {}
            }
        }
    }
    // Daily closing price per venue, question and candidate.
    let mut closing: BTreeMap<(String, String, String, String), Option<f64>> = BTreeMap::new();
    for row in markets::read_clean_rows(repo).unwrap_or_default() {
        let date = row.fetched_at[..10].to_string();
        if date.as_str() < since {
            continue;
        }
        closing.insert((date, row.venue, row.question, row.candidate_id), row.price);
    }
    let history: Vec<Value> = closing
        .into_iter()
        .map(|((date, venue, question, candidate_id), price)| {
            json!({ "date": date, "venue": venue, "question": question, "candidate_id": candidate_id, "price": price })
        })
        .collect();
    Ok(json!({
        "fetched_at": snapshot.fetched_at,
        "sources": sources_out,
        "latest": quotes,
        "unmatched": unmatched,
        "history": history,
    }))
}

fn attention_signal(repo: &Repo, since: &str) -> Result<Value> {
    let rows: Vec<Value> = attention::read_clean_rows(repo)
        .unwrap_or_default()
        .into_iter()
        .filter(|r| r.date.as_str() >= since)
        .map(|r| json!({ "date": r.date, "candidate_id": r.candidate_id, "views": r.views }))
        .collect();
    let fetched_at = newest_raw(&repo.raw_attention_dir())?.and_then(|d| d["fetched_at"].as_str().map(String::from));
    Ok(json!({ "fetched_at": fetched_at, "rows": rows }))
}

fn news_signal(repo: &Repo, sources: &SourcesConfig, since: &str) -> Result<Value> {
    let rows = news::read_clean_rows(repo).unwrap_or_default();
    let mut daily: BTreeMap<(String, String), u32> = BTreeMap::new();
    for row in &rows {
        let date = row.published_at[..10].to_string();
        if date.as_str() < since {
            continue;
        }
        for id in row.candidate_ids.split_whitespace() {
            *daily.entry((date.clone(), id.to_string())).or_default() += 1;
        }
    }
    let headlines: Vec<Value> = rows
        .iter()
        .rev()
        .take(120)
        .map(|r| {
            json!({
                "published_at": r.published_at,
                "outlet": r.outlet,
                "title": r.title,
                "url": r.url,
                "candidate_ids": r.candidate_ids.split_whitespace().collect::<Vec<_>>(),
            })
        })
        .collect();
    let latest = newest_raw(&repo.raw_news_dir())?;
    let feeds: Vec<Value> = latest
        .as_ref()
        .and_then(|d| d["feeds"].as_array().cloned())
        .unwrap_or_default()
        .into_iter()
        .map(|mut f| {
            let outlet = f["id"]
                .as_str()
                .and_then(|id| sources.feed(id))
                .map(|s| s.outlet.clone());
            f["outlet"] = json!(outlet);
            f
        })
        .collect();
    Ok(json!({
        "fetched_at": latest.as_ref().and_then(|d| d["fetched_at"].as_str().map(String::from)),
        "feeds": feeds,
        "daily": daily
            .into_iter()
            .map(|((date, candidate_id), items)| json!({ "date": date, "candidate_id": candidate_id, "items": items }))
            .collect::<Vec<_>>(),
        "headlines": headlines,
    }))
}

/// The last 28 windows (a week at four drops a day) of Grok Bot's X measurements.
fn x_signal(repo: &Repo) -> Result<Value> {
    let rows = grok::read_clean_rows(repo).unwrap_or_default();
    let windows: BTreeSet<&str> = rows.iter().map(|r| r.window_end.as_str()).collect();
    let keep: BTreeSet<&str> = windows.iter().rev().take(28).copied().collect();
    let out: Vec<Value> = rows
        .iter()
        .filter(|r| keep.contains(r.window_end.as_str()))
        .map(|r| {
            json!({
                "window_start": r.window_start,
                "window_end": r.window_end,
                "candidate_id": r.candidate_id,
                "mentions": r.mentions,
                "mentions_method": r.mentions_method,
                "sample_size": r.sample_size,
                "unique_authors": r.unique_authors,
                "engagement": r.engagement,
                "sentiment_pos": r.sentiment_pos,
                "sentiment_neg": r.sentiment_neg,
                "sentiment_neu": r.sentiment_neu,
                "bot_share_estimate": r.bot_share_estimate,
                "top_topics": r.top_topics.as_deref().map(|t| t.split(" | ").collect::<Vec<_>>()),
            })
        })
        .collect();
    Ok(json!({ "rows": out }))
}

fn events_signal(repo: &Repo) -> Result<Value> {
    let rows = events::read_clean_rows(repo).unwrap_or_default();
    Ok(Value::Array(
        rows.iter()
            .rev()
            .take(60)
            .map(|r| {
                json!({
                    "date": r.date,
                    "event_id": r.event_id,
                    "kind": r.kind,
                    "candidate_ids": r.candidate_ids.split_whitespace().collect::<Vec<_>>(),
                    "summary": r.summary,
                    "source_url": r.source_url,
                    "source_title": r.source_title,
                    "proposed_change": r.proposed_change,
                    "reported_by": r.reported_by,
                })
            })
            .collect(),
    ))
}

#[cfg(test)]
mod tests {
    use super::is_forecast_name;

    #[test]
    fn forecast_names_are_strict() {
        assert!(is_forecast_name("2026-10-09T14.json"));
        assert!(!is_forecast_name("series.json"));
        assert!(!is_forecast_name("../etc/passwd"));
        assert!(!is_forecast_name("2026-10-09T14.json.bak"));
        assert!(!is_forecast_name("2026-10-0xT14.json"));
    }
}
