//! News feeds (RSS or Atom), read as a feed reader would. Keeps the headline, link and
//! publication time of items that name a candidate; matching also looks at the item's summary,
//! which is never stored.

use fr2027_collectors::names::NameMatcher;
use fr2027_collectors::news::Item;

use crate::{Client, Error, Result};

/// Every item in the feed, and the ones that name a candidate.
pub fn parse(body: &[u8], feed_id: &str, matcher: &NameMatcher) -> Result<(u32, Vec<Item>)> {
    let feed = feed_rs::parser::parse(body).map_err(|e| Error::parse(feed_id, e))?;
    let total = u32::try_from(feed.entries.len()).unwrap_or(u32::MAX);
    let mut items = Vec::new();
    for entry in feed.entries {
        let Some(title) = entry
            .title
            .as_ref()
            .map(|t| clean_text(&t.content))
            .filter(|t| !t.is_empty())
        else {
            continue;
        };
        let Some(url) = entry
            .links
            .iter()
            .map(|l| l.href.as_str())
            .find(|h| h.starts_with("https://") || h.starts_with("http://"))
            .or_else(|| Some(entry.id.as_str()).filter(|id| id.starts_with("http")))
            .map(canonical_url)
        else {
            continue;
        };
        let summary = entry
            .summary
            .as_ref()
            .map(|s| clean_text(&s.content))
            .unwrap_or_default();
        let candidate_ids = matcher.find(&format!("{title}\n{summary}"));
        if candidate_ids.is_empty() {
            continue;
        }
        let published_at = entry
            .published
            .or(entry.updated)
            .and_then(|d| jiff::Timestamp::from_second(d.timestamp()).ok())
            .map(crate::timestamp);
        items.push(Item {
            feed: feed_id.to_string(),
            title: title.chars().take(500).collect(),
            url,
            published_at,
            candidate_ids,
        });
    }
    Ok((total, items))
}

pub fn fetch(client: &Client, url: &str, feed_id: &str, matcher: &NameMatcher) -> Result<(u32, Vec<Item>)> {
    parse(&client.get_bytes(url, None)?, feed_id, matcher)
}

/// Strips tags, decodes the entities feeds leave in titles, and collapses whitespace.
fn clean_text(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut in_tag = false;
    for c in text.chars() {
        match c {
            '<' => in_tag = true,
            '>' if in_tag => {
                in_tag = false;
                out.push(' ');
            }
            c if !in_tag => out.push(c),
            _ => {}
        }
    }
    let decoded = out
        .replace("&nbsp;", " ")
        .replace("&rsquo;", "’")
        .replace("&lsquo;", "‘")
        .replace("&laquo;", "«")
        .replace("&raquo;", "»")
        .replace("&quot;", "\"")
        .replace("&#039;", "'")
        .replace("&#39;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&");
    decoded.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Drops tracking parameters and fragments so the same article has one link.
pub fn canonical_url(url: &str) -> String {
    let url = url.trim();
    let without_fragment = url.split('#').next().unwrap_or(url);
    let Some((base, query)) = without_fragment.split_once('?') else {
        return without_fragment.to_string();
    };
    let kept: Vec<&str> = query
        .split('&')
        .filter(|p| {
            let key = p.split('=').next().unwrap_or("");
            !(key.is_empty() || key.starts_with("utm_") || key.starts_with("at_") || key == "xtor" || key == "xtref")
        })
        .collect();
    if kept.is_empty() {
        base.to_string()
    } else {
        format!("{base}?{}", kept.join("&"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cleans_titles() {
        assert_eq!(
            clean_text("  Le <b>RN</b>&nbsp;et  l&rsquo;Europe "),
            "Le RN et l’Europe"
        );
    }

    #[test]
    fn canonical_urls_drop_tracking() {
        assert_eq!(
            canonical_url("https://www.lemonde.fr/a.html?xtor=RSS-3208#x"),
            "https://www.lemonde.fr/a.html"
        );
        assert_eq!(
            canonical_url("https://x.fr/a?id=3&utm_source=rss&utm_medium=feed"),
            "https://x.fr/a?id=3"
        );
        assert_eq!(canonical_url("https://x.fr/a"), "https://x.fr/a");
    }
}
