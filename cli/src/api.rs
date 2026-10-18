//! The UI's data, one JSON document per name. `fr2027 export` writes them as static files
//! under `api/` for the public site; `fr2027 serve` computes them on each request. Both use
//! these functions, so the local app and the public page show the same thing.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use anyhow::{Context, Result, anyhow};
use fr2027_collectors::{
    Repo, attention, events, gdelt, grok, list_files, markets, news, polls, read, sources::SourcesConfig,
};
use jiff::{Timestamp, ToSpan, civil::Date, tz::TimeZone};
use serde::Serialize;
use serde_json::{Value, json};

use crate::data::Inputs;
use crate::forecast::SERIES_FILE;

/// Every document the UI reads, as `api/<name>.json`.
pub const NAMES: [&str; 9] = [
    "forecast",
    "blend",
    "series",
    "polls",
    "history",
    "track-record",
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
        "blend" => blend(repo, now)?,
        "series" => read_json(&repo.forecasts_dir().join(SERIES_FILE))?,
        "polls" => polls_document(&polls::read_clean_rows(repo)?)?,
        "history" => history(repo)?,
        "track-record" => {
            let path = repo.root().join(TRACK_RECORD_FILE);
            if path.exists() {
                track_record(&read_json(&path)?)?
            } else {
                json!({ "model_version": null, "elections": [], "by_horizon": [], "reliability": [] })
            }
        }
        "signals" => signals(repo, now)?,
        "health" => health(repo, now)?,
        "candidates" => {
            let inputs = Inputs::load(repo)?;
            serde_json::to_value(&inputs.config)?
        }
        _ => return Ok(None),
    }))
}

