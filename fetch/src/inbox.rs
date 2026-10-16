//! The agent inbox: a Cloudflare Worker (app/worker) where Grok Bot posts what it collects. The
//! pipeline lists pending items, validates each, and resolves it as accepted, proposed or
//! rejected, with a note the bot can read back.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{Client, Result};

#[derive(Clone, Debug, Deserialize)]
pub struct PendingItem {
    pub id: String,
    pub kind: String,
    pub received_at: String,
    pub payload: Value,
}

#[derive(Deserialize)]
struct Pending {
    items: Vec<PendingItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Resolution {
    pub id: String,
    /// accepted, proposed or rejected
    pub status: String,
    pub note: String,
    #[serde(rename = "ref")]
    pub reference: Option<String>,
}

#[derive(Serialize)]
struct ResolveBody<'a> {
    status: &'a str,
    note: &'a str,
    #[serde(rename = "ref")]
    reference: Option<&'a str>,
}

fn base(url: &str) -> &str {
    url.trim_end_matches('/')
}

pub fn pending(client: &Client, url: &str, token: &str) -> Result<Vec<PendingItem>> {
    let response: Pending = client.get_json(&format!("{}/inbox/pending", base(url)), Some(token))?;
    Ok(response.items)
}

pub fn resolve(client: &Client, url: &str, token: &str, resolution: &Resolution) -> Result<()> {
    let note: String = resolution.note.chars().take(2000).collect();
    let _: Value = client.post_json(
        &format!(
            "{}/inbox/items/{}/resolve",
            base(url),
            crate::encode_segment(&resolution.id)
        ),
        token,
        &ResolveBody {
            status: &resolution.status,
            note: &note,
            reference: resolution.reference.as_deref(),
        },
    )?;
    Ok(())
}
