//! `fr2027 forward`: log the signal flags seen for the first time ([`fr2027_collectors::forward`])
//! into a new `data/forward/<hour>.json`, so they can be scored against later polls.

use anyhow::{Result, bail};
use fr2027_collectors::{
    Repo, SchemaKind,
    config::load_model_params,
    forward::{self, Flag, FlagFile, FlagKind},
    grok, pretty_json, signals, write_new,
};
use fr2027_model::{SignalSource, day_number, signals::detect};
use jiff::{Timestamp, tz::TimeZone};

use crate::data::Inputs;

/// Flags about days longer ago than this are not logged: they would not be forward.
const RECENT_DAYS: i32 = 7;

#[derive(clap::Args)]
pub struct Args {
    /// Time recorded as detected_at, which also names the file (default: now).
    #[arg(long)]
    now: Option<Timestamp>,
}

pub fn run(repo: &Repo, args: &Args) -> Result<()> {
    let inputs = Inputs::load(repo)?;
    let params = load_model_params(repo)?;
    let field = inputs.config.field()?;
    let now = args.now.unwrap_or_else(Timestamp::now);
    let today = now.to_zoned(TimeZone::UTC).date();
    let recent = |date: jiff::civil::Date| day_number(today) - day_number(date) <= RECENT_DAYS;

    let (logged, rejections) = forward::read(repo, &inputs.schemas)?;
    if let Some(rejection) = rejections.first() {
        bail!(
            "{} is invalid: {}",
            repo.relative(&rejection.path),
            rejection.errors.join("; ")
        );
    }
    let known = forward::logged_keys(&logged);

    let mut flags: Vec<Flag> = detect(&signals::read_clean(repo)?, &field, &params.signals, today)
        .into_iter()
        .filter(|s| recent(s.date))
        .map(|s| Flag {
            kind: FlagKind::Spike,
            candidate_id: field.candidates[s.candidate].id.clone(),
            date: s.date.to_string(),
            score: (s.score * 10_000.0).round() / 10_000.0,
            detail: s
                .ratios
                .iter()
                .map(|(source, ratio)| {
                    let name = match source {
                        SignalSource::Attention => "page views",
                        SignalSource::News => "articles",
                        SignalSource::XSupport => "X support",
                    };
                    format!("{name} {ratio:.1}× their baseline")
                })
                .collect::<Vec<_>>()
                .join(", "),
        })
        .collect();
    flags.extend(
        forward::x_shifts(&grok::read_clean_rows(repo)?)
            .into_iter()
            .filter(|f| f.date.parse().is_ok_and(recent)),
    );
    flags.retain(|f| !known.contains(&f.key()));
    if flags.is_empty() {
        println!("no new signal flag");
        return Ok(());
    }
    let file = FlagFile {
        schema_version: "1.0".into(),
        detected_at: now.strftime("%Y-%m-%dT%H:%M:%SZ").to_string(),
        flags,
    };
    let value = serde_json::to_value(&file)?;
    let violations = inputs.schemas.violations(SchemaKind::ForwardFlags, &value);
    if !violations.is_empty() {
        bail!("refusing to write the forward log: {}", violations.join("; "));
    }
    let path = repo.forward_dir().join(format!("{}.json", now.strftime("%Y-%m-%dT%H")));
    if write_new(&path, &pretty_json(&value)?)? {
        println!("wrote {} ({} new flags)", repo.relative(&path), file.flags.len());
    } else {
        println!("{} already exists; nothing written this hour", repo.relative(&path));
    }
    Ok(())
}
