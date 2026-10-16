//! `fr2027 inbox`: bring in what Grok Bot posted to the inbox Worker.
//!
//! `pull` validates each pending item with the same rules as ingest and writes the good ones:
//! - X drops to data/raw/grok/ and events to data/raw/events/;
//! - field changes to config/candidates.yaml (config/sources.yaml for a new candidate's names)
//!   and research/LOG.md, applied block by block (`fr2027_collectors::field_change`);
//! - polls to data/proposals/polls/, which the Live workflow opens as a pull request;
//! - poll checks: a blind second entry of a proposed poll from another routine run. When it
//!   matches the proposal, the proposal is written to data/raw/polls/ (committed by this run);
//!   when it differs, both items are rejected with every difference and the proposal stays open.
//!
//! Rejections are resolved at once with the reasons, which the bot reads back. Everything
//! written is listed in `<dir>/resolutions.json` and resolved by `resolve` only after the commit
//! is pushed, so a failed run leaves the items pending for the next one. What the workflow must
//! do on the proposal pull requests is in `<dir>/pr-actions.json`, and the field changes applied
//! in `<dir>/field-changes.json` (the commit step allows the config files only when it lists one).

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use fr2027_collectors::{
    Repo, SchemaKind, events,
    field_change::{self, FieldFiles},
    grok, list_files, poll_check,
    polls::{self, PollEntry},
    pretty_json, read,
    schema::yaml_to_json,
    validate_value, write, write_new,
};
use fr2027_fetch::{
    Client,
    inbox::{self, PendingItem, Resolution},
};
use jiff::civil::Date;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::data::Inputs;

#[derive(clap::Subcommand)]
pub enum Command {
    /// Validate pending items, write the accepted ones, and list what to do after the push.
    Pull(Args),
    /// Mark the items listed by `pull` as accepted or proposed (run after the push).
    Resolve(ResolveArgs),
}

#[derive(clap::Args)]
pub struct Args {
    /// The Worker's base URL.
    #[arg(long, env = "FR2027_INBOX_URL")]
    url: String,
    /// The working directory: `open/<pull request number>/<poll_id>.yaml` (the open poll
    /// proposals, gathered by the workflow) in, the lists for the later steps out.
    #[arg(long, default_value = ".inbox")]
    dir: PathBuf,
}

#[derive(clap::Args)]
pub struct ResolveArgs {
    #[arg(long, env = "FR2027_INBOX_URL")]
    url: String,
    #[arg(long, default_value = ".inbox")]
    dir: PathBuf,
    /// Link to the pull request that carries proposed polls, recorded on those items.
    #[arg(long)]
    proposal_ref: Option<String>,
}

const TOKEN_VAR: &str = "INBOX_PIPELINE_TOKEN";
const RESOLUTIONS: &str = "resolutions.json";
const PR_ACTIONS: &str = "pr-actions.json";
const FIELD_CHANGES: &str = "field-changes.json";
const LOG: &str = "research/LOG.md";

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

fn dir_path(repo: &Repo, dir: &Path) -> PathBuf {
    if dir.is_absolute() {
        dir.to_path_buf()
    } else {
        repo.root().join(dir)
    }
}

fn pull(repo: &Repo, args: &Args) -> Result<()> {
    let dir = dir_path(repo, &args.dir);
    let user_agent = Inputs::load(repo)?.sources.user_agent;
    let token = token()?;
    let client = Client::new(&user_agent)?;
    let items = inbox::pending(&client, &args.url, &token)?;
    println!("{} pending item(s)", items.len());
    let open = read_open_proposals(&dir.join("open"))?;
    println!("{} open poll proposal(s)", open.len());
    let today = jiff::Timestamp::now().to_zoned(jiff::tz::TimeZone::UTC).date();
    let outcome = process(repo, &items, &open, today)?;
    // The lists first: the files are written, so the later steps must know of them even if
    // sending a rejection fails.
    write(&dir.join(RESOLUTIONS), &pretty_json(&outcome.after_push)?)?;
    write(&dir.join(PR_ACTIONS), &pretty_json(&outcome.pr_actions)?)?;
    write(&dir.join(FIELD_CHANGES), &pretty_json(&outcome.field_changes)?)?;
    for resolution in &outcome.now {
        send(&client, &args.url, &token, resolution)?;
    }
    println!(
        "{} item(s) to resolve after the push, {} pull request action(s), {} field change(s); listed in {}",
        outcome.after_push.len(),
        outcome.pr_actions.len(),
        outcome.field_changes.len(),
        dir.display()
    );
    Ok(())
}

