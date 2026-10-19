//! Jev, TypeSafe's classification model, reading how a news article portrays each candidate it
//! names: `POST https://api.typesafe.ai/v1/systemone` with the article as the state and one Choice
//! question per candidate (favourable, unfavourable, neutral, or not about them), answered with a
//! probability for each option. The key is `TYPESAFE_API_KEY`; Jev bills input tokens only.

use std::collections::BTreeMap;

use fr2027_collectors::gdelt::Stance;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::{Client, Error, Result};

const ENDPOINT: &str = "https://api.typesafe.ai/v1/systemone";
/// Pinned, so a classification can be reproduced.
pub const MODEL: &str = "jev-1.14.0";
pub const KEY_VARIABLE: &str = "TYPESAFE_API_KEY";
const ATTEMPTS: u32 = 4;

pub struct Jev {
    client: Client,
    key: String,
}

#[derive(Deserialize)]
struct Response {
    answers: BTreeMap<String, Answer>,
}

#[derive(Deserialize)]
struct Answer {
    probabilities: BTreeMap<String, f64>,
}

impl Jev {
    /// Jev with the key in the environment; `None` without one.
    pub fn from_env(user_agent: &str) -> Result<Option<Self>> {
        let Some(key) = std::env::var(KEY_VARIABLE).ok().filter(|k| !k.trim().is_empty()) else {
            return Ok(None);
        };
        Ok(Some(Jev {
            client: Client::with_timeout(user_agent, std::time::Duration::from_secs(60))?,
            key: key.trim().to_string(),
        }))
    }

    /// How the article portrays each of `candidates` (id, full name), by id.
    pub fn stance(
        &self,
        title: Option<&str>,
        body: &str,
        candidates: &[(String, String)],
    ) -> Result<BTreeMap<String, Stance>> {
        let request = request(title, body, candidates);
        let mut attempt = 0;
        let response: Response = loop {
            attempt += 1;
            match self.client.post_json(ENDPOINT, &self.key, &request) {
                Ok(response) => break response,
                // Rate limits and server errors are worth another try; a bad request is not.
                Err(error) if attempt < ATTEMPTS && error.status().is_none_or(|s| s == 429 || s >= 500) => {
                    std::thread::sleep(std::time::Duration::from_secs(2u64.pow(attempt)));
                }
                Err(error) => return Err(error),
            }
        };
        candidates
            .iter()
            .enumerate()
            .map(|(i, (id, _))| {
                let answer = response
                    .answers
                    .get(&question_id(i))
                    .ok_or_else(|| Error::parse("jev", format!("no answer for {id}")))?;
                let p = |option: &str| answer.probabilities.get(option).copied().unwrap_or(0.0);
                Ok((
                    id.clone(),
                    Stance {
                        favourable: round4(p("favourable")),
                        unfavourable: round4(p("unfavourable")),
                        neutral: round4(p("neutral")),
                        not_about: round4(p("not_about")),
                    },
                ))
            })
            .collect()
    }
}

fn question_id(i: usize) -> String {
    format!("c{i}")
}

fn round4(x: f64) -> f64 {
    (x * 10_000.0).round() / 10_000.0
}

/// The request: the article, then one question per candidate. The instructions ask for the
/// article's own reporting, not the views of the people it quotes.
pub fn request(title: Option<&str>, body: &str, candidates: &[(String, String)]) -> Value {
    let state = match title {
        Some(title) => format!("Title: {title}\n\n{body}"),
        None => body.to_string(),
    };
    let questions: serde_json::Map<String, Value> = candidates
        .iter()
        .enumerate()
        .map(|(i, (_, name))| {
            (
                question_id(i),
                json!({
                    "type": "choice",
                    "instructions": format!(
                        "How does this news article portray {name}? Judge the article's own reporting and framing of {name}, not the opinions of people it quotes, and not other politicians."
                    ),
                    "criteria": {
                        "favourable": format!("Presents {name} or their actions, campaign or prospects in a positive light"),
                        "unfavourable": format!("Presents {name} critically: scandal, failure, attack, falling support, controversy"),
                        "neutral": format!("Mentions {name} factually without a positive or negative slant"),
                        "not_about": format!("{name} is only named in passing or not at all"),
                    },
                }),
            )
        })
        .collect();
    json!({ "model": MODEL, "state": state, "questions": questions })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asks_one_choice_per_candidate() {
        let candidates = [
            ("glucksmann".to_string(), "Raphaël Glucksmann".to_string()),
            ("melenchon".to_string(), "Jean-Luc Mélenchon".to_string()),
        ];
        let request = request(Some("Glucksmann grimpe"), "Texte.", &candidates);
        assert_eq!(request["model"], MODEL);
        assert_eq!(request["state"], "Title: Glucksmann grimpe\n\nTexte.");
        assert_eq!(request["questions"].as_object().unwrap().len(), 2);
        let c1 = &request["questions"]["c1"];
        assert_eq!(c1["type"], "choice");
        assert!(c1["instructions"].as_str().unwrap().contains("Jean-Luc Mélenchon"));
        let options: Vec<&String> = c1["criteria"].as_object().unwrap().keys().collect();
        assert_eq!(options, ["favourable", "neutral", "not_about", "unfavourable"]);
    }
}
