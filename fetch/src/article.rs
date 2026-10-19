//! The text of a news article, so Jev can read how it portrays a candidate: the page at its
//! address, or the Internet Archive's copy closest to the day GDELT listed it when the page is gone
//! or refuses the collector. The collector asks under the project's own User-Agent, as every other
//! reader here does.
//!
//! The text is the article's paragraphs: those inside `<article>` when it holds enough of them,
//! else every paragraph outside navigation, headers, footers and asides, each of at least
//! [`MIN_PARAGRAPH_WORDS`] words. A page whose paragraphs hold fewer than [`MIN_WORDS`] words
//! (a home page after a redirect, a paywall, a cookie wall), or whose text never names the
//! candidate its headline matched, is not the article.

use std::sync::Mutex;
use std::time::{Duration, Instant};

use encoding_rs::{Encoding, UTF_8, WINDOWS_1252};
use fr2027_collectors::gdelt::Reached;
use jiff::civil::Date;
use scraper::{ElementRef, Html, Selector};

use crate::Client;

/// Fewest words in a page's paragraphs for it to be the article.
pub const MIN_WORDS: usize = 80;
/// Shorter paragraphs are captions, bylines or buttons.
pub const MIN_PARAGRAPH_WORDS: usize = 7;
/// How long one page may take.
pub const TIMEOUT: std::time::Duration = std::time::Duration::from_secs(20);

const ARCHIVE_INDEX: &str = "https://web.archive.org/cdx/search/cdx";
const ARCHIVE: &str = "https://web.archive.org/web";
/// The Internet Archive asks clients to pace themselves: one request at a time per process, at least
/// this far apart.
const ARCHIVE_SPACING: Duration = Duration::from_millis(1200);
/// After a refusal (429, 5xx) or a timeout, every reader waits this long before the next try.
const ARCHIVE_WAITS: [Duration; 4] = [
    Duration::from_secs(10),
    Duration::from_secs(30),
    Duration::from_secs(60),
    Duration::from_secs(120),
];
/// When the next request to the Archive may go.
static ARCHIVE_TURN: Mutex<Option<Instant>> = Mutex::new(None);

/// An article's text and where it was read.
#[derive(Clone, Debug, PartialEq)]
pub struct Text {
    pub reached: Reached,
    pub title: Option<String>,
    pub body: String,
}

impl Text {
    fn unreachable() -> Self {
        Text {
            reached: Reached::Unreachable,
            title: None,
            body: String::new(),
        }
    }
}

/// Reads the article at `url`, listed by GDELT on `day`. `is_article` checks the text is the one
/// the headline promised (it names the candidate).
pub fn read(client: &Client, url: &str, day: Date, is_article: &dyn Fn(&str) -> bool) -> Text {
    let read_page = |bytes: Vec<u8>, reached: Reached| {
        let (title, body) = extract(&decode(&bytes));
        let text = format!("{}\n{body}", title.as_deref().unwrap_or_default());
        (body.split_whitespace().count() >= MIN_WORDS && is_article(&text)).then_some(Text { reached, title, body })
    };
    // A page that refuses or is gone is not retried: the archive is asked instead.
    if let Some(text) = client.get_once(url).ok().and_then(|b| read_page(b, Reached::Live)) {
        return text;
    }
    archived(client, url, day)
        .and_then(|b| read_page(b, Reached::Archive))
        .unwrap_or_else(Text::unreachable)
}

/// The Internet Archive's copy of `url` closest to `day`, as the page was served (`id_`).
fn archived(client: &Client, url: &str, day: Date) -> Option<Vec<u8>> {
    let query = format!(
        "{ARCHIVE_INDEX}?url={}&output=json&fl=timestamp,original&filter=statuscode:200&closest={}&sort=closest&limit=1",
        encode(url),
        day.strftime("%Y%m%d")
    );
    let index: Vec<Vec<String>> = serde_json::from_slice(&archive_get(client, &query)?).ok()?;
    // The first row is the header.
    let row = index.get(1)?;
    let (timestamp, original) = (row.first()?, row.get(1)?);
    archive_get(client, &format!("{ARCHIVE}/{timestamp}id_/{original}"))
}

/// GET from the Internet Archive in turn with every other reader of this process, waiting and
/// asking again after a refusal or a timeout; `None` once the waits are spent, or for any other
/// answer (a 404 is no copy).
fn archive_get(client: &Client, url: &str) -> Option<Vec<u8>> {
    for attempt in 0..=ARCHIVE_WAITS.len() {
        {
            let mut turn = ARCHIVE_TURN.lock().expect("archive turn");
            if let Some(next) = *turn {
                std::thread::sleep(next.saturating_duration_since(Instant::now()));
            }
            *turn = Some(Instant::now() + ARCHIVE_SPACING);
        }
        match client.get_once(url) {
            Ok(bytes) => return Some(bytes),
            Err(error) if error.status().is_none_or(|s| s == 429 || s >= 500) => {
                let wait = ARCHIVE_WAITS.get(attempt)?;
                let mut turn = ARCHIVE_TURN.lock().expect("archive turn");
                let after = Instant::now() + *wait;
                *turn = Some(turn.map_or(after, |next| next.max(after)));
            }
            Err(_) => return None,
        }
    }
    None
}

fn encode(url: &str) -> String {
    url.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => (b as char).to_string(),
            _ => format!("%{b:02X}"),
        })
        .collect()
}

