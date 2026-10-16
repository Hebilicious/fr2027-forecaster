//! `fr2027 serve`: the UI and its data on one local port. `/api/<name>.json` is computed from
//! the repository on every request (see `api`), so a new forecast shows up without a restart;
//! everything else is the built UI.

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
use fr2027_collectors::Repo;
use jiff::Timestamp;
use serde_json::json;
use tower_http::services::{ServeDir, ServeFile};

use crate::api;

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

pub fn run(repo: Repo, args: Args) -> Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(serve(repo, args))
}

pub fn router(repo: Repo, web_dir: PathBuf) -> Router {
    let index = web_dir.join("index.html");
    Router::new()
        .route("/api/{*path}", get(document))
        .with_state(Arc::new(repo))
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

fn not_found(message: &str) -> Response {
    (StatusCode::NOT_FOUND, Json(json!({ "error": message }))).into_response()
}

async fn document(State(repo): State<Arc<Repo>>, Path(path): Path<String>) -> Response {
    let Some(name) = path.strip_suffix(".json").filter(|n| api::NAMES.contains(n)) else {
        return not_found("no such document");
    };
    let name = name.to_string();
    let result = tokio::task::spawn_blocking(move || api::document(&repo, &name, Timestamp::now())).await;
    match result {
        Ok(Ok(Some(value))) => Json(value).into_response(),
        Ok(Ok(None)) => not_found("no such document"),
        Ok(Err(error)) => {
            let message = format!("{error:#}");
            let status = if message.contains("no forecast yet") || message.contains("No such file") {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            (status, Json(json!({ "error": message }))).into_response()
        }
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": error.to_string() })),
        )
            .into_response(),
    }
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    use super::*;

    async fn status(path: &str) -> StatusCode {
        let dir = tempfile::tempdir().unwrap();
        let web = dir.path().join("web");
        std::fs::create_dir_all(&web).unwrap();
        std::fs::write(web.join("index.html"), "<!doctype html>").unwrap();
        let app = router(Repo::new(dir.path()), web);
        let response = app
            .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
            .await
            .unwrap();
        response.status()
    }

    #[tokio::test]
    async fn routes_documents_and_the_ui() {
        // Building the router used to panic on overlapping /api routes.
        assert_eq!(status("/api/nothing.json").await, StatusCode::NOT_FOUND);
        assert_eq!(status("/api/a/b/c").await, StatusCode::NOT_FOUND);
        // An empty repository has no forecast yet.
        assert_eq!(status("/api/forecast.json").await, StatusCode::NOT_FOUND);
        assert_eq!(status("/").await, StatusCode::OK);
        assert_eq!(status("/round1").await, StatusCode::OK);
    }
}
