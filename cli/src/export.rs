//! `fr2027 export`: the public site as static files: the built UI, plus every document in
//! `api` written to `api/<name>.json`. The hourly workflow deploys the result to Cloudflare.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{Context, Result, bail};
use fr2027_collectors::{Repo, write};
use jiff::Timestamp;

use crate::api;
use crate::data::Inputs;

/// Exit code when a publication blackout is in force: nothing was built, nothing should deploy.
pub const BLACKOUT: u8 = 3;

#[derive(clap::Args)]
pub struct Args {
    /// Output directory, emptied first (default: .site at the repository root).
    #[arg(long)]
    out: Option<PathBuf>,
    /// The built UI to copy in (default: app/web/dist).
    #[arg(long)]
    web_dir: Option<PathBuf>,
    /// Time the documents are computed at (default: now).
    #[arg(long)]
    now: Option<Timestamp>,
}

pub fn run(repo: &Repo, args: &Args) -> Result<ExitCode> {
    let now = args.now.unwrap_or_else(Timestamp::now);
    let inputs = Inputs::load(repo)?;
    if let Some(window) = inputs.sources.blackout_at(now) {
        eprintln!(
            "publication blackout from {} until {} (config/sources.yaml): not building the public site",
            window.from, window.until
        );
        return Ok(ExitCode::from(BLACKOUT));
    }
    let out = args.out.clone().unwrap_or_else(|| repo.root().join(".site"));
    let web_dir = args.web_dir.clone().unwrap_or_else(|| repo.root().join("app/web/dist"));
    if !web_dir.join("index.html").exists() {
        bail!(
            "{} has no index.html; run `moon run web:build` first",
            web_dir.display()
        );
    }
    if out.exists() {
        std::fs::remove_dir_all(&out).with_context(|| format!("emptying {}", out.display()))?;
    }
    copy_dir(&web_dir, &out)?;
    for name in api::NAMES {
        let value = api::document(repo, name, now)?.expect("every listed name is a document");
        write(
            &out.join("api").join(format!("{name}.json")),
            &serde_json::to_vec(&value)?,
        )?;
    }
    println!("wrote {} ({} documents in api/)", out.display(), api::NAMES.len());
    Ok(ExitCode::SUCCESS)
}

fn copy_dir(from: &Path, to: &Path) -> Result<()> {
    std::fs::create_dir_all(to)?;
    for entry in std::fs::read_dir(from).with_context(|| format!("reading {}", from.display()))? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&entry.path(), &target)?;
        } else {
            std::fs::copy(entry.path(), &target)?;
        }
    }
    Ok(())
}
