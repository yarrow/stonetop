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

use crate::keys::{BackgroundKey, BackstoryKey, MoveKey, PlaybookKey, SpecialPossessionKey};

#[cfg(feature = "ssr")]
mod generated;

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
    pub grants_moves: &'static [Grant],
    pub grants_possession: Option<Grant>,
    pub grants_topic: Option<Grant>,
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

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub enum Grant {
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
