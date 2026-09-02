//! If different playbooks have identically-named keys, then those keys have the same value.

use std::collections::HashMap;

use codegen::key::Key;
use codegen::schema::Playbook;
use codegen::{json5_playbook, playbook_names};
use serde::Serialize;

#[derive(Default)]
struct Seen {
    /// Item key to the first playbook that used it and to that item's content.
    items: HashMap<String, (String, String)>,
    /// Playbook keys seen so far.
    playbooks: Vec<String>,
    /// One line per violated invariant.
    problems: Vec<String>,
}

impl Seen {
    fn record(&mut self, playbook: &str, item: &(impl Key + Serialize)) {
        let key = item.key();
        let content = json5::to_string(item).expect("an item can be serialized");
        if let Some((first, first_content)) = self.items.get(&key) {
            if first == playbook {
                self.problems.push(format!("{key}: used twice by {playbook}"));
            } else if *first_content != content {
                self.problems.push(format!(
                    "{key}: {first}'s and {playbook}'s differ — give one a distinct keyPrefix"
                ));
            }
        } else {
            self.items.insert(key, (playbook.to_string(), content));
        }
    }

    fn record_playbook(&mut self, playbook: &Playbook) -> String {
        let key = playbook.key();
        if self.playbooks.contains(&key) {
            self.problems.push(format!("{key}: two playbooks share this key"));
        }
        self.playbooks.push(key.clone());
        key
    }

    /// The violated invariants, sorted, one per line.
    fn report(&self) -> String {
        let mut problems = self.problems.clone();
        problems.sort();
        problems.join("\n")
    }
}

/// Record every `Key` item reachable from `playbook`.
fn visit_playbook(playbook: &Playbook, seen: &mut Seen) {
    let pb_key = seen.record_playbook(playbook);
    for background in &playbook.backgrounds {
        seen.record(&pb_key, background);
        for a_move in background.moves() {
            seen.record(&pb_key, a_move);
        }
    }
    for possession in &playbook.special_possessions.options {
        seen.record(&pb_key, possession);
    }
    for a_move in &playbook.moves {
        seen.record(&pb_key, a_move);
    }
    for backstory in &playbook.backstory {
        seen.record(&pb_key, backstory);
    }
}

#[test]
fn keys_consistent() {
    let mut seen = Seen::default();
    for name in playbook_names() {
        let playbook = json5_playbook(&name).unwrap_or_else(|e| panic!("{e:#}"));
        visit_playbook(&playbook, &mut seen);
    }
    assert!(seen.problems.is_empty(), "Inconsistent keys:\n{}", seen.report());
}
