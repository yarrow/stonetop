use codegen;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use codegen::playbook::Playbook;
use codegen::playbook_from_str;

fn json5_playbook(playbook_name: &str) -> Result<Playbook> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("json5")
        .join(playbook_name)
        .with_added_extension("json5");
    let source =
        fs::read_to_string(&path).with_context(|| format!("when reading {}", &path.display()))?;
    playbook_from_str(&source).with_context(|| "when parsing {playbook_name}")
}

fn loads_ok(playbook_name: &str) {
    if let Err(e) = json5_playbook(playbook_name) {
        panic!("Failed: ${e:#?}");
    }
}
#[test]
fn blessed() {
    loads_ok("blessed");
}
#[test]
fn fox() {
    loads_ok("fox");
}
#[test]
fn heavy() {
    loads_ok("heavy");
}
#[test]
fn judge() {
    loads_ok("judge");
}
#[test]
fn lightbearer() {
    loads_ok("lightbearer");
}
#[test]
fn marshal() {
    loads_ok("marshal");
}
#[test]
fn ranger() {
    loads_ok("ranger");
}
#[test]
fn seeker() {
    loads_ok("seeker");
}
#[test]
fn would_be_hero() {
    loads_ok("would-be-hero");
}
