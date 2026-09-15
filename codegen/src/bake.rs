//! Bake the Fixed content into `stonetop/src/fixed/generated.rs`: one named static per item, a
//! `fixed_part()` method on each key enum that matches every variant to its static, and one
//! static for the Setting overview. The match is exhaustive, so a key added to `keys.rs` without
//! a re-bake is a compile error in the `ssr` build, not a gap. Run as `cargo xtask bake`, which
//! writes the file and then runs `rustfmt` on it.

use std::collections::BTreeMap;
use std::fmt::{Display, Write as _};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};
use databake::{Bake, CrateEnv};
use stonetop::fixed;
use stonetop::keys::{
    BackgroundKey, BackstoryKey, GizmoKey, MoveKey, PlaybookKey, SpecialPossessionKey,
};

use crate::key::Key;
use crate::schema::{self, Gear, Playbook, SettingOverview};
use crate::{json5_gear, json5_playbook, json5_setting_overview, playbook_names};

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
/// If the Setting overview, the gear file, or a playbook fails to parse, or rustfmt is missing
/// or rejects the output.
pub fn baked_source() -> Result<String> {
    let overview = json5_setting_overview()?;
    let gear = json5_gear()?;
    let playbooks = playbook_names()
        .iter()
        .map(|name| json5_playbook(name))
        .collect::<Result<Vec<Playbook>>>()?;
    let mut out = String::from(HEADER);
    bake_setting_overview(&overview, &mut out)?;
    let gizmos = bake_gizmos(&gear, &mut out)?;
    let moves = bake_moves(&playbooks, &mut out)?;
    let backgrounds = bake_backgrounds(&playbooks, &moves, &mut out)?;
    let special_possessions = bake_special_possessions(&playbooks, &mut out)?;
    let backstories = bake_backstories(&playbooks, &mut out)?;
    let items = ItemStatics { moves, backgrounds, special_possessions, backstories };
    let playbook_statics = bake_playbooks(&playbooks, &items, &mut out)?;
    bake_fixed_part("MoveKey", "MoveFixed", &items.moves, &mut out)?;
    bake_fixed_part("BackgroundKey", "BackgroundFixed", &items.backgrounds, &mut out)?;
    bake_fixed_part(
        "SpecialPossessionKey",
        "SpecialPossessionFixed",
        &items.special_possessions,
        &mut out,
    )?;
    bake_fixed_part("BackstoryKey", "BackstoryFixed", &items.backstories, &mut out)?;
    bake_fixed_part("PlaybookKey", "PlaybookFixed", &playbook_statics, &mut out)?;
    bake_fixed_part("GizmoKey", "GizmoFixed", &gizmos, &mut out)?;
    rustfmt(&out)
}

const HEADER: &str = "\
//! Baked Fixed content: written by `cargo xtask bake` from `codegen/json5/`, checked by
//! `codegen/tests/generated_fresh.rs`. Do not edit; change the json5 and run the command.
//!
//! The Setting overview comes first, then gizmos grouped by the gear file's sections. The
//! playbook statics are grouped by playbook, a shared item appearing once under the first
//! playbook that uses it, as in `keys.rs`. Each playbook's static references its items'
//! statics; a Special Possession's kit names its gizmos by key. Each kind's `fixed_part()`
//! matches every key to its static.

";

/// Write the Setting overview's one static, which `fixed::setting_overview()` returns.
fn bake_setting_overview(overview: &SettingOverview, out: &mut String) -> Result<()> {
    let env = CrateEnv::default();
    writeln!(out, "// Setting overview\n")?;
    let baked = overview.to_fixed().bake(&env);
    writeln!(
        out,
        "pub(super) static SETTING_OVERVIEW: stonetop::fixed::SettingOverviewFixed = {baked};\n"
    )?;
    Ok(())
}

/// Write one static per gizmo, section by section in the gear file's order, returning each
/// key's static's name.
fn bake_gizmos(gear: &Gear, out: &mut String) -> Result<BTreeMap<GizmoKey, String>> {
    let env = CrateEnv::default();
    let mut statics = BTreeMap::new();
    for (section, gizmos) in gear.sections() {
        writeln!(out, "// Gear: {section}\n")?;
        for gizmo in gizmos {
            let key = gizmo.key();
            assert!(!statics.contains_key(&key), "{key}: two gizmos in gear.json5 share this key");
            let name = format!("GIZMO_{}", screaming_snake(&key.to_string()));
            let baked = gizmo.to_fixed().bake(&env);
            writeln!(out, "static {name}: stonetop::fixed::GizmoFixed = {baked};\n")?;
            statics.insert(key, name);
        }
    }
    Ok(statics)
}

/// The name of every item static written so far, by key, so that a later static can reference
/// an item instead of baking its content a second time.
struct ItemStatics {
    moves: BTreeMap<MoveKey, String>,
    backgrounds: BTreeMap<BackgroundKey, String>,
    special_possessions: BTreeMap<SpecialPossessionKey, String>,
    backstories: BTreeMap<BackstoryKey, String>,
}