/// Resolves one item. A follow-up that the inbox refuses (the item was settled since) is only
/// reported.
fn send(client: &Client, url: &str, token: &str, resolution: &Resolution) -> Result<()> {
    match inbox::resolve(client, url, token, resolution) {
        Ok(()) => {
            println!("resolved {}: {}", resolution.id, resolution.status);
            Ok(())
        }
        Err(error) if resolution.followup => {
            println!("could not resolve {} again ({error}); leaving it", resolution.id);
            Ok(())
        }
        Err(error) => Err(error.into()),
    }
}

fn resolve(repo: &Repo, args: &ResolveArgs) -> Result<()> {
    let path = dir_path(repo, &args.dir).join(RESOLUTIONS);
    let Ok(bytes) = std::fs::read(&path) else {
        println!("no manifest at {}; nothing to resolve", path.display());
        return Ok(());
    };
    let resolutions: Vec<Resolution> = serde_json::from_slice(&bytes).context("reading the manifest")?;
    if !resolutions.is_empty() {
        let client = Client::new(&Inputs::load(repo)?.sources.user_agent)?;
        let token = token()?;
        for mut resolution in resolutions {
            if resolution.status == "proposed"
                && let Some(link) = &args.proposal_ref
            {
                resolution.reference = Some(link.clone());
            }
            send(&client, &args.url, &token, &resolution)?;
        }
    }
    std::fs::remove_file(&path)?;
    Ok(())
}

/// What the workflow does on a proposal pull request after the push. `{commit}` in the comment
/// stands for the link to the commit that carries this run's data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrAction {
    pub number: u64,
    pub comment: String,
    pub close: bool,
    /// Only once this run's commit is on main: the comment or closure says a poll landed.
    pub needs_commit: bool,
}

/// A poll file on the branch of an open proposal pull request.
#[derive(Clone, Debug)]
pub struct OpenProposal {
    pub pr: u64,
    pub poll_id: String,
    pub bytes: Vec<u8>,
    /// The inbox item that sent it (from the proposal's sidecar), to settle it later.
    pub inbox_item: Option<String>,
}

/// Written beside each proposed poll and committed on the proposal branch: the inbox item to
/// settle when a check comes, and what the pull request lists for the checking routine (where to
/// read the poll, never its numbers).
#[derive(Debug, Serialize, Deserialize)]
struct Sidecar {
    poll_id: String,
    inbox_item: String,
    #[serde(default)]
    source_url: String,
    #[serde(default)]
    notice_url: Option<String>,
    #[serde(default)]
    supersedes: Option<String>,
    #[serde(default)]
    entered_by_run: String,
}

/// Reads `<dir>/<pull request number>/<poll_id>.yaml` and the optional `<poll_id>.json` sidecar.
pub fn read_open_proposals(dir: &Path) -> Result<Vec<OpenProposal>> {
    let mut open = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Ok(open);
    };
    for entry in entries {
        let path = entry?.path();
        let Some(pr) = path.file_name().and_then(|n| n.to_str()).and_then(|n| n.parse().ok()) else {
            continue;
        };
        for file in list_files(&path, "yaml")? {
            let poll_id = file.file_stem().unwrap_or_default().to_string_lossy().into_owned();
            let inbox_item = std::fs::read(file.with_extension("json"))
                .ok()
                .and_then(|bytes| serde_json::from_slice::<Sidecar>(&bytes).ok())
                .map(|sidecar| sidecar.inbox_item);
            open.push(OpenProposal {
                pr,
                bytes: read(&file)?,
                poll_id,
                inbox_item,
            });
        }
    }
    open.sort_by_key(|p| (p.pr, p.poll_id.clone()));
    Ok(open)
}

/// What one pull decided.
#[derive(Debug, Default)]
pub struct Outcome {
    /// Resolved at once: rejections, for which nothing is written.
    pub now: Vec<Resolution>,
    /// Resolved after the push: items whose files this run commits or proposes.
    pub after_push: Vec<Resolution>,
    pub pr_actions: Vec<PrAction>,
    /// Ids of the field changes applied.
    pub field_changes: Vec<String>,
}

