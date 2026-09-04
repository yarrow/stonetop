//! Bake the Fixed content into `stonetop/src/fixed/generated.rs`: one named static per item, and a
//! `fixed_part()` method on each key enum that matches every variant to its static. The match is
//! exhaustive, so a key added to `keys.rs` without a re-bake is a compile error in the `ssr`
//! build, not a gap. Run as `cargo xtask bake`, which writes the file and then runs `rustfmt` on
//! it.

use std::collections::BTreeMap;
use std::fmt::{Display, Write as _};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};
use databake::{Bake, CrateEnv};
use stonetop::fixed;
use stonetop::keys::{BackgroundKey, BackstoryKey, MoveKey, SpecialPossessionKey};

use crate::key::Key;
use crate::schema::{self, Playbook};
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
    let backgrounds = bake_backgrounds(&playbooks, &moves, &mut out)?;
    let special_possessions = bake_special_possessions(&playbooks, &mut out)?;
    let backstories = bake_backstories(&playbooks, &mut out)?;
    bake_fixed_part("MoveKey", "MoveFixed", &moves, &mut out)?;
    bake_fixed_part("BackgroundKey", "BackgroundFixed", &backgrounds, &mut out)?;
    bake_fixed_part(
        "SpecialPossessionKey",
        "SpecialPossessionFixed",
        &special_possessions,
        &mut out,
    )?;
    bake_fixed_part("BackstoryKey", "BackstoryFixed", &backstories, &mut out)?;
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

/// Write one static per Background, in playbook order with shared Backgrounds once (none are
/// shared today, but the loop guards against it exactly as Moves' does), returning each key's
/// static's name. A Background's anonymous Moves are baked by `bake_moves`; here they're
/// referenced by name via `moves`, not re-baked, so `moves` must already hold every Move these
/// playbooks offer.
fn bake_backgrounds(
    playbooks: &[Playbook],
    moves: &BTreeMap<MoveKey, String>,
    out: &mut String,
) -> Result<BTreeMap<BackgroundKey, String>> {
    let env = CrateEnv::default();
    let mut statics = BTreeMap::new();
    for playbook in playbooks {
        writeln!(out, "// {}: Backgrounds\n", playbook.name)?;
        for background in &playbook.backgrounds {
            let key = background.key();
            if statics.contains_key(&key) {
                continue;
            }
            let name = format!("BACKGROUND_{}", screaming_snake(&key.to_string()));
            let baked = bake_background(background, moves, &env);
            writeln!(out, "static {name}: stonetop::fixed::BackgroundFixed = {baked};\n")?;
            statics.insert(key, name);
        }
    }
    Ok(statics)
}

/// `background` baked to a `BackgroundFixed` literal. Every field bakes as `MoveFixed`'s
/// fields do, except `description`: a Move chunk there references its already-baked static in
/// `moves` rather than baking a second copy of the Move's content.
fn bake_background(
    background: &schema::Background,
    moves: &BTreeMap<MoveKey, String>,
    env: &CrateEnv,
) -> String {
    let background = background.to_fixed();
    let key = background.key.bake(env);
    let name = background.name.bake(env);
    let chunks: Vec<String> = background
        .description
        .iter()
        .map(|chunk| bake_background_chunk(chunk, moves, env))
        .collect();
    let grants_moves = background.grants_moves.bake(env);
    let grants_possession = background.grants_possession.bake(env);
    let grants_topic = background.grants_topic.bake(env);
    format!(
        "stonetop::fixed::BackgroundFixed {{ \
             key: {key}, name: {name}, description: &[{}], \
             grants_moves: {grants_moves}, grants_possession: {grants_possession}, \
             grants_topic: {grants_topic} }}",
        chunks.join(", ")
    )
}

/// One chunk of a Background's `description`, baked. A Move chunk names its already-baked
/// static in `moves` instead of baking the Move's content a second time; every other chunk
/// bakes normally.
fn bake_background_chunk(
    chunk: &fixed::BackgroundChunk,
    moves: &BTreeMap<MoveKey, String>,
    env: &CrateEnv,
) -> String {
    match chunk {
        fixed::BackgroundChunk::Move(a_move) => {
            let name = moves
                .get(&a_move.key)
                .unwrap_or_else(|| panic!("{}: Move not baked before its Background", a_move.key));
            format!("stonetop::fixed::BackgroundChunk::Move(&{name})")
        }
        other => other.bake(env).to_string(),
    }
}

/// Write one static per Special Possession, in playbook order with shared Special Possessions
/// once, returning each key's static's name.
fn bake_special_possessions(
    playbooks: &[Playbook],
    out: &mut String,
) -> Result<BTreeMap<SpecialPossessionKey, String>> {
    let env = CrateEnv::default();
    let mut statics = BTreeMap::new();
    for playbook in playbooks {
        writeln!(out, "// {}: Special Possessions\n", playbook.name)?;
        for possession in &playbook.special_possessions.options {
            let key = possession.key();
            if statics.contains_key(&key) {
                continue;
            }
            let name = format!("SPECIAL_POSSESSION_{}", screaming_snake(&key.to_string()));
            let baked = possession.to_fixed().bake(&env);
            writeln!(out, "static {name}: stonetop::fixed::SpecialPossessionFixed = {baked};\n")?;
            statics.insert(key, name);
        }
    }
    Ok(statics)
}

/// Write one static per Backstory, in playbook order with shared Backstories once, returning
/// each key's static's name.
fn bake_backstories(
    playbooks: &[Playbook],
    out: &mut String,
) -> Result<BTreeMap<BackstoryKey, String>> {
    let env = CrateEnv::default();
    let mut statics = BTreeMap::new();
    for playbook in playbooks {
        writeln!(out, "// {}: Backstories\n", playbook.name)?;
        for backstory in &playbook.backstory {
            let key = backstory.key();
            if statics.contains_key(&key) {
                continue;
            }
            let name = format!("BACKSTORY_{}", screaming_snake(&key.to_string()));
            let baked = backstory.to_fixed().bake(&env);
            writeln!(out, "static {name}: stonetop::fixed::BackstoryFixed = {baked};\n")?;
            statics.insert(key, name);
        }
    }
    Ok(statics)
}

/// Write the `fixed_part()` that matches every key of `key_type` to its static in `statics`.
/// The arms come from the items found, not from the key enum's variants, so that a variant
/// without an item fails to compile as a non-exhaustive match rather than being papered over
/// here.
fn bake_fixed_part<K: Display + Ord>(
    key_type: &str,
    fixed_type: &str,
    statics: &BTreeMap<K, String>,
    out: &mut String,
) -> Result<()> {
    writeln!(
        out,
        "impl stonetop::keys::{key_type} {{\n\
             /// The printed content this key names.\n\
             #[must_use]\n\
             pub fn fixed_part(self) -> &'static stonetop::fixed::{fixed_type} {{\n\
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
