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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpecialPossession {
    pub name: String,
    #[serde(default)]
    pub key_prefix: Option<String>,
    pub description: String,
    pub resource: Option<Resource>,
    #[serde(default)]
    pub pick: Vec<String>,
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
