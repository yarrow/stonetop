use std::collections::HashSet;

use heck::ToShoutySnakeCase;
use indexmap::{IndexMap, IndexSet};

use codegen::key::Key;
use codegen::schema::Playbook;
use codegen::{json5_playbook, playbook_names};

/// The kinds of item in `ItemKey`, in the order their keys appear there. Each kind's name
/// is both how its keys are grouped as they're recorded and the heading of the sections it
/// introduces.
const KINDS: [&str; 4] = ["Backgrounds", "Special Possessions", "Moves", "Backstories"];

fn main() {
    let mut genkey = GenKey::default();
    for name in playbook_names() {
        let playbook = json5_playbook(&name).unwrap_or_else(|e| panic!("{e:#}"));
        visit_playbook(&playbook, &mut genkey);
    }

    println!("pub enum PlaybookKey {{");
    for playbook in genkey.playbooks.keys() {
        println!("    {playbook},");
    }
    println!("}}");

    // Playbooks share items — every playbook has the same Improved Stat move, for
    // instance — and a shared item gets one variant, listed under the first playbook to
    // use it. Later playbooks get a comment where that variant would have been.
    let mut emitted: HashSet<String> = HashSet::new();
    println!("pub enum ItemKey {{");
    for kind in KINDS {
        for (playbook, keys) in &genkey.keys_for[kind] {
            println!("\n    // {kind} for {}", genkey.playbooks[playbook.as_str()]);
            for item in keys {
                if emitted.insert(item.clone()) {
                    println!("    {item},");
                } else {
                    println!("    // {item}");
                }
            }
        }
    }
    println!("}}");

    for (playbook, moves) in &genkey.keys_for["Moves"] {
        let shout = playbook.to_shouty_snake_case();
        println!("pub const {shout}_MOVES: [ItemKey; {}] = [", moves.len());
        for mv in moves {
            println!("    ItemKey::{mv},");
        }
        println!("];");
    }
}

#[derive(Default, Debug)]
struct GenKey {
    /// Playbook key to the playbook's name as it reads mid-sentence ("the Blessed").
    playbooks: IndexMap<String, String>,
    /// Kind to playbook key to the keys of that kind belonging to that playbook.
    keys_for: IndexMap<&'static str, IndexMap<String, IndexSet<String>>>,
}

impl GenKey {
    /// Record `playbook` itself, returning its key.
    fn record_playbook(&mut self, playbook: &Playbook) -> String {
        let key = playbook.key();
        let name = playbook
            .name
            .strip_prefix("The ")
            .map_or_else(|| playbook.name.clone(), |rest| format!("the {rest}"));
        self.playbooks.insert(key.clone(), name);
        key
    }
    fn record(&mut self, kind: &'static str, playbook: &str, key: &str) {
        let the_keys =
            self.keys_for.entry(kind).or_default().entry(playbook.to_string()).or_default();
        the_keys.insert(key.to_string());
    }
}
/// Record every `Key` item reachable from `playbook`.
fn visit_playbook(playbook: &Playbook, genkey: &mut GenKey) {
    let pb_key = genkey.record_playbook(playbook);

    for background in &playbook.backgrounds {
        genkey.record("Backgrounds", &pb_key, &background.key());
        for mv in background.moves() {
            genkey.record("Moves", &pb_key, &mv.key());
        }
    }
    for possession in &playbook.special_possessions.options {
        genkey.record("Special Possessions", &pb_key, &possession.key());
    }
    for mv in &playbook.moves {
        genkey.record("Moves", &pb_key, &mv.key());
    }
    for backstory in &playbook.backstory {
        genkey.record("Backstories", &pb_key, &backstory.key());
    }
}
