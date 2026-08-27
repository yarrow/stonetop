//! Workspace maintenance tasks, run as `cargo xtask <task>`.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use duct::cmd;

const USAGE: &str = "\
usage: cargo xtask <task>

tasks:
  generate-keys    regenerate stonetop/src/item_keys.rs from codegen/json5";

fn main() -> Result<()> {
    match std::env::args().nth(1).as_deref() {
        Some("generate-keys") => generate_keys(),
        Some(task) => bail!("unknown task `{task}`\n\n{USAGE}"),
        None => bail!("{USAGE}"),
    }
}

/// Run codegen's `generate-keys` binary, capture its output in
/// `stonetop/src/item_keys.rs`, and format the result.
fn generate_keys() -> Result<()> {
    let root = workspace_root();
    let output = root.join("stonetop/src/item_keys.rs");

    cmd!(cargo(), "run", "--package", "codegen", "--bin", "generate-keys")
        .dir(&root)
        .stdout_path(&output)
        .run()
        .context("running codegen's generate-keys binary")?;

    let rustfmt = which::which("rustfmt")
        .context("rustfmt not found; install it with `rustup component add rustfmt`")?;
    cmd!(rustfmt, "--edition", "2024", &output)
        .run()
        .with_context(|| format!("formatting {}", output.display()))?;

    println!("wrote {}", output.display());
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
