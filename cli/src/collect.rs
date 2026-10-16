//! `fr2027 collect`: read one live source and write one new raw file. A source that fails is
//! recorded in the file with its error; the command fails only when every source failed, so one
//! broken feed never stops the rest.

use anyhow::{Result, bail};
use fr2027_collectors::{
    Repo, SchemaKind, attention, markets, names::LabelMatcher, names::NameMatcher, news, pretty_json, write_new,
};
use fr2027_fetch::{Client, kalshi, polymarket, rss, timestamp, wikipedia};
use jiff::{Timestamp, ToSpan, civil::Date, tz::TimeZone};

use crate::data::Inputs;

#[derive(clap::Subcommand)]
pub enum Source {
    /// Prediction-market prices (config/sources.yaml `markets`) → data/raw/markets/<hour>.json.
    Markets,
    /// New headlines naming a candidate (config/sources.yaml `feeds`) → data/raw/news/<hour>.json.
    News,
    /// Daily Wikipedia page views per candidate → data/raw/attention/<date>.json. Skips when
    /// today's file exists.
    Attention(AttentionArgs),
}

#[derive(clap::Args)]
pub struct AttentionArgs {
    /// First day (default: seven days before --end).
    #[arg(long)]
    start: Option<Date>,
    /// Last day (default: yesterday, UTC).
    #[arg(long)]
    end: Option<Date>,
}

pub fn run(repo: &Repo, source: &Source) -> Result<()> {
    let inputs = Inputs::load(repo)?;
    let client = Client::new(&inputs.sources.user_agent)?;
    let now = Timestamp::now();
    match source {
        Source::Markets => collect_markets(repo, &inputs, &client, now),
        Source::News => collect_news(repo, &inputs, &client, now),
        Source::Attention(args) => collect_attention(repo, &inputs, &client, now, args),
    }
}

/// Checks a document against its schema and its own rules, then writes it as a new raw file.
fn write_document<T: serde::Serialize>(
    repo: &Repo,
    inputs: &Inputs,
    path: &std::path::Path,
    kind: SchemaKind,
    document: &T,
    errors: Vec<String>,
) -> Result<()> {
    let value = serde_json::to_value(document)?;
    let mut problems = inputs.schemas.violations(kind, &value);
    problems.extend(errors);
    if !problems.is_empty() {
        bail!("refusing to write {}: {}", repo.relative(path), problems.join("; "));
    }
    if write_new(path, &pretty_json(&value)?)? {
        println!("wrote {}", repo.relative(path));
    } else {
        println!("{} already exists; nothing written", repo.relative(path));
    }
    Ok(())
}

fn collect_markets(repo: &Repo, inputs: &Inputs, client: &Client, now: Timestamp) -> Result<()> {
    let fetched_at = timestamp(now);
    let path = repo
        .raw_markets_dir()
        .join(format!("{}.json", markets::file_stem(&fetched_at)));
    if path.exists() {
        println!("{} already exists; nothing to do this hour", repo.relative(&path));
        return Ok(());
    }
    let labels = LabelMatcher::new(&inputs.sources, &inputs.names());
    let mut out = Vec::new();
    for source in &inputs.sources.markets {
        let result = match source.venue.as_str() {
            "polymarket" => polymarket::fetch(client, &source.event, &labels),
            "kalshi" => kalshi::fetch(client, &source.event, &labels),
            other => bail!("unknown venue `{other}`"),
        };
        let (status, error, contracts) = match result {
            Ok(contracts) => ("ok", None, contracts),
            Err(error) => {
                eprintln!("warning: {}: {error}", source.id);
                ("error", Some(error.to_string()), Vec::new())
            }
        };
        println!("{}: {status}, {} priced contracts", source.id, contracts.len());
        out.push(markets::Market {
            id: source.id.clone(),
            venue: source.venue.clone(),
            question: source.question.clone(),
            event: source.event.clone(),
            url: source.url.clone(),
            status: status.into(),
            error,
            contracts,
        });
    }
    if !out.is_empty() && out.iter().all(|m| m.status == "error") {
        bail!("every market failed; nothing written");
    }
    let snapshot = markets::Snapshot {
        schema_version: "1.0".into(),
        fetched_at,
        markets: out,
    };
    let errors = markets::check(None, &snapshot, &inputs.config);
    write_document(repo, inputs, &path, SchemaKind::Markets, &snapshot, errors)
}

