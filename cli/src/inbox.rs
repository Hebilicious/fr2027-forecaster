//! `fr2027 inbox`: bring in what Grok Bot posted to the inbox Worker.
//!
//! `pull` validates each pending item with the same rules as ingest and writes the good ones:
//! X drops to data/raw/grok/, events to data/raw/events/, polls to data/proposals/polls/ for a
//! pull request (a poll moves the forecast, so a person merges it). Rejections are resolved at
//! once with the reasons, which the bot reads back. Everything written is listed in a manifest
//! and resolved by `resolve` only after the commit is pushed, so a failed run leaves the items
//! pending for the next one.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use fr2027_collectors::{
    Repo, SchemaKind, events, grok, polls, pretty_json, schema::yaml_to_json, validate_value, write, write_new,
};
use fr2027_fetch::{
    Client,
    inbox::{self, PendingItem, Resolution},
};
use serde_json::Value;

use crate::data::Inputs;

#[derive(clap::Subcommand)]
pub enum Command {
    /// Validate pending items, write the accepted ones, and list them in the manifest.
    Pull(Args),
    /// Mark the items in the manifest as accepted or proposed (run after the push).
    Resolve(ResolveArgs),
}

#[derive(clap::Args)]
pub struct Args {
    /// The Worker's base URL.
    #[arg(long, env = "FR2027_INBOX_URL")]
    url: String,
    /// Where to list the items to resolve after the push.
    #[arg(long, default_value = ".inbox/resolutions.json")]
    manifest: PathBuf,
}

#[derive(clap::Args)]
pub struct ResolveArgs {
    #[arg(long, env = "FR2027_INBOX_URL")]
    url: String,
    #[arg(long, default_value = ".inbox/resolutions.json")]
    manifest: PathBuf,
    /// Link to the pull request that carries proposed polls, recorded on those items.
    #[arg(long)]
    proposal_ref: Option<String>,
}

const TOKEN_VAR: &str = "INBOX_PIPELINE_TOKEN";

fn token() -> Result<String> {
    std::env::var(TOKEN_VAR)
        .ok()
        .filter(|t| !t.is_empty())
        .with_context(|| format!("set {TOKEN_VAR} to the inbox pipeline token"))
}

pub fn run(repo: &Repo, command: &Command) -> Result<()> {
    match command {
        Command::Pull(args) => pull(repo, args),
        Command::Resolve(args) => resolve(repo, args),
    }
}

fn manifest_path(repo: &Repo, manifest: &Path) -> PathBuf {
    if manifest.is_absolute() {
        manifest.to_path_buf()
    } else {
        repo.root().join(manifest)
    }
}

fn pull(repo: &Repo, args: &Args) -> Result<()> {
    let inputs = Inputs::load(repo)?;
    let token = token()?;
    let client = Client::new(&inputs.sources.user_agent)?;
    let items = inbox::pending(&client, &args.url, &token)?;
    println!("{} pending item(s)", items.len());
    let mut to_resolve = Vec::new();
    for item in items {
        let resolution = match handle(repo, &inputs, &item) {
            Ok(resolution) => resolution,
            Err(errors) => Resolution {
                id: item.id.clone(),
                status: "rejected".into(),
                note: errors.join("; "),
                reference: None,
            },
        };
        println!("{} ({}): {} {}", item.id, item.kind, resolution.status, resolution.note);
        if resolution.status == "rejected" {
            inbox::resolve(&client, &args.url, &token, &resolution)?;
        } else {
            to_resolve.push(resolution);
        }
    }
    let path = manifest_path(repo, &args.manifest);
    write(&path, &pretty_json(&to_resolve)?)?;
    println!(
        "{} item(s) to resolve after the push, listed in {}",
        to_resolve.len(),
        path.display()
    );
    Ok(())
}

