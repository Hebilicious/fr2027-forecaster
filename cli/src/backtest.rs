//! `fr2027 backtest`: replay the model on past elections, score it, fit the error settings, and
//! write `research/backtest/results.json` and `report.md`.

use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use fr2027_backtest::{
    config::BacktestConfig,
    signals::{self, AttentionRow, NewsRow, SignalSources},
};
use fr2027_collectors::{Repo, csv_bytes, write};
use fr2027_fetch::{Client, gdelt as gdelt_api, timestamp, wikipedia};
use jiff::Timestamp;

use crate::data::Inputs;

#[derive(clap::Args)]
pub struct Args {
    /// Override the number of simulations per replay and setting in config/backtest.yaml.
    #[arg(long)]
    simulations: Option<u32>,
    /// Write here instead of research/backtest/.
    #[arg(long)]
    out_dir: Option<PathBuf>,
}

pub const OUT_DIR: &str = "research/backtest";

pub fn run(repo: &Repo, args: &Args) -> Result<()> {
    let report = fr2027_backtest::run(repo, args.simulations, &mut |line| eprintln!("{line}"))?;
    let out_dir = args.out_dir.clone().unwrap_or_else(|| repo.root().join(OUT_DIR));
    std::fs::create_dir_all(&out_dir)?;
    let mut json = serde_json::to_vec_pretty(&report)?;
    json.push(b'\n');
    std::fs::write(out_dir.join("results.json"), json)?;
    std::fs::write(out_dir.join("report.md"), report.markdown())?;

    let line = |label: &str, s: &fr2027_backtest::score::Summary| {
        println!(
            "  {label:<22} qualify Brier {:.3} log-loss {:.3} | win Brier {:.3} log-loss {:.3} | share log score {:.1} | round 2 log score {:.2}",
            s.qualify_brier, s.qualify_log_loss, s.win_brier, s.win_log_loss, s.share_log_score, s.round2_log_score
        );
    };
    println!("wrote {}", repo.relative(&out_dir.join("report.md")));
    line("latest-poll average", &report.baseline.overall);
    line("model, current", &report.current_scores.overall);
    line("model, fitted", &report.fitted_scores.overall);
    for param in fr2027_backtest::report::Param::ALL {
        println!(
            "  {:<34} current {:<6} fitted {}",
            param.name(),
            param.get(&report.current),
            param.get(&report.fitted)
        );
    }
    Ok(())
}

#[derive(clap::Args)]
pub struct SignalsArgs {
    /// The election, as listed in config/backtest.yaml.
    year: i16,
    /// Fetch only this source (default: both).
    #[arg(long, value_enum)]
    only: Option<SignalSource>,
    /// Fetch only from this day (default: the window's start in signals.yaml), to split a long
    /// fetch across machines; the pieces are then concatenated.
    #[arg(long)]
    start: Option<jiff::civil::Date>,
    /// Fetch only to this day (default: the window's end).
    #[arg(long)]
    end: Option<jiff::civil::Date>,
}

#[derive(Clone, Copy, PartialEq, clap::ValueEnum)]
enum SignalSource {
    /// Wikipedia page views → attention.csv.
    Attention,
    /// GDELT news volume → gdelt.csv.
    News,
}

/// Fetches every candidate's page views and news volume over the window in
/// `data/backtest/<year>/signals.yaml`, and rewrites `attention.csv` and `gdelt.csv` beside it.
/// Unlike live raw files these are rewritten whole: they are a fixed past, fetched again only to
/// add a candidate or fix a query. A file is written only when every candidate's fetch worked.
pub fn fetch_signals(repo: &Repo, args: &SignalsArgs) -> Result<()> {
    let config = BacktestConfig::load(repo)?;
    let Some(entry) = config.elections.iter().find(|e| e.year == args.year) else {
        bail!("{} is not an election in config/backtest.yaml", args.year);
    };
    let dir = repo.root().join(&entry.dir);
    let mut sources = SignalSources::load(&dir)?;
    sources.start = args.start.map_or(sources.start, |d| d.max(sources.start));
    sources.end = args.end.map_or(sources.end, |d| d.min(sources.end));
    if sources.start > sources.end {
        bail!("the days asked for are outside {}'s window", args.year);
    }
    let inputs = Inputs::load(repo)?;
    let client = Client::new(&inputs.sources.user_agent)?;
    let wanted = |source| args.only.is_none_or(|only| only == source);
    if wanted(SignalSource::Attention) {
        let rows = fetch_attention(&client, &inputs.sources.wikipedia.project, &sources)?;
        let path = dir.join(signals::ATTENTION_FILE);
        write(&path, &csv_bytes(&rows, signals::ATTENTION_HEADER)?)?;
        println!("wrote {} ({} rows)", repo.relative(&path), rows.len());
    }
    if wanted(SignalSource::News) {
        let client = Client::with_timeout(&inputs.sources.user_agent, gdelt_api::TIMEOUT)?;
        let rows = fetch_news(&client, &sources)?;
        let path = dir.join(signals::NEWS_FILE);
        write(&path, &csv_bytes(&rows, signals::NEWS_HEADER)?)?;
        println!("wrote {} ({} rows)", repo.relative(&path), rows.len());
    }
    Ok(())
}

fn fetch_attention(client: &Client, project: &str, sources: &SignalSources) -> Result<Vec<AttentionRow>> {
    let mut rows = Vec::new();
    for (candidate_id, entry) in &sources.candidates {
        let Some(article) = &entry.wikipedia else { continue };
        let Some(days) = wikipedia::fetch(client, project, article, sources.start, sources.end)
            .with_context(|| format!("{candidate_id}: page views"))?
        else {
            bail!("{candidate_id}: no Wikipedia article `{article}`");
        };
        let retrieved_at = timestamp(Timestamp::now());
        for day in days {
            rows.push(AttentionRow {
                date: day.date.parse().context("Wikimedia date")?,
                candidate_id: candidate_id.clone(),
                article: article.clone(),
                views: day.views,
                retrieved_at: retrieved_at.clone(),
            });
        }
        // Wikimedia asks clients to keep request rates modest.
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
    rows.sort_by(|a, b| (a.date, &a.candidate_id).cmp(&(b.date, &b.candidate_id)));
    Ok(rows)
}

fn fetch_news(client: &Client, sources: &SignalSources) -> Result<Vec<NewsRow>> {
    let matcher = gdelt_api::Matcher::new(&sources.candidates);
    let counts = gdelt_api::count_days(
        client,
        &matcher,
        sources.start,
        sources.end,
        gdelt_api::WORKERS,
        &|line| eprintln!("GDELT: {line}"),
    )
    .context("GDELT")?;
    let retrieved_at = timestamp(Timestamp::now());
    let mut rows = Vec::new();
    for (person, (candidate_id, days)) in matcher.people().iter().zip(counts) {
        let query = person.description(&sources.candidates[&candidate_id].names);
        for day in days {
            rows.push(NewsRow {
                date: day.date.parse().context("GDELT date")?,
                candidate_id: candidate_id.clone(),
                query: query.clone(),
                articles: day.articles,
                total_articles: day.total_articles,
                tone: day.tone,
                total_tone: day.total_tone,
                files: day.files,
                retrieved_at: retrieved_at.clone(),
            });
        }
    }
    rows.sort_by(|a, b| (a.date, &a.candidate_id).cmp(&(b.date, &b.candidate_id)));
    Ok(rows)
}
