//! Every named item (move, background, resource, ...) that appears in more
//! than one playbook must be identical everywhere it appears.

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use codegen::name_key::{NameKey, NameType};
use codegen::playbook::{BackgroundChunk, Playbook};
use codegen::{json5_playbook, playbook_names};

#[derive(Default)]
struct Seen {
    /// The Debug rendering first recorded for each name.
    values: HashMap<NameType, String>,
    /// Names that were later seen with a different rendering.
    conflicts: HashSet<NameType>,
}

impl Seen {
    fn record(&mut self, item: &dyn NameKey) {
        let (key, value) = item.name_type_and_debug();
        match self.values.entry(key) {
            Entry::Vacant(vacant) => {
                vacant.insert(value);
            }
            Entry::Occupied(occupied) => {
                if *occupied.get() != value {
                    self.conflicts.insert(occupied.key().clone());
                }
            }
        }
    }

    /// Conflicting names, sorted by type then name, one per line.
    fn conflict_report(&self) -> String {
        let mut keys: Vec<&NameType> = self.conflicts.iter().collect();
        keys.sort_by_key(|key| (&key.typ, &key.name));
        keys.iter()
            .map(|key| format!("  {}: {}", key.typ, key.name))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Record every `NameKey` item reachable from `playbook`.
fn visit_playbook(playbook: &Playbook, seen: &mut Seen) {
    seen.record(playbook);
    for background in &playbook.backgrounds {
        seen.record(background);
        for chunk in &background.description {
            if let BackgroundChunk::Has(resource) = chunk {
                seen.record(resource);
            }
        }
    }
    for possession in &playbook.special_possessions.options {
        seen.record(possession);
        if let Some(resource) = &possession.resource {
            seen.record(resource);
        }
    }
    for a_move in &playbook.moves {
        seen.record(a_move);
        for resource in &a_move.resource {
            seen.record(resource);
        }
    }
    for backstory in &playbook.backstory {
        seen.record(backstory);
    }
}

#[test]
fn same_name_means_same_value() {
    let mut seen = Seen::default();
    for name in playbook_names() {
        let playbook = json5_playbook(&name).unwrap_or_else(|e| panic!("{e:#}"));
        visit_playbook(&playbook, &mut seen);
    }
    assert!(
        seen.conflicts.is_empty(),
        "items with the same name but different values:\n{}",
        seen.conflict_report()
    );
}
