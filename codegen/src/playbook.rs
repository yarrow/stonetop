use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playbook {
    pub appearance: Vec<Appearance>,
    pub backgrounds: Vec<Background>,
    pub backstory: Vec<Backstory>,
    pub damage: String,
    pub description: String,
    pub grants_moves: Vec<GrantsElement>,
    pub hp: i64,
    pub instinct: Vec<Instinct>,
    pub intro: Intro,
    pub moves: Vec<Move>,
    pub moves_footnote: Option<String>,
    pub name: String,
    pub origin: Vec<Origin>,
    pub special_possessions: SpecialPossessions,
    pub starting_move_choices: i64,
    pub starting_moves_note: String,
    pub stat_modifiers: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Appearance {
    pub items: Vec<String>,
    pub tag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Background {
    pub description: Vec<Description>,
    pub grants_moves: Option<Vec<GrantsElement>>,
    pub grants_possession: Option<PurpleGrants>,
    pub grants_topic: Option<PurpleGrants>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Description {
    pub checklist: Option<DescriptionChecklist>,
    pub crunch: Option<String>,
    pub flavor: Option<String>,
    pub has: Option<Has>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescriptionChecklist {
    pub options: Option<Vec<String>>,
    pub options_precheckable: Option<Vec<OptionsPrecheckable>>,
    pub rows: Option<Vec<Appearance>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsPrecheckable {
    pub prechecked: Option<bool>,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Has {
    pub can_be: i64,
    pub name: String,
    pub start: Start,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Start {
    Empty,
    Full,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GrantsElement {
    PurpleString(String),
    StringArray(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PurpleGrants {
    PurpleString(String),
    StringArray(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Backstory {
    pub list: Vec<List>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List {
    pub choices: Option<Vec<String>>,
    pub items: Option<Vec<String>>,
    pub tag: Option<String>,
    pub text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instinct {
    pub description: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intro {
    pub name: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Move {
    pub checklist: Option<MoveChecklist>,
    pub description: String,
    pub max_picks: Option<i64>,
    pub name: String,
    pub replaces: Option<String>,
    pub requirement: Option<Requirement>,
    pub resource: Option<ResourceUnion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveChecklist {
    pub options: Option<Vec<String>>,
    pub options_with_level: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Requirement {
    pub level: Option<i64>,
    pub moves: Option<Vec<String>>,
    pub playbook: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResourceUnion {
    HasArray(Vec<Has>),
    ResourceClass(ResourceClass),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClass {
    pub can_be: CanBe,
    pub name: String,
    pub start: Start,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CanBe {
    Integer(i64),
    StringArray(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Origin {
    pub location: String,
    pub naming: NamingUnion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NamingUnion {
    NamingClass(NamingClass),
    PurpleString(String),
    StringArray(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamingClass {
    pub intro: String,
    pub name_parts: Vec<Appearance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecialPossessions {
    pub options: Vec<OptionElement>,
    pub pick_count: i64,
    pub pick_note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionElement {
    pub description: String,
    pub name: String,
    pub pick: Option<Vec<String>>,
    pub preselected: Option<bool>,
    pub resource: Option<ResourceClass>,
}
