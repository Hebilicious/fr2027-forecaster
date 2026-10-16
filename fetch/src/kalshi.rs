//! Kalshi, through its public market-data API: one event, one market per candidate.
//! `GET https://api.elections.kalshi.com/trade-api/v2/markets?event_ticker=<event>`.

use fr2027_collectors::markets::{Contract, implied_price};
use fr2027_collectors::names::LabelMatcher;
use serde::Deserialize;
use serde_json::Value;

use crate::{Client, Error, Result};

const API: &str = "https://api.elections.kalshi.com/trade-api/v2";

#[derive(Deserialize)]
struct Page {
    markets: Vec<Market>,
    #[serde(default)]
    cursor: Option<String>,
}

#[derive(Deserialize)]
struct Market {
    yes_sub_title: Option<String>,
    title: Option<String>,
    status: Option<String>,
    yes_bid_dollars: Option<Value>,
    yes_ask_dollars: Option<Value>,
    last_price_dollars: Option<Value>,
    volume_fp: Option<Value>,
}

fn number(value: Option<&Value>) -> Option<f64> {
    match value? {
        Value::Number(n) => n.as_f64(),
        Value::String(s) => s.trim().parse().ok(),
        _ => None,
    }
}

/// One page of markets: its priced, active contracts and the cursor of the next page.
pub fn parse(body: &[u8], labels: &LabelMatcher) -> Result<(Vec<Contract>, Option<String>)> {
    let page: Page = serde_json::from_slice(body).map_err(|e| Error::parse("kalshi markets", e))?;
    let mut contracts = Vec::new();
    for market in page.markets {
        if market.status.as_deref() != Some("active") {
            continue;
        }
        let Some(label) = market.yes_sub_title.filter(|s| !s.is_empty()).or(market.title) else {
            continue;
        };
        let bid = number(market.yes_bid_dollars.as_ref()).filter(|b| *b > 0.0);
        let ask = number(market.yes_ask_dollars.as_ref()).filter(|a| *a < 1.0);
        let last = number(market.last_price_dollars.as_ref()).filter(|l| *l > 0.0);
        let price = implied_price(bid, ask, last);
        if price.is_none() {
            continue;
        }
        contracts.push(Contract {
            candidate_id: labels.find(&label),
            label,
            price,
            bid,
            ask,
            last,
            volume: number(market.volume_fp.as_ref()),
        });
    }
    Ok((contracts, page.cursor.filter(|c| !c.is_empty())))
}

pub fn fetch(client: &Client, event: &str, labels: &LabelMatcher) -> Result<Vec<Contract>> {
    let mut contracts = Vec::new();
    let mut cursor: Option<String> = None;
    for _ in 0..10 {
        let mut url = format!("{API}/markets?limit=200&event_ticker={}", crate::encode_segment(event));
        if let Some(c) = &cursor {
            url.push_str(&format!("&cursor={}", crate::encode_segment(c)));
        }
        let (page, next) = parse(&client.get_bytes(&url, None)?, labels)?;
        contracts.extend(page);
        match next {
            Some(next) => cursor = Some(next),
            None => break,
        }
    }
    contracts.sort_by(|a, b| {
        b.price
            .unwrap_or(0.0)
            .total_cmp(&a.price.unwrap_or(0.0))
            .then(a.label.cmp(&b.label))
    });
    Ok(contracts)
}
