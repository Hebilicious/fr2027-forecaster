//! `config/sources.yaml`: what the live collectors read and how candidates are recognised.

use std::collections::BTreeMap;

use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{Error, Repo, Result, SchemaKind, Schemas, config::CandidatesConfig, read, schema::yaml_to_json};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SourcesConfig {
    pub schema_version: String,
    pub user_agent: String,
    pub markets: Vec<MarketSource>,
    pub feeds: Vec<FeedSource>,
    pub wikipedia: WikipediaSource,
    pub candidates: BTreeMap<String, CandidateSources>,
    pub publication_blackouts: Vec<Blackout>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MarketSource {
    pub id: String,
    pub venue: String,
    pub question: String,
    pub event: String,
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedSource {
    pub id: String,
    pub outlet: String,
    pub url: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct WikipediaSource {
    pub project: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CandidateSources {
    pub wikipedia: Option<String>,
    pub names: Vec<String>,
    #[serde(default)]
    pub market_names: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Blackout {
    pub from: String,
    pub until: String,
}

impl SourcesConfig {
    pub const FILE: &'static str = "sources.yaml";

    /// Loads and checks the file: schema, and exactly one entry per candidate in candidates.yaml.
    pub fn load(repo: &Repo, schemas: &Schemas, candidates: &CandidatesConfig) -> Result<Self> {
        let path = repo.config_dir().join(Self::FILE);
        let text = String::from_utf8(read(&path)?).map_err(|_| Error::invalid(&path, "not UTF-8"))?;
        let value = yaml_to_json(&text).map_err(|e| Error::invalid(&path, e))?;
        let violations = schemas.violations(SchemaKind::Sources, &value);
        if !violations.is_empty() {
            return Err(Error::invalid(&path, violations.join("; ")));
        }
        let config: Self = serde_json::from_value(value).map_err(|e| Error::invalid(&path, e.to_string()))?;
        let mut errors = Vec::new();
        for candidate in &candidates.candidates {
            if !config.candidates.contains_key(&candidate.id) {
                errors.push(format!("no entry for candidate `{}`", candidate.id));
            }
        }
        for id in config.candidates.keys() {
            if candidates.candidate(id).is_none() {
                errors.push(format!("`{id}` is not in config/candidates.yaml"));
            }
        }
        let mut ids: Vec<&str> = config
            .markets
            .iter()
            .map(|m| m.id.as_str())
            .chain(config.feeds.iter().map(|f| f.id.as_str()))
            .collect();
        ids.sort_unstable();
        if let Some(pair) = ids.windows(2).find(|w| w[0] == w[1]) {
            errors.push(format!("id `{}` is used twice", pair[0]));
        }
        for window in &config.publication_blackouts {
            match (window.from.parse::<Timestamp>(), window.until.parse::<Timestamp>()) {
                (Ok(from), Ok(until)) if from < until => {}
                _ => errors.push(format!(
                    "publication blackout {} – {} is not a valid window",
                    window.from, window.until
                )),
            }
        }
        if errors.is_empty() {
            Ok(config)
        } else {
            Err(Error::invalid(&path, errors.join("; ")))
        }
    }

    /// The blackout window `now` falls in, if any.
    pub fn blackout_at(&self, now: Timestamp) -> Option<&Blackout> {
        self.publication_blackouts.iter().find(|w| {
            let from = w.from.parse::<Timestamp>().ok();
            let until = w.until.parse::<Timestamp>().ok();
            matches!((from, until), (Some(f), Some(u)) if f <= now && now < u)
        })
    }

    pub fn feed(&self, id: &str) -> Option<&FeedSource> {
        self.feeds.iter().find(|f| f.id == id)
    }
}