/// The latest forecast pooled with the newest market snapshot (model/src/blend.rs). Computed on
/// every build, so it follows the markets hourly while the forecast refits when its inputs move.
/// It carries what the UI needs to pool again at any other weight (its slider): each side's
/// probabilities, the markets' share on people outside the field, and the floor.
fn blend(repo: &Repo, now: Timestamp) -> Result<Value> {
    let params = fr2027_collectors::config::load_model_params(repo)?.market_blend;
    params.check()?;
    let empty = json!({
        "forecast": null, "model_weight": params.model_weight, "probability_floor": params.probability_floor,
        "market_fetched_at": null, "market_used": false, "market_others_win": 0.0, "candidates": [],
    });
    let Some((file, forecast)) = latest_forecast(repo)? else {
        return Ok(empty);
    };
    let candidates: Vec<&Value> = forecast["candidates"]
        .as_array()
        .map(|a| a.iter().collect())
        .unwrap_or_default();
    let ids: Vec<&str> = candidates.iter().filter_map(|c| c["candidate_id"].as_str()).collect();
    let newest: Option<markets::Snapshot> = newest_raw(&repo.raw_markets_dir())?
        .map(serde_json::from_value)
        .transpose()?;
    let fresh = markets::fresh_snapshot(repo, now, params.max_age_hours)?;
    let market = |question: &str| match &fresh {
        Some(s) => markets::venue_average(s, question, &ids),
        None => (vec![None; ids.len()], 0.0),
    };
    let model = |key: &str| -> Vec<f64> { candidates.iter().map(|c| c[key].as_f64().unwrap_or(0.0)).collect() };
    let (win_prices, win_others) = market("win");
    let (qualify_prices, _) = market("qualify");
    let (ballot_prices, _) = market("on_ballot");
    let win_market = fr2027_model::blend::WinPrices {
        prices: win_prices,
        others: win_others,
    };
    let (win_normalised, others_normalised) = win_market.normalised();
    let (p_win, p_qualify, p_run) = (model("p_win"), model("p_qualify_r1"), model("p_run"));
    let win_blend = fr2027_model::blend::pool_win(&p_win, &win_market, &params);
    let round = |x: f64| (x * 10_000.0).round() / 10_000.0;
    let opt = |x: Option<f64>| x.map(round);
    let rows: Vec<Value> = ids
        .iter()
        .enumerate()
        .map(|(i, id)| {
            json!({
                "candidate_id": id,
                "model": { "win": p_win[i], "qualify": p_qualify[i], "run": p_run[i] },
                "market": { "win": opt(win_normalised[i]), "qualify": opt(qualify_prices[i]), "run": opt(ballot_prices[i]) },
                // Who runs is already pooled with the ballot markets inside the model.
                "blend": {
                    "win": round(win_blend[i]),
                    "qualify": round(fr2027_model::blend::pool_binary(p_qualify[i], qualify_prices[i], &params)),
                    "run": p_run[i],
                },
            })
        })
        .collect();
    Ok(json!({
        "forecast": file,
        "model_weight": params.model_weight,
        "probability_floor": params.probability_floor,
        "market_fetched_at": newest.as_ref().map(|s| s.fetched_at.clone()),
        "market_used": fresh.is_some(),
        "market_others_win": round(others_normalised),
        "candidates": rows,
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

/// One poll as the UI reads it: the poll's details once, then each scenario's shares. The clean
/// CSV repeats every detail on each candidate's row and adds provenance the UI doesn't show
/// (`index_url`, `retrieved_at`, `content_hash`, `method`, `population`); that stays in
/// `data/clean/polls.csv`.
#[derive(Serialize)]
struct PollSummary<'a> {
    poll_id: &'a str,
    firm: &'a str,
    sponsor: Option<&'a str>,
    field_start: Date,
    field_end: Date,
    published_at: Date,
    sample_size: u32,
    entry: &'a str,
    source_url: &'a str,
    notice_url: Option<&'a str>,
    scenarios: Vec<ScenarioShares<'a>>,
}

#[derive(Serialize)]
struct ScenarioShares<'a> {
    round: u8,
    scenario_id: &'a str,
    /// Percent, by candidate id.
    shares: BTreeMap<&'a str, f64>,
}

/// The clean poll rows grouped back into polls and scenarios, in file order.
fn polls_document(rows: &[polls::PollRow]) -> Result<Value> {
    let mut summaries: Vec<PollSummary> = Vec::new();
    for row in rows {
        let poll = match summaries.iter().position(|p| p.poll_id == row.poll_id) {
            Some(index) => &mut summaries[index],
            None => {
                summaries.push(PollSummary {
                    poll_id: &row.poll_id,
                    firm: &row.firm,
                    sponsor: row.sponsor.as_deref(),
                    field_start: row.field_start,
                    field_end: row.field_end,
                    published_at: row.published_at,
                    sample_size: row.sample_size,
                    entry: &row.entry,
                    source_url: &row.source_url,
                    notice_url: row.notice_url.as_deref(),
                    scenarios: Vec::new(),
                });
                summaries.last_mut().expect("just pushed")
            }
        };
        let scenario = match poll
            .scenarios
            .iter()
            .position(|s| s.round == row.round && s.scenario_id == row.scenario_id)
        {
            Some(index) => &mut poll.scenarios[index],
            None => {
                poll.scenarios.push(ScenarioShares {
                    round: row.round,
                    scenario_id: &row.scenario_id,
                    shares: BTreeMap::new(),
                });
                poll.scenarios.last_mut().expect("just pushed")
            }
        };
        scenario.shares.insert(&row.candidate_id, row.share);
    }
    Ok(json!({ "polls": serde_json::to_value(summaries)? }))
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

/// The backtest's report, from which `track-record` keeps what the UI shows.
pub const TRACK_RECORD_FILE: &str = "research/backtest/results.json";

/// How the model in config/model.yaml did when replayed on past elections: whether its 80% ranges
/// held the result as often as they claim, by how long before round 1, and whether its
/// probabilities came true as often as they said (`research/backtest/results.json`).
fn track_record(results: &Value) -> Result<Value> {
    let scores = &results["current_scores"];
    let by_horizon: Vec<Value> = scores["by_horizon"]
        .as_object()
        .context("backtest results: no current_scores.by_horizon")?
        .iter()
        .map(|(days, summary)| {
            Ok(json!({
                "days": days.parse::<u32>().with_context(|| format!("backtest results: horizon `{days}`"))?,
                "replays": summary["replays"],
                "coverage80": summary["coverage80"],
                "share_mae_points": summary["share_mae_points"],
            }))
        })
        .collect::<Result<_>>()?;
    let elections: Vec<Value> = results["elections"]
        .as_array()
        .context("backtest results: no elections")?
        .iter()
        .map(|e| e["year"].clone())
        .collect();
    Ok(json!({
        "model_version": results["model_version"],
        "elections": elections,
        "by_horizon": by_horizon,
        "reliability": scores["reliability"],
    }))
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

/// How often a source is expected to update. The UI translates these codes.
#[derive(Clone, Copy, Serialize)]
enum Cadence {
    /// Whenever a pollster publishes: no fixed schedule.
    #[serde(rename = "on_publication")]
    OnPublication,
    #[serde(rename = "hourly")]
    Hourly,
    #[serde(rename = "12h")]
    Every12Hours,
    #[serde(rename = "daily")]
    Daily,
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
    let news_volume = newest_raw(&repo.raw_gdelt_dir())?;
    let grok_drop = newest_raw(&repo.raw_grok_dir())?;
    let event = events::read_clean_rows(repo).ok().and_then(|rows| rows.last().cloned());
    let field = |doc: &Option<Value>, key: &str| doc.as_ref().and_then(|d| d[key].as_str().map(String::from));
    // name, last update, warn after this many hours (None: never), expected cadence
    let sources: [(&str, Option<String>, Option<i64>, Cadence); 7] = [
        ("polls", last_poll, None, Cadence::OnPublication),
        ("markets", field(&markets, "fetched_at"), Some(3), Cadence::Hourly),
        ("news", field(&news, "fetched_at"), Some(3), Cadence::Hourly),
        ("attention", field(&attention, "fetched_at"), Some(50), Cadence::Daily),
        (
            "news_volume",
            field(&news_volume, "fetched_at"),
            Some(50),
            Cadence::Daily,
        ),
        ("x", field(&grok_drop, "collected_at"), Some(48), Cadence::Every12Hours),
        ("events", event.map(|e| e.reported_at), None, Cadence::Every12Hours),
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
    for area in ["polls", "grok", "markets", "news", "attention", "gdelt", "events"] {
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
        "news": news_signal(repo, &inputs.sources, &since(30), &since(90))?,
        "x": x_signal(repo)?,
        "events": events_signal(repo)?,
        "forward": forward_signal(repo, &inputs)?,
    }))
}

/// Days after a flag over which the latent share's move is measured.
const FORWARD_DAYS: i32 = 14;

/// The forward log scored against the latest latent share history (`series.json`): each flag's
/// move in the candidate's share over the next [`FORWARD_DAYS`] days, against the typical move
/// over any such stretch. A flag too recent to have its stretch is pending.
fn forward_signal(repo: &Repo, inputs: &Inputs) -> Result<Value> {
    let (logged, _) = fr2027_collectors::forward::read(repo, &inputs.schemas)?;
    let series_path = repo.forecasts_dir().join(SERIES_FILE);
    let points = if series_path.exists() {
        read_json(&series_path)?["points"].clone()
    } else {
        json!([])
    };
    let mut series: BTreeMap<String, BTreeMap<i32, f64>> = BTreeMap::new();
    for point in points.as_array().into_iter().flatten() {
        let (Some(id), Some(date), Some(mean)) = (
            point["candidate_id"].as_str(),
            point["date"].as_str().and_then(|d| d.parse::<Date>().ok()),
            point["mean"].as_f64(),
        ) else {
            continue;
        };
        series
            .entry(id.into())
            .or_default()
            .insert(fr2027_model::day_number(date), mean);
    }
    // The share's move from the last point on or before `day` to the first point at least
    // FORWARD_DAYS later.
    let change = |candidate: &str, day: i32| -> Option<f64> {
        let points = series.get(candidate)?;
        let (_, start) = points.range(..=day).next_back()?;
        let (_, end) = points.range(day + FORWARD_DAYS..).next()?;
        Some(end - start)
    };
    let typical: Vec<f64> = series
        .iter()
        .flat_map(|(id, points)| {
            points
                .keys()
                .filter_map(|d| change(id, *d))
                .map(f64::abs)
                .collect::<Vec<_>>()
        })
        .collect();
    let mean = |values: &[f64]| (!values.is_empty()).then(|| values.iter().sum::<f64>() / values.len() as f64);
    let mut flags = Vec::new();
    let (mut spike_moves, mut x_agree, mut x_scored) = (Vec::new(), 0usize, 0usize);
    for l in &logged {
        let day = l.flag.date.parse::<Date>().map(fr2027_model::day_number).ok();
        let moved = day.and_then(|d| change(&l.flag.candidate_id, d));
        if let Some(m) = moved {
            match l.flag.kind {
                fr2027_collectors::forward::FlagKind::Spike => spike_moves.push(m.abs()),
                fr2027_collectors::forward::FlagKind::XShift => {
                    x_scored += 1;
                    if m * l.flag.score > 0.0 {
                        x_agree += 1;
                    }
                }
            }
        }
        flags.push(json!({
            "detected_at": l.detected_at,
            "kind": l.flag.kind,
            "candidate_id": l.flag.candidate_id,
            "date": l.flag.date,
            "score": l.flag.score,
            "detail": l.flag.detail,
            "change": moved.map(|m| (m * 10_000.0).round() / 10_000.0),
        }));
    }
    flags.reverse();
    flags.truncate(60);
    let round = |x: Option<f64>| x.map(|v| (v * 10_000.0).round() / 10_000.0);
    Ok(json!({
        "horizon_days": FORWARD_DAYS,
        "flags": flags,
        "summary": {
            "logged": logged.len(),
            "spikes_scored": spike_moves.len(),
            "spike_mean_move": round(mean(&spike_moves)),
            "typical_mean_move": round(mean(&typical)),
            "x_shifts_scored": x_scored,
            "x_shifts_agreeing": x_agree,
        },
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

/// Headlines and their daily counts since `since`, and GDELT's daily news volume since `volume_since`.
fn news_signal(repo: &Repo, sources: &SourcesConfig, since: &str, volume_since: &str) -> Result<Value> {
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
        .filter(|r| &r.published_at[..10] >= since)
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
        "volume": gdelt::read_clean_rows(repo)
            .unwrap_or_default()
            .into_iter()
            .filter(|r| r.date.as_str() >= volume_since)
            .map(|r| json!({
                "date": r.date,
                "candidate_id": r.candidate_id,
                "articles": r.articles,
                "total_articles": r.total_articles,
            }))
            .collect::<Vec<_>>(),
    }))
}

/// The last 28 windows (two weeks at two drops a day) of Grok Bot's X measurements. Stance shares
/// come with their 95% intervals and the counts behind them, so the page can pool several windows;
/// `drop_kappa` is the drop's labelling agreement, and `rubric_version` is empty for the first drops,
/// read for sentiment before the stance rubric.
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
                "rubric_version": r.rubric_version,
                "top_topics": r.top_topics.as_deref().map(|t| t.split(" | ").collect::<Vec<_>>()),
                "stance_posts_drawn": r.stance_posts_drawn,
                "stance_off_topic": r.stance_off_topic,
                "stance_support": r.stance_support,
                "stance_oppose": r.stance_oppose,
                "stance_neutral": r.stance_neutral,
                "support_share": r.support_share,
                "support_low": r.support_low,
                "support_high": r.support_high,
                "oppose_share": r.oppose_share,
                "oppose_low": r.oppose_low,
                "oppose_high": r.oppose_high,
                "neutral_share": r.neutral_share,
                "neutral_low": r.neutral_low,
                "neutral_high": r.neutral_high,
                "drop_kappa": r.drop_kappa,
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
                    "summary": {"fr": r.summary_fr, "en": r.summary_en},
                    "source_url": r.source_url,
                    "source_title": r.source_title,
                    "reported_by": r.reported_by,
                })
            })
            .collect(),
    ))
}

