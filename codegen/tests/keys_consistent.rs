//! The json5 files and `stonetop/src/keys.rs` agree: every name resolves to an enum variant (else
//! the `Key` impl panics); every variant of every `XKey` enum is produced by some name; no
//! playbook uses a key twice; and if different playbooks have identically-named keys, then those
//! keys have the same value.

use std::any::type_name;
use std::collections::BTreeMap;
use std::fmt::{Debug, Display};

use codegen::key::Key;
use codegen::schema::Playbook;
use codegen::{json5_playbook, playbook_names};
use serde::Serialize;
use stonetop::keys::{BackgroundKey, BackstoryKey, MoveKey, PlaybookKey, SpecialPossessionKey};
use strum::IntoEnumIterator;

/// Every item of one kind seen so far, keyed by its `XKey` value, to the first playbook that used
/// it and that item's content.
struct Seen<XKey: Ord>(BTreeMap<XKey, (PlaybookKey, String)>);

impl<XKey: Ord> Default for Seen<XKey> {
    fn default() -> Self {
        Self(BTreeMap::new())
    }
}

impl<XKey: Ord + Copy + Debug + Display> Seen<XKey> {
    fn record(
        &mut self,
        playbook: PlaybookKey,
        item: &(impl Key<Key = XKey> + Serialize),
        problems: &mut Vec<String>,
    ) {
        let key = item.key();
        let content = json5::to_string(item).expect("an item can be serialized");
        if let Some((first, first_content)) = self.0.get(&key) {
            if *first == playbook {
                problems.push(format!("{key}: used twice by {playbook}"));
            } else if *first_content != content {
                problems.push(format!(
                    "{key}: {first}'s and {playbook}'s differ — give one a distinct keyPrefix"
                ));
            }
        } else {
            self.0.insert(key, (playbook, content));
        }
    }

    /// Every `XKey` variant must be produced by some name — it once was, so if not some name is no
    /// longer here.
    fn check_every_variant_used(&self, problems: &mut Vec<String>)
    where
        XKey: IntoEnumIterator,
    {
        for key in XKey::iter() {
            if !self.0.contains_key(&key) {
                problems.push(format!("{key}: no item produces this {}", type_name::<XKey>()));
            }
        }
    }
}

#[derive(Default)]
struct AllSeen {
    playbooks: Vec<PlaybookKey>,
    backgrounds: Seen<BackgroundKey>,
    special_possessions: Seen<SpecialPossessionKey>,
    moves: Seen<MoveKey>,
    backstories: Seen<BackstoryKey>,
    problems: Vec<String>,
}

impl AllSeen {
    fn record_playbook(&mut self, playbook: &Playbook) -> PlaybookKey {
        let key = playbook.key();
        if self.playbooks.contains(&key) {
            self.problems.push(format!("{key}: two playbooks share this key"));
        }
        self.playbooks.push(key);
        key
    }

    fn check_every_variant_used(&mut self) {
        for key in PlaybookKey::iter() {
            if !self.playbooks.contains(&key) {
                self.problems.push(format!("{key}: no playbook produces this PlaybookKey"));
            }
        }
        self.backgrounds.check_every_variant_used(&mut self.problems);
        self.special_possessions.check_every_variant_used(&mut self.problems);
        self.moves.check_every_variant_used(&mut self.problems);
        self.backstories.check_every_variant_used(&mut self.problems);
    }

    /// The problems, sorted, one per line.
    fn report(&self) -> String {
        let mut problems = self.problems.clone();
        problems.sort();
        problems.join("\n")
    }
}

/// Record every `XKey` item reachable from `playbook`.
fn visit_playbook(playbook: &Playbook, seen: &mut AllSeen) {
    let pb_key = seen.record_playbook(playbook);
    for background in &playbook.backgrounds {
        seen.backgrounds.record(pb_key, background, &mut seen.problems);
        for a_move in background.moves() {
            seen.moves.record(pb_key, a_move, &mut seen.problems);
        }
    }
    for possession in &playbook.special_possessions.options {
        seen.special_possessions.record(pb_key, possession, &mut seen.problems);
    }
    for a_move in &playbook.moves {
        seen.moves.record(pb_key, a_move, &mut seen.problems);
    }
    for backstory in &playbook.backstory {
        seen.backstories.record(pb_key, backstory, &mut seen.problems);
    }
}

#[test]
fn keys_consistent() {
    let mut seen = AllSeen::default();
    for name in playbook_names() {
        let playbook = json5_playbook(&name).unwrap_or_else(|e| panic!("{e:#}"));
        visit_playbook(&playbook, &mut seen);
    }
    seen.check_every_variant_used();
    assert!(seen.problems.is_empty(), "Inconsistent keys:\n{}", seen.report());
}
