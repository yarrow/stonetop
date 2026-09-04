//! We're replacing runs of ○ circles with number, but for now only those backed by a `Resource`.
//! The place to put the number is marked with `{resource}` in the description.
//! A description with no backing `Resource` keeps its circles, and the set of those
//! leftover runs is pinned by an allowlist — which will also complain if an allowed circle
//! disappers — so also edit this test when you remove a circle.

use std::collections::BTreeSet;

use codegen::{json5_playbook, playbook_names};

fn display_name(playbook_name: &str) -> String {
    let playbook = json5_playbook(playbook_name).unwrap_or_else(|e| panic!("{e:#}"));
    playbook.name.strip_prefix("The ").unwrap_or(&playbook.name).to_string()
}

#[test]
fn resource_placeholder_matches_struct() {
    let mut problems = Vec::new();
    for name in playbook_names() {
        let playbook = json5_playbook(&name).unwrap_or_else(|e| panic!("{e:#}"));
        for possession in &playbook.special_possessions.options {
            let count = possession.description.matches("{resource}").count();
            match (possession.resource.is_some(), count) {
                (true, 1) | (false, 0) => {}
                (has_resource, n) => problems.push(format!(
                    "{name} {:?}: resource.is_some() == {has_resource} but {n} {{resource}} placeholder(s)",
                    possession.name
                )),
            }
        }
        for a_move in playbook.moves() {
            if a_move.description.contains("{resource}") {
                problems.push(format!(
                    "{name} {:?}: a Move description has a {{resource}} placeholder",
                    a_move.name
                ));
            }
        }
    }
    problems.sort();
    assert!(problems.is_empty(), "{{resource}} placeholder mismatches:\n{}", problems.join("\n"));
}

/// A pick-list item's short name, for the allowlist label: the leading ◇ glyphs and the
/// parenthesised tags are stripped, e.g. "◇ Crossbow (<em>far</em>, ...)" -> "Crossbow".
fn pick_item_label(item: &str) -> String {
    item.trim_start_matches(['◇', ' ']).split('(').next().unwrap_or(item).trim().to_string()
}

fn has_leftover_circle(s: &str) -> bool {
    s.contains('○') || s.contains('◯')
}

#[test]
fn leftover_circles_match_allowlist() {
    // (playbook, item) pairs: not-yet-modelled resources, pinned deliberately (ADR-0010).
    let allowed: BTreeSet<(String, String)> = [
        ("Fox", "Distillery"),
        ("Heavy", "Distillery"),
        ("Lightbearer", "Distillery"),
        ("Marshal", "Distillery"),
        ("Ranger", "Distillery"),
        ("Seeker", "Distillery"),
        ("Fox", "Burglar's kit"),
        ("Lightbearer", "Glassworks"),
        ("Heavy", "Weapons of war Crossbow"),
        ("Marshal", "Weapons of war Composite bow"),
    ]
    .into_iter()
    .map(|(playbook, item)| (playbook.to_string(), item.to_string()))
    .collect();

    let mut found = BTreeSet::new();
    for name in playbook_names() {
        let playbook = json5_playbook(&name).unwrap_or_else(|e| panic!("{e:#}"));
        let display = display_name(&name);
        let mut record = |item: String, text: &str| {
            if has_leftover_circle(text) {
                found.insert((display.clone(), item));
            }
        };
        for a_move in playbook.moves() {
            record(a_move.name.clone(), &a_move.description);
        }
        for possession in &playbook.special_possessions.options {
            record(possession.name.clone(), &possession.description);
            for item in &possession.pick {
                record(format!("{} {}", possession.name, pick_item_label(item)), item);
            }
        }
    }

    let mut problems: Vec<String> = Vec::new();
    for (playbook, item) in allowed.difference(&found) {
        problems.push(format!(
            "{playbook} {item:?}: allowlisted but has no leftover ○/◯ — remove it from the allowlist"
        ));
    }
    for (playbook, item) in found.difference(&allowed) {
        problems.push(format!(
            "{playbook} {item:?}: has a leftover ○/◯ not on the allowlist — add it or give it a resource"
        ));
    }
    problems.sort();
    assert!(
        problems.is_empty(),
        "leftover ○/◯ runs don't match the allowlist:\n{}",
        problems.join("\n")
    );
}
