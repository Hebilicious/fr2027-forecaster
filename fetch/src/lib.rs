//! Everything that reads the network. Each source has a pure `parse` function, tested on saved
//! responses in `tests/`, and a `fetch` function that does the HTTP request and calls it.

pub mod http;
pub mod inbox;
pub mod kalshi;
pub mod polymarket;
pub mod rss;
pub mod wikipedia;

pub use http::Client;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{url}: {message}")]
    Http { url: String, message: String },
    #[error("{url}: HTTP {status}{}", body.as_deref().map(|b| format!(": {b}")).unwrap_or_default())]
    Status {
        url: String,
        status: u16,
        body: Option<String>,
    },
    #[error("{context}: {message}")]
    Parse { context: String, message: String },
}

impl Error {
    pub fn parse(context: impl Into<String>, message: impl std::fmt::Display) -> Self {
        Error::Parse {
            context: context.into(),
            message: message.to_string(),
        }
    }

    /// The HTTP status, when the server answered with an error.
    pub fn status(&self) -> Option<u16> {
        match self {
            Error::Status { status, .. } => Some(*status),
            _ => None,
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// A `jiff` timestamp as the repository writes it: `2026-10-09T20:17:03Z`.
pub fn timestamp(t: jiff::Timestamp) -> String {
    t.strftime("%Y-%m-%dT%H:%M:%SZ").to_string()
}

/// Percent-encodes everything but unreserved characters, for a path segment.
pub fn encode_segment(segment: &str) -> String {
    let mut out = String::with_capacity(segment.len());
    for byte in segment.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => out.push(byte as char),
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    #[test]
    fn encodes_titles() {
        assert_eq!(super::encode_segment("Jean-Luc_Mélenchon"), "Jean-Luc_M%C3%A9lenchon");
        assert_eq!(super::encode_segment("a/b c"), "a%2Fb%20c");
    }
}
