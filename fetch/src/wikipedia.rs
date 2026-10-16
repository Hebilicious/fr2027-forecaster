//! Daily user page views of a Wikipedia article, from the Wikimedia pageviews API.
//! `GET https://wikimedia.org/api/rest_v1/metrics/pageviews/per-article/<project>/all-access/user/<article>/daily/<start>00/<end>00`.

use fr2027_collectors::attention::DayViews;
use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, Error, Result};

const API: &str = "https://wikimedia.org/api/rest_v1/metrics/pageviews/per-article";

#[derive(Deserialize)]
struct Response {
    items: Vec<ResponseItem>,
}

#[derive(Deserialize)]
struct ResponseItem {
    timestamp: String,
    views: u64,
}

pub fn parse(body: &[u8]) -> Result<Vec<DayViews>> {
    let response: Response = serde_json::from_slice(body).map_err(|e| Error::parse("pageviews", e))?;
    response
        .items
        .into_iter()
        .map(|item| {
            // 2026100100 → 2026-10-01
            let t = item.timestamp.as_str();
            if t.len() < 8 || !t[..8].bytes().all(|b| b.is_ascii_digit()) {
                return Err(Error::parse("pageviews", format!("unexpected timestamp `{t}`")));
            }
            Ok(DayViews {
                date: format!("{}-{}-{}", &t[..4], &t[4..6], &t[6..8]),
                views: item.views,
            })
        })
        .collect()
}

/// `Ok(None)` when Wikimedia has no data for the article (404): it doesn't exist under that title.
pub fn fetch(client: &Client, project: &str, article: &str, start: Date, end: Date) -> Result<Option<Vec<DayViews>>> {
    let url = format!(
        "{API}/{project}/all-access/user/{}/daily/{}00/{}00",
        crate::encode_segment(article),
        start.strftime("%Y%m%d"),
        end.strftime("%Y%m%d"),
    );
    match client.get_bytes(&url, None) {
        Ok(body) => parse(&body).map(Some),
        Err(error) if error.status() == Some(404) => Ok(None),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parses_daily_items() {
        let body = br#"{"items":[{"project":"fr.wikipedia","article":"Marine_Le_Pen","granularity":"daily","timestamp":"2026100100","access":"all-access","agent":"user","views":5123}]}"#;
        let days = super::parse(body).unwrap();
        assert_eq!(days.len(), 1);
        assert_eq!(days[0].date, "2026-10-01");
        assert_eq!(days[0].views, 5123);
    }
}
