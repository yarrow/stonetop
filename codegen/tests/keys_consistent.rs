//! The json5 files and `stonetop/src/keys.rs` agree: every name resolves to an enum variant (else
//! the `Key` impl panics); every variant of every `XKey` enum is produced by some name; no
//! playbook uses a key twice; and if different playbooks have identically-named keys, then those
//! keys have the same value. For gear: every `GizmoKey` is produced by exactly one gizmo, every
//! reference from a possession resolves to a gizmo and names it as the gear file does, a
//! possession's kit is its references in order, and a qualifier appears in its gizmo's prose.

use std::any::type_name;
use std::collections::BTreeMap;
use std::fmt::{Debug, Display};

use codegen::key::{Key, gizmo_key};
use codegen::schema::{Gear, Gizmo, Playbook};
use codegen::{json5_gear, json5_playbook, playbook_names};
use serde::Serialize;
use stonetop::fixed::GizmoKit;
use stonetop::keys::{
    BackgroundKey, BackstoryKey, GizmoKey, MoveKey, PlaybookKey, SpecialPossessionKey,
};
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

/// Every gizmo in the gear file by its key, with a problem for each key produced twice.
fn gizmos_by_key<'a>(gear: &'a Gear, problems: &mut Vec<String>) -> BTreeMap<GizmoKey, &'a Gizmo> {
    let mut by_key = BTreeMap::new();
    for gizmo in gear.gizmos() {
        let key = gizmo.key();
        if by_key.insert(key, gizmo).is_some() {
            problems.push(format!("{key}: two gizmos in gear.json5 produce this GizmoKey"));
        }
    }
    by_key
}

#[test]
fn every_gizmo_key_is_produced_by_exactly_one_gizmo() {
    let gear = json5_gear().unwrap_or_else(|e| panic!("{e:#}"));
    let mut problems = Vec::new();
    let by_key = gizmos_by_key(&gear, &mut problems);
    for key in GizmoKey::iter() {
        if !by_key.contains_key(&key) {
            problems.push(format!("{key}: no gizmo in gear.json5 produces this GizmoKey"));
        }
    }
    problems.sort();
    assert!(problems.is_empty(), "Inconsistent gizmo keys:\n{}", problems.join("\n"));
}

#[test]
fn every_reference_resolves_and_names_its_gizmo_and_kits_are_the_references_in_order() {
    let gear = json5_gear().unwrap_or_else(|e| panic!("{e:#}"));
    let mut problems = Vec::new();
    let by_key = gizmos_by_key(&gear, &mut problems);
    for name in playbook_names() {
        let playbook = json5_playbook(&name).unwrap_or_else(|e| panic!("{e:#}"));
        for possession in &playbook.special_possessions.options {
            let label = format!("{name} {:?}", possession.name);
            let mut expected = Vec::new();
            for target in possession.gizmo.iter().map(String::as_str) {
                match gizmo_key(target) {
                    Ok(key) => expected.push(key),
                    Err(e) => problems.push(format!("{label}: {e}")),
                }
            }
            for reference in possession.gizmo_references() {
                let key = match gizmo_key(reference.target) {
                    Ok(key) => key,
                    Err(e) => {
                        problems.push(format!("{label}: {e}"));
                        continue;
                    }
                };
                expected.push(key);
                // A bare reference is spoken as written, so it must be the gizmo's own name;
                // a qualified gizmo needs the phrase form, since its description carries the
                // qualifier.
                if reference.phrase.is_none()
                    && !by_key[&key].name.eq_ignore_ascii_case(reference.target)
                {
                    problems.push(format!(
                        "{label}: `{{{}}}` is not the name of {key}; write `{{phrase|{}}}`",
                        reference.target, reference.target
                    ));
                }
            }
            let kit = possession.to_fixed().kit;
            if kit.gizmos() != expected.as_slice() {
                problems.push(format!("{label}: kit {kit:?} is not its references {expected:?}"));
            }
            match (&possession.gizmo, &kit) {
                (Some(_), GizmoKit::One(_)) | (None, GizmoKit::Referenced(_)) => {}
                (gizmo, kit) => problems.push(format!("{label}: gizmo {gizmo:?} but kit {kit:?}")),
            }
        }
    }
    problems.sort();
    assert!(problems.is_empty(), "Gizmo references:\n{}", problems.join("\n"));
}

#[test]
fn a_gizmos_qualifier_appears_in_its_description() {
    let gear = json5_gear().unwrap_or_else(|e| panic!("{e:#}"));
    let mut problems = Vec::new();
    for gizmo in gear.gizmos() {
        if let Some(qualifier) = &gizmo.qualifier
            && !gizmo.description.to_lowercase().contains(&qualifier.to_lowercase())
        {
            problems.push(format!(
                "{}: qualifier {qualifier:?} is not in the description {:?}",
                gizmo.key(),
                gizmo.description
            ));
        }
    }
    problems.sort();
    assert!(problems.is_empty(), "Qualifiers missing from prose:\n{}", problems.join("\n"));
}