fn collect_news(repo: &Repo, inputs: &Inputs, client: &Client, now: Timestamp) -> Result<()> {
    let fetched_at = timestamp(now);
    let path = repo
        .raw_news_dir()
        .join(format!("{}.json", news::file_stem(&fetched_at)));
    if path.exists() {
        println!("{} already exists; nothing to do this hour", repo.relative(&path));
        return Ok(());
    }
    let mut seen = news::ingest(repo, &inputs.schemas, &inputs.config, &inputs.sources)?.seen_urls;
    let matcher = NameMatcher::new(&inputs.sources);
    let mut feeds = Vec::new();
    let mut items = Vec::new();
    for feed in &inputs.sources.feeds {
        match rss::fetch(client, &feed.url, &feed.id, &matcher) {
            Ok((total, found)) => {
                let fresh: Vec<_> = found.into_iter().filter(|i| seen.insert(i.url.clone())).collect();
                println!("{}: {total} items, {} new naming a candidate", feed.id, fresh.len());
                items.extend(fresh);
                feeds.push(news::FeedStatus {
                    id: feed.id.clone(),
                    status: "ok".into(),
                    items: total,
                    error: None,
                });
            }
            Err(error) => {
                eprintln!("warning: {}: {error}", feed.id);
                feeds.push(news::FeedStatus {
                    id: feed.id.clone(),
                    status: "error".into(),
                    items: 0,
                    error: Some(error.to_string()),
                });
            }
        }
    }
    if !feeds.is_empty() && feeds.iter().all(|f| f.status == "error") {
        bail!("every feed failed; nothing written");
    }
    let snapshot = news::Snapshot {
        schema_version: "1.0".into(),
        fetched_at,
        feeds,
        items,
    };
    let errors = news::check(None, &snapshot, &inputs.config);
    write_document(repo, inputs, &path, SchemaKind::News, &snapshot, errors)
}

fn collect_attention(
    repo: &Repo,
    inputs: &Inputs,
    client: &Client,
    now: Timestamp,
    args: &AttentionArgs,
) -> Result<()> {
    let fetched_at = timestamp(now);
    let today = now.to_zoned(TimeZone::UTC).date();
    let path = repo
        .raw_attention_dir()
        .join(format!("{}.json", today.strftime("%Y-%m-%d")));
    if path.exists() {
        println!("{} already exists; nothing to do today", repo.relative(&path));
        return Ok(());
    }
    let end = args.end.unwrap_or(today.yesterday()?);
    let start = args.start.unwrap_or(end.checked_sub(6.days())?);
    if start > end {
        bail!("--start {start} is after --end {end}");
    }
    let project = &inputs.sources.wikipedia.project;
    let mut articles = Vec::new();
    for (candidate_id, entry) in &inputs.sources.candidates {
        let Some(article) = &entry.wikipedia else { continue };
        let (status, error, days) = match wikipedia::fetch(client, project, article, start, end) {
            Ok(Some(days)) => ("ok", None, days),
            Ok(None) => ("missing", None, Vec::new()),
            Err(error) => {
                eprintln!("warning: {article}: {error}");
                ("error", Some(error.to_string()), Vec::new())
            }
        };
        articles.push(attention::ArticleViews {
            candidate_id: candidate_id.clone(),
            article: article.clone(),
            status: status.into(),
            error,
            days,
        });
        // Wikimedia asks clients to keep request rates modest.
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
    let ok = articles.iter().filter(|a| a.status == "ok").count();
    println!("{ok} of {} articles with data, {start} to {end}", articles.len());
    if ok == 0 {
        bail!("no article returned data; nothing written");
    }
    let fetch = attention::Fetch {
        schema_version: "1.0".into(),
        fetched_at,
        project: project.clone(),
        start: start.to_string(),
        end: end.to_string(),
        articles,
    };
    let errors = attention::check(None, &fetch, &inputs.config);
    write_document(repo, inputs, &path, SchemaKind::Attention, &fetch, errors)
}