#[cfg(test)]
mod tests {
    use super::{is_forecast_name, polls_document, track_record};
    use fr2027_collectors::polls::PollRow;
    use serde_json::json;

    #[test]
    fn forecast_names_are_strict() {
        assert!(is_forecast_name("2026-10-09T14.json"));
        assert!(!is_forecast_name("series.json"));
        assert!(!is_forecast_name("../etc/passwd"));
        assert!(!is_forecast_name("2026-10-09T14.json.bak"));
        assert!(!is_forecast_name("2026-10-0xT14.json"));
    }

    #[test]
    fn track_record_keeps_coverage_and_reliability() {
        let results = json!({
            "model_version": "0.3.0",
            "elections": [{ "year": 2017 }, { "year": 2022 }],
            "current_scores": {
                "by_horizon": {
                    "182": { "replays": 2, "coverage80": 0.7, "share_mae_points": 3.1, "win_brier": 0.05 },
                    "7": { "replays": 2, "coverage80": 0.8, "share_mae_points": 1.2 }
                },
                "reliability": [{ "lo": 0.0, "hi": 0.1, "events": 9, "mean_forecast": 0.01, "observed": 0.0 }]
            }
        });
        let record = track_record(&results).unwrap();
        assert_eq!(record["elections"], json!([2017, 2022]));
        assert_eq!(
            record["by_horizon"][0],
            json!({ "days": 182, "replays": 2, "coverage80": 0.7, "share_mae_points": 3.1 })
        );
        assert_eq!(record["by_horizon"][1]["days"], 7);
        assert_eq!(record["reliability"][0]["events"], 9);
        assert!(track_record(&json!({})).is_err());
    }

