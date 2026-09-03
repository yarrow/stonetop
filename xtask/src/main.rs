//! Workspace maintenance tasks, run as `cargo xtask <task>`.

use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};

const USAGE: &str = "\
usage: cargo xtask <task>

tasks:
  bake    rewrite stonetop/src/fixed/generated.rs from codegen/json5/";

fn main() -> Result<()> {
    match std::env::args().nth(1).as_deref() {
        Some("bake") => bake(),
        Some(task) => bail!("unknown task `{task}`\n\n{USAGE}"),
        None => bail!("{USAGE}"),
    }
}

/// Run codegen's `bake` binary, which writes `stonetop/src/fixed/generated.rs`, then confirm
/// the result is rustfmt-clean. The file is meant to be clean by construction, so a failure
/// here is a bug in the generator, not something to fix by reformatting.
fn bake() -> Result<()> {
    let root = workspace_root();
    let output = root.join("stonetop/src/fixed/generated.rs");

    run(Command::new(cargo())
        .args(["run", "--quiet", "--package", "codegen", "--bin", "bake"])
        .current_dir(&root))
    .context("running codegen's bake binary")?;

    run(Command::new("rustfmt").args(["--edition", "2024", "--check"]).arg(&output))
        .with_context(|| format!("{} is not rustfmt-clean", output.display()))?;
    Ok(())
}

/// Run `command` to completion, failing if it can't be started or exits unsuccessfully.
fn run(command: &mut Command) -> Result<()> {
    let status = command.status().with_context(|| format!("starting {command:?}"))?;
    if !status.success() {
        bail!("{command:?} exited with {status}");
    }
    Ok(())
}

/// The workspace root, i.e. the parent of the `xtask` crate.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask lives directly under the workspace root")
        .to_path_buf()
}

/// The cargo that invoked us, so nested runs use the same toolchain.
fn cargo() -> String {
    std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string())
}
