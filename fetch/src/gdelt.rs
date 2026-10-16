//! Daily article counts from the GDELT DOC 2.0 API.
//! `GET https://api.gdeltproject.org/api/v2/doc/doc?query=<q>&mode=timelinevolraw&format=json&startdatetime=<start>000000&enddatetime=<end>235959`.
//!
//! GDELT asks for at most one request every 5 seconds and answers a faster one with a plain-text
//! notice (status 429, sometimes 200); it keeps refusing for several minutes after that, and a
//! quick retry starts the wait again. So [`Reader`] waits between requests, never retries
//! quickly, and backs off for minutes when refused or cut off.

use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use fr2027_collectors::gdelt::DayCount;
use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, Error, Result};

const API: &str = "https://api.gdeltproject.org/api/v2/doc/doc";
/// GDELT's limit is one request every 5 seconds; keep a margin.
const SPACING: Duration = Duration::from_secs(8);
/// The first day the DOC API covers: earlier start dates are refused ("Invalid query start date").
pub const FIRST_DAY: Date = jiff::civil::date(2017, 1, 1);

/// Days per request: about 15 seconds of GDELT's time for 90 days.
const WINDOW_DAYS: i64 = 92;
/// How long one request may take.
pub const TIMEOUT: Duration = Duration::from_secs(120);
/// Waits after a refusal or a dropped connection, before trying the same request again.
const BACK_OFF: [Duration; 4] = [
    Duration::from_secs(60),
    Duration::from_secs(180),
    Duration::from_secs(300),
    Duration::from_secs(600),
];

#[derive(Deserialize)]
struct Response {
    timeline: Vec<Series>,
}

#[derive(Deserialize)]
struct Series {
    data: Vec<Point>,
}

#[derive(Deserialize)]
struct Point {
    /// `20170101T000000Z`
    date: String,
    value: u64,
    norm: u64,
}

/// Parses a `timelinevolraw` answer into one count per day, adding up finer time steps.
pub fn parse(body: &[u8]) -> Result<Vec<DayCount>> {
    if body.first() != Some(&b'{') {
        let text = String::from_utf8_lossy(body);
        return Err(Error::parse("gdelt", format!("not JSON: {}", text.trim())));
    }
    let response: Response = serde_json::from_slice(body).map_err(|e| Error::parse("gdelt", e))?;
    let mut days: BTreeMap<String, (u64, u64)> = BTreeMap::new();
    for point in response.timeline.into_iter().flat_map(|s| s.data) {
        let t = point.date.as_str();
        if t.len() < 8 || !t[..8].bytes().all(|b| b.is_ascii_digit()) {
            return Err(Error::parse("gdelt", format!("unexpected date `{t}`")));
        }
        let day = days
            .entry(format!("{}-{}-{}", &t[..4], &t[4..6], &t[6..8]))
            .or_default();
        day.0 += point.value;
        day.1 += point.norm;
    }
    Ok(days
        .into_iter()
        .map(|(date, (articles, total_articles))| DayCount {
            date,
            articles,
            total_articles,
        })
        .collect())
}

fn is_rate_limited(body: &[u8]) -> bool {
    body.starts_with(b"Please limit requests")
}

/// Reads GDELT politely: one request at most every [`SPACING`].
pub struct Reader<'a> {
    client: &'a Client,
    last_request: Option<Instant>,
}

impl<'a> Reader<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self {
            client,
            last_request: None,
        }
    }

    /// Daily counts of articles matching `query` from `start` to `end`, both included, read in
    /// windows of at most [`WINDOW_DAYS`] days (a longer timeline takes GDELT over a minute).
    /// Days before [`FIRST_DAY`], which GDELT does not cover, are skipped.
    pub fn fetch(&mut self, query: &str, start: Date, end: Date) -> Result<Vec<DayCount>> {
        let mut days = Vec::new();
        let mut from = start.max(FIRST_DAY);
        while from <= end {
            let to = from
                .checked_add(jiff::Span::new().days(WINDOW_DAYS - 1))
                .map_err(|e| Error::parse("gdelt", e))?
                .min(end);
            days.extend(self.fetch_window(query, from, to)?);
            from = to.tomorrow().map_err(|e| Error::parse("gdelt", e))?;
        }
        Ok(days)
    }

    fn fetch_window(&mut self, query: &str, start: Date, end: Date) -> Result<Vec<DayCount>> {
        let url = format!(
            "{API}?query={}&mode=timelinevolraw&format=json&startdatetime={}000000&enddatetime={}235959",
            encode_query(query),
            start.strftime("%Y%m%d"),
            end.strftime("%Y%m%d"),
        );
        let mut last_error = None;
        for wait in BACK_OFF.iter().map(Some).chain([None]) {
            self.pause(SPACING);
            let response = self.client.get_once(&url);
            self.last_request = Some(Instant::now());
            match response {
                Ok(body) if !is_rate_limited(&body) => return parse(&body).map_err(|e| Error::parse(&url, e)),
                Ok(_) => last_error = Some(Error::parse(&url, "GDELT asked to slow down")),
                // Refused, cut off or timed out: GDELT is busy or still holding a refusal.
                Err(error) if error.status().is_none_or(|s| s == 429 || s >= 500) => last_error = Some(error),
                Err(error) => return Err(error),
            }
            if let Some(wait) = wait {
                eprintln!(
                    "GDELT: {}; waiting {}s",
                    last_error.as_ref().expect("set above"),
                    wait.as_secs()
                );
                self.pause(*wait);
            }
        }
        Err(last_error.expect("at least one attempt"))
    }

    fn pause(&self, at_least: Duration) {
        if let Some(last) = self.last_request {
            let elapsed = last.elapsed();
            if elapsed < at_least {
                std::thread::sleep(at_least - elapsed);
            }
        }
    }
}

/// Percent-encodes a query string value.
fn encode_query(query: &str) -> String {
    let mut out = String::with_capacity(query.len() * 2);
    for byte in query.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' | b':' => out.push(byte as char),
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    #[test]
    fn sums_time_steps_into_days() {
        let body =
            br#"{"query_details":{"title":"q","date_resolution":"15m"},"timeline":[{"series":"Article Count","data":[
            {"date":"20261001T000000Z","value":2,"norm":100},
            {"date":"20261001T001500Z","value":3,"norm":120},
            {"date":"20261002T000000Z","value":0,"norm":90}]}]}"#;
        let days = super::parse(body).unwrap();
        assert_eq!(days.len(), 2);
        assert_eq!(
            (days[0].date.as_str(), days[0].articles, days[0].total_articles),
            ("2026-10-01", 5, 220)
        );
        assert_eq!(days[1].articles, 0);
    }

    #[test]
    fn empty_timeline_is_no_days() {
        assert!(super::parse(br#"{"timeline":[]}"#).unwrap().is_empty());
    }

    #[test]
    fn rate_limit_notice_is_not_data() {
        let notice = b"Please limit requests to one every 5 seconds";
        assert!(super::is_rate_limited(notice));
        assert!(super::parse(notice).is_err());
    }

    #[test]
    fn encodes_queries() {
        assert_eq!(
            super::encode_query("(\"Le Pen\" OR \"Mélenchon\") sourcecountry:FR"),
            "%28%22Le%20Pen%22%20OR%20%22M%C3%A9lenchon%22%29%20sourcecountry:FR"
        );
    }
}
