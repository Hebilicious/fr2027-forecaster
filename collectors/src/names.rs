//! Recognising candidates in text: headlines, market contract labels.

use crate::sources::SourcesConfig;

/// Folds accents, ligatures, typographic apostrophes, dashes and unusual spaces so "Édouard"
/// matches "Edouard" and "d’Arc" matches "d'Arc". Case is kept: "Le Maire" is a name, "le maire"
/// is a mayor.
pub fn fold(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            'à' | 'â' | 'ä' | 'á' | 'ã' | 'å' => out.push('a'),
            'À' | 'Â' | 'Ä' | 'Á' | 'Ã' | 'Å' => out.push('A'),
            'ç' => out.push('c'),
            'Ç' => out.push('C'),
            'é' | 'è' | 'ê' | 'ë' => out.push('e'),
            'É' | 'È' | 'Ê' | 'Ë' => out.push('E'),
            'î' | 'ï' | 'í' | 'ì' => out.push('i'),
            'Î' | 'Ï' | 'Í' | 'Ì' => out.push('I'),
            'ô' | 'ö' | 'ó' | 'ò' | 'õ' => out.push('o'),
            'Ô' | 'Ö' | 'Ó' | 'Ò' | 'Õ' => out.push('O'),
            'ù' | 'û' | 'ü' | 'ú' => out.push('u'),
            'Ù' | 'Û' | 'Ü' | 'Ú' => out.push('U'),
            'ÿ' => out.push('y'),
            'Ÿ' => out.push('Y'),
            'œ' => out.push_str("oe"),
            'Œ' => out.push_str("OE"),
            'æ' => out.push_str("ae"),
            'Æ' => out.push_str("AE"),
            '’' | '‘' | 'ʼ' | '`' => out.push('\''),
            '‐' | '‑' | '‒' | '–' | '—' => out.push('-'),
            '\u{a0}' | '\u{202f}' | '\u{2009}' => out.push(' '),
            c => out.push(c),
        }
    }
    out
}

/// Whether `needle` occurs in `haystack` as whole words: the characters on either side of the
/// match are not letters or digits. Both should already be folded.
fn contains_words(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return false;
    }
    let mut start = 0;
    while let Some(offset) = haystack[start..].find(needle) {
        let at = start + offset;
        let end = at + needle.len();
        let before = haystack[..at].chars().next_back();
        let after = haystack[end..].chars().next();
        if !before.is_some_and(char::is_alphanumeric) && !after.is_some_and(char::is_alphanumeric) {
            return true;
        }
        start = at + haystack[at..].chars().next().map_or(1, char::len_utf8);
    }
    false
}

/// Finds candidates named in free text, using the `names` of each candidate in
/// config/sources.yaml.
pub struct NameMatcher {
    patterns: Vec<(String, Vec<String>)>,
}

impl NameMatcher {
    pub fn new(sources: &SourcesConfig) -> Self {
        let patterns = sources
            .candidates
            .iter()
            .map(|(id, entry)| (id.clone(), entry.names.iter().map(|n| fold(n)).collect()))
            .collect();
        Self { patterns }
    }

    /// The ids of every candidate named in `text`, in config order.
    pub fn find(&self, text: &str) -> Vec<String> {
        let folded = fold(text);
        self.patterns
            .iter()
            .filter(|(_, names)| names.iter().any(|n| contains_words(&folded, n)))
            .map(|(id, _)| id.clone())
            .collect()
    }
}

/// Matches a market contract label ("Marine Le Pen", "Édouard Philippe") to a candidate id: the
/// label must equal, ignoring case and accents, the candidate's name in candidates.yaml, one of
/// their `names`, or one of their `market_names`.
pub struct LabelMatcher {
    labels: Vec<(String, String)>,
}

impl LabelMatcher {
    pub fn new(sources: &SourcesConfig, names: &[(String, String)]) -> Self {
        let key = |s: &str| fold(s).to_lowercase().split_whitespace().collect::<Vec<_>>().join(" ");
        let mut labels = Vec::new();
        for (id, name) in names {
            labels.push((key(name), id.clone()));
        }
        for (id, entry) in &sources.candidates {
            for label in entry.names.iter().chain(&entry.market_names) {
                labels.push((key(label), id.clone()));
            }
        }
        Self { labels }
    }

    pub fn find(&self, label: &str) -> Option<String> {
        let key = fold(label)
            .to_lowercase()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        self.labels.iter().find(|(l, _)| *l == key).map(|(_, id)| id.clone())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::sources::{CandidateSources, SourcesConfig};

    fn sources(entries: &[(&str, &[&str])]) -> SourcesConfig {
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

    #[test]
    fn folds_accents_but_keeps_case() {
        assert_eq!(fold("Édouard Philippe"), "Edouard Philippe");
        assert_eq!(fold("Jean‑Luc Mélenchon"), "Jean-Luc Melenchon");
        assert_eq!(fold("l’Élysée"), "l'Elysee");
    }

    #[test]
    fn finds_whole_words_only() {
        let matcher = NameMatcher::new(&sources(&[
            ("attal", &["Gabriel Attal", "Attal"]),
            ("le_maire", &["Bruno Le Maire"]),
            ("melenchon", &["Mélenchon"]),
            ("dupont_aignan", &["Dupont-Aignan"]),
        ]));
        assert_eq!(
            matcher.find("Gabriel Attal répond à Mélenchon"),
            vec!["attal", "melenchon"]
        );
        assert_eq!(matcher.find("Jacques Attali publie un livre"), Vec::<String>::new());
        assert_eq!(matcher.find("Le maire de Lyon et le Maire"), Vec::<String>::new());
        assert_eq!(matcher.find("Bruno Le Maire, candidat ?"), vec!["le_maire"]);
        assert_eq!(matcher.find("MELENCHON"), Vec::<String>::new());
        assert_eq!(matcher.find("Melenchon, sans accent"), vec!["melenchon"]);
        assert_eq!(matcher.find("Nicolas Dupont-Aignan"), vec!["dupont_aignan"]);
    }

    #[test]
    fn matches_market_labels_exactly() {
        let config = sources(&[("le_pen", &["Marine Le Pen", "Le Pen"])]);
        let matcher = LabelMatcher::new(&config, &[("philippe".into(), "Édouard Philippe".into())]);
        assert_eq!(matcher.find("Edouard  philippe").as_deref(), Some("philippe"));
        assert_eq!(matcher.find("Marine Le Pen").as_deref(), Some("le_pen"));
        assert_eq!(matcher.find("Jean-Marie Le Pen"), None);
        assert_eq!(matcher.find("Christine Lagarde"), None);
    }
}
