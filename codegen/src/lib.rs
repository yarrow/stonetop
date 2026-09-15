#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_macros, unused_mut, unused_variables)
)]
use anyhow::{Context, Result};
use schema::{Gear, Playbook, SettingOverview};
pub mod bake;
pub mod fixed;
pub mod key;
pub mod schema;

pub fn playbook_from_str(json5_source: &str) -> Result<Playbook> {
    json5::from_str(json5_source).context("can't parse into a Playbook")
}

pub fn json5_playbook(playbook_name: &str) -> Result<Playbook> {
    let source = json5_source(&format!("{playbook_name}.json5"))?;
    playbook_from_str(&source).with_context(|| format!("when parsing {playbook_name}"))
}

/// The text of the file `name` under `codegen/json5/`.
fn json5_source(name: &str) -> Result<String> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("json5").join(name);
    std::fs::read_to_string(&path).with_context(|| format!("when reading {}", path.display()))
}

/// `json5_source`, the text of a gear file, parsed.
pub fn gear_from_str(json5_source: &str) -> Result<Gear> {
    json5::from_str(json5_source).context("can't parse into Gear")
}

/// `gear.json5`, parsed.
pub fn json5_gear() -> Result<Gear> {
    gear_from_str(&json5_source("gear.json5")?).context("when parsing gear")
}

/// `json5_source`, the text of the Setting overview, parsed.
pub fn setting_overview_from_str(json5_source: &str) -> Result<SettingOverview> {
    json5::from_str(json5_source).context("can't parse into a SettingOverview")
}

/// `setting-overview.json5`, parsed.
pub fn json5_setting_overview() -> Result<SettingOverview> {
    setting_overview_from_str(&json5_source("setting-overview.json5")?)
        .context("when parsing the Setting overview")
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
