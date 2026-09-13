//! The Fixed half of the Fixed/State split: playbook content as printed, immutable and
//! identical for everyone. The shapes follow `codegen/src/schema.rs` with three changes:
//! every `String` is a `&'static str`, every `Vec<T>` is a `&'static [T]`, and every keyed
//! item carries a `key` of its kind's enum and has no `key_prefix`.
//!
//! Values are built by `codegen` from the json5 and baked into `generated.rs`, which is
//! compiled only behind the `ssr` feature and gives each key enum a total `fixed_part()` method.
//! The `Bake` derives are behind the `codegen` feature and name the path each type is reached
//! by from the generated file.

#[cfg(feature = "codegen")]
use databake::Bake;

use crate::Die;
use crate::keys::{BackgroundKey, BackstoryKey, MoveKey, PlaybookKey, SpecialPossessionKey};

#[cfg(feature = "ssr")]
mod generated;

// Playbook ---------------------------------------------------------------

/// A playbook as printed, in printed order. Its Backgrounds, Special Possessions, Moves, and
/// Backstories are references to those items' own statics, each reachable by its own key too.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub struct PlaybookFixed {
    pub key: PlaybookKey,
    pub name: &'static str,
    pub description: &'static str,
    pub backgrounds: [&'static BackgroundFixed; 3],
    pub instinct: [Instinct; 5],
    pub appearance: [TaggedRow; 4],
    pub origin_choices: &'static [Origin],
    pub stats_to_assign: [i8; 6],
    pub damage: Die,
    pub hp: u8,
    pub special_possessions: SpecialPossessions,
    pub starting_move_choices: u8,
    pub grants_moves: &'static [GrantMove],
    pub moves: &'static [&'static MoveFixed],
    pub moves_footnote: Option<&'static str>,
    pub intro: Intro,
    pub backstory: &'static [&'static BackstoryFixed],
}

impl PlaybookFixed {
    /// Whether the playbook grants the Move called `move_name` outright, with no choice.
    #[must_use]
    pub fn grants_outright(&self, move_name: MoveKey) -> bool {
        self.grants_moves
            .iter()
            .any(|grant| matches!(grant, GrantMove::Simply(name) if *name == move_name))
    }
}

/// The drive that pulls a character toward trouble.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub struct Instinct {
    pub title: &'static str,
    pub description: &'static str,
}

/// Where a character is from, and how they're named there.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub struct Origin {
    pub location: &'static str,
    pub naming: Naming,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub enum Naming {
    Instructions(&'static str),
    Names(&'static [&'static str]),
    MixAndMatch(NameParts),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub struct NameParts {
    pub intro: &'static str,
    pub name_parts: &'static [TaggedRow],
}

/// The Special Possessions section: the pick note and counts, and the options in printed order.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub struct SpecialPossessions {
    pub pick_note: &'static str,
    pub pick_count: u8,
    /// How many of the leading `options` the playbook starts with.
    pub preselected: u8,
    pub options: &'static [&'static SpecialPossessionFixed],
}

/// The Introductions section.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub struct Intro {
    pub title: &'static str,
    pub text: &'static str,
}

// Move -------------------------------------------------------------------

/// A Move as printed.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub struct MoveFixed {
    pub key: MoveKey,
    pub name: &'static str,
    pub description: &'static str,
    pub requires: &'static [Requirement],
    pub max_picks: u8,
    pub resource: &'static [Resource],
    pub checklist: Option<MoveChecklist>,
}

/// A condition a character must meet to take a Move. Every `Requirement` in a `Move`'s `requires`
/// field is, er, required. (`NeedsOneOf` requires either of the two `Move`s mentioned to have been
/// already taken).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub enum Requirement {
    Level(u8),
    Needs(MoveKey),
    NeedsOneOf(MoveKey, MoveKey),
    NeedsStrength,
    NeedsSixInPotentialFG,
    Replaces(MoveKey),
    NeedsPlaybook(PlaybookKey),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub enum MoveChecklist {
    Options(&'static [&'static str]),
    OptionsWithLevel(&'static [&'static str]),
}

// Resource ---------------------------------------------------------------

/// A countable pool held by a Move or possession, with a capacity and a starting fullness.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub struct Resource {
    pub hold: &'static str,
    pub can_be: CanBe,
    pub start: EmptyFull,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub enum CanBe {
    Max(u8),
    Labels(&'static [&'static str]),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub enum EmptyFull {
    Empty,
    Full,
}

// Background -------------------------------------------------------------

/// A Background as printed: flavor text and mechanical chunks in reading order.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub struct BackgroundFixed {
    pub key: BackgroundKey,
    pub name: &'static str,
    pub description: &'static [BackgroundChunk],
    pub grants_moves: &'static [GrantMove],
    pub grants_possession: Option<GrantPossession>,
    pub grants_topic: Option<GrantTopic>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub enum BackgroundChunk {
    Flavor(&'static str),
    /// An anonymous Move this Background grants. It's a keyed Move like any other, baked and
    /// reachable by its own `MoveKey`; this is a reference to that same static, not a second
    /// copy of its content.
    Move(&'static MoveFixed),
    Checklist(BackgroundChecklist),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub enum BackgroundChecklist {
    Options(&'static [&'static str]),
    OptionsPrecheckable(&'static [PrecheckableOption]),
    Rows(&'static [TaggedRow]),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub struct PrecheckableOption {
    pub prechecked: bool,
    pub text: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub struct TaggedRow {
    pub tag: &'static str,
    pub items: &'static [&'static str],
}

/// A Move granted outright, or a choice between two.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub enum GrantMove {
    Simply(MoveKey),
    ChooseOne(MoveKey, MoveKey),
}

/// A Special Possession granted outright, or a choice between two.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub enum GrantPossession {
    Simply(SpecialPossessionKey),
    ChooseOne(SpecialPossessionKey, SpecialPossessionKey),
}

/// A topic (for the Seeker's Lore) granted outright, or a choice among several. Topics have
/// no key enum; they are the printed names.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub enum GrantTopic {
    Simply(&'static str),
    ChooseOne(&'static [&'static str]),
}

// SpecialPossession ------------------------------------------------------

/// A Special Possession as printed.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub struct SpecialPossessionFixed {
    pub key: SpecialPossessionKey,
    pub name: &'static str,
    pub description: &'static str,
    pub resource: Option<Resource>,
    pub pick: &'static [&'static str],
}

// Backstory --------------------------------------------------------------

/// A Backstory as printed: one named list of text and choices.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub struct BackstoryFixed {
    pub key: BackstoryKey,
    pub name: &'static str,
    pub list: &'static [BackstoryItem],
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub enum BackstoryItem {
    Text(&'static str),
    Choices(&'static [&'static str]),
    ChoiceRow(TaggedRow),
}
