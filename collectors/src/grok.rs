//! Grok drops: `data/raw/grok/YYYY-MM-DDTHH-MM.json` → `data/clean/grok.csv`.

use std::collections::BTreeSet;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{
    Rejection, Repo, Result, SchemaKind, Schemas, config::CandidatesConfig, content_hash, csv_bytes, list_files, read,
    read_csv, write, write_quarantine,
};

pub const CLEAN_FILE: &str = "grok.csv";

#[derive(Clone, Debug, Deserialize)]
struct Drop {
    collected_at: String,
    window: Window,
    candidates: Vec<CandidateActivity>,
    #[serde(default)]
    rubric_version: Option<String>,
    #[serde(default)]
    sampling: Option<Sampling>,
    #[serde(default)]
    double_labels: Option<DoubleLabels>,
}

/// How routine 1 drew the posts behind `stance` (only the fields the checks use).
#[derive(Clone, Debug, Deserialize)]
struct Sampling {
    posts_per_candidate: u64,
}

/// The posts labelled twice: `counts[first][second]`, in [`LABELS`] order once parsed.
#[derive(Clone, Debug, Deserialize)]
struct DoubleLabels {
    posts: u64,
    counts: LabelTable,
}

#[derive(Clone, Debug, Deserialize)]
struct LabelTable {
    support: LabelCounts,
    oppose: LabelCounts,
    neutral: LabelCounts,
    off_topic: LabelCounts,
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct LabelCounts {
    support: u64,
    oppose: u64,
    neutral: u64,
    off_topic: u64,
}

/// The four labels of the stance rubric, in the order of every table here.
const LABELS: [&str; 4] = ["support", "oppose", "neutral", "off_topic"];

impl LabelCounts {
    fn cells(self) -> [u64; 4] {
        [self.support, self.oppose, self.neutral, self.off_topic]
    }
}

impl LabelTable {
    /// `matrix()[first][second]`, rows and columns in [`LABELS`] order.
    fn matrix(&self) -> [[u64; 4]; 4] {
        [
            self.support.cells(),
            self.oppose.cells(),
            self.neutral.cells(),
            self.off_topic.cells(),
        ]
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct Stance {
    read_all: bool,
    posts_drawn: u64,
    off_topic: u64,
    support: u64,
    oppose: u64,
    neutral: u64,
}

impl Stance {
    /// Posts about the candidate: the denominator of every share.
    fn relevant(self) -> u64 {
        self.support + self.oppose + self.neutral
    }

    /// The first-pass totals, in [`LABELS`] order.
    fn cells(self) -> [u64; 4] {
        [self.support, self.oppose, self.neutral, self.off_topic]
    }
}

#[derive(Clone, Debug, Deserialize)]
struct Window {
    start: String,
    end: String,
}

#[derive(Clone, Debug, Deserialize)]
struct CandidateActivity {
    candidate_id: String,
    mentions: Option<u64>,
    unique_authors: Option<u64>,
    engagement: Option<u64>,
    sentiment: Option<Sentiment>,
    top_topics: Option<Vec<String>>,
    bot_share_estimate: Option<f64>,
    #[serde(default)]
    mentions_method: Option<String>,
    #[serde(default)]
    sample_size: Option<u64>,
    #[serde(default)]
    stance: Option<Stance>,
}

#[derive(Clone, Debug, Deserialize)]
struct Sentiment {
    pos: f64,
    neg: f64,
    neu: f64,
    method: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GrokRow {
    pub window_start: String,
    pub window_end: String,
    pub collected_at: String,
    pub candidate_id: String,
    pub mentions: Option<u64>,
    pub mentions_method: Option<String>,
    pub sample_size: Option<u64>,
    pub unique_authors: Option<u64>,
    pub engagement: Option<u64>,
    pub sentiment_pos: Option<f64>,
    pub sentiment_neg: Option<f64>,
    pub sentiment_neu: Option<f64>,
    pub sentiment_method: Option<String>,
    pub bot_share_estimate: Option<f64>,
    /// Topics joined with ` | `.
    pub top_topics: Option<String>,
    /// The stance rubric the labels follow (`docs/x-stance-rubric.md`); empty for old drops.
    pub rubric_version: Option<String>,
    /// Whether every eligible post was read (fewer mentions than the sampling threshold).
    pub stance_read_all: Option<bool>,
    pub stance_posts_drawn: Option<u64>,
    pub stance_off_topic: Option<u64>,
    pub stance_support: Option<u64>,
    pub stance_oppose: Option<u64>,
    pub stance_neutral: Option<u64>,
    /// Shares among relevant posts (support + oppose + neutral), each with its 95% Wilson
    /// interval; empty when no drawn post was about the candidate.
    pub support_share: Option<f64>,
    pub support_low: Option<f64>,
    pub support_high: Option<f64>,
    pub oppose_share: Option<f64>,
    pub oppose_low: Option<f64>,
    pub oppose_high: Option<f64>,
    pub neutral_share: Option<f64>,
    pub neutral_low: Option<f64>,
    pub neutral_high: Option<f64>,
    /// Drop-level, repeated on each of the drop's rows: posts labelled twice, the share of them
    /// given the same label both times, and Cohen's kappa over the four labels.
    pub drop_double_labelled: Option<u64>,
    pub drop_agreement: Option<f64>,
    pub drop_kappa: Option<f64>,
    pub source_file: String,
    pub content_hash: String,
}

#[derive(Clone, Debug, Default)]
pub struct GrokIngest {
    pub rows: Vec<GrokRow>,
    pub accepted: usize,
    pub rejections: Vec<Rejection>,
}

pub fn ingest(repo: &Repo, schemas: &Schemas, config: &CandidatesConfig) -> Result<GrokIngest> {
    let mut ingest = GrokIngest::default();
    for path in list_files(&repo.raw_grok_dir(), "json")? {
        let bytes = read(&path)?;
        match validate_file(&path, &bytes, schemas, config) {
            Ok(drop) => {
                ingest.accepted += 1;
                let source_file = repo.relative(&path);
                let hash = content_hash(&bytes);
                let reliability = drop.double_labels.as_ref().map(|d| Reliability::of(&d.counts.matrix()));
                for activity in drop.candidates {
                    let stance = activity.stance;
                    let shares = stance.and_then(StanceShares::of);
                    // Old drops carry sentiment; new ones carry stance, from which it is derived.
                    let sentiment = activity.sentiment.or_else(|| {
                        stance.zip(shares).zip(drop.rubric_version.as_deref()).map(
                            |((stance, shares), rubric)| {
                                let method = format!(
                                    "stance counts, rubric {rubric}: support, oppose, neutral among {} relevant of {} drawn posts",
                                    stance.relevant(),
                                    stance.posts_drawn
                                );
                                Sentiment {
                                    pos: shares.support.share,
                                    neg: shares.oppose.share,
                                    neu: shares.neutral.share,
                                    method,
                                }
                            },
                        )
                    });
                    let (support, oppose, neutral) = (
                        shares.map(|s| s.support),
                        shares.map(|s| s.oppose),
                        shares.map(|s| s.neutral),
                    );
                    ingest.rows.push(GrokRow {
                        window_start: drop.window.start.clone(),
                        window_end: drop.window.end.clone(),
                        collected_at: drop.collected_at.clone(),
                        candidate_id: activity.candidate_id,
                        mentions: activity.mentions,
                        mentions_method: activity.mentions_method,
                        sample_size: activity.sample_size,
                        unique_authors: activity.unique_authors,
                        engagement: activity.engagement,
                        sentiment_pos: sentiment.as_ref().map(|s| s.pos),
                        sentiment_neg: sentiment.as_ref().map(|s| s.neg),
                        sentiment_neu: sentiment.as_ref().map(|s| s.neu),
                        sentiment_method: sentiment.map(|s| s.method),
                        bot_share_estimate: activity.bot_share_estimate,
                        // An empty list is stored as an empty cell, like null.
                        top_topics: activity.top_topics.filter(|t| !t.is_empty()).map(|t| t.join(" | ")),
                        rubric_version: stance.and(drop.rubric_version.clone()),
                        stance_read_all: stance.map(|s| s.read_all),
                        stance_posts_drawn: stance.map(|s| s.posts_drawn),
                        stance_off_topic: stance.map(|s| s.off_topic),
                        stance_support: stance.map(|s| s.support),
                        stance_oppose: stance.map(|s| s.oppose),
                        stance_neutral: stance.map(|s| s.neutral),
                        support_share: support.map(|i| i.share),
                        support_low: support.map(|i| i.low),
                        support_high: support.map(|i| i.high),
                        oppose_share: oppose.map(|i| i.share),
                        oppose_low: oppose.map(|i| i.low),
                        oppose_high: oppose.map(|i| i.high),
                        neutral_share: neutral.map(|i| i.share),
                        neutral_low: neutral.map(|i| i.low),
                        neutral_high: neutral.map(|i| i.high),
                        drop_double_labelled: reliability.map(|r| r.posts),
                        drop_agreement: reliability.and_then(|r| r.agreement),
                        drop_kappa: reliability.and_then(|r| r.kappa),
                        source_file: source_file.clone(),
                        content_hash: hash.clone(),
                    });
                }
            }
            Err(errors) => ingest.rejections.push(Rejection { path, errors }),
        }
    }
    ingest
        .rows
        .sort_by(|a, b| (&a.window_end, &a.candidate_id).cmp(&(&b.window_end, &b.candidate_id)));
    Ok(ingest)
}

fn validate_file(path: &Path, bytes: &[u8], schemas: &Schemas, config: &CandidatesConfig) -> Result<Drop, Vec<String>> {
    let value: serde_json::Value = serde_json::from_slice(bytes).map_err(|e| vec![format!("not JSON: {e}")])?;
    let stem = path
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    validate_value(value, Some(&stem), schemas, config)
}

/// `2026-10-09T14:00:00Z` → `2026-10-09T14-00`, the file stem of the drop whose window ends then.
pub fn file_stem(window_end: &str) -> String {
    window_end.get(..16).map(|s| s.replace(':', "-")).unwrap_or_default()
}

/// Checks a drop against the schema, the candidate list and, when given, its file stem. Returns
/// the drop's file stem.
pub fn validate(
    value: serde_json::Value,
    stem: Option<&str>,
    schemas: &Schemas,
    config: &CandidatesConfig,
) -> Result<String, Vec<String>> {
    validate_value(value, stem, schemas, config).map(|drop| file_stem(&drop.window.end))
}

fn validate_value(
    value: serde_json::Value,
    stem: Option<&str>,
    schemas: &Schemas,
    config: &CandidatesConfig,
) -> Result<Drop, Vec<String>> {
    let violations = schemas.violations(SchemaKind::GrokDrop, &value);
    if !violations.is_empty() {
        return Err(violations);
    }
    let drop: Drop = serde_json::from_value(value).map_err(|e| vec![e.to_string()])?;
    let mut errors = Vec::new();
    let expected = file_stem(&drop.window.end);
    if let Some(stem) = stem
        && stem != expected
    {
        errors.push(format!("file name `{stem}` must be the window end `{expected}`"));
    }
    if drop.window.start >= drop.window.end {
        errors.push("window.start must be before window.end".into());
    }
    if drop.collected_at < drop.window.end {
        errors.push("collected_at must not be before window.end".into());
    }
    let mut seen = BTreeSet::new();
    for activity in &drop.candidates {
        if config.candidate(&activity.candidate_id).is_none() {
            errors.push(format!("unknown candidate `{}`", activity.candidate_id));
        }
        if !seen.insert(activity.candidate_id.clone()) {
            errors.push(format!("candidate `{}` appears twice", activity.candidate_id));
        }
        if let Some(s) = &activity.sentiment {
            let total = s.pos + s.neg + s.neu;
            if (total - 1.0).abs() > 0.02 {
                errors.push(format!(
                    "sentiment for `{}` sums to {total:.3}, expected 1",
                    activity.candidate_id
                ));
            }
        }
        if let Some(stance) = activity.stance {
            check_stance(&drop, activity, stance, &mut errors);
        }
    }
    if let Some(double) = &drop.double_labels {
        check_double_labels(&drop, double, &mut errors);
    }
    if errors.is_empty() { Ok(drop) } else { Err(errors) }
}

/// A candidate's stance counts: they add up, agree with the rest of the entry, and come with the
/// rubric and sampling that produced them.
fn check_stance(drop: &Drop, activity: &CandidateActivity, stance: Stance, errors: &mut Vec<String>) {
    let id = &activity.candidate_id;
    let labelled = stance.relevant() + stance.off_topic;
    if labelled != stance.posts_drawn {
        errors.push(format!(
            "stance for `{id}`: support + oppose + neutral + off_topic = {labelled}, but posts_drawn is {}",
            stance.posts_drawn
        ));
    }
    if drop.rubric_version.is_none() || drop.sampling.is_none() {
        errors.push(format!(
            "stance for `{id}` needs rubric_version and sampling on the drop"
        ));
    }
    if activity.sentiment.is_some() {
        errors.push(format!(
            "`{id}` has both stance and sentiment; send sentiment null, the pipeline derives it from stance"
        ));
    }
    if let Some(size) = activity.sample_size
        && size != stance.posts_drawn
    {
        errors.push(format!(
            "`{id}`: sample_size {size} must equal stance.posts_drawn {}",
            stance.posts_drawn
        ));
    }
    if activity.mentions_method.as_deref() == Some("x_counts_endpoint")
        && let Some(mentions) = activity.mentions
        && stance.posts_drawn > mentions
    {
        errors.push(format!(
            "`{id}`: {} posts drawn but only {mentions} mentions in the window",
            stance.posts_drawn
        ));
    }
    if let Some(sampling) = &drop.sampling
        && !stance.read_all
        && stance.posts_drawn > sampling.posts_per_candidate
    {
        errors.push(format!(
            "`{id}`: {} posts drawn, more than sampling.posts_per_candidate {}",
            stance.posts_drawn, sampling.posts_per_candidate
        ));
    }
}

/// The double-label table: its cells add up to `posts`, and its first-pass labels fit within the
/// drop's stance counts, since every post labelled twice is one of the posts drawn.
fn check_double_labels(drop: &Drop, double: &DoubleLabels, errors: &mut Vec<String>) {
    let matrix = double.counts.matrix();
    let cells: u64 = matrix.iter().flatten().sum();
    if cells != double.posts {
        errors.push(format!(
            "double_labels: the counts add up to {cells}, but posts is {}",
            double.posts
        ));
    }
    let mut first_pass = [0u64; 4];
    for stance in drop.candidates.iter().filter_map(|c| c.stance) {
        for (total, n) in first_pass.iter_mut().zip(stance.cells()) {
            *total += n;
        }
    }
    for (k, label) in LABELS.iter().enumerate() {
        let labelled_twice: u64 = matrix[k].iter().sum();
        if labelled_twice > first_pass[k] {
            errors.push(format!(
                "double_labels: {labelled_twice} posts first labelled {label}, more than the {} {label} posts in the drop's stance counts",
                first_pass[k]
            ));
        }
    }
}

/// z for a two-sided 95% interval.
const Z95: f64 = 1.959_963_984_540_054;

/// Shares, intervals and kappa are stored to four decimals, so the clean file is stable.
fn round4(x: f64) -> f64 {
    (x * 10_000.0).round() / 10_000.0
}

/// A share and its 95% Wilson score interval.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Interval {
    pub share: f64,
    pub low: f64,
    pub high: f64,
}

/// The 95% Wilson score interval for `k` successes out of `n`; `None` when `n` is 0.
///
/// centre = (p + z²/2n) / (1 + z²/n), half-width = z / (1 + z²/n) · √(p(1−p)/n + z²/4n²).
pub fn wilson(k: u64, n: u64) -> Option<Interval> {
    if n == 0 {
        return None;
    }
    let (k, n) = (k as f64, n as f64);
    let p = k / n;
    let z2 = Z95 * Z95;
    let denominator = 1.0 + z2 / n;
    let centre = (p + z2 / (2.0 * n)) / denominator;
    let half = Z95 / denominator * (p * (1.0 - p) / n + z2 / (4.0 * n * n)).sqrt();
    Some(Interval {
        share: round4(p),
        low: round4((centre - half).max(0.0)),
        high: round4((centre + half).min(1.0)),
    })
}

/// Support, oppose and neutral shares among the posts about the candidate.
#[derive(Clone, Copy, Debug)]
struct StanceShares {
    support: Interval,
    oppose: Interval,
    neutral: Interval,
}

impl StanceShares {
    fn of(stance: Stance) -> Option<Self> {
        let n = stance.relevant();
        Some(Self {
            support: wilson(stance.support, n)?,
            oppose: wilson(stance.oppose, n)?,
            neutral: wilson(stance.neutral, n)?,
        })
    }
}

/// Agreement between the two labelling passes of a drop.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Reliability {
    /// Posts labelled twice.
    pub posts: u64,
    /// Share of them given the same label twice; `None` when there are none.
    pub agreement: Option<f64>,
    /// Cohen's kappa, (p_o − p_e) / (1 − p_e); `None` when there are no posts or when chance
    /// agreement is already 1 (both passes used one and the same label for every post).
    pub kappa: Option<f64>,
}

impl Reliability {
    /// From `matrix[first][second]`, the counts of posts by first-pass and second-pass label.
    pub fn of(matrix: &[[u64; 4]; 4]) -> Self {
        let posts: u64 = matrix.iter().flatten().sum();
        if posts == 0 {
            return Self {
                posts,
                agreement: None,
                kappa: None,
            };
        }
        let n = posts as f64;
        let observed = (0..4).map(|k| matrix[k][k]).sum::<u64>() as f64 / n;
        let chance: f64 = (0..4)
            .map(|k| {
                let first: u64 = matrix[k].iter().sum();
                let second: u64 = matrix.iter().map(|row| row[k]).sum();
                (first as f64 / n) * (second as f64 / n)
            })
            .sum();
        let kappa = (chance < 1.0).then(|| round4((observed - chance) / (1.0 - chance)));
        Self {
            posts,
            agreement: Some(round4(observed)),
            kappa,
        }
    }
}

/// The bytes of `data/clean/grok.csv` for this ingest.
pub fn clean_bytes(ingest: &GrokIngest) -> Result<Vec<u8>> {
    csv_bytes(
        &ingest.rows,
        "window_start,window_end,collected_at,candidate_id,mentions,mentions_method,sample_size,unique_authors,engagement,sentiment_pos,sentiment_neg,sentiment_neu,sentiment_method,bot_share_estimate,top_topics,\
         rubric_version,stance_read_all,stance_posts_drawn,stance_off_topic,stance_support,stance_oppose,stance_neutral,\
         support_share,support_low,support_high,oppose_share,oppose_low,oppose_high,neutral_share,neutral_low,neutral_high,\
         drop_double_labelled,drop_agreement,drop_kappa,source_file,content_hash",
    )
}

/// Writes `data/clean/grok.csv` and rewrites `data/quarantine/grok/`.
pub fn write_clean(repo: &Repo, ingest: &GrokIngest) -> Result<()> {
    write(&repo.clean_dir().join(CLEAN_FILE), &clean_bytes(ingest)?)?;
    write_quarantine(repo, "grok", &ingest.rejections)
}

/// Reads `data/clean/grok.csv`.
pub fn read_clean_rows(repo: &Repo) -> Result<Vec<GrokRow>> {
    read_csv(&repo.clean_dir().join(CLEAN_FILE))
}
