//! Workspace maintenance tasks, run as `cargo xtask <task>`.

use anyhow::{Result, bail};

const USAGE: &str = "\
usage: cargo xtask <task>

tasks:
  (none yet)";

fn main() -> Result<()> {
    match std::env::args().nth(1).as_deref() {
        Some(task) => bail!("unknown task `{task}`\n\n{USAGE}"),
        None => bail!("{USAGE}"),
    }
}
