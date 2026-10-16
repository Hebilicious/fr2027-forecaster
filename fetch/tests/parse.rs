//! Parsing saved responses: the market fixtures are trimmed real API answers from 2026-10-09,
//! the feed is synthetic.

use std::collections::BTreeMap;

use fr2027_collectors::names::{LabelMatcher, NameMatcher};
use fr2027_collectors::sources::{CandidateSources, SourcesConfig};
use fr2027_fetch::{kalshi, polymarket, rss};

fn sources() -> SourcesConfig {
    let entries: &[(&str, &[&str])] = &[
        ("le_pen", &["Marine Le Pen", "Le Pen"]),
        ("bardella", &["Jordan Bardella", "Bardella"]),
        ("philippe", &["Édouard Philippe"]),
        ("glucksmann", &["Raphaël Glucksmann", "Glucksmann"]),
        ("faure", &["Olivier Faure"]),
        ("villepin", &["Dominique de Villepin", "Villepin"]),
        ("le_maire", &["Bruno Le Maire"]),
    ];
    SourcesConfig {
        candidates: entries
            .iter()
            .map(|(id, names)| {
                (
                    id.to_string(),
                    CandidateSources {
                        wikipedia: None,
                        names: names.iter().map(|n| n.to_string()).collect(),
                        market_names: Vec::new(),
                    },
                )
            })
            .collect::<BTreeMap<_, _>>(),
        ..SourcesConfig::default()
    }
}

fn labels() -> LabelMatcher {
    LabelMatcher::new(&sources(), &[("philippe".into(), "Édouard Philippe".into())])
}

#[test]
fn polymarket_event() {
    let body = include_bytes!("fixtures/polymarket-event.json");
    let contracts = polymarket::parse(body, &labels()).unwrap();
    let le_pen = contracts.iter().find(|c| c.label == "Marine Le Pen").unwrap();
    assert_eq!(le_pen.candidate_id.as_deref(), Some("le_pen"));
    assert_eq!(le_pen.bid, Some(0.41));
    assert_eq!(le_pen.ask, Some(0.411));
    assert_eq!(le_pen.price, Some(0.4105));
    let philippe = contracts.iter().find(|c| c.label == "Édouard Philippe").unwrap();
    assert_eq!(philippe.candidate_id.as_deref(), Some("philippe"));
    // Unknown to the field, but kept with its label.
    assert!(
        contracts
            .iter()
            .any(|c| c.label == "David Lisnard" && c.candidate_id.is_none())
    );
    // With no bids, the last trade is the price.
    let bertrand = contracts.iter().find(|c| c.label == "Xavier Bertrand").unwrap();
    assert_eq!(
        (bertrand.bid, bertrand.ask, bertrand.price),
        (None, Some(0.001), Some(0.001))
    );
    // Inactive placeholder markets are dropped.
    assert!(!contracts.iter().any(|c| c.label == "Person AA"));
    // Highest price first.
    assert_eq!(contracts[0].label, "Marine Le Pen");
}

#[test]
fn kalshi_markets() {
    let body = include_bytes!("fixtures/kalshi-markets.json");
    let (contracts, cursor) = kalshi::parse(body, &labels()).unwrap();
    assert_eq!(cursor, None);
    let le_pen = contracts
        .iter()
        .find(|c| c.candidate_id.as_deref() == Some("le_pen"))
        .unwrap();
    assert_eq!(le_pen.price, Some(0.465));
    let villepin = contracts
        .iter()
        .find(|c| c.candidate_id.as_deref() == Some("villepin"))
        .unwrap();
    assert_eq!(villepin.price, Some(0.0075));
    assert_eq!(villepin.volume, Some(16182.58));
}

#[test]
fn feed_items_naming_a_candidate() {
    let matcher = NameMatcher::new(&sources());
    let (total, items) = rss::parse(include_bytes!("fixtures/feed.xml"), "example", &matcher).unwrap();
    assert_eq!(total, 4);
    assert_eq!(items.len(), 3, "{items:#?}");
    assert_eq!(items[0].candidate_ids, vec!["bardella", "le_pen"]);
    assert_eq!(
        items[0].url,
        "https://www.example.fr/politique/article/2026/10/09/meeting_123.html"
    );
    assert_eq!(items[0].published_at.as_deref(), Some("2026-10-09T16:12:00Z"));
    // Named only in the summary, which is used for matching but not stored.
    assert_eq!(items[1].candidate_ids, vec!["faure", "glucksmann"]);
    assert_eq!(items[1].title, "Primaire : le dernier débat avant le vote");
    // No date in the feed.
    assert_eq!(items[2].published_at, None);
    assert_eq!(items[2].candidate_ids, vec!["philippe"]);
}