fn resolve(repo: &Repo, args: &ResolveArgs) -> Result<()> {
    let path = manifest_path(repo, &args.manifest);
    let Ok(bytes) = std::fs::read(&path) else {
        println!("no manifest at {}; nothing to resolve", path.display());
        return Ok(());
    };
    let resolutions: Vec<Resolution> = serde_json::from_slice(&bytes).context("reading the manifest")?;
    if resolutions.is_empty() {
        return Ok(());
    }
    let client = Client::new(&Inputs::load(repo)?.sources.user_agent)?;
    let token = token()?;
    for mut resolution in resolutions {
        if resolution.status == "proposed"
            && let Some(link) = &args.proposal_ref
        {
            resolution.reference = Some(link.clone());
        }
        inbox::resolve(&client, &args.url, &token, &resolution)?;
        println!("resolved {}: {}", resolution.id, resolution.status);
    }
    std::fs::remove_file(&path)?;
    Ok(())
}

/// Validates one item and writes it. `Err` holds the reasons it is rejected.
fn handle(repo: &Repo, inputs: &Inputs, item: &PendingItem) -> Result<Resolution, Vec<String>> {
    let accepted = |status: &str, note: String, path: &Path| Resolution {
        id: item.id.clone(),
        status: status.into(),
        note,
        reference: Some(repo.relative(path)),
    };
    let io = |e: fr2027_collectors::Error| vec![format!("could not write: {e}")];
    match item.kind.as_str() {
        "x_drop" => {
            let mut payload = item.payload.clone();
            let dropped = strip_post_ids(&mut payload);
            let stem = grok::validate(payload.clone(), None, &inputs.schemas, &inputs.config)?;
            let path = repo.raw_grok_dir().join(format!("{stem}.json"));
            let bytes = pretty_json(&payload).map_err(|e| vec![e.to_string()])?;
            if !write_new(&path, &bytes).map_err(io)? && !same_content(&path, &bytes) {
                return Err(vec![format!(
                    "a drop for the window ending {stem} already exists; send each window once"
                )]);
            }
            let note = if dropped > 0 {
                format!("accepted; {dropped} sample post id(s) dropped (post ids are not stored)")
            } else {
                "accepted".into()
            };
            Ok(accepted("accepted", note, &path))
        }
        "event" => {
            let event: events::Event = validate_value(item.payload.clone(), &inputs.schemas, SchemaKind::Event)?;
            let errors = events::check(None, &event, &inputs.config);
            if !errors.is_empty() {
                return Err(errors);
            }
            let path = repo.raw_events_dir().join(format!("{}.json", event.event_id));
            let bytes = pretty_json(&item.payload).map_err(|e| vec![e.to_string()])?;
            if !write_new(&path, &bytes).map_err(io)? && !same_content(&path, &bytes) {
                return Err(vec![format!(
                    "event_id `{}` already exists; pick another slug",
                    event.event_id
                )]);
            }
            Ok(accepted("accepted", "accepted".into(), &path))
        }
        "poll" => {
            let Some(poll_id) = item.payload["poll_id"].as_str().map(String::from) else {
                return Err(vec!["poll_id is missing".into()]);
            };
            if !poll_id
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
            {
                return Err(vec![format!(
                    "poll_id `{poll_id}` must be lowercase letters, digits and dashes"
                )]);
            }
            let yaml = poll_yaml(&item.payload).map_err(|e| vec![e])?;
            let raw = repo.raw_polls_dir().join(format!("{poll_id}.yaml"));
            polls::validate_file(&raw, yaml.as_bytes(), &inputs.schemas, &inputs.config)?;
            if raw.exists() {
                if same_content(&raw, yaml.as_bytes()) {
                    return Ok(accepted("accepted", "already merged".into(), &raw));
                }
                return Err(vec![format!(
                    "poll `{poll_id}` is already in the repository; to correct it, send a new poll_id with `supersedes: {poll_id}`"
                )]);
            }
            let path = repo.proposals_dir().join("polls").join(format!("{poll_id}.yaml"));
            if !write_new(&path, yaml.as_bytes()).map_err(io)? {
                return Err(vec![format!("poll `{poll_id}` was already proposed in this run")]);
            }
            Ok(accepted(
                "proposed",
                "valid; proposed in a pull request for review".into(),
                &path,
            ))
        }
        other => Err(vec![format!("unknown kind `{other}`; expected x_drop, poll or event")]),
    }
}

