//! `fr2027 backtest`: replay the model on past elections, score it, fit the error settings, and
//! write `research/backtest/results.json` and `report.md`.

use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use fr2027_backtest::{
    config::BacktestConfig,
    signals::{self, AttentionRow, NewsRow, SignalSources},
};
use fr2027_collectors::{Repo, csv_bytes, gdelt, write};
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
}

/// Fetches every candidate's page views and news volume over the window in
/// `data/backtest/<year>/signals.yaml`, and rewrites `attention.csv` and `gdelt.csv` beside it.
/// Unlike live raw files these are rewritten whole: they are a fixed past, fetched again only to
/// add a candidate or fix a query.
pub fn fetch_signals(repo: &Repo, args: &SignalsArgs) -> Result<()> {
    let config = BacktestConfig::load(repo)?;
    let Some(entry) = config.elections.iter().find(|e| e.year == args.year) else {
        bail!("{} is not an election in config/backtest.yaml", args.year);
    };
    let dir = repo.root().join(&entry.dir);
    let sources = SignalSources::load(&dir)?;
    let inputs = Inputs::load(repo)?;
    let client = Client::new(&inputs.sources.user_agent)?;
    let project = &inputs.sources.wikipedia.project;
    let mut reader = gdelt_api::Reader::new(&client);
    let (mut attention, mut news) = (Vec::new(), Vec::new());
    let mut failures = Vec::new();
    for (candidate_id, entry) in &sources.candidates {
        if let Some(article) = &entry.wikipedia {
            match wikipedia::fetch(&client, project, article, sources.start, sources.end) {
                Ok(Some(days)) => {
                    let retrieved_at = timestamp(Timestamp::now());
                    for day in days {
                        attention.push(AttentionRow {
                            date: day.date.parse().context("Wikimedia date")?,
                            candidate_id: candidate_id.clone(),
                            article: article.clone(),
                            views: day.views,
                            retrieved_at: retrieved_at.clone(),
                        });
                    }
                }
                Ok(None) => failures.push(format!("{candidate_id}: no Wikipedia article `{article}`")),
                Err(error) => failures.push(format!("{candidate_id}: page views: {error}")),
            }
        }
        let query = gdelt::query(&entry.names);
        match reader.fetch(&query, sources.start, sources.end) {
            Ok(days) => {
                let retrieved_at = timestamp(Timestamp::now());
                for day in days {
                    news.push(NewsRow {
                        date: day.date.parse().context("GDELT date")?,
                        candidate_id: candidate_id.clone(),
                        query: query.clone(),
                        articles: day.articles,
                        total_articles: day.total_articles,
                        retrieved_at: retrieved_at.clone(),
                    });
                }
            }
            Err(error) => failures.push(format!("{candidate_id}: GDELT: {error}")),
        }
        eprintln!("{candidate_id}: done");
    }
    if !failures.is_empty() {
        bail!("nothing written; failed: {}", failures.join("; "));
    }
    attention.sort_by(|a, b| (a.date, &a.candidate_id).cmp(&(b.date, &b.candidate_id)));
    news.sort_by(|a, b| (a.date, &a.candidate_id).cmp(&(b.date, &b.candidate_id)));
    write(
        &dir.join(signals::ATTENTION_FILE),
        &csv_bytes(&attention, signals::ATTENTION_HEADER)?,
    )?;
    write(&dir.join(signals::NEWS_FILE), &csv_bytes(&news, signals::NEWS_HEADER)?)?;
    println!(
        "wrote {} ({} rows) and {} ({} rows)",
        repo.relative(&dir.join(signals::ATTENTION_FILE)),
        attention.len(),
        repo.relative(&dir.join(signals::NEWS_FILE)),
        news.len()
    );
    Ok(())
}
