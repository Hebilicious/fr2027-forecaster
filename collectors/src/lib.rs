//! Everything between what sources write into the repository and what the model reads.
//!
//! Raw files under `data/raw/` are never changed. Ingest validates each one against its JSON
//! Schema in `schemas/` and the canonical candidate list, writes the accepted rows to
//! `data/clean/`, and copies each rejected file to `data/quarantine/` beside a note saying
//! why. Clean files are rebuilt from raw on every ingest, sorted, so their history in Git is
//! the history of the data.

pub mod attention;
pub mod config;
pub mod events;
pub mod grok;
pub mod markets;
pub mod names;
pub mod news;
pub mod polls;
pub mod repo;
pub mod schema;
pub mod sources;

use std::path::{Path, PathBuf};

use serde::{Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};

pub use repo::Repo;
pub use schema::{SchemaKind, Schemas};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("{path}: {message}")]
    Invalid { path: PathBuf, message: String },
    #[error("schema `{0}`: {1}")]
    Schema(String, String),
    #[error(transparent)]
    Csv(#[from] csv::Error),
    #[error(transparent)]
    Model(#[from] fr2027_model::ModelError),
}

impl Error {
    pub fn invalid(path: &Path, message: impl Into<String>) -> Self {
        Error::Invalid {
            path: path.to_path_buf(),
            message: message.into(),
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub fn read(path: &Path) -> Result<Vec<u8>> {
    std::fs::read(path).map_err(|source| Error::Io {
        path: path.to_path_buf(),
        source,
    })
}

pub fn write(path: &Path, contents: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|source| Error::Io {
            path: parent.to_path_buf(),
            source,
        })?;
    }
    std::fs::write(path, contents).map_err(|source| Error::Io {
        path: path.to_path_buf(),
        source,
    })
}

/// `sha256:<hex>` of the bytes.
pub fn content_hash(bytes: &[u8]) -> String {
    format!("sha256:{}", hex::encode(Sha256::digest(bytes)))
}

/// Files directly in `dir` with the given extension, sorted by name. A missing directory has none.
pub fn list_files(dir: &Path, extension: &str) -> Result<Vec<PathBuf>> {
    let io = |source| Error::Io {
        path: dir.to_path_buf(),
        source,
    };
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(source) => return Err(io(source)),
    };
    let mut files = Vec::new();
    for entry in entries {
        let path = entry.map_err(io)?.path();
        if path.is_file() && path.extension().is_some_and(|e| e == extension) {
            files.push(path);
        }
    }
    files.sort();
    Ok(files)
}

/// A rejected raw file and the reasons.
#[derive(Clone, Debug, PartialEq)]
pub struct Rejection {
    pub path: PathBuf,
    pub errors: Vec<String>,
}

/// Rewrites `data/quarantine/<area>/` to hold exactly the current rejections: a copy of each
/// rejected file and `<file>.error.txt` beside it.
pub fn write_quarantine(repo: &Repo, area: &str, rejections: &[Rejection]) -> Result<()> {
    let dir = repo.quarantine_dir().join(area);
    if dir.exists() {
        std::fs::remove_dir_all(&dir).map_err(|source| Error::Io {
            path: dir.clone(),
            source,
        })?;
    }
    for rejection in rejections {
        let name = rejection
            .path
            .file_name()
            .expect("raw files have names")
            .to_string_lossy()
            .into_owned();
        write(&dir.join(&name), &read(&rejection.path)?)?;
        let mut note = format!("{} was rejected:\n", repo.relative(&rejection.path));
        for error in &rejection.errors {
            note.push_str("- ");
            note.push_str(error);
            note.push('\n');
        }
        write(&dir.join(format!("{name}.error.txt")), note.as_bytes())?;
    }
    Ok(())
}

/// A JSON raw file that passed its schema and its own checks.
pub struct Accepted<T> {
    pub path: PathBuf,
    pub document: T,
    pub hash: String,
}

/// Reads every `*.json` file in `dir`, in name order, validating each against `kind` and then
/// `check` (which sees the file stem and the parsed document and returns any further errors).
pub fn read_json_documents<T: DeserializeOwned>(
    dir: &Path,
    schemas: &Schemas,
    kind: SchemaKind,
    check: impl Fn(&str, &T) -> Vec<String>,
) -> Result<(Vec<Accepted<T>>, Vec<Rejection>)> {
    let mut accepted = Vec::new();
    let mut rejections = Vec::new();
    for path in list_files(dir, "json")? {
        let bytes = read(&path)?;
        let stem = path
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();
        match validate_json(&bytes, schemas, kind).and_then(|document| {
            let errors = check(&stem, &document);
            if errors.is_empty() { Ok(document) } else { Err(errors) }
        }) {
            Ok(document) => accepted.push(Accepted {
                hash: content_hash(&bytes),
                path,
                document,
            }),
            Err(errors) => rejections.push(Rejection { path, errors }),
        }
    }
    Ok((accepted, rejections))
}

/// Parses `bytes` as JSON and checks it against `kind`.
pub fn validate_json<T: DeserializeOwned>(bytes: &[u8], schemas: &Schemas, kind: SchemaKind) -> Result<T, Vec<String>> {
    let value: serde_json::Value = serde_json::from_slice(bytes).map_err(|e| vec![format!("not JSON: {e}")])?;
    validate_value(value, schemas, kind)
}

/// Checks a JSON value against `kind` and deserialises it.
pub fn validate_value<T: DeserializeOwned>(
    value: serde_json::Value,
    schemas: &Schemas,
    kind: SchemaKind,
) -> Result<T, Vec<String>> {
    let violations = schemas.violations(kind, &value);
    if !violations.is_empty() {
        return Err(violations);
    }
    serde_json::from_value(value).map_err(|e| vec![e.to_string()])
}

/// CSV bytes for `rows`, with `header` alone when there are none.
pub fn csv_bytes<T: Serialize>(rows: &[T], header: &str) -> Result<Vec<u8>> {
    if rows.is_empty() {
        return Ok(format!("{header}\n").into_bytes());
    }
    let mut writer = csv::Writer::from_writer(Vec::new());
    for row in rows {
        writer.serialize(row)?;
    }
    writer.into_inner().map_err(|e| Error::Csv(e.into_error().into()))
}

/// Reads a CSV file written by [`csv_bytes`].
pub fn read_csv<T: DeserializeOwned>(path: &Path) -> Result<Vec<T>> {
    let bytes = read(path)?;
    let mut reader = csv::Reader::from_reader(bytes.as_slice());
    reader
        .deserialize()
        .collect::<Result<Vec<T>, _>>()
        .map_err(|e| Error::invalid(path, e.to_string()))
}

/// Pretty JSON with a trailing newline, as every JSON file in the repository is written.
pub fn pretty_json<T: Serialize>(value: &T) -> Result<Vec<u8>, serde_json::Error> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}

/// Writes a new raw file, refusing to replace an existing one: raw files are never edited.
pub fn write_new(path: &Path, contents: &[u8]) -> Result<bool> {
    if path.exists() {
        return Ok(false);
    }
    write(path, contents)?;
    Ok(true)
}