/// Where the proposal a check is compared with comes from.
enum Proposal<'a> {
    /// Sent in this pull: data/proposals/polls/<poll_id>.yaml, not yet in a pull request.
    ThisPull {
        item: String,
    },
    Open(&'a OpenProposal),
}

/// The running state of one pull.
struct Pull<'a> {
    repo: &'a Repo,
    open: &'a [OpenProposal],
    today: Date,
    out: Outcome,
    /// Polls proposed in this pull: poll_id → inbox item.
    proposed: BTreeMap<String, String>,
    /// Open proposals a newer poll replaced: (pull request, poll_id).
    replaced: BTreeSet<(u64, String)>,
    comments: BTreeMap<u64, Vec<String>>,
    landed_in: BTreeSet<u64>,
}

/// Items in the order they depend on each other: events (a field change may cite them), then
/// field changes (a poll may use a new candidate), then polls, then the checks of polls.
fn phase(kind: &str) -> u8 {
    match kind {
        "field_change" => 1,
        "poll" => 2,
        "poll_check" => 3,
        _ => 0,
    }
}

pub fn process(repo: &Repo, items: &[PendingItem], open: &[OpenProposal], today: Date) -> Result<Outcome> {
    let mut items: Vec<&PendingItem> = items.iter().collect();
    items.sort_by_key(|item| phase(&item.kind));
    let mut pull = Pull {
        repo,
        open,
        today,
        out: Outcome::default(),
        proposed: BTreeMap::new(),
        replaced: BTreeSet::new(),
        comments: BTreeMap::new(),
        landed_in: BTreeSet::new(),
    };
    let mut inputs = Inputs::load(repo)?;
    for item in items {
        let result = match item.kind.as_str() {
            "x_drop" | "event" => handle_data(repo, &inputs, item).map(|r| pull.out.after_push.push(r)),
            "field_change" => {
                let result = pull.field_change(&inputs, item);
                if result.is_ok() {
                    inputs = Inputs::load(repo)?;
                }
                result
            }
            "poll" => pull.poll(&inputs, item),
            "poll_check" => pull.check(&inputs, item),
            other => Err(vec![format!(
                "unknown kind `{other}`; expected x_drop, event, poll, poll_check or field_change"
            )]),
        };
        if let Err(errors) = result {
            pull.out
                .now
                .push(resolution(&item.id, "rejected", errors.join("; "), None));
        }
    }
    pull.close_finished();
    for r in pull.out.now.iter().chain(&pull.out.after_push) {
        println!("{}: {} {}", r.id, r.status, r.note);
    }
    Ok(pull.out)
}

fn resolution(id: &str, status: &str, note: String, reference: Option<String>) -> Resolution {
    Resolution {
        id: id.into(),
        status: status.into(),
        note,
        reference,
        followup: false,
    }
}