/// Whether `path` already holds exactly `bytes`: an item a previous run wrote but could not
/// resolve is accepted again rather than rejected as a duplicate.
fn same_content(path: &Path, bytes: &[u8]) -> bool {
    std::fs::read(path).is_ok_and(|existing| existing == bytes)
}

/// Removes `sample_post_ids` from every candidate, returning how many ids were dropped.
fn strip_post_ids(payload: &mut Value) -> usize {
    let mut dropped = 0;
    if let Some(candidates) = payload.get_mut("candidates").and_then(Value::as_array_mut) {
        for candidate in candidates {
            if let Some(object) = candidate.as_object_mut()
                && let Some(ids) = object.remove("sample_post_ids")
            {
                dropped += ids.as_array().map_or(0, Vec::len);
            }
        }
    }
    dropped
}

/// The poll as YAML with its fields in the usual order, so a proposed file reads like the rest.
fn poll_yaml(payload: &Value) -> Result<String, String> {
    const ORDER: [&str; 18] = [
        "schema_version",
        "poll_id",
        "supersedes",
        "firm",
        "sponsor",
        "field_start",
        "field_end",
        "published_at",
        "sample_size",
        "method",
        "population",
        "source_url",
        "notice_url",
        "index_url",
        "retrieved_at",
        "entry",
        "notes",
        "scenarios",
    ];
    let Some(object) = payload.as_object() else {
        return Err("payload must be an object".into());
    };
    if let Some(unknown) = object.keys().find(|k| !ORDER.contains(&k.as_str())) {
        return Err(format!("unknown field `{unknown}`"));
    }
    let mut out = String::new();
    for key in ORDER {
        if let Some(value) = object.get(key) {
            let mut single = serde_json::Map::new();
            single.insert(key.into(), value.clone());
            out.push_str(&serde_saphyr::to_string(&Value::Object(single)).map_err(|e| e.to_string())?);
        }
    }
    // A poll must parse back to exactly what was sent.
    if yaml_to_json(&out)? != *payload {
        return Err("the poll does not survive conversion to YAML; check for unusual values".into());
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poll_yaml_keeps_field_order_and_round_trips() {
        let payload = serde_json::json!({
            "scenarios": [{"scenario_id": "S1", "round": 1, "shares": {"le_pen": 33.5, "philippe": 20}}],
            "poll_id": "ifop-2026-10-12",
            "schema_version": "1.0",
            "firm": "ifop",
            "sponsor": null,
            "field_start": "2026-10-08",
            "field_end": "2026-10-10",
            "published_at": "2026-10-12",
            "sample_size": 1105,
            "method": "online",
            "population": "registered",
            "source_url": "https://example.org/a.pdf",
            "retrieved_at": "2026-10-12T08:00:00Z",
            "entry": "primary"
        });
        let yaml = poll_yaml(&payload).unwrap();
        assert!(yaml.starts_with("schema_version:"), "{yaml}");
        assert!(yaml.find("poll_id").unwrap() < yaml.find("scenarios").unwrap());
        assert_eq!(yaml_to_json(&yaml).unwrap(), payload);
        assert!(poll_yaml(&serde_json::json!({"poll_id": "x", "extra": 1})).is_err());
    }

    #[test]
    fn post_ids_are_dropped() {
        let mut payload = serde_json::json!({"candidates": [
            {"candidate_id": "a", "sample_post_ids": ["1", "2"]},
            {"candidate_id": "b", "sample_post_ids": null},
            {"candidate_id": "c"}
        ]});
        assert_eq!(strip_post_ids(&mut payload), 2);
        assert!(payload["candidates"][0].get("sample_post_ids").is_none());
        assert!(payload["candidates"][1].get("sample_post_ids").is_none());
    }
}
