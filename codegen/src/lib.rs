#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_macros, unused_mut, unused_variables)
)]
use anyhow::{Context, Result};
use schema::{Gear, Playbook};
pub mod bake;
pub mod fixed;
pub mod key;
pub mod schema;

pub fn playbook_from_str(json5_source: &str) -> Result<Playbook> {
    json5::from_str(json5_source).context("can't parse into a Playbook")
}

pub fn json5_playbook(playbook_name: &str) -> Result<Playbook> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("json5")
        .join(playbook_name)
        .with_extension("json5");
    let source = std::fs::read_to_string(&path)
        .with_context(|| format!("when reading {}", path.display()))?;
    playbook_from_str(&source).with_context(|| format!("when parsing {playbook_name}"))
}

/// `json5_source`, the text of a gear file, parsed.
pub fn gear_from_str(json5_source: &str) -> Result<Gear> {
    json5::from_str(json5_source).context("can't parse into Gear")
}

/// `gear.json5`, parsed.
pub fn json5_gear() -> Result<Gear> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("json5/gear.json5");
    let source = std::fs::read_to_string(&path)
        .with_context(|| format!("when reading {}", path.display()))?;
    gear_from_str(&source).context("when parsing gear")
}

#[must_use]
pub fn playbook_names() -> Vec<String> {
    [
        "blessed",
        "fox",
        "heavy",
        "judge",
        "lightbearer",
        "marshal",
        "ranger",
        "seeker",
        "would-be-hero",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}
