//! `fr2027`: validate the repository's data, ingest raw files, run the forecast, and serve the
//! local UI. Every subcommand works on the repository found above the current directory, or on
//! `--repo`.

mod forecast;
mod serve;

use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::Result;
use clap::{Parser, Subcommand};
use fr2027_collectors::{Repo, Schemas, config::CandidatesConfig, config::load_model_params, grok, polls};

#[derive(Parser)]
#[command(name = "fr2027", about = "Forecaster for the 2027 French presidential election")]
struct Cli {
    /// Repository root (default: found above the current directory).
    #[arg(long, global = true)]
    repo: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Check configuration, every raw file and the clean files without writing anything; fails
    /// on any rejection or on clean files that are out of date.
    Validate,
    /// Rebuild data/clean/ from data/raw/ and rewrite data/quarantine/.
    Ingest,
    /// Fit the model to data/clean/ and write data/forecasts/<YYYY-MM-DDTHH>.json.
    Forecast(forecast::Args),
    /// Serve the UI and its JSON API.
    Serve(serve::Args),
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("error: {error:#}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: Cli) -> Result<ExitCode> {
    let repo = match cli.repo {
        Some(root) => Repo::new(root),
        None => Repo::discover(&std::env::current_dir()?)?,
    };
    match cli.command {
        Command::Validate => validate(&repo),
        Command::Ingest => ingest(&repo).map(|()| ExitCode::SUCCESS),
        Command::Forecast(args) => forecast::run(&repo, &args).map(|()| ExitCode::SUCCESS),
        Command::Serve(args) => serve::run(repo, args).map(|()| ExitCode::SUCCESS),
    }
}

fn validate(repo: &Repo) -> Result<ExitCode> {
    let schemas = Schemas::load(repo)?;
    let config = CandidatesConfig::load(repo, &schemas)?;
    let params = load_model_params(repo)?;
    let polls = polls::ingest(repo, &schemas, &config)?;
    let grok = grok::ingest(repo, &schemas, &config)?;
    println!(
        "config: {} candidates in {} slots; model {} ({} simulations)",
        config.candidates.len(),
        config.slots.len(),
        params.model_version,
        params.simulations
    );
    println!(
        "polls: {} accepted, {} rejected",
        polls.accepted.len(),
        polls.rejections.len()
    );
    println!(
        "grok drops: {} accepted, {} rejected",
        grok.accepted,
        grok.rejections.len()
    );
    let mut failed = false;
    for rejection in polls.rejections.iter().chain(&grok.rejections) {
        failed = true;
        eprintln!("rejected {}:", repo.relative(&rejection.path));
        for error in &rejection.errors {
            eprintln!("  - {error}");
        }
    }
    let expected = [
        (polls::CLEAN_FILE, polls::clean_bytes(&polls)?),
        (grok::CLEAN_FILE, grok::clean_bytes(&grok)?),
    ];
    for (file, bytes) in expected {
        if std::fs::read(repo.clean_dir().join(file)).ok().as_deref() != Some(bytes.as_slice()) {
            failed = true;
            eprintln!("data/clean/{file} is out of date with data/raw/: run `moon run cli:ingest`");
        }
    }
    Ok(if failed { ExitCode::FAILURE } else { ExitCode::SUCCESS })
}

fn ingest(repo: &Repo) -> Result<()> {
    let schemas = Schemas::load(repo)?;
    let config = CandidatesConfig::load(repo, &schemas)?;
    let polls = polls::ingest(repo, &schemas, &config)?;
    polls::write_clean(repo, &polls)?;
    let grok = grok::ingest(repo, &schemas, &config)?;
    grok::write_clean(repo, &grok)?;
    println!(
        "polls: {} accepted ({} rows), {} superseded, {} quarantined",
        polls.accepted.len(),
        polls.rows.len(),
        polls.superseded.len(),
        polls.rejections.len()
    );
    println!(
        "grok drops: {} accepted ({} rows), {} quarantined",
        grok.accepted,
        grok.rows.len(),
        grok.rejections.len()
    );
    for rejection in polls.rejections.iter().chain(&grok.rejections) {
        eprintln!(
            "quarantined {}: {}",
            repo.relative(&rejection.path),
            rejection.errors.join("; ")
        );
    }
    Ok(())
}
