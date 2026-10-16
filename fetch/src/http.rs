//! A blocking HTTP client with an honest User-Agent, timeouts, and a short, polite retry on
//! rate limits and server errors. Proxies come from the usual environment variables.

use std::time::Duration;

use serde::de::DeserializeOwned;

use crate::{Error, Result};

pub struct Client {
    inner: reqwest::blocking::Client,
}

const RETRY_WAITS: [Duration; 2] = [Duration::from_secs(2), Duration::from_secs(6)];
const MAX_BODY_IN_ERROR: usize = 300;

impl Client {
    pub fn new(user_agent: &str) -> Result<Self> {
        let inner = reqwest::blocking::Client::builder()
            .user_agent(user_agent)
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .map_err(|e| Error::Http {
                url: String::new(),
                message: e.to_string(),
            })?;
        Ok(Self { inner })
    }

    /// GET `url` and return the body, retrying twice on 429 and 5xx.
    pub fn get_bytes(&self, url: &str, bearer: Option<&str>) -> Result<Vec<u8>> {
        self.send(url, |c| {
            let request = c.get(url);
            match bearer {
                Some(token) => request.bearer_auth(token),
                None => request,
            }
        })
    }

    pub fn get_json<T: DeserializeOwned>(&self, url: &str, bearer: Option<&str>) -> Result<T> {
        let bytes = self.get_bytes(url, bearer)?;
        serde_json::from_slice(&bytes).map_err(|e| Error::parse(url, format!("not the expected JSON: {e}")))
    }

    /// POST a JSON body and parse the JSON answer. Not retried: the caller decides.
    pub fn post_json<B: serde::Serialize, T: DeserializeOwned>(&self, url: &str, bearer: &str, body: &B) -> Result<T> {
        let response = self
            .inner
            .post(url)
            .bearer_auth(bearer)
            .json(body)
            .send()
            .map_err(|e| Error::Http {
                url: url.into(),
                message: e.to_string(),
            })?;
        let bytes = checked(url, response)?;
        serde_json::from_slice(&bytes).map_err(|e| Error::parse(url, format!("not the expected JSON: {e}")))
    }

    fn send(
        &self,
        url: &str,
        build: impl Fn(&reqwest::blocking::Client) -> reqwest::blocking::RequestBuilder,
    ) -> Result<Vec<u8>> {
        let mut attempt = 0;
        loop {
            let result = build(&self.inner).send().map_err(|e| Error::Http {
                url: url.into(),
                message: e.to_string(),
            });
            let retry = match &result {
                Ok(response) => response.status().as_u16() == 429 || response.status().is_server_error(),
                Err(_) => true,
            };
            if retry && attempt < RETRY_WAITS.len() {
                std::thread::sleep(RETRY_WAITS[attempt]);
                attempt += 1;
                continue;
            }
            return checked(url, result?);
        }
    }
}

fn checked(url: &str, response: reqwest::blocking::Response) -> Result<Vec<u8>> {
    let status = response.status();
    let bytes = response.bytes().map_err(|e| Error::Http {
        url: url.into(),
        message: e.to_string(),
    })?;
    if status.is_success() {
        return Ok(bytes.to_vec());
    }
    let body = String::from_utf8_lossy(&bytes);
    let body = body.trim();
    Err(Error::Status {
        url: url.into(),
        status: status.as_u16(),
        body: (!body.is_empty()).then(|| body.chars().take(MAX_BODY_IN_ERROR).collect()),
    })
}
