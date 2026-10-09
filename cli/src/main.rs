//! `fr2027`: validate the repository's data, ingest raw files, run the forecast, and serve the
//! local UI. Every subcommand works on the repository found above the current directory, or on
//! `--repo`.

mod api;
mod collect;
mod data;
mod export;
mod forecast;
mod inbox;
mod serve;

use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::Result;
use clap::{Parser, Subcommand};
use fr2027_collectors::{Repo, config::load_model_params};

use crate::data::{Ingested, Inputs};

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
    /// Read one live source and write a new file under data/raw/.
    #[command(subcommand)]
    Collect(collect::Source),
    /// Bring in what Grok Bot posted to the inbox Worker.
    #[command(subcommand)]
    Inbox(inbox::Command),
    /// Write the public site: the built UI plus api/*.json.
    Export(export::Args),
    /// Serve the UI and its JSON documents locally.
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
        Command::Collect(source) => collect::run(&repo, &source).map(|()| ExitCode::SUCCESS),
        Command::Inbox(command) => inbox::run(&repo, &command).map(|()| ExitCode::SUCCESS),
        Command::Export(args) => export::run(&repo, &args),
        Command::Serve(args) => serve::run(repo, args).map(|()| ExitCode::SUCCESS),
    }
}

fn validate(repo: &Repo) -> Result<ExitCode> {
    let inputs = Inputs::load(repo)?;
    let params = load_model_params(repo)?;
    let ingested = Ingested::read(repo, &inputs)?;
    println!(
        "config: {} candidates in {} slots; {} markets, {} feeds; model {} ({} simulations)",
        inputs.config.candidates.len(),
        inputs.config.slots.len(),
        inputs.sources.markets.len(),
        inputs.sources.feeds.len(),
        params.model_version,
        params.simulations
    );
    for line in ingested.summary() {
        println!("{line}");
    }
    let mut failed = false;
    for rejection in ingested.rejections() {
        failed = true;
        eprintln!("rejected {}:", repo.relative(&rejection.path));
        for error in &rejection.errors {
            eprintln!("  - {error}");
        }
    }
    for (file, bytes) in ingested.clean_files()? {
        if std::fs::read(repo.clean_dir().join(file)).ok().as_deref() != Some(bytes.as_slice()) {
            failed = true;
            eprintln!("data/clean/{file} is out of date with data/raw/: run `moon run cli:ingest`");
        }
    }
    Ok(if failed { ExitCode::FAILURE } else { ExitCode::SUCCESS })
}

fn ingest(repo: &Repo) -> Result<()> {
    let inputs = Inputs::load(repo)?;
    let ingested = Ingested::read(repo, &inputs)?;
    ingested.write(repo)?;
    for line in ingested.summary() {
        println!("{line}");
    }
    for rejection in ingested.rejections() {
        eprintln!(
            "quarantined {}: {}",
            repo.relative(&rejection.path),
            rejection.errors.join("; ")
        );
    }
    Ok(())
}