impl Pull<'_> {
    fn field_change(&mut self, inputs: &Inputs, item: &PendingItem) -> Result<(), Vec<String>> {
        let change = field_change::parse(item.payload.clone(), &inputs.schemas)?;
        let paths = [
            self.repo.config_dir().join("candidates.yaml"),
            self.repo.config_dir().join("sources.yaml"),
            self.repo.root().join(LOG),
        ];
        let text =
            |path: &Path| std::fs::read_to_string(path).map_err(|e| vec![format!("reading {}: {e}", path.display())]);
        let files = FieldFiles {
            candidates: text(&paths[0])?,
            sources: text(&paths[1])?,
            log: text(&paths[2])?,
        };
        let reference = Some("config/candidates.yaml".to_string());
        if field_change::already_applied(&files.log, &change.change_id) {
            self.out.after_push.push(resolution(
                &item.id,
                "accepted",
                format!(
                    "already applied (research/LOG.md has {})",
                    field_change::log_marker(&change.change_id)
                ),
                reference,
            ));
            return Ok(());
        }
        let events = self.repo.raw_events_dir();
        let changed = field_change::apply(
            &change,
            &files,
            &inputs.schemas,
            |id| events.join(format!("{id}.json")).is_file(),
            self.today,
        )?;
        for (path, (old, new)) in paths.iter().zip([
            (&files.candidates, &changed.candidates),
            (&files.sources, &changed.sources),
            (&files.log, &changed.log),
        ]) {
            if old != new {
                write(path, new.as_bytes()).map_err(|e| vec![format!("could not write: {e}")])?;
            }
        }
        self.out.field_changes.push(change.change_id.clone());
        self.out.after_push.push(resolution(
            &item.id,
            "accepted",
            format!(
                "applied to config/candidates.yaml ({} operation(s)) and logged in research/LOG.md",
                change.operations.len()
            ),
            reference,
        ));
        Ok(())
    }

    fn poll(&mut self, inputs: &Inputs, item: &PendingItem) -> Result<(), Vec<String>> {
        let (poll_id, yaml, entry) = parse_poll(self.repo, inputs, &item.payload, SchemaKind::Poll)?;
        if entry.agent.is_none() {
            return Err(vec![
                "a poll sent to the inbox must carry agent {name, skill_version, run_id} (docs/grok-bot.md)".into(),
            ]);
        }
        let raw = self.repo.raw_polls_dir().join(format!("{poll_id}.yaml"));
        if raw.exists() {
            if same_content(&raw, yaml.as_bytes()) {
                self.out.after_push.push(resolution(
                    &item.id,
                    "accepted",
                    "already on main".into(),
                    Some(self.repo.relative(&raw)),
                ));
                return Ok(());
            }
            return Err(vec![format!(
                "poll `{poll_id}` is already in the repository; to correct it, send a new poll_id with `supersedes: {poll_id}`"
            )]);
        }
        if let Some(old) = &entry.supersedes {
            self.check_supersedes(inputs, old)?;
        }
        let dir = self.repo.proposals_dir().join("polls");
        let path = dir.join(format!("{poll_id}.yaml"));
        let io = |e: fr2027_collectors::Error| vec![format!("could not write: {e}")];
        if !write_new(&path, yaml.as_bytes()).map_err(io)? {
            return Err(vec![format!("poll `{poll_id}` was already proposed in this run")]);
        }
        let sidecar = Sidecar {
            poll_id: poll_id.clone(),
            inbox_item: item.id.clone(),
            source_url: entry.source_url.clone(),
            notice_url: entry.notice_url.clone(),
            supersedes: entry.supersedes.clone(),
            entered_by_run: entry.agent.as_ref().map(|a| a.run_id.clone()).unwrap_or_default(),
        };
        write(
            &dir.join(format!("{poll_id}.json")),
            &pretty_json(&sidecar).map_err(|e| vec![e.to_string()])?,
        )
        .map_err(io)?;
        let open_proposals: &[OpenProposal] = self.open;
        for open in open_proposals.iter().filter(|o| o.poll_id == poll_id) {
            if self.replaced.insert((open.pr, poll_id.clone())) {
                self.comment(
                    open.pr,
                    format!(
                        "`{poll_id}` is replaced by a newer proposal (inbox item {}); this copy will not land.",
                        item.id
                    ),
                );
                if let Some(old) = &open.inbox_item {
                    self.out.now.push(Resolution {
                        followup: true,
                        ..resolution(
                            old,
                            "rejected",
                            format!("replaced by a newer proposal of `{poll_id}` (inbox item {})", item.id),
                            None,
                        )
                    });
                }
            }
        }
        self.proposed.insert(poll_id, item.id.clone());
        self.out.after_push.push(resolution(
            &item.id,
            "proposed",
            "valid; proposed in a pull request, and committed to main when a blind poll_check from another routine run matches it".into(),
            Some(self.repo.relative(&path)),
        ));
        Ok(())
    }

    /// A superseding poll must name a poll on main that nothing supersedes yet.
    fn check_supersedes(&self, inputs: &Inputs, old: &str) -> Result<(), Vec<String>> {
        if !self.repo.raw_polls_dir().join(format!("{old}.yaml")).exists() {
            return Err(vec![format!(
                "supersedes unknown poll `{old}` (not in data/raw/polls/)"
            )]);
        }
        let ingest = polls::ingest(self.repo, &inputs.schemas, &inputs.config).map_err(|e| vec![e.to_string()])?;
        if ingest.superseded.iter().any(|id| id == old) {
            return Err(vec![format!("poll `{old}` is already superseded by another file")]);
        }
        Ok(())
    }

    fn check(&mut self, inputs: &Inputs, item: &PendingItem) -> Result<(), Vec<String>> {
        let (poll_id, _, check) = parse_poll(self.repo, inputs, &item.payload, SchemaKind::PollCheck)?;
        let raw = self.repo.raw_polls_dir().join(format!("{poll_id}.yaml"));
        if raw.exists() {
            return Err(vec![format!("nothing to check: `{poll_id}` is already on main")]);
        }
        let open_proposals: &[OpenProposal] = self.open;
        let proposal = if let Some(item) = self.proposed.get(&poll_id) {
            Proposal::ThisPull { item: item.clone() }
        } else if let Some(open) = open_proposals
            .iter()
            .rev()
            .find(|o| o.poll_id == poll_id && !self.replaced.contains(&(o.pr, poll_id.clone())))
        {
            Proposal::Open(open)
        } else {
            return Err(vec![format!(
                "nothing to check: no open proposal for `{poll_id}` (send the poll first, kind poll)"
            )]);
        };
        let pending = self.repo.proposals_dir().join("polls").join(format!("{poll_id}.yaml"));
        let bytes = match &proposal {
            Proposal::ThisPull { .. } => read(&pending).map_err(|e| vec![e.to_string()])?,
            Proposal::Open(open) => open.bytes.clone(),
        };
        let proposed = polls::validate_file(&raw, &bytes, &inputs.schemas, &inputs.config).map_err(|e| {
            vec![format!(
                "the proposal of `{poll_id}` no longer validates: {}",
                e.join("; ")
            )]
        })?;
        poll_check::independence(&proposed, &check).map_err(|e| vec![e])?;
        let checker = &check.agent.as_ref().expect("checked by independence").run_id;
        let place = match &proposal {
            Proposal::ThisPull { .. } => "sent in the same pull".to_string(),
            Proposal::Open(open) => format!("in pull request #{}", open.pr),
        };

        let differences = poll_check::differences(&proposed, &check);
        if !differences.is_empty() {
            let listed = differences.join("; ");
            let proposer_note = format!(
                "a blind check (inbox item {}, run {checker}) differs: {listed}. Re-read those figures in the notice or publication; if yours are wrong, resend the corrected poll with the same poll_id. The proposal stays open for another check",
                item.id
            );
            match &proposal {
                Proposal::ThisPull { item: poll_item } => {
                    // Still proposed in a pull request, but the item says the numbers are in doubt.
                    self.out.after_push.retain(|r| r.id != *poll_item);
                    self.out
                        .now
                        .push(resolution(poll_item, "rejected", proposer_note, None));
                }
                Proposal::Open(open) => {
                    if let Some(old) = &open.inbox_item {
                        self.out.now.push(Resolution {
                            followup: true,
                            ..resolution(old, "rejected", proposer_note, None)
                        });
                    }
                    let lines: String = differences.iter().map(|d| format!("- {d}\n")).collect();
                    self.comment(
                        open.pr,
                        format!(
                            "A blind check of `{poll_id}` (inbox item {}, run {checker}) differs from this proposal in:\n\n{lines}\nThe notice or publication is the reference. The proposal stays open for another check.",
                            item.id
                        ),
                    );
                }
            }
            // Every note and comment names what differs, never a value from either entry.
            return Err(vec![format!(
                "differs from the proposal of `{poll_id}` ({place}): {listed}. Re-read those figures in the notice or publication and send the check again"
            )]);
        }

        if let Some(old) = &proposed.supersedes {
            self.check_supersedes(inputs, old)?;
        }
        if !write_new(&raw, &bytes).map_err(|e| vec![format!("could not write: {e}")])? {
            return Err(vec![format!("nothing to check: `{poll_id}` is already on main")]);
        }
        let landed = format!(
            "a blind check (inbox item {}, run {checker}) matches the proposal on {}; committed to data/raw/polls/{poll_id}.yaml",
            item.id,
            poll_check::COMPARED
        );
        let reference = Some(self.repo.relative(&raw));
        match proposal {
            Proposal::ThisPull { item: poll_item } => {
                let _ = std::fs::remove_file(&pending);
                let _ = std::fs::remove_file(pending.with_extension("json"));
                self.proposed.remove(&poll_id);
                self.out.after_push.retain(|r| r.id != poll_item);
                self.out
                    .after_push
                    .push(resolution(&poll_item, "accepted", landed.clone(), reference.clone()));
            }
            Proposal::Open(open) => {
                if let Some(old) = &open.inbox_item {
                    self.out.after_push.push(Resolution {
                        followup: true,
                        ..resolution(old, "accepted", landed.clone(), reference.clone())
                    });
                }
                self.landed_in.insert(open.pr);
                self.comment(
                    open.pr,
                    format!(
                        "A blind check of `{poll_id}` (inbox item {}, run {checker}) matches this proposal on {}. The file is committed to main in {{commit}}.",
                        item.id,
                        poll_check::COMPARED
                    ),
                );
            }
        }
        self.out.after_push.push(resolution(
            &item.id,
            "accepted",
            format!("matches the proposal ({place}); committed to data/raw/polls/{poll_id}.yaml"),
            reference,
        ));
        Ok(())
    }

    fn comment(&mut self, pr: u64, text: String) {
        self.comments.entry(pr).or_default().push(text);
    }

    /// One action per pull request that has comments, closing those whose every poll is on
    /// main or replaced.
    fn close_finished(&mut self) {
        let prs: BTreeSet<u64> = self.open.iter().map(|o| o.pr).collect();
        for pr in prs {
            let finished = self.open.iter().filter(|o| o.pr == pr).all(|o| {
                self.replaced.contains(&(pr, o.poll_id.clone()))
                    || self.repo.raw_polls_dir().join(format!("{}.yaml", o.poll_id)).exists()
            });
            let mut comments = self.comments.remove(&pr).unwrap_or_default();
            if finished {
                comments.push("Every poll in this pull request is on main or replaced; closing.".into());
            }
            if !comments.is_empty() {
                self.out.pr_actions.push(PrAction {
                    number: pr,
                    comment: comments.join("\n\n"),
                    close: finished,
                    needs_commit: self.landed_in.contains(&pr),
                });
            }
        }
    }
}

