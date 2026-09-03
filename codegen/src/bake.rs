//! Bake the Fixed content into `stonetop/src/fixed/generated.rs`: one named static per item,
//! and a `fixed_part()` method on each key enum that matches every variant to its static. The match
//! is exhaustive, so a key added to `keys.rs` without a re-bake is a compile error in the `ssr`
//! build, not a gap. Run as `cargo xtask bake`, which writes the file and then runs `rustfmt`
//! on it.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};
use databake::{Bake, CrateEnv};
use stonetop::keys::MoveKey;

use crate::key::Key;
use crate::schema::Playbook;
use crate::{json5_playbook, playbook_names};

/// Where the baked file lives.
#[must_use]
pub fn generated_path() -> PathBuf {
    workspace_root().join("stonetop/src/fixed/generated.rs")
}

/// The workspace root, i.e. the parent of the `codegen` crate.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("codegen lives directly under the workspace root")
        .to_path_buf()
}

/// The baked file's contents, rustfmt-clean, as `cargo xtask bake` would write them now.
///
/// # Errors
///
/// If a playbook fails to parse, or rustfmt is missing or rejects the output.
pub fn baked_source() -> Result<String> {
    let playbooks = playbook_names()
        .iter()
        .map(|name| json5_playbook(name))
        .collect::<Result<Vec<Playbook>>>()?;
    let mut out = String::from(HEADER);
    let moves = bake_moves(&playbooks, &mut out)?;
    bake_fixed(&moves, &mut out)?;
    rustfmt(&out)
}

const HEADER: &str = "\
//! Baked Fixed content: written by `cargo xtask bake` from `codegen/json5/`, checked by
//! `codegen/tests/generated_fresh.rs`. Do not edit; change the json5 and run the command.
//!
//! The statics are grouped by playbook, a shared item appearing once under the first playbook
//! that uses it, as in `keys.rs`. Each kind's `fixed_part()` matches every key to its static.

";

/// Write one static per Move, in playbook order with shared Moves once, returning each key's
/// static's name.
fn bake_moves(playbooks: &[Playbook], out: &mut String) -> Result<BTreeMap<MoveKey, String>> {
    let env = CrateEnv::default();
    let mut statics = BTreeMap::new();
    for playbook in playbooks {
        writeln!(out, "// {}: Moves\n", playbook.name)?;
        for a_move in playbook.moves() {
            let key = a_move.key();
            if statics.contains_key(&key) {
                continue;
            }
            let name = format!("MOVE_{}", screaming_snake(&key.to_string()));
            let baked = a_move.to_fixed().bake(&env);
            writeln!(out, "static {name}: stonetop::fixed::MoveFixed = {baked};\n")?;
            statics.insert(key, name);
        }
    }
    Ok(statics)
}

/// Write the `MoveKey::fixed` that matches each key to its static in `statics`. The arms come
/// from the Moves found, not from `MoveKey`'s variants, so that a variant without a Move fails
/// to compile as a non-exhaustive match rather than being papered over here.
fn bake_fixed(statics: &BTreeMap<MoveKey, String>, out: &mut String) -> Result<()> {
    writeln!(
        out,
        "impl stonetop::keys::MoveKey {{\n\
             /// The printed Move this key names.\n\
             #[must_use]\n\
             pub fn fixed_part(self) -> &'static stonetop::fixed::MoveFixed {{\n\
                 match self {{"
    )?;
    for (key, name) in statics {
        writeln!(out, "Self::{key} => &{name},")?;
    }
    writeln!(out, "}}\n}}\n}}")?;
    Ok(())
}

/// `FromRaisedByWolves` as a static's name: `FROM_RAISED_BY_WOLVES`.
fn screaming_snake(variant: &str) -> String {
    let mut name = String::with_capacity(variant.len() * 2);
    for (i, c) in variant.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            name.push('_');
        }
        name.extend(c.to_uppercase());
    }
    name
}

/// `source` formatted by rustfmt with the workspace's `rustfmt.toml`, so that what `bake`
/// writes is what `cargo fmt` would leave alone.
fn rustfmt(source: &str) -> Result<String> {
    let config = workspace_root().join("rustfmt.toml");
    let mut child = Command::new("rustfmt")
        .args(["--edition", "2024", "--config-path"])
        .arg(&config)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("running rustfmt; install it with `rustup component add rustfmt`")?;
    let mut stdin = child.stdin.take().expect("stdin was piped");
    let source = source.to_owned();
    // rustfmt reads all of stdin before writing, but a separate writer can't deadlock either way.
    let writer = std::thread::spawn(move || stdin.write_all(source.as_bytes()));
    let output = child.wait_with_output().context("waiting for rustfmt")?;
    writer.join().expect("the writer thread doesn't panic").context("feeding rustfmt")?;
    if !output.status.success() {
        bail!("rustfmt rejected the baked source:\n{}", String::from_utf8_lossy(&output.stderr));
    }
    String::from_utf8(output.stdout).context("rustfmt's output is not UTF-8")
}

#[cfg(test)]
mod test {
    use super::screaming_snake;

    #[test]
    fn words_are_split_at_capitals() {
        assert_eq!(screaming_snake("FromRaisedByWolves"), "FROM_RAISED_BY_WOLVES");
    }

    #[test]
    fn single_letter_words_are_words() {
        assert_eq!(screaming_snake("AGoodDog"), "A_GOOD_DOG");
        assert_eq!(screaming_snake("ButIGetUpAgain"), "BUT_I_GET_UP_AGAIN");
    }
}
