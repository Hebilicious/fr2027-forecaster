//! Where things live in the repository.

use std::path::{Path, PathBuf};

use crate::{Error, Result};

#[derive(Clone, Debug)]
pub struct Repo {
    root: PathBuf,
}

impl Repo {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// The nearest ancestor of `start` holding both `schemas/` and `config/`.
    pub fn discover(start: &Path) -> Result<Self> {
        let mut dir = Some(start);
        while let Some(candidate) = dir {
            if candidate.join("schemas").is_dir() && candidate.join("config").is_dir() {
                return Ok(Self::new(candidate));
            }
            dir = candidate.parent();
        }
        Err(Error::invalid(
            start,
            "not inside the forecaster repository (no schemas/ and config/ above it)",
        ))
    }

    pub fn root(&self) -> &Path {
        &self.root
    }
    pub fn schemas_dir(&self) -> PathBuf {
        self.root.join("schemas")
    }
    pub fn config_dir(&self) -> PathBuf {
        self.root.join("config")
    }
    pub fn raw_polls_dir(&self) -> PathBuf {
        self.root.join("data/raw/polls")
    }
    pub fn raw_grok_dir(&self) -> PathBuf {
        self.root.join("data/raw/grok")
    }
    pub fn raw_markets_dir(&self) -> PathBuf {
        self.root.join("data/raw/markets")
    }
    pub fn raw_news_dir(&self) -> PathBuf {
        self.root.join("data/raw/news")
    }
    pub fn raw_attention_dir(&self) -> PathBuf {
        self.root.join("data/raw/attention")
    }
    pub fn raw_events_dir(&self) -> PathBuf {
        self.root.join("data/raw/events")
    }
    /// Poll files an agent sent, validated, waiting for a pull request.
    pub fn proposals_dir(&self) -> PathBuf {
        self.root.join("data/proposals")
    }
    pub fn clean_dir(&self) -> PathBuf {
        self.root.join("data/clean")
    }
    pub fn quarantine_dir(&self) -> PathBuf {
        self.root.join("data/quarantine")
    }
    pub fn forecasts_dir(&self) -> PathBuf {
        self.root.join("data/forecasts")
    }

    /// `path` relative to the repository root, with forward slashes.
    pub fn relative(&self, path: &Path) -> String {
        path.strip_prefix(&self.root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/")
    }
}
