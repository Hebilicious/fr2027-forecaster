//! The JSON Schemas in `schemas/`, compiled once.

use serde_json::Value;

use crate::{Error, Repo, Result, read};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SchemaKind {
    Candidates,
    Poll,
    GrokDrop,
    Forecast,
    Series,
    Sources,
    Markets,
    News,
    Attention,
    Event,
}

impl SchemaKind {
    pub const ALL: [SchemaKind; 10] = [
        SchemaKind::Candidates,
        SchemaKind::Poll,
        SchemaKind::GrokDrop,
        SchemaKind::Forecast,
        SchemaKind::Series,
        SchemaKind::Sources,
        SchemaKind::Markets,
        SchemaKind::News,
        SchemaKind::Attention,
        SchemaKind::Event,
    ];

    pub fn file_name(self) -> &'static str {
        match self {
            SchemaKind::Candidates => "candidates.schema.json",
            SchemaKind::Poll => "poll.schema.json",
            SchemaKind::GrokDrop => "grok_drop.schema.json",
            SchemaKind::Forecast => "forecast.schema.json",
            SchemaKind::Series => "series.schema.json",
            SchemaKind::Sources => "sources.schema.json",
            SchemaKind::Markets => "markets.schema.json",
            SchemaKind::News => "news.schema.json",
            SchemaKind::Attention => "attention.schema.json",
            SchemaKind::Event => "event.schema.json",
        }
    }
}

pub struct Schemas {
    validators: Vec<(SchemaKind, jsonschema::Validator)>,
}

impl Schemas {
    pub fn load(repo: &Repo) -> Result<Self> {
        let mut validators = Vec::new();
        for kind in SchemaKind::ALL {
            let path = repo.schemas_dir().join(kind.file_name());
            let schema: Value =
                serde_json::from_slice(&read(&path)?).map_err(|e| Error::invalid(&path, format!("not JSON: {e}")))?;
            let validator = jsonschema::validator_for(&schema)
                .map_err(|e| Error::Schema(kind.file_name().into(), e.to_string()))?;
            validators.push((kind, validator));
        }
        Ok(Self { validators })
    }

    /// Every violation, as `<instance path>: <message>`; empty when valid.
    pub fn violations(&self, kind: SchemaKind, instance: &Value) -> Vec<String> {
        let validator = &self
            .validators
            .iter()
            .find(|(k, _)| *k == kind)
            .expect("every schema kind is loaded")
            .1;
        validator
            .iter_errors(instance)
            .map(|error| {
                let path = error.instance_path().to_string();
                if path.is_empty() {
                    error.to_string()
                } else {
                    format!("{path}: {error}")
                }
            })
            .collect()
    }
}

/// Parses YAML into a JSON value, so YAML files validate against the same JSON Schemas.
pub fn yaml_to_json(text: &str) -> Result<Value, String> {
    serde_saphyr::from_str::<Value>(text).map_err(|e| e.to_string())
}
