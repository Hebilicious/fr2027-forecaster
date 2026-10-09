//! Everything between what sources write into the repository and what the model reads.
//!
//! Raw files under `data/raw/` are never changed. Ingest validates each one against its JSON
//! Schema in `schemas/` and the canonical candidate list, writes the accepted rows to
//! `data/clean/`, and copies each rejected file to `data/quarantine/` beside a note saying
//! why. Clean files are rebuilt from raw on every ingest, sorted, so their history in Git is
//! the history of the data.

pub mod config;
pub mod grok;
pub mod polls;
pub mod repo;
pub mod schema;

use std::path::{Path, PathBuf};

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
