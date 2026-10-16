//! Edits to a YAML file's text that leave every untouched line as it was, comments included.
//!
//! The configuration files are lists of blocks under a few top-level keys. A change rewrites
//! whole blocks (one candidate, one slot) or adds new ones; it never re-serialises the file.
//! Callers read the result back and compare it with what they meant to write.

use serde_json::Value;

use crate::schema::yaml_to_json;

/// One entry of a top-level block sequence or mapping: lines `start..end` of the file, without
/// the blank and comment lines that follow it.
#[derive(Clone, Debug)]
pub struct Item {
    pub start: usize,
    pub end: usize,
    pub value: Value,
}

/// The entries under a top-level `key:`, and the indentation they use.
#[derive(Clone, Debug)]
pub struct Section {
    pub indent: usize,
    pub items: Vec<Item>,
}

fn indent_of(line: &str) -> usize {
    line.len() - line.trim_start_matches(' ').len()
}

/// Blank, or only a comment.
fn is_filler(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.is_empty() || trimmed.starts_with('#')
}

/// The block under the top-level `key:`. A sequence's items start with `- `; a mapping's items
/// are its keys. Flow style (`key: [...]`) is refused: it has no blocks to edit.
pub fn section(lines: &[&str], key: &str, sequence: bool) -> Result<Section, String> {
    let header = lines
        .iter()
        .position(|line| {
            line.strip_prefix(key)
                .and_then(|rest| rest.strip_prefix(':'))
                .is_some_and(is_filler)
        })
        .ok_or_else(|| format!("no top-level `{key}:` block"))?;
    let first = (header + 1..lines.len())
        .find(|&i| !is_filler(lines[i]))
        .ok_or_else(|| format!("`{key}:` is empty"))?;
    let indent = indent_of(lines[first]);
    let starts_item = |line: &str| {
        let body = &line[indent..];
        if sequence {
            body == "-" || body.starts_with("- ")
        } else {
            !body.starts_with('-') && !body.starts_with('#')
        }
    };
    if !starts_item(lines[first]) || (!sequence && indent == 0) {
        return Err(format!(
            "`{key}:` is not a block {}",
            if sequence { "list" } else { "mapping" }
        ));
    }
    // The section ends at the first content line less indented than its items, or as indented
    // but not an item (the next top-level key when items sit at column 0).
    let end = (first..lines.len())
        .find(|&i| {
            let line = lines[i];
            !is_filler(line) && (indent_of(line) < indent || (indent_of(line) == indent && !starts_item(line)))
        })
        .unwrap_or(lines.len());
    let starts: Vec<usize> = (first..end)
        .filter(|&i| !is_filler(lines[i]) && indent_of(lines[i]) == indent && starts_item(lines[i]))
        .collect();
    let mut items = Vec::new();
    for (n, &start) in starts.iter().enumerate() {
        let limit = starts.get(n + 1).copied().unwrap_or(end);
        let end = (start..limit)
            .rev()
            .find(|&i| !is_filler(lines[i]))
            .expect("the start line")
            + 1;
        let text: String = lines[start..end]
            .iter()
            .map(|line| format!("{}\n", line.get(indent..).unwrap_or("")))
            .collect();
        let parsed = yaml_to_json(&text).map_err(|e| format!("`{key}:` entry at line {}: {e}", start + 1))?;
        let value = if sequence {
            match parsed {
                Value::Array(mut values) if values.len() == 1 => values.remove(0),
                _ => return Err(format!("`{key}:` entry at line {} is not one list item", start + 1)),
            }
        } else {
            parsed
        };
        items.push(Item { start, end, value });
    }
    Ok(Section { indent, items })
}

/// Replacements of whole line ranges, applied together.
#[derive(Default)]
pub struct Edits {
    edits: Vec<(usize, usize, String)>,
}

impl Edits {
    /// Replaces lines `start..end` with `text` (whole lines, each ending in a newline).
    pub fn replace(&mut self, start: usize, end: usize, text: String) {
        self.edits.push((start, end, text));
    }

    /// Inserts `text` before line `at`.
    pub fn insert(&mut self, at: usize, text: String) {
        self.edits.push((at, at, text));
    }

    pub fn apply(mut self, lines: &[&str]) -> String {
        // Stable sort: insertions at one place keep the order they were made in.
        self.edits.sort_by_key(|&(start, _, _)| start);
        let mut out = String::new();
        let mut next = 0;
        for (start, end, text) in self.edits {
            for line in &lines[next..start.max(next)] {
                out.push_str(line);
                out.push('\n');
            }
            out.push_str(&text);
            next = next.max(end);
        }
        for line in &lines[next..] {
            out.push_str(line);
            out.push('\n');
        }
        out
    }
}

/// The lines of `text`, without their newlines. A final newline adds no empty line.
pub fn lines(text: &str) -> Vec<&str> {
    text.lines().collect()
}

/// A string as a YAML scalar in the files' style: double quotes, or single quotes when the text
/// holds more double quotes than single ones (as the formatter would write it).
pub fn quote(text: &str) -> String {
    if text.matches('"').count() > text.matches('\'').count() {
        format!("'{}'", text.replace('\'', "''"))
    } else {
        serde_json::to_string(text).expect("a string serialises")
    }
}

/// An id as a plain scalar, quoted only when YAML would read it as something else.
pub fn plain_id(id: &str) -> String {
    const RESERVED: [&str; 12] = [
        "null", "true", "false", "yes", "no", "on", "off", "y", "n", "nan", "inf", "~",
    ];
    if RESERVED.contains(&id) {
        quote(id)
    } else {
        id.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXT: &str = "# header comment\nlist:\n  # about a\n  - id: a\n    x: 1\n\n  # about b\n  - { id: b, x: 2 }\nmap:\n  a: { n: 1 }\n  b: 2\n\n# trailing\n";

    #[test]
    fn finds_blocks_and_their_comments_stay_outside() {
        let lines = lines(TEXT);
        let list = section(&lines, "list", true).unwrap();
        assert_eq!(list.indent, 2);
        assert_eq!(
            list.items.iter().map(|i| (i.start, i.end)).collect::<Vec<_>>(),
            [(3, 5), (7, 8)]
        );
        assert_eq!(list.items[1].value, serde_json::json!({"id": "b", "x": 2}));
        let map = section(&lines, "map", false).unwrap();
        assert_eq!(map.items.len(), 2);
        assert_eq!(map.items[1].value, serde_json::json!({"b": 2}));
        assert!(section(&lines, "nope", true).is_err());
    }

    #[test]
    fn edits_keep_everything_else() {
        let lines = lines(TEXT);
        let mut edits = Edits::default();
        edits.replace(3, 5, "  - id: a\n    x: 9\n".into());
        edits.insert(8, "  - id: c\n".into());
        let out = edits.apply(&lines);
        assert_eq!(
            out,
            TEXT.replace("x: 1", "x: 9").replace("x: 2 }\n", "x: 2 }\n  - id: c\n")
        );
    }

    #[test]
    fn quoting() {
        assert_eq!(quote("Le Pen's \\ party"), "\"Le Pen's \\\\ party\"");
        assert_eq!(quote("say \"no\""), "'say \"no\"'");
        assert_eq!(plain_id("no"), "\"no\"");
        assert_eq!(plain_id("le_pen"), "le_pen");
    }
}
