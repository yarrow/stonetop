//! The Fixed half of the Fixed/State split: playbook and gear content as printed, immutable
//! and identical for everyone. The shapes follow `codegen/src/schema.rs` with three changes:
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
use crate::keys::{
    BackgroundKey, BackstoryKey, GizmoKey, MoveKey, PlaybookKey, SpecialPossessionKey,
};

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

/// A countable pool held by a Move, possession, or gizmo, with a capacity and a starting
/// fullness.
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

/// A Special Possession as printed. Its `description` and `pick` entries name the gizmos of
/// its kit inline, as `{Candle}` or `{a lantern|Lantern}` (see [`gizmo_references`]); `kit`
/// holds those gizmos' keys in the same order, or the one gizmo of a kit of one, whose
/// `description` is empty because the gizmo's own description serves.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub struct SpecialPossessionFixed {
    pub key: SpecialPossessionKey,
    pub name: &'static str,
    pub description: &'static str,
    pub resource: Option<Resource>,
    pub pick: &'static [&'static str],
    pub kit: GizmoKit,
}

/// The gizmos a Special Possession makes available to the character who selects it.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub enum GizmoKit {
    /// A kit of one: the possession is this gizmo (the Blessed's Sacred pouch, the Ranger's
    /// Composite bow). Selecting the possession creates instance 0 of the gizmo, because the
    /// print puts its resource circles on the playbook.
    One(GizmoKey),
    /// The gizmos referenced in the possession's description and pick entries, in order of
    /// appearance; empty for a Reserve, an ability, or a follower. Selecting the possession
    /// makes them available and creates no instance, except that each picked Weapon of War
    /// creates instance 0 of its gizmo when picked.
    Referenced(&'static [GizmoKey]),
}

impl GizmoKit {
    /// Every gizmo in the kit, in order.
    #[must_use]
    pub fn gizmos(&self) -> &[GizmoKey] {
        match self {
            Self::One(key) => std::slice::from_ref(key),
            Self::Referenced(keys) => keys,
        }
    }
}

// Gizmo ------------------------------------------------------------------

/// A gizmo as printed: one thing a character can carry, defined once for the gear sheet, the
/// Inventory insert, and every kit that names it.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub struct GizmoFixed {
    pub key: GizmoKey,
    /// The bold head as printed: "Long spear", "Battleaxe". A grade of what the gizmo holds
    /// belongs in the name instead: "Skin of fine whisky", "Skin of common whisky".
    pub name: &'static str,
    /// The material printed after the name (iron, bronze, fine steel, boiled leather), which
    /// tells same-named gizmos apart. Iron is the unmarked default in the key.
    pub material: Option<&'static str>,
    /// The gear sheet's starred piercing upgrade, when this is the upgraded variant.
    pub piercing: Option<u8>,
    pub slots: SlotCount,
    /// Everything after the name as printed once the name has taken what belongs to it,
    /// material and tags included, with `{resource}` where a run of circles was.
    pub description: &'static str,
    pub resource: Option<Resource>,
}

/// How many Inventory slots a gizmo occupies: one per printed diamond.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "codegen", derive(Bake), databake(path = stonetop::fixed))]
pub enum SlotCount {
    Zero,
    One,
    Two,
}

impl SlotCount {
    /// The printed diamonds: "", "◇", or "◇◇".
    #[must_use]
    pub fn diamonds(self) -> &'static str {
        match self {
            Self::Zero => "",
            Self::One => "◇",
            Self::Two => "◇◇",
        }
    }
}

/// One `{…}` reference to a gizmo in a possession's description or pick entry: `{Candle}`
/// names the gizmo and is spoken as written, `{a lantern|Lantern}` names it after the bar and
/// is spoken as the printed phrase before it. `{resource}` is a resource placeholder, not a
/// reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GizmoReference<'a> {
    /// The printed phrase, when it differs from the name.
    pub phrase: Option<&'a str>,
    /// The gizmo's name, with its material if it has one: "Lantern", "Long spear, fine steel".
    pub target: &'a str,
    /// Where the whole `{…}` sits in the text.
    pub span: std::ops::Range<usize>,
}

impl GizmoReference<'_> {
    /// The words spoken for the reference: the phrase if given, else the target.
    #[must_use]
    pub fn spoken(&self) -> &str {
        self.phrase.unwrap_or(self.target)
    }
}

/// The marker in a description where a resource's value is spoken.
pub const RESOURCE_PLACEHOLDER: &str = "{resource}";

/// Every gizmo reference in `text`, in order of appearance. An unclosed brace is plain text.
#[must_use]
pub fn gizmo_references(text: &str) -> Vec<GizmoReference<'_>> {
    let mut references = Vec::new();
    let mut from = 0;
    while let Some(open) = text[from..].find('{') {
        let start = from + open;
        let Some(close) = text[start..].find('}') else { break };
        let end = start + close + 1;
        let inner = &text[start + 1..end - 1];
        if inner != &RESOURCE_PLACEHOLDER[1..RESOURCE_PLACEHOLDER.len() - 1] {
            let (phrase, target) = match inner.split_once('|') {
                Some((phrase, target)) => (Some(phrase), target),
                None => (None, inner),
            };
            references.push(GizmoReference { phrase, target, span: start..end });
        }
        from = end;
    }
    references
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
    /// A printed sub-heading within the backstory, one level below its name: the Seeker's
    /// Collection has "Major Arcana" and "Minor Arcana".
    Heading(&'static str),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bare_and_phrased_references_are_found_in_order_and_the_placeholder_is_skipped() {
        let text = "{beeswax}, {candles|Candle}, {resource} uses, {a lantern|Lantern} etc.";
        let found = gizmo_references(text);
        assert_eq!(
            found,
            vec![
                GizmoReference { phrase: None, target: "beeswax", span: 0..9 },
                GizmoReference { phrase: Some("candles"), target: "Candle", span: 11..27 },
                GizmoReference { phrase: Some("a lantern"), target: "Lantern", span: 46..65 },
            ]
        );
        assert_eq!(found[0].spoken(), "beeswax");
        assert_eq!(found[1].spoken(), "candles");
        assert_eq!(&text[found[2].span.clone()], "{a lantern|Lantern}");
    }

    #[test]
    fn text_without_references_has_none() {
        assert!(gizmo_references("({resource} uses): each use produces valuables").is_empty());
        assert!(gizmo_references("no braces at all").is_empty());
        assert!(gizmo_references("an unclosed { brace").is_empty());
    }

    #[test]
    fn a_kit_of_one_is_a_slice_of_one() {
        assert_eq!(GizmoKit::One(GizmoKey::SacredPouch).gizmos(), &[GizmoKey::SacredPouch]);
        assert_eq!(GizmoKit::Referenced(&[]).gizmos(), &[] as &[GizmoKey]);
    }
}
