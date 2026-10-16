//! Polymarket, through its public Gamma API: one event, one Yes/No market per candidate.
//! `GET https://gamma-api.polymarket.com/events?slug=<event>`.

use fr2027_collectors::markets::{Contract, implied_price};
use fr2027_collectors::names::LabelMatcher;
use serde::Deserialize;
use serde_json::Value;

use crate::{Client, Error, Result};

const GAMMA: &str = "https://gamma-api.polymarket.com";

#[derive(Deserialize)]
struct Event {
    markets: Vec<Market>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Market {
    group_item_title: Option<String>,
    question: Option<String>,
    #[serde(default)]
    active: bool,
    #[serde(default)]
    closed: bool,
    best_bid: Option<Value>,
    best_ask: Option<Value>,
    last_trade_price: Option<Value>,
    volume_num: Option<Value>,
}

/// A number the API may send as a number or a string.
fn number(value: Option<&Value>) -> Option<f64> {
    match value? {
        Value::Number(n) => n.as_f64(),
        Value::String(s) => s.trim().parse().ok(),
        _ => None,
    }
}

/// The contracts of the event's open markets that have a price, matched to candidates.
pub fn parse(body: &[u8], labels: &LabelMatcher) -> Result<Vec<Contract>> {
    let events: Vec<Event> = serde_json::from_slice(body).map_err(|e| Error::parse("polymarket event", e))?;
    let event = events
        .into_iter()
        .next()
        .ok_or_else(|| Error::parse("polymarket event", "no event with this slug"))?;
    let mut contracts = Vec::new();
    for market in event.markets {
        if !market.active || market.closed {
            continue;
        }
        let Some(label) = market.group_item_title.or(market.question) else {
            continue;
        };
        // An empty book shows as bid 0 / ask 1.
        let bid = number(market.best_bid.as_ref()).filter(|b| *b > 0.0);
        let ask = number(market.best_ask.as_ref()).filter(|a| *a < 1.0);
        let last = number(market.last_trade_price.as_ref()).filter(|l| *l > 0.0);
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
            volume: number(market.volume_num.as_ref()),
        });
    }
    contracts.sort_by(|a, b| {
        b.price
            .unwrap_or(0.0)
            .total_cmp(&a.price.unwrap_or(0.0))
            .then(a.label.cmp(&b.label))
    });
    Ok(contracts)
}

pub fn fetch(client: &Client, event: &str, labels: &LabelMatcher) -> Result<Vec<Contract>> {
    let url = format!("{GAMMA}/events?slug={}", crate::encode_segment(event));
    parse(&client.get_bytes(&url, None)?, labels)
}
