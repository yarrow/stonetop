use serde::{Deserialize, Deserializer, Serialize};
use stonetop::Die;
use stonetop::keys::{MoveKey, PlaybookKey, SpecialPossessionKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Playbook {
    pub name: String,
    pub description: String,
    pub backgrounds: [Background; 3],
    pub instinct: [Instinct; 5],
    pub appearance: [TaggedRow; 4],
    pub origin_choices: Vec<Origin>,
    pub stats_to_assign: [i8; 6],
    pub damage: Die,
    pub hp: u8,
    pub special_possessions: SpecialPossessions,
    pub starting_move_choices: u8,
    pub grants_moves: Vec<GrantMove>,
    pub moves: Vec<Move>,
    pub moves_footnote: Option<String>,
    pub intro: Intro,
    pub backstory: Vec<Backstory>,
}

impl Playbook {
    /// Every Move this playbook offers, its backgrounds' anonymous Moves first, then its own.
    pub fn moves(&self) -> impl Iterator<Item = &Move> {
        self.backgrounds.iter().flat_map(Background::moves).chain(&self.moves)
    }
}

// Background -------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Background {
    pub name: String,
    pub description: Vec<BackgroundChunk>,
    #[serde(default)]
    pub grants_moves: Vec<GrantMove>,
    pub grants_possession: Option<GrantPossession>,
    pub grants_topic: Option<GrantTopic>,
}