    #[test]
    fn polls_are_grouped_by_poll_and_scenario() {
        let row = |poll_id: &str, round: u8, scenario_id: &str, candidate_id: &str, share: f64| PollRow {
            poll_id: poll_id.into(),
            firm: "ifop".into(),
            sponsor: None,
            field_start: "2026-09-01".parse().unwrap(),
            field_end: "2026-09-03".parse().unwrap(),
            published_at: "2026-09-04".parse().unwrap(),
            sample_size: 1000,
            method: Some("online".into()),
            population: None,
            round,
            scenario_id: scenario_id.into(),
            candidate_id: candidate_id.into(),
            share,
            source_url: "https://example.org/p".into(),
            notice_url: None,
            index_url: None,
            entry: "primary".into(),
            retrieved_at: "2026-09-04T10:00Z".into(),
            content_hash: "sha256:0".into(),
        };
        let rows = [
            row("p1", 1, "A", "a", 30.0),
            row("p1", 1, "A", "b", 70.0),
            row("p1", 2, "A", "a", 45.0),
            row("p2", 1, "A", "a", 40.0),
        ];
        let doc = polls_document(&rows).unwrap();
        let polls = doc["polls"].as_array().unwrap();
        assert_eq!(polls.len(), 2);
        assert_eq!(polls[0]["poll_id"], "p1");
        assert_eq!(polls[0]["field_end"], "2026-09-03");
        assert_eq!(
            polls[0]["scenarios"],
            serde_json::json!([
                { "round": 1, "scenario_id": "A", "shares": { "a": 30.0, "b": 70.0 } },
                { "round": 2, "scenario_id": "A", "shares": { "a": 45.0 } },
            ])
        );
        assert!(polls[0].get("content_hash").is_none());
    }
}
