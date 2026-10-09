//! Every kind of raw data, ingested together: what `validate` checks and `ingest` writes.

use anyhow::Result;
use fr2027_collectors::{
    Rejection, Repo, Schemas, attention, config::CandidatesConfig, events, grok, markets, news, polls,
    sources::SourcesConfig,
};

/// The configuration every command reads.
pub struct Inputs {
    pub schemas: Schemas,
    pub config: CandidatesConfig,
    pub sources: SourcesConfig,
}

impl Inputs {
    pub fn load(repo: &Repo) -> Result<Self> {
        let schemas = Schemas::load(repo)?;
        let config = CandidatesConfig::load(repo, &schemas)?;
        let sources = SourcesConfig::load(repo, &schemas, &config)?;
        Ok(Self {
            schemas,
            config,
            sources,
        })
    }

    /// Candidate ids and names, for matching market labels.
    pub fn names(&self) -> Vec<(String, String)> {
        self.config
            .candidates
            .iter()
            .map(|c| (c.id.clone(), c.name.clone()))
            .collect()
    }
}

pub struct Ingested {
    pub polls: polls::PollIngest,
    pub grok: grok::GrokIngest,
    pub markets: markets::MarketsIngest,
    pub news: news::NewsIngest,
    pub attention: attention::AttentionIngest,
    pub events: events::EventsIngest,
}

impl Ingested {
    pub fn read(repo: &Repo, inputs: &Inputs) -> Result<Self> {
        let Inputs {
            schemas,
            config,
            sources,
        } = inputs;
        Ok(Self {
            polls: polls::ingest(repo, schemas, config)?,
            grok: grok::ingest(repo, schemas, config)?,
            markets: markets::ingest(repo, schemas, config)?,
            news: news::ingest(repo, schemas, config, sources)?,
            attention: attention::ingest(repo, schemas, config)?,
            events: events::ingest(repo, schemas, config)?,
        })
    }

    /// Each clean file and the bytes this ingest gives it.
    pub fn clean_files(&self) -> Result<Vec<(&'static str, Vec<u8>)>> {
        Ok(vec![
            (polls::CLEAN_FILE, polls::clean_bytes(&self.polls)?),
            (grok::CLEAN_FILE, grok::clean_bytes(&self.grok)?),
            (markets::CLEAN_FILE, markets::clean_bytes(&self.markets)?),
            (news::CLEAN_FILE, news::clean_bytes(&self.news)?),
            (attention::CLEAN_FILE, attention::clean_bytes(&self.attention)?),
            (events::CLEAN_FILE, events::clean_bytes(&self.events)?),
        ])
    }

    pub fn write(&self, repo: &Repo) -> Result<()> {
        polls::write_clean(repo, &self.polls)?;
        grok::write_clean(repo, &self.grok)?;
        markets::write_clean(repo, &self.markets)?;
        news::write_clean(repo, &self.news)?;
        attention::write_clean(repo, &self.attention)?;
        events::write_clean(repo, &self.events)?;
        Ok(())
    }

    pub fn rejections(&self) -> impl Iterator<Item = &Rejection> {
        self.polls
            .rejections
            .iter()
            .chain(&self.grok.rejections)
            .chain(&self.markets.rejections)
            .chain(&self.news.rejections)
            .chain(&self.attention.rejections)
            .chain(&self.events.rejections)
    }

    pub fn summary(&self) -> Vec<String> {
        vec![
            format!(
                "polls: {} accepted ({} rows), {} superseded, {} rejected",
                self.polls.accepted.len(),
                self.polls.rows.len(),
                self.polls.superseded.len(),
                self.polls.rejections.len()
            ),
            format!(
                "grok drops: {} accepted ({} rows), {} rejected",
                self.grok.accepted,
                self.grok.rows.len(),
                self.grok.rejections.len()
            ),
            format!(
                "market snapshots: {} accepted ({} rows kept), {} rejected",
                self.markets.accepted,
                self.markets.rows.len(),
                self.markets.rejections.len()
            ),
            format!(
                "news snapshots: {} accepted ({} headlines), {} rejected",
                self.news.accepted,
                self.news.rows.len(),
                self.news.rejections.len()
            ),
            format!(
                "attention fetches: {} accepted ({} rows), {} rejected",
                self.attention.accepted,
                self.attention.rows.len(),
                self.attention.rejections.len()
            ),
            format!(
                "events: {} accepted, {} rejected",
                self.events.rows.len(),
                self.events.rejections.len()
            ),
        ]
    }
}
