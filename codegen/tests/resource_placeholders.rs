//! Every run of ○ circles in the playbooks and the gear file is a `Resource`, and the place its
//! value is spoken is marked with `{resource}` in the description: a possession or gizmo has
//! exactly one placeholder if it has a resource and none otherwise. A description with no
//! backing `Resource` would keep its circles, and the set of those leftover runs is pinned by
//! an allowlist, empty since gizmos got their structs; a new circle run goes red here until it
//! gets a resource or is deliberately allowlisted.

use std::collections::BTreeSet;

use codegen::{json5_gear, json5_playbook, playbook_names};

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
    let gear = json5_gear().unwrap_or_else(|e| panic!("{e:#}"));
    for gizmo in gear.gizmos() {
        let count = gizmo.description.matches("{resource}").count();
        match (gizmo.resource.is_some(), count) {
            (true, 1) | (false, 0) => {}
            (has_resource, n) => problems.push(format!(
                "gear {:?}: resource.is_some() == {has_resource} but {n} {{resource}} placeholder(s)",
                gizmo.name
            )),
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
    // (playbook, item) pairs: not-yet-modelled resources, pinned deliberately. Empty now that
    // every circle run in the playbooks belongs to a gizmo or possession with a struct.
    let allowed: BTreeSet<(String, String)> = BTreeSet::new();

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
    let gear = json5_gear().unwrap_or_else(|e| panic!("{e:#}"));
    for gizmo in gear.gizmos() {
        if has_leftover_circle(&gizmo.description) {
            found.insert(("gear".to_string(), gizmo.name.clone()));
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
