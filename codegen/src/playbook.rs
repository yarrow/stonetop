use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Playbook {
    pub name: String,
    pub description: String,
    pub backgrounds: Vec<Background>,
    pub instinct: Vec<Instinct>,
    pub appearance: Vec<TaggedRow>,
    pub origin: Vec<Origin>,
    pub stats_to_assign: Vec<i8>,
    pub damage: String,
    pub hp: u8,
    pub special_possessions: SpecialPossessions,
    pub starting_moves_note: String,
    pub starting_move_choices: u8,
    pub grants_moves: Vec<Grant>,
    pub moves: Vec<Move>,
    pub moves_footnote: Option<String>,
    pub intro: Intro,
    pub backstory: Vec<Backstory>,
}

// Background -------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Background {
    pub name: String,
    pub description: Vec<BackgroundChunk>,
    #[serde(default)]
    pub grants_moves: Vec<Grant>,
    pub grants_possession: Option<Grant>,
    pub grants_topic: Option<Grant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BackgroundChunk {
    Flavor(String),
    Crunch(String),
    Checklist(BackgroundChecklist),
    Has(Resource),
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Grant {
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
    pub key: Option<String>,
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
    pub key: Option<String>,
    pub description: String,
    pub requirement: Option<Requirement>,
    #[serde(default = "one")]
    pub max_picks: u8,
    #[serde(default, deserialize_with = "one_or_many")]
    pub resource: Vec<Resource>,
    pub replaces: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Requirement {
    pub level: Option<u8>,
    #[serde(default)]
    pub moves: Vec<String>,
    pub playbook: Option<String>,
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
