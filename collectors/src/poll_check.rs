//! Blind double entry of polls. An agent proposes a poll; another routine run enters the same
//! poll from its notice or publication without reading the proposal's numbers. The proposal
//! counts only when both entries agree on everything compared here.

use std::collections::BTreeMap;

use crate::polls::{PollEntry, ScenarioEntry};

/// The fields a check must reproduce exactly. Everything else (urls, notes, sponsor, method,
/// population, published_at) is taken from the proposal.
pub const COMPARED: &str =
    "poll_id, supersedes, field_start, field_end, sample_size, and every scenario's round, candidate set and shares";

/// Refuses a check that does not come from another routine run than the proposal.
pub fn independence(proposal: &PollEntry, check: &PollEntry) -> Result<(), String> {
    let Some(checker) = &check.agent else {
        return Err("a poll_check must carry agent {name, skill_version, run_id}".into());
    };
    let Some(proposer) = &proposal.agent else {
        return Err(format!(
            "the proposal of `{}` names no agent run, so no check can show it comes from another run; resend the poll with agent.run_id",
            proposal.poll_id
        ));
    };
    if checker.run_id == proposer.run_id {
        return Err(format!(
            "the check comes from the same routine run (`{}`) as the proposal; a check must be entered by another run",
            checker.run_id
        ));
    }
    Ok(())
}

/// What differs between a proposal and its check, one line each; empty when they agree.
/// Scenarios are matched by round and candidate set, not by scenario id or order.
///
/// The lines name the fields and the candidates whose shares differ, never a value from either
/// entry: both routines read them back, and each must settle a disagreement by re-reading the
/// notice or publication, not by taking the other's figure.
pub fn differences(proposal: &PollEntry, check: &PollEntry) -> Vec<String> {
    let mut out = Vec::new();
    let mut field = |name: &str, differs: bool| {
        if differs {
            out.push(format!("{name} differs"));
        }
    };
    field("poll_id", proposal.poll_id != check.poll_id);
    field("supersedes", proposal.supersedes != check.supersedes);
    field("field_start", proposal.field_start != check.field_start);
    field("field_end", proposal.field_end != check.field_end);
    field("sample_size", proposal.sample_size != check.sample_size);

    let proposed = by_ballot(&proposal.scenarios);
    let checked = by_ballot(&check.scenarios);
    let mut ballots: Vec<&Ballot> = proposed.keys().chain(checked.keys()).collect();
    ballots.sort();
    ballots.dedup();
    for ballot in ballots {
        let ours = proposed.get(ballot).map_or(&[][..], Vec::as_slice);
        let theirs = checked.get(ballot).map_or(&[][..], Vec::as_slice);
        let round = ballot.0;
        if theirs.is_empty() || ours.is_empty() {
            let (has, lacks) = if theirs.is_empty() {
                ("proposal", "check")
            } else {
                ("check", "proposal")
            };
            for _ in ours.iter().chain(theirs) {
                out.push(format!(
                    "round {round}: a scenario of the {has} tests a set of candidates that no scenario of the {lacks} tests"
                ));
            }
            continue;
        }
        let name = describe(ballot);
        if ours.len() != theirs.len() {
            out.push(format!("{name}: the number of scenarios differs"));
            continue;
        }
        for (p, c) in ours.iter().zip(theirs) {
            let ids: Vec<&str> = p
                .shares
                .iter()
                .filter(|(id, share)| c.shares.get(*id) != Some(share))
                .map(|(id, _)| id.as_str())
                .collect();
            if !ids.is_empty() {
                out.push(format!(
                    "{name}, scenario {} (check: {}): {} differ",
                    p.scenario_id,
                    c.scenario_id,
                    ids.join(", ")
                ));
            }
        }
    }
    out
}

/// A ballot: the round and the sorted candidate ids tested.
type Ballot = (u8, Vec<String>);

