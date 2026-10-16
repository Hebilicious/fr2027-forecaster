//! `config/candidates.yaml` (the canonical candidate list and field model) and
//! `config/model.yaml` (model parameters).

use fr2027_model::{Candidate, Field, ModelParams, Slot, SlotOption};
use jiff::civil::Date;
use serde::{Deserialize, Serialize};

use crate::{Error, Repo, Result, SchemaKind, Schemas, read, schema::yaml_to_json};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CandidatesConfig {
    pub schema_version: String,
    pub as_of: Date,
    pub blocs: Vec<String>,
    pub candidates: Vec<CandidateEntry>,
    pub slots: Vec<SlotEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CandidateEntry {
    pub id: String,
    pub name: String,
    pub party: String,
    pub bloc: String,
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub declared_on: Option<Date>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub sources: Vec<SourceRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceRef {
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<Date>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SlotEntry {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rationale: Option<String>,
    pub options: Vec<SlotOption>,
}

impl CandidatesConfig {
    pub fn load(repo: &Repo, schemas: &Schemas) -> Result<Self> {
        let path = repo.config_dir().join("candidates.yaml");
        let text = String::from_utf8(read(&path)?).map_err(|_| Error::invalid(&path, "not UTF-8"))?;
        let value = yaml_to_json(&text).map_err(|e| Error::invalid(&path, e))?;
        let violations = schemas.violations(SchemaKind::Candidates, &value);
        if !violations.is_empty() {
            return Err(Error::invalid(&path, violations.join("; ")));
        }
        let config: Self = serde_json::from_value(value).map_err(|e| Error::invalid(&path, e.to_string()))?;
        config.field().map_err(|e| Error::invalid(&path, e.to_string()))?;
        Ok(config)
    }

    /// The model's view of the field. Fails when a candidate's bloc is unknown, a slot names an
    /// unknown candidate, a candidate sits in no slot or in two, or a slot's options sum past 1.
    pub fn field(&self) -> Result<Field, fr2027_model::ModelError> {
        let candidates = self
            .candidates
            .iter()
            .map(|c| Candidate {
                id: c.id.clone(),
                name: c.name.clone(),
                bloc: c.bloc.clone(),
            })
            .collect();
        let slots: Vec<Slot> = self
            .slots
            .iter()
            .map(|s| Slot {
                id: s.id.clone(),
                options: s.options.clone(),
            })
            .collect();
        Field::new(self.blocs.clone(), candidates, &slots)
    }

    pub fn candidate(&self, id: &str) -> Option<&CandidateEntry> {
        self.candidates.iter().find(|c| c.id == id)
    }
}

pub fn load_model_params(repo: &Repo) -> Result<ModelParams> {
    let path = repo.config_dir().join("model.yaml");
    let text = String::from_utf8(read(&path)?).map_err(|_| Error::invalid(&path, "not UTF-8"))?;
    serde_saphyr::from_str(&text).map_err(|e| Error::invalid(&path, e.to_string()))
}
