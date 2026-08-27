use std::collections::VecDeque;

use heck::ToShoutySnakeCase;
use indexmap::{IndexMap, IndexSet};

use codegen::key::Key;
use codegen::playbook::Playbook;
use codegen::{json5_playbook, playbook_names};

fn main() {
    let mut genkey = GenKey::default();
    for name in playbook_names() {
        let playbook = json5_playbook(&name).unwrap_or_else(|e| panic!("{e:#}"));
        visit_playbook(&playbook, &mut genkey);
    }
    for kind in ["Playbook", "Background", "SpecialPossession", "Backstory"] {
        println!("pub enum {kind}Key {{");
        let keys_for = genkey.keys_for.get(kind).unwrap();
        for item in keys_for {
            println!("    {item},");
        }
        println!("}}");
    }
    println!("pub enum MoveKey {{");
    let keys_for = genkey.keys_for.get("Move").unwrap();
    let mut comment: VecDeque<(String, String)> =
        genkey.first_move.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
    let (mut playbook, mut first_mv) = comment.pop_front().unwrap_or_default();
    for item in keys_for {
        if *item == first_mv {
            println!("\n    // Moves for {playbook}");
            (playbook, first_mv) = comment.pop_front().unwrap_or_default();
        }
        println!("    {item},");
    }
    println!("}}");

    for (playbook, first) in &genkey.first_move {
        let shout = playbook.to_shouty_snake_case();
        println!(
            "pub const {shout}_MOVE_KEYS: (MoveKey, MoveKey) = (MoveKey::{first}, MoveKey::{});",
            genkey.last_move[playbook],
        );
    }
}

#[derive(Default, Debug)]
struct GenKey {
    first_move: IndexMap<String, String>,
    last_move: IndexMap<String, String>,
    keys_for: IndexMap<String, IndexSet<String>>,
}

impl GenKey {
    fn record_move(&mut self, playbook: &str, key: &str) {
        self.first_move.entry(playbook.to_string()).or_insert(key.to_string());
        self.last_move.insert(playbook.to_string(), key.to_string());
        let the_keys = self.keys_for.entry("Move".to_string()).or_default();
        the_keys.insert(key.to_string());
    }
    fn record(&mut self, kind: &str, key: &str) {
        let the_keys = self.keys_for.entry(kind.to_string()).or_default();
        the_keys.insert(key.to_string());
    }
}
/// Record every `Key` item reachable from `playbook`.
fn visit_playbook(playbook: &Playbook, genkey: &mut GenKey) {
    let pb_key = playbook.key();
    genkey.record("Playbook", &pb_key);

    // Background moves come first so they fall inside this playbook's MoveKey range.
    for background in &playbook.backgrounds {
        genkey.record("Background", &background.key());
        for mv in background.moves() {
            genkey.record_move(&pb_key, &mv.key());
        }
    }
    for possession in &playbook.special_possessions.options {
        genkey.record("SpecialPossession", &possession.key());
    }
    for mv in &playbook.moves {
        genkey.record_move(&pb_key, &mv.key());
    }
    for backstory in &playbook.backstory {
        genkey.record("Backstory", &backstory.key());
    }
}
