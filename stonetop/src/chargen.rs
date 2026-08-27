use crate::Die;

#[derive(Debug, Clone)]
pub struct Playbook {
    pub name: String,
    pub description: String,
    pub backgrounds: Vec<Background>,
    pub instinct: Vec<Instinct>,
    pub appearance: Vec<TaggedRow>,
    pub origin_choices: Vec<Origin>,
    pub origin: u8,
    pub stats_to_assign: Vec<i8>, // to be assigned by the player
    pub stats: Vec<i8>,           // as assigned
    pub damage: Die,
    pub hp: u8,
    pub special_possessions: SpecialPossessions,
    pub starting_moves_note: String, // This will be calcuted, eventually
    pub starting_move_choices: u8,
    pub grants_moves: Vec<Grant>,
    pub moves: Vec<Move>,
    pub moves_footnote: Option<String>,
    pub intro: Intro,
    pub backstory: Vec<Backstory>,
}

// Background -------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Background {
    pub name: String,
    pub description: Vec<BackgroundChunk>,
    pub grants_moves: Vec<Grant>,
    pub grants_possession: Option<Grant>,
    pub grants_topic: Option<Grant>,
}

#[derive(Debug, Clone)]
pub enum BackgroundChunk {
    Flavor(String),
    Crunch(String),
    Checklist(BackgroundChecklist),
    Has(Resource),
}

#[derive(Debug, Clone)]
pub struct Resource {
    pub hold: String,
    pub can_be: CanBe,
    pub start: EmptyFull,
}

#[derive(Debug, Clone)]
pub enum EmptyFull {
    Empty,
    Full,
}

#[derive(Debug, Clone)]
pub enum BackgroundChecklist {
    Options(Vec<String>),
    OptionsPrecheckable(Vec<PrecheckableOption>),
    Rows(Vec<TaggedRow>),
}

#[derive(Debug, Clone)]
pub struct PrecheckableOption {
    pub prechecked: bool,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct TaggedRow {
    pub tag: String,
    pub items: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Grant {
    Simply(String),
    ChooseOne(Vec<String>),
}

// Instinct, Origin, Special Possessions ----------------------------------

#[derive(Debug, Clone)]
pub struct Instinct {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct Origin {
    pub location: String,
    pub naming: Naming,
}

#[derive(Debug, Clone)]
pub struct SpecialPossessions {
    pub pick_note: String,
    pub pick_count: u8,
    /// How many of the leading `options` the playbook starts with.
    pub preselected: u8,
    pub options: Vec<SpecialPossession>,
}

#[derive(Debug, Clone)]
pub struct SpecialPossession {
    pub name: String,
    pub key: Option<String>,
    pub description: String,
    pub resource: Option<Resource>,
    pub pick: Vec<String>,
}

// Backstory and Intro ----------------------------------------------------

#[derive(Debug, Clone)]
pub struct Backstory {
    pub name: String,
    pub list: Vec<BackstoryItem>,
}

#[derive(Debug, Clone)]
pub enum BackstoryItem {
    Text { text: String },
    Choices { choices: Vec<String> },
    ChoiceRow(TaggedRow),
}

#[derive(Debug, Clone)]
pub struct Intro {
    pub title: String,
    pub text: String,
}

// Move -------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Move {
    pub name: String,
    pub key: Option<String>,
    pub description: String,
    pub requirement: Option<Requirement>,
    pub max_picks: u8,
    pub resource: Vec<Resource>,
    pub replaces: Option<String>,
    pub checklist: Option<MoveChecklist>,
}

#[derive(Debug, Clone)]
pub enum MoveChecklist {
    Options(Vec<String>),
    OptionsWithLevel(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct Requirement {
    pub level: Option<u8>,
    pub moves: Vec<String>,
    pub playbook: Option<String>,
}

#[derive(Debug, Clone)]
pub enum CanBe {
    Max(u8),
    Labels(Vec<String>),
}

#[derive(Debug, Clone)]
pub enum Naming {
    Instructions(String),
    Names(Vec<String>),
    MixAndMatch(NameParts),
}

#[derive(Debug, Clone)]
pub struct NameParts {
    pub intro: String,
    pub name_parts: Vec<TaggedRow>,
}
