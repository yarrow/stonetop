use anyhow::{Context, Result};
use playbook::Playbook;
pub mod playbook;

pub fn playbook_from_str(json5_source: &str) -> Result<Playbook> {
    json5::from_str(json5_source).context("can't parse into a Playbook")
}
