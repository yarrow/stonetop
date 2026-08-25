#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_macros, unused_mut, unused_variables)
)]
use anyhow::{Context, Result};
use playbook::Playbook;
pub mod name_key;
pub mod playbook;

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