/// Checks a poll or poll-check payload: id, field order (no unknown fields), schema `kind`
/// (a check also passes the poll schema), candidates and sums.
fn parse_poll(
    repo: &Repo,
    inputs: &Inputs,
    payload: &Value,
    kind: SchemaKind,
) -> Result<(String, String, PollEntry), Vec<String>> {
    let Some(poll_id) = payload["poll_id"].as_str().map(String::from) else {
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
    if kind != SchemaKind::Poll {
        let violations = inputs.schemas.violations(kind, payload);
        if !violations.is_empty() {
            return Err(violations);
        }
    }
    let yaml = poll_yaml(payload).map_err(|e| vec![e])?;
    let path = repo.raw_polls_dir().join(format!("{poll_id}.yaml"));
    let entry = polls::validate_file(&path, yaml.as_bytes(), &inputs.schemas, &inputs.config)?;
    Ok((poll_id, yaml, entry))
}

/// X drops and events: validated and written as new raw files.
fn handle_data(repo: &Repo, inputs: &Inputs, item: &PendingItem) -> Result<Resolution, Vec<String>> {
    let accepted = |note: String, path: &Path| resolution(&item.id, "accepted", note, Some(repo.relative(path)));
    let io = |e: fr2027_collectors::Error| vec![format!("could not write: {e}")];
    if item.kind == "x_drop" {
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
        return Ok(accepted(note, &path));
    }
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
    Ok(accepted("accepted".into(), &path))
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
    const ORDER: [&str; 19] = [
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
        "agent",
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
mod pull_tests;

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
            "entry": "primary",
            "agent": {"name": "Grok Bot fr2027", "skill_version": "polls-2", "run_id": "polls-2026-10-12T07:30Z"}
        });
        let yaml = poll_yaml(&payload).unwrap();
        assert!(yaml.starts_with("schema_version:"), "{yaml}");
        assert!(yaml.find("poll_id").unwrap() < yaml.find("scenarios").unwrap());
        assert!(yaml.find("agent").unwrap() < yaml.find("scenarios").unwrap());
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
