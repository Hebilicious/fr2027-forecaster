//! `fr2027 serve`: the UI's JSON API, read straight from the repository on every request so a
//! new forecast file shows up without a restart, plus the built UI itself.

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context, Result};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use fr2027_collectors::{Repo, Schemas, config::CandidatesConfig, polls};
use jiff::Timestamp;
use serde_json::{Value, json};
use tower_http::services::{ServeDir, ServeFile};

use crate::forecast::SERIES_FILE;

#[derive(clap::Args)]
pub struct Args {
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
    #[arg(long, default_value_t = 8027)]
    port: u16,
    /// The built UI (default: app/web/dist).
    #[arg(long)]
    web_dir: Option<PathBuf>,
}

struct AppState {
    repo: Repo,
}

type Shared = Arc<AppState>;

pub fn run(repo: Repo, args: Args) -> Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(serve(repo, args))
}

pub fn router(repo: Repo, web_dir: PathBuf) -> Router {
    let state = Arc::new(AppState { repo });
    let index = web_dir.join("index.html");
    Router::new()
        .route("/api/forecasts", get(list_forecasts))
        .route("/api/forecasts/latest", get(latest_forecast))
        .route("/api/forecasts/{name}", get(forecast_by_name))
        .route("/api/history", get(history))
        .route("/api/series", get(series))
        .route("/api/polls", get(poll_rows))
        .route("/api/candidates", get(candidates))
        .route("/api/health", get(health))
        .route(
            "/api/{*rest}",
            get(|| async { ApiError::not_found("no such endpoint") }),
        )
        .with_state(state)
        .fallback_service(ServeDir::new(&web_dir).fallback(ServeFile::new(index)))
}

async fn serve(repo: Repo, args: Args) -> Result<()> {
    let web_dir = args.web_dir.unwrap_or_else(|| repo.root().join("app/web/dist"));
    if !web_dir.join("index.html").exists() {
        eprintln!(
            "warning: {} has no index.html; build the UI with `moon run web:build` (the API still works)",
            web_dir.display()
        );
    }
    let address: SocketAddr = format!("{}:{}", args.host, args.port)
        .parse()
        .with_context(|| format!("invalid address {}:{}", args.host, args.port))?;
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .with_context(|| format!("cannot listen on {address}; choose another port with --port"))?;
    println!("serving http://{}", listener.local_addr()?);
    axum::serve(listener, router(repo, web_dir))
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await?;
    Ok(())
}

struct ApiError(StatusCode, String);

impl ApiError {
    fn not_found(message: impl Into<String>) -> Self {
        Self(StatusCode::NOT_FOUND, message.into())
    }
    fn internal(error: impl std::fmt::Display) -> Self {
        Self(StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.0, Json(json!({ "error": self.1 }))).into_response()
    }
}

type ApiResult = Result<Json<Value>, ApiError>;

fn is_forecast_name(name: &str) -> bool {
    // YYYY-MM-DDTHH.json
    let bytes = name.as_bytes();
    bytes.len() == 18
        && name.ends_with(".json")
        && bytes[..13].iter().enumerate().all(|(i, b)| match i {
            4 | 7 => *b == b'-',
            10 => *b == b'T',
            _ => b.is_ascii_digit(),
        })
}

fn forecast_names(repo: &Repo) -> Result<Vec<String>, ApiError> {
    let mut names: Vec<String> = match std::fs::read_dir(repo.forecasts_dir()) {
        Ok(entries) => entries
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|n| is_forecast_name(n))
            .collect(),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
        Err(error) => return Err(ApiError::internal(error)),
    };
    names.sort();
    Ok(names)
}

fn read_json(path: &std::path::Path) -> Result<Value, ApiError> {
    let bytes = std::fs::read(path).map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => ApiError::not_found(format!("{} does not exist", path.display())),
        _ => ApiError::internal(e),
    })?;
    serde_json::from_slice(&bytes).map_err(ApiError::internal)
}

async fn list_forecasts(State(state): State<Shared>) -> ApiResult {
    let mut names = forecast_names(&state.repo)?;
    names.reverse();
    Ok(Json(json!({ "forecasts": names })))
}

async fn latest_forecast(State(state): State<Shared>) -> ApiResult {
    let names = forecast_names(&state.repo)?;
    let name = names
        .last()
        .ok_or_else(|| ApiError::not_found("no forecast yet: run `moon run cli:forecast`"))?;
    read_json(&state.repo.forecasts_dir().join(name)).map(Json)
}

async fn forecast_by_name(State(state): State<Shared>, Path(name): Path<String>) -> ApiResult {
    if !is_forecast_name(&name) {
        return Err(ApiError::not_found("forecast names look like 2026-10-09T14.json"));
    }
    read_json(&state.repo.forecasts_dir().join(name)).map(Json)
}