/// The page as text: its declared charset, else UTF-8 when it decodes cleanly, else Windows-1252,
/// which older French pages use.
fn decode(bytes: &[u8]) -> String {
    let head = String::from_utf8_lossy(&bytes[..bytes.len().min(4096)]).to_lowercase();
    let declared = head
        .split("charset=")
        .nth(1)
        .map(|rest| rest.trim_start_matches(['"', '\'']))
        .map(|rest| {
            rest.split(|c: char| !(c.is_alphanumeric() || c == '-' || c == '_'))
                .next()
                .unwrap_or_default()
        })
        .and_then(|label| Encoding::for_label(label.as_bytes()));
    let encoding = declared.unwrap_or_else(|| {
        if std::str::from_utf8(bytes).is_ok() {
            UTF_8
        } else {
            WINDOWS_1252
        }
    });
    encoding.decode(bytes).0.into_owned()
}

/// The page's title and all its article paragraphs, joined by newlines: the classifier reads the
/// whole article, since the part of a long piece left out could carry its framing.
pub fn extract(html: &str) -> (Option<String>, String) {
    let page = Html::parse_document(html);
    let select = |css: &str| Selector::parse(css).expect("a valid selector");
    let words = |e: &ElementRef| {
        e.text()
            .collect::<Vec<_>>()
            .join(" ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    };
    let og_title = page
        .select(&select(r#"meta[property="og:title"]"#))
        .next()
        .and_then(|m| m.value().attr("content"))
        .map(str::to_string);
    let title = og_title
        .or_else(|| page.select(&select("title")).next().map(|t| words(&t)))
        .filter(|t| !t.is_empty());

    let skipped = ["nav", "header", "footer", "aside", "figcaption", "form"];
    let outside_chrome = |p: &ElementRef| {
        !p.ancestors()
            .filter_map(ElementRef::wrap)
            .any(|a| skipped.contains(&a.value().name()))
    };
    let paragraphs = |root: Option<ElementRef>| -> Vec<String> {
        let p = select("p");
        let found: Vec<ElementRef> = match root {
            Some(root) => root.select(&p).collect(),
            None => page.select(&p).collect(),
        };
        found
            .into_iter()
            .filter(outside_chrome)
            .map(|e| words(&e))
            .filter(|t| t.split_whitespace().count() >= MIN_PARAGRAPH_WORDS)
            .collect()
    };
    let in_article = page
        .select(&select("article"))
        .map(|a| paragraphs(Some(a)))
        .max_by_key(|ps| ps.iter().map(|p| p.len()).sum::<usize>())
        .unwrap_or_default();
    let enough = |ps: &[String]| ps.iter().map(|p| p.split_whitespace().count()).sum::<usize>() >= MIN_WORDS;
    let chosen = if enough(&in_article) {
        in_article
    } else {
        paragraphs(None)
    };

    (title, chosen.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAGE: &str = r#"<html><head><meta charset="utf-8"><title>Site</title>
        <meta property="og:title" content="Glucksmann grimpe dans les sondages"></head>
        <body><nav><p>Accueil Politique Monde Économie Culture Sport Société Planète</p></nav>
        <article><p>Après sa victoire à la primaire, Raphaël Glucksmann gagne quatre points dans le dernier sondage.</p>
        <p>Photo AFP</p>
        <p>Le député européen devance désormais Jean-Luc Mélenchon chez les électeurs de gauche, selon l'institut.</p></article>
        <footer><p>Mentions légales, conditions générales et politique de confidentialité du site.</p></footer></body></html>"#;

    #[test]
    fn keeps_the_article_paragraphs_and_its_title() {
        let (title, body) = extract(PAGE);
        assert_eq!(title.as_deref(), Some("Glucksmann grimpe dans les sondages"));
        assert_eq!(
            body,
            "Après sa victoire à la primaire, Raphaël Glucksmann gagne quatre points dans le dernier sondage.\n\
             Le député européen devance désormais Jean-Luc Mélenchon chez les électeurs de gauche, selon l'institut."
        );
    }

    #[test]
    fn reads_older_pages_in_their_charset() {
        let latin1 = b"<html><head><meta http-equiv=\"Content-Type\" content=\"text/html; charset=iso-8859-1\"></head><p>\xc9douard Philippe \xe0 Matignon</p></html>";
        assert!(decode(latin1).contains("Édouard Philippe à Matignon"));
        let undeclared = b"<p>Fran\xe7ois Fillon</p>";
        assert!(decode(undeclared).contains("François Fillon"));
        assert!(decode("<p>Mélenchon</p>".as_bytes()).contains("Mélenchon"));
    }

    #[test]
    fn encodes_an_address_for_the_archive_index() {
        assert_eq!(encode("http://a.fr/x?y=1&z"), "http%3A%2F%2Fa.fr%2Fx%3Fy%3D1%26z");
    }
}

#[cfg(test)]
mod network {
    use super::*;

    /// A 2017 article gone from its site (404), read from the Internet Archive.
    /// `cargo test -p fr2027-fetch -- --ignored archive`
    #[test]
    #[ignore = "reads the network"]
    fn reads_a_gone_article_from_the_archive() {
        let client = Client::with_timeout(
            "fr2027-forecaster/0.2 (+https://github.com/Hebilicious/fr2027-forecaster)",
            TIMEOUT,
        )
        .unwrap();
        let url = "http://www.lcp.fr/afp/francois-hollande-assure-laisser-un-pays-en-bien-meilleur-etat";
        let text = read(&client, url, jiff::civil::date(2017, 4, 20), &|t| {
            t.contains("Hollande")
        });
        assert_eq!(text.reached, Reached::Archive, "{text:?}");
        assert!(text.body.split_whitespace().count() >= MIN_WORDS);
    }
}
