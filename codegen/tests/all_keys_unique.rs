//! Every key must be unique

use std::collections::HashSet;

use codegen::key::Key;
use codegen::playbook::Playbook;
use codegen::{json5_playbook, playbook_names};

#[derive(Default)]
struct Seen {
    /// Keys seen
    keys: HashSet<String>,
    /// Keys duplicated
    conflicts: HashSet<String>,
}

impl Seen {
    fn record(&mut self, item: &dyn Key) {
        let key = item.key();
        if self.keys.contains(&key) {
            self.conflicts.insert(key);
        } else {
            self.keys.insert(key);
        }
    }

    /// Conflicting names, sorted by type then name, one per line.
    fn conflict_report(&self) -> String {
        let mut conflicts: Vec<_> = self.conflicts.iter().collect();
        conflicts.sort();
        conflicts
            .iter()
            .map(|k| k.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Record every `Key` item reachable from `playbook`.
fn visit_playbook(playbook: &Playbook, seen: &mut Seen) {
    seen.record(playbook);
    for background in &playbook.backgrounds {
        seen.record(background);
    }
    for possession in &playbook.special_possessions.options {
        seen.record(possession);
    }
    for a_move in &playbook.moves {
        seen.record(a_move);
    }
    for backstory in &playbook.backstory {
        seen.record(backstory);
    }
}

#[test]
fn all_keys_unique() {
    let mut seen = Seen::default();
    for name in playbook_names() {
        let playbook = json5_playbook(&name).unwrap_or_else(|e| panic!("{e:#}"));
        visit_playbook(&playbook, &mut seen);
    }
    assert!(
        seen.conflicts.is_empty(),
        "Duplicate keys:\n{}",
        seen.conflict_report()
    );
}