impl Background {
    /// The anonymous Moves granted by this Background. (Currently zero or one, unlikely to
    /// change.)
    pub fn moves(&self) -> impl Iterator<Item = &Move> {
        self.description.iter().filter_map(|chunk| match chunk {
            BackgroundChunk::Move(a_move) => Some(a_move),
            _ => None,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BackgroundChunk {
    Flavor(String),
    /// Some backgrounds explicitly grant names Moves. These are also available to other
    /// backgrounds, though not for free.  And some backgrounds grant what are in
    /// effect anonymous Moves, not available to other backgrounds.  The rules don't call
    /// these Moves, but mechanically they are identical. We want to have just one
    /// "From Raised by Wolves" section (for instance), so we sometimes jam what might
    /// more naturally be a list of anonymous Moves into one `Move(Move)`.
    Move(Move),
    Checklist(BackgroundChecklist),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Resource {
    pub hold: String,
    pub can_be: CanBe,
    pub start: EmptyFull,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum EmptyFull {
    Empty,
    Full,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BackgroundChecklist {
    Options(Vec<String>),
    OptionsPrecheckable(Vec<PrecheckableOption>),
    Rows(Vec<TaggedRow>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PrecheckableOption {
    #[serde(default)]
    pub prechecked: bool,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TaggedRow {
    pub tag: String,
    pub items: Vec<String>,
}

/// A Move granted outright, or a choice between two.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum GrantMove {
    Simply(#[serde(deserialize_with = "crate::key::move_key")] MoveKey),
    ChooseOne(#[serde(deserialize_with = "crate::key::move_key_pair")] (MoveKey, MoveKey)),
}

/// A Special Possession granted outright, or a choice between two.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum GrantPossession {
    Simply(#[serde(deserialize_with = "crate::key::possession_key")] SpecialPossessionKey),
    ChooseOne(
        #[serde(deserialize_with = "crate::key::possession_key_pair")]
        (SpecialPossessionKey, SpecialPossessionKey),
    ),
}

/// A topic (for the Seeker's Lore) granted outright, or a choice among several. Topics have
/// no key enum; they are the printed names.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum GrantTopic {
    Simply(String),
    ChooseOne(Vec<String>),
}

// Instinct, Origin, Special Possessions ----------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Instinct {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Origin {
    pub location: String,
    pub naming: Naming,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpecialPossessions {
    pub pick_note: String,
    pub pick_count: u8,
    /// How many of the leading `options` the playbook starts with.
    #[serde(default)]
    pub preselected: u8,
    pub options: Vec<SpecialPossession>,
}

/// A Special Possession. Its `description` and `pick` entries refer to the gizmos of its kit
/// inline, as `{Candle}` or `{a lantern|Lantern}`; a kit of one names its gizmo in `gizmo`
/// instead and has no description of its own.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpecialPossession {
    pub name: String,
    #[serde(default)]
    pub key_prefix: Option<String>,
    #[serde(default)]
    pub description: String,
    pub resource: Option<Resource>,
    #[serde(default)]
    pub pick: Vec<String>,
    /// The one gizmo of a kit of one, as a reference target: "Sacred pouch", "Composite bow".
    pub gizmo: Option<String>,
}

// Gear -------------------------------------------------------------------

/// `gear.json5`: every gizmo, in the printed sections, in printed order.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Gear {
    pub weapons_of_war: Vec<Gizmo>,
    pub bronze_weapons: Vec<Gizmo>,
    pub armor: Vec<Gizmo>,
    pub light_sources: Vec<Gizmo>,
    pub tools_and_trades: Vec<Gizmo>,
    pub writing_implements: Vec<Gizmo>,
    pub exotic_stuff: Vec<Gizmo>,
    pub trade_goods: Vec<Gizmo>,
    pub slotted_items: Vec<Gizmo>,
    pub small_items: Vec<Gizmo>,
    pub kit_contents: Vec<Gizmo>,
}

impl Gear {
    /// The sections in printed order, each with its printed heading.
    #[must_use]
    pub fn sections(&self) -> [(&'static str, &[Gizmo]); 11] {
        [
            ("Weapons of war", &self.weapons_of_war),
            ("Bronze weapons", &self.bronze_weapons),
            ("Armor", &self.armor),
            ("Light sources", &self.light_sources),
            ("Tools & trades", &self.tools_and_trades),
            ("Writing implements", &self.writing_implements),
            ("Exotic stuff", &self.exotic_stuff),
            ("Trade goods", &self.trade_goods),
            ("Slotted items", &self.slotted_items),
            ("Small items", &self.small_items),
            ("Kit contents", &self.kit_contents),
        ]
    }

    /// Every gizmo, section by section in printed order.
    pub fn gizmos(&self) -> impl Iterator<Item = &Gizmo> {
        self.sections().into_iter().flat_map(|(_, gizmos)| gizmos)
    }
}

/// A gizmo as written in `gear.json5`: see the comment at the top of that file for the fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Gizmo {
    pub name: String,
    pub qualifier: Option<String>,
    pub piercing: Option<u8>,
    pub slots: u8,
    /// The gear sheet's price band, 0 to 4. Recorded while the sheet is open; nothing reads it
    /// yet, so it doesn't reach the Fixed type.
    pub value: Option<u8>,
    #[serde(default)]
    pub description: String,
    pub resource: Option<Resource>,
}

// Backstory and Intro ----------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Backstory {
    pub name: String,
    pub list: Vec<BackstoryItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum BackstoryItem {
    Text { text: String },
    Choices { choices: Vec<String> },
    ChoiceRow(TaggedRow),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Intro {
    pub title: String,
    pub text: String,
}

// Move -------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Move {
    pub name: String,
    #[serde(default)]
    pub key_prefix: Option<String>,
    pub description: String,
    #[serde(default)]
    pub requires: Vec<Requirement>,
    #[serde(default = "one")]
    pub max_picks: u8,
    #[serde(default, deserialize_with = "one_or_many")]
    pub resource: Vec<Resource>,
    pub checklist: Option<MoveChecklist>,
}

fn one() -> u8 {
    1
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}
pub fn one_or_many<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let v = OneOrMany::deserialize(deserializer)?;
    Ok(match v {
        OneOrMany::One(val) => vec![val],
        OneOrMany::Many(val) => val,
    })
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MoveChecklist {
    Options(Vec<String>),
    OptionsWithLevel(Vec<String>),
}

/// A condition a character must meet to take a Move. Every `Requirement` in a `Move`'s `requires`
/// field is, er, required. (`NeedsOneOf` requires either of the two `Move`s mentioned to have been
/// already taken).
///
/// Names are written in the json5 as they appear on the referenced item — `"Spirit Tongue"`,
/// not `"SpiritTongue"` — and resolved to keys on the way in (see `key.rs`), so a name that
/// no longer matches any item is a parse error.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Requirement {
    Level(u8),
    Needs(#[serde(deserialize_with = "crate::key::move_key")] MoveKey),
    NeedsOneOf(#[serde(deserialize_with = "crate::key::move_key_pair")] (MoveKey, MoveKey)),
    NeedsStrength,
    NeedsSixInPotentialFG,
    Replaces(#[serde(deserialize_with = "crate::key::move_key")] MoveKey),
    NeedsPlaybook(#[serde(deserialize_with = "crate::key::playbook_key")] PlaybookKey),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CanBe {
    Max(u8),
    Labels(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Naming {
    Instructions(String),
    Names(Vec<String>),
    MixAndMatch(NameParts),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NameParts {
    pub intro: String,
    pub name_parts: Vec<TaggedRow>,
}