/// Scenarios grouped by ballot; within a ballot, sorted by their shares so that two polls
/// listing the same ballots in another order pair up.
fn by_ballot(scenarios: &[ScenarioEntry]) -> BTreeMap<Ballot, Vec<&ScenarioEntry>> {
    let mut map: BTreeMap<Ballot, Vec<&ScenarioEntry>> = BTreeMap::new();
    for s in scenarios {
        map.entry((s.round, s.shares.keys().cloned().collect()))
            .or_default()
            .push(s);
    }
    for group in map.values_mut() {
        group.sort_by(|a, b| {
            let a: Vec<f64> = a.shares.values().copied().collect();
            let b: Vec<f64> = b.shares.values().copied().collect();
            a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal)
        });
    }
    map
}

fn describe((round, ids): &Ballot) -> String {
    format!("round {round} [{}]", ids.join(", "))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(yaml: &str) -> PollEntry {
        let value = crate::schema::yaml_to_json(yaml).unwrap();
        serde_json::from_value(value).unwrap()
    }

    const BASE: &str = r#"
poll_id: ifop-2026-10-14-10314
firm: ifop
sponsor: null
field_start: 2026-10-12
field_end: 2026-10-14
published_at: 2026-10-15
sample_size: 1100
method: online
population: registered
source_url: https://example.org/a.pdf
retrieved_at: 2026-10-15T08:00:00Z
entry: primary
agent: { name: Grok Bot fr2027, skill_version: polls-2, run_id: polls-2026-10-15T07:30Z }
scenarios:
  - { scenario_id: S1, round: 1, shares: { a: 30, b: 40, c: 30 } }
  - { scenario_id: S2, round: 1, shares: { a: 35, b: 65 } }
  - { scenario_id: H1, round: 2, shares: { a: 48, b: 52 } }
"#;

    #[test]
    fn same_numbers_in_another_order_and_ids_agree() {
        let check = BASE
            .replace("run_id: polls-", "run_id: checks-")
            .replace("method: online", "method: mixed")
            .replace(
                "  - { scenario_id: S1, round: 1, shares: { a: 30, b: 40, c: 30 } }\n  - { scenario_id: S2, round: 1, shares: { a: 35, b: 65 } }",
                "  - { scenario_id: X1, round: 1, shares: { b: 65, a: 35 } }\n  - { scenario_id: X2, round: 1, shares: { c: 30, b: 40, a: 30 } }",
            );
        let (p, c) = (entry(BASE), entry(&check));
        assert_eq!(differences(&p, &c), Vec::<String>::new());
        assert!(independence(&p, &c).is_ok());
    }

    #[test]
    fn every_difference_is_named_without_values() {
        let check = BASE
            .replace("sample_size: 1100", "sample_size: 1010")
            .replace("field_start: 2026-10-12", "field_start: 2026-10-11")
            .replace("{ a: 30, b: 40, c: 30 }", "{ a: 31, b: 39, c: 30 }")
            .replace("{ a: 35, b: 65 }", "{ a: 35, c: 65 }")
            .replace("  - { scenario_id: H1, round: 2, shares: { a: 48, b: 52 } }\n", "");
        let diff = differences(&entry(BASE), &entry(&check));
        assert_eq!(
            diff,
            vec![
                "field_start differs",
                "sample_size differs",
                "round 1: a scenario of the proposal tests a set of candidates that no scenario of the check tests",
                "round 1 [a, b, c], scenario S1 (check: S1): a, b differ",
                "round 1: a scenario of the check tests a set of candidates that no scenario of the proposal tests",
                "round 2: a scenario of the proposal tests a set of candidates that no scenario of the check tests",
            ]
        );
        // No share, size or date from either entry.
        let text = diff.join("; ");
        for value in ["1100", "1010", "2026", "30", "31", "39", "40", "35", "65", "48", "52"] {
            assert!(!text.contains(value), "`{value}` leaks into: {text}");
        }
    }

    #[test]
    fn a_check_from_the_same_run_or_without_a_run_is_refused() {
        let p = entry(BASE);
        assert!(independence(&p, &p).unwrap_err().contains("same routine run"));
        let anonymous = entry(&BASE.replace(
            "agent: { name: Grok Bot fr2027, skill_version: polls-2, run_id: polls-2026-10-15T07:30Z }\n",
            "",
        ));
        assert!(independence(&p, &anonymous).unwrap_err().contains("must carry agent"));
        assert!(independence(&anonymous, &p).unwrap_err().contains("names no agent run"));
    }
}