/// Every forecast's headline numbers, oldest first.
async fn history(State(state): State<Shared>) -> ApiResult {
    let mut runs = Vec::new();
    for name in forecast_names(&state.repo)? {
        let forecast = read_json(&state.repo.forecasts_dir().join(&name))?;
        let candidates: Vec<Value> = forecast["candidates"]
            .as_array()
            .map(|cs| {
                cs.iter()
                    .map(|c| {
                        json!({
                            "candidate_id": c["candidate_id"],
                            "p_win": c["p_win"],
                            "p_qualify_r1": c["p_qualify_r1"],
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();
        runs.push(json!({
            "file": name,
            "as_of": forecast["as_of"],
            "generated_at": forecast["generated_at"],
            "new_polls": forecast["changes"]["new_polls"],
            "candidates": candidates,
        }));
    }
    Ok(Json(json!({ "runs": runs })))
}

async fn series(State(state): State<Shared>) -> ApiResult {
    read_json(&state.repo.forecasts_dir().join(SERIES_FILE)).map(Json)
}

async fn poll_rows(State(state): State<Shared>) -> ApiResult {
    let rows = polls::read_clean_rows(&state.repo).map_err(ApiError::internal)?;
    Ok(Json(json!({ "rows": rows })))
}

async fn candidates(State(state): State<Shared>) -> ApiResult {
    let schemas = Schemas::load(&state.repo).map_err(ApiError::internal)?;
    let config = CandidatesConfig::load(&state.repo, &schemas).map_err(ApiError::internal)?;
    serde_json::to_value(config).map(Json).map_err(ApiError::internal)
}

/// Freshness of every input and output the UI depends on, with plain-language warnings.
async fn health(State(state): State<Shared>) -> ApiResult {
    let repo = &state.repo;
    let now = Timestamp::now();
    let mut warnings = Vec::new();

    let names = forecast_names(repo)?;
    let latest = match names.last() {
        Some(name) => {
            let forecast = read_json(&repo.forecasts_dir().join(name))?;
            if let Some(generated) = forecast["generated_at"]
                .as_str()
                .and_then(|g| g.parse::<Timestamp>().ok())
            {
                let hours = now.duration_since(generated).as_secs() / 3600;
                if hours > 36 {
                    warnings.push(format!("The latest forecast is {hours} hours old."));
                }
            }
            if let Some(model_warnings) = forecast["warnings"].as_array() {
                warnings.extend(model_warnings.iter().filter_map(|w| w.as_str().map(String::from)));
            }
            json!({
                "file": name,
                "generated_at": forecast["generated_at"],
                "as_of": forecast["as_of"],
                "last_field_end": forecast["aggregation"]["last_field_end"],
                "polls_used": forecast["aggregation"]["polls_used"],
            })
        }
        None => {
            warnings.push("No forecast has been run yet.".into());
            Value::Null
        }
    };

    let mut quarantine = serde_json::Map::new();
    for area in ["polls", "grok"] {
        let files: Vec<String> = std::fs::read_dir(repo.quarantine_dir().join(area))
            .map(|entries| {
                let mut files: Vec<String> = entries
                    .filter_map(|e| e.ok())
                    .map(|e| e.file_name().to_string_lossy().into_owned())
                    .filter(|n| !n.ends_with(".error.txt"))
                    .collect();
                files.sort();
                files
            })
            .unwrap_or_default();
        if !files.is_empty() {
            warnings.push(format!(
                "{} quarantined {area} file(s) in data/quarantine/{area}/.",
                files.len()
            ));
        }
        quarantine.insert(area.into(), json!(files));
    }

    let collectors = json!([
        { "name": "polls (manual entry)", "file": "data/clean/polls.csv", "modified": modified(&repo.clean_dir().join(polls::CLEAN_FILE)) },
        { "name": "grok drops", "file": "data/clean/grok.csv", "modified": modified(&repo.clean_dir().join("grok.csv")) },
    ]);
    Ok(Json(json!({
        "checked_at": now.strftime("%Y-%m-%dT%H:%M:%SZ").to_string(),
        "latest_forecast": latest,
        "forecast_count": names.len(),
        "collectors": collectors,
        "quarantine": quarantine,
        "warnings": warnings,
    })))
}

fn modified(path: &std::path::Path) -> Value {
    std::fs::metadata(path)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| Timestamp::try_from(t).ok())
        .map(|t| json!(t.strftime("%Y-%m-%dT%H:%M:%SZ").to_string()))
        .unwrap_or(Value::Null)
}

#[cfg(test)]
mod tests {
    use super::is_forecast_name;

    #[test]
    fn forecast_names_are_strict() {
        assert!(is_forecast_name("2026-10-09T14.json"));
        assert!(!is_forecast_name("series.json"));
        assert!(!is_forecast_name("../etc/passwd"));
        assert!(!is_forecast_name("2026-10-09T14.json.bak"));
        assert!(!is_forecast_name("2026-10-0xT14.json"));
    }
}