/// `&NAME` for `key`'s static in `statics`.
fn static_ref<K: Display + Ord>(statics: &BTreeMap<K, String>, key: &K) -> String {
    let name =
        statics.get(key).unwrap_or_else(|| panic!("{key}: referenced before its static was baked"));
    format!("&{name}")
}

/// `&A, &B, …` for `keys`' statics in `statics`: the body of an array or slice literal, for
/// the caller to bracket.
fn static_refs<K: Display + Ord>(
    statics: &BTreeMap<K, String>,
    keys: impl IntoIterator<Item = K>,
) -> String {
    let refs: Vec<String> = keys.into_iter().map(|key| static_ref(statics, &key)).collect();
    refs.join(", ")
}

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
            let reference = static_ref(moves, &a_move.key);
            format!("stonetop::fixed::BackgroundChunk::Move({reference})")
        }
        other => other.bake(env).to_string(),
    }
}

/// Write one static per Special Possession, in playbook order with shared Special Possessions
/// once, returning each key's static's name. (A playbook's Special Possessions *section*, which
/// references these statics, is `bake_special_possessions_section`.)
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

/// Write one static per playbook, returning each key's static's name. A playbook's
/// Backgrounds, Special Possessions, Moves, and Backstories are baked by the item functions;
/// here they're referenced by name via `items`, not re-baked, so `items` must already hold
/// every item these playbooks offer.
fn bake_playbooks(
    playbooks: &[Playbook],
    items: &ItemStatics,
    out: &mut String,
) -> Result<BTreeMap<PlaybookKey, String>> {
    let env = CrateEnv::default();
    let mut statics = BTreeMap::new();
    for playbook in playbooks {
        let key = playbook.key();
        let name = format!("PLAYBOOK_{}", screaming_snake(&key.to_string()));
        writeln!(out, "// {}\n", playbook.name)?;
        let baked = bake_playbook(playbook, items, &env);
        writeln!(out, "static {name}: stonetop::fixed::PlaybookFixed = {baked};\n")?;
        statics.insert(key, name);
    }
    Ok(statics)
}

/// `playbook` baked to a `PlaybookFixed` literal. Every field bakes as `MoveFixed`'s fields do,
/// except the four that hold items: `backgrounds`, `special_possessions.options`, `moves`, and
/// `backstory` reference the items' already-baked statics in `items` rather than baking a
/// second copy of each item's content.
fn bake_playbook(playbook: &Playbook, items: &ItemStatics, env: &CrateEnv) -> String {
    let fixed = playbook.to_fixed();
    let key = fixed.key.bake(env);
    let name = fixed.name.bake(env);
    let description = fixed.description.bake(env);
    let backgrounds =
        static_refs(&items.backgrounds, fixed.backgrounds.iter().map(|background| background.key));
    let instinct = fixed.instinct.bake(env);
    let appearance = fixed.appearance.bake(env);
    let origin_choices = fixed.origin_choices.bake(env);
    let stats_to_assign = fixed.stats_to_assign.bake(env);
    let damage = fixed.damage.bake(env);
    let hp = fixed.hp.bake(env);
    let special_possessions =
        bake_special_possessions_section(&fixed.special_possessions, items, env);
    let starting_move_choices = fixed.starting_move_choices.bake(env);
    let grants_moves = fixed.grants_moves.bake(env);
    let moves = static_refs(&items.moves, fixed.moves.iter().map(|a_move| a_move.key));
    let moves_footnote = fixed.moves_footnote.bake(env);
    let intro = fixed.intro.bake(env);
    let backstory =
        static_refs(&items.backstories, fixed.backstory.iter().map(|backstory| backstory.key));
    format!(
        "stonetop::fixed::PlaybookFixed {{ \
             key: {key}, name: {name}, description: {description}, \
             backgrounds: [{backgrounds}], instinct: {instinct}, appearance: {appearance}, \
             origin_choices: {origin_choices}, stats_to_assign: {stats_to_assign}, \
             damage: {damage}, hp: {hp}, special_possessions: {special_possessions}, \
             starting_move_choices: {starting_move_choices}, grants_moves: {grants_moves}, \
             moves: &[{moves}], moves_footnote: {moves_footnote}, intro: {intro}, \
             backstory: &[{backstory}] }}"
    )
}

/// A playbook's `special_possessions` section baked, its `options` referencing the Special
/// Possessions' statics already written by `bake_special_possessions`, via `items`.
fn bake_special_possessions_section(
    section: &fixed::SpecialPossessions,
    items: &ItemStatics,
    env: &CrateEnv,
) -> String {
    let pick_note = section.pick_note.bake(env);
    let pick_count = section.pick_count.bake(env);
    let preselected = section.preselected.bake(env);
    let options = static_refs(
        &items.special_possessions,
        section.options.iter().map(|possession| possession.key),
    );
    format!(
        "stonetop::fixed::SpecialPossessions {{ \
             pick_note: {pick_note}, pick_count: {pick_count}, preselected: {preselected}, \
             options: &[{options}] }}"
    )
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
