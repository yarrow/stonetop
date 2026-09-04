//! Conversion from the schema types, as parsed from json5, to their `stonetop::fixed`
//! counterparts.
//!
//! The Fixed types have `&'static str` and `&'static [T]` fields, so every owned string and
//! vector is leaked on the way across. This is deliberate: `codegen` is a build tool that runs
//! once, bakes what it built, and exits, so nothing is ever freed anyway.

use stonetop::fixed;

use crate::key::Key;
use crate::schema;

fn leak_str(s: &str) -> &'static str {
    String::from(s).leak()
}

fn leak_vec<T>(v: Vec<T>) -> &'static [T] {
    v.leak()
}

fn leak_strs(v: &[String]) -> &'static [&'static str] {
    leak_vec(v.iter().map(|s| leak_str(s)).collect())
}

impl schema::Move {
    /// This Move as `stonetop` will see it, with its key resolved.
    ///
    /// # Panics
    ///
    /// If the Move's name doesn't resolve to a `MoveKey` (see [`Key::key`]).
    #[must_use]
    pub fn to_fixed(&self) -> fixed::MoveFixed {
        fixed::MoveFixed {
            key: self.key(),
            name: leak_str(&self.name),
            description: leak_str(&self.description),
            requires: leak_vec(self.requires.iter().map(schema::Requirement::to_fixed).collect()),
            max_picks: self.max_picks,
            resource: leak_vec(self.resource.iter().map(schema::Resource::to_fixed).collect()),
            checklist: self.checklist.as_ref().map(schema::MoveChecklist::to_fixed),
        }
    }
}

impl schema::Requirement {
    #[must_use]
    pub fn to_fixed(&self) -> fixed::Requirement {
        use schema::Requirement as S;
        match self {
            S::Level(level) => fixed::Requirement::Level(*level),
            S::Needs(key) => fixed::Requirement::Needs(*key),
            S::NeedsOneOf((first, second)) => fixed::Requirement::NeedsOneOf(*first, *second),
            S::NeedsStrength => fixed::Requirement::NeedsStrength,
            S::NeedsSixInPotentialFG => fixed::Requirement::NeedsSixInPotentialFG,
            S::Replaces(key) => fixed::Requirement::Replaces(*key),
            S::NeedsPlaybook(key) => fixed::Requirement::NeedsPlaybook(*key),
        }
    }
}

impl schema::MoveChecklist {
    #[must_use]
    pub fn to_fixed(&self) -> fixed::MoveChecklist {
        match self {
            Self::Options(options) => fixed::MoveChecklist::Options(leak_strs(options)),
            Self::OptionsWithLevel(options) => {
                fixed::MoveChecklist::OptionsWithLevel(leak_strs(options))
            }
        }
    }
}

impl schema::Resource {
    #[must_use]
    pub fn to_fixed(&self) -> fixed::Resource {
        fixed::Resource {
            hold: leak_str(&self.hold),
            can_be: self.can_be.to_fixed(),
            start: self.start.to_fixed(),
        }
    }
}

impl schema::CanBe {
    #[must_use]
    pub fn to_fixed(&self) -> fixed::CanBe {
        match self {
            Self::Max(max) => fixed::CanBe::Max(*max),
            Self::Labels(labels) => fixed::CanBe::Labels(leak_strs(labels)),
        }
    }
}

impl schema::EmptyFull {
    #[must_use]
    pub fn to_fixed(&self) -> fixed::EmptyFull {
        match self {
            Self::Empty => fixed::EmptyFull::Empty,
            Self::Full => fixed::EmptyFull::Full,
        }
    }
}

// Background -------------------------------------------------------------

impl schema::Background {
    /// This Background as `stonetop` will see it, with its key resolved. Its anonymous Moves
    /// are freshly converted here too: equal by value to, though a different leaked instance
    /// from, the Move statics `codegen::bake` shares with the rest of the playbook.
    ///
    /// # Panics
    ///
    /// If the Background's name doesn't resolve to a `BackgroundKey` (see [`Key::key`]).
    #[must_use]
    pub fn to_fixed(&self) -> fixed::BackgroundFixed {
        fixed::BackgroundFixed {
            key: self.key(),
            name: leak_str(&self.name),
            description: leak_vec(
                self.description.iter().map(schema::BackgroundChunk::to_fixed).collect(),
            ),
            grants_moves: leak_vec(self.grants_moves.iter().map(schema::Grant::to_fixed).collect()),
            grants_possession: self.grants_possession.as_ref().map(schema::Grant::to_fixed),
            grants_topic: self.grants_topic.as_ref().map(schema::Grant::to_fixed),
        }
    }
}

impl schema::BackgroundChunk {
    #[must_use]
    pub fn to_fixed(&self) -> fixed::BackgroundChunk {
        match self {
            Self::Flavor(text) => fixed::BackgroundChunk::Flavor(leak_str(text)),
            // Leaked to get a `'static` reference; equal by value to, not the same instance
            // as, the Move's own static (see `codegen::bake::bake_background_chunk`).
            Self::Move(a_move) => {
                fixed::BackgroundChunk::Move(Box::leak(Box::new(a_move.to_fixed())))
            }
            Self::Checklist(checklist) => fixed::BackgroundChunk::Checklist(checklist.to_fixed()),
        }
    }
}

impl schema::BackgroundChecklist {
    #[must_use]
    pub fn to_fixed(&self) -> fixed::BackgroundChecklist {
        match self {
            Self::Options(options) => fixed::BackgroundChecklist::Options(leak_strs(options)),
            Self::OptionsPrecheckable(options) => fixed::BackgroundChecklist::OptionsPrecheckable(
                leak_vec(options.iter().map(schema::PrecheckableOption::to_fixed).collect()),
            ),
            Self::Rows(rows) => fixed::BackgroundChecklist::Rows(leak_vec(
                rows.iter().map(schema::TaggedRow::to_fixed).collect(),
            )),
        }
    }
}

impl schema::PrecheckableOption {
    #[must_use]
    pub fn to_fixed(&self) -> fixed::PrecheckableOption {
        fixed::PrecheckableOption { prechecked: self.prechecked, text: leak_str(&self.text) }
    }
}

impl schema::TaggedRow {
    #[must_use]
    pub fn to_fixed(&self) -> fixed::TaggedRow {
        fixed::TaggedRow { tag: leak_str(&self.tag), items: leak_strs(&self.items) }
    }
}

impl schema::Grant {
    #[must_use]
    pub fn to_fixed(&self) -> fixed::Grant {
        match self {
            Self::Simply(s) => fixed::Grant::Simply(leak_str(s)),
            Self::ChooseOne(options) => fixed::Grant::ChooseOne(leak_strs(options)),
        }
    }
}

// SpecialPossession ------------------------------------------------------

impl schema::SpecialPossession {
    /// This Special Possession as `stonetop` will see it, with its key resolved.
    ///
    /// # Panics
    ///
    /// If the Special Possession's name doesn't resolve to a `SpecialPossessionKey` (see
    /// [`Key::key`]).
    #[must_use]
    pub fn to_fixed(&self) -> fixed::SpecialPossessionFixed {
        fixed::SpecialPossessionFixed {
            key: self.key(),
            name: leak_str(&self.name),
            description: leak_str(&self.description),
            resource: self.resource.as_ref().map(schema::Resource::to_fixed),
            pick: leak_strs(&self.pick),
        }
    }
}

// Backstory --------------------------------------------------------------

impl schema::Backstory {
    /// This Backstory as `stonetop` will see it, with its key resolved.
    ///
    /// # Panics
    ///
    /// If the Backstory's name doesn't resolve to a `BackstoryKey` (see [`Key::key`]).
    #[must_use]
    pub fn to_fixed(&self) -> fixed::BackstoryFixed {
        fixed::BackstoryFixed {
            key: self.key(),
            name: leak_str(&self.name),
            list: leak_vec(self.list.iter().map(schema::BackstoryItem::to_fixed).collect()),
        }
    }
}

impl schema::BackstoryItem {
    #[must_use]
    pub fn to_fixed(&self) -> fixed::BackstoryItem {
        match self {
            Self::Text { text } => fixed::BackstoryItem::Text(leak_str(text)),
            Self::Choices { choices } => fixed::BackstoryItem::Choices(leak_strs(choices)),
            Self::ChoiceRow(row) => fixed::BackstoryItem::ChoiceRow(row.to_fixed()),
        }
    }
}

#[cfg(test)]
mod test {
    use stonetop::fixed::{CanBe, EmptyFull, MoveChecklist, MoveFixed, Requirement, Resource};
    use stonetop::keys::MoveKey;

    use crate::schema::Move;

    fn parse(json5_source: &str) -> Move {
        json5::from_str(json5_source).unwrap_or_else(|e| panic!("{e:#}"))
    }

    #[test]
    fn defaults_come_across_as_empty() {
        let fixed = parse(r#"{ name: "Payback", description: "<p>Deal +1d4.</p>" }"#).to_fixed();
        assert_eq!(
            fixed,
            MoveFixed {
                key: MoveKey::Payback,
                name: "Payback",
                description: "<p>Deal +1d4.</p>",
                requires: &[],
                max_picks: 1,
                resource: &[],
                checklist: None,
            }
        );
    }

    #[test]
    fn every_field_comes_across() {
        let fixed = parse(
            r#"{
                name: "Superior Stat",
                keyPrefix: "Pw",
                description: "<p>Hit things.</p>",
                requires: [{ level: 2 }, { needsOneOf: ["Wild Speech", "Spirit Tongue"] }],
                maxPicks: 2,
                resource: { hold: "Keep fighting", canBe: 5, start: "empty" },
                checklist: { optionsWithLevel: ["one", "two"] },
            }"#,
        )
        .to_fixed();
        assert_eq!(
            fixed,
            MoveFixed {
                key: MoveKey::PwSuperiorStat,
                name: "Superior Stat",
                description: "<p>Hit things.</p>",
                requires: &[
                    Requirement::Level(2),
                    Requirement::NeedsOneOf(MoveKey::WildSpeech, MoveKey::SpiritTongue)
                ],
                max_picks: 2,
                resource: &[Resource {
                    hold: "Keep fighting",
                    can_be: CanBe::Max(5),
                    start: EmptyFull::Empty
                }],
                checklist: Some(MoveChecklist::OptionsWithLevel(&["one", "two"])),
            }
        );
    }

    #[test]
    fn labelled_resources_keep_their_labels() {
        let fixed = parse(
            r#"{
                name: "Payback",
                description: "",
                resource: [{ hold: "Marks", canBe: ["a", "b"], start: "full" }],
            }"#,
        )
        .to_fixed();
        assert_eq!(fixed.resource[0].can_be, CanBe::Labels(&["a", "b"]));
        assert_eq!(fixed.resource[0].start, EmptyFull::Full);
    }
}

#[cfg(test)]
mod background_test {
    use stonetop::fixed::{
        BackgroundChecklist, BackgroundChunk, Grant, PrecheckableOption, TaggedRow,
    };
    use stonetop::keys::{BackgroundKey, MoveKey};

    use crate::schema::Background;

    fn parse(json5_source: &str) -> Background {
        json5::from_str(json5_source).unwrap_or_else(|e| panic!("{e:#}"))
    }

    #[test]
    fn every_chunk_kind_and_a_simple_grant_come_across() {
        let fixed = parse(
            r#"{
                name: "Raised by Wolves",
                description: [
                    { flavor: "<p>flavor</p>" },
                    { move: { name: "From Raised by Wolves", description: "<p>granted move</p>" } },
                    { checklist: { options: ["a", "b"] } },
                ],
                grantsMoves: ["Trackless Step"],
                grantsPossession: "Sacred Pouch",
            }"#,
        )
        .to_fixed();
        assert_eq!(fixed.key, BackgroundKey::RaisedByWolves);
        assert_eq!(fixed.name, "Raised by Wolves");
        assert_eq!(fixed.description[0], BackgroundChunk::Flavor("<p>flavor</p>"));
        let BackgroundChunk::Move(a_move) = &fixed.description[1] else {
            panic!("expected a Move chunk")
        };
        assert_eq!(a_move.key, MoveKey::FromRaisedByWolves);
        assert_eq!(a_move.description, "<p>granted move</p>");
        assert_eq!(
            fixed.description[2],
            BackgroundChunk::Checklist(BackgroundChecklist::Options(&["a", "b"]))
        );
        assert_eq!(fixed.grants_moves, &[Grant::Simply("Trackless Step")]);
        assert_eq!(fixed.grants_possession, Some(Grant::Simply("Sacred Pouch")));
        assert_eq!(fixed.grants_topic, None);
    }

    #[test]
    fn precheckable_and_row_checklists_and_a_choose_one_grant_come_across() {
        let fixed = parse(
            r#"{
                name: "Vessel",
                description: [
                    { checklist: { optionsPrecheckable: [
                        { prechecked: true, text: "one" },
                        { text: "two" },
                    ] } },
                    { checklist: { rows: [{ tag: "Tag", items: ["x", "y"] }] } },
                ],
                grantsTopic: ["a", "b"],
            }"#,
        )
        .to_fixed();
        assert_eq!(fixed.key, BackgroundKey::Vessel);
        assert_eq!(
            fixed.description[0],
            BackgroundChunk::Checklist(BackgroundChecklist::OptionsPrecheckable(&[
                PrecheckableOption { prechecked: true, text: "one" },
                PrecheckableOption { prechecked: false, text: "two" },
            ]))
        );
        assert_eq!(
            fixed.description[1],
            BackgroundChunk::Checklist(BackgroundChecklist::Rows(&[TaggedRow {
                tag: "Tag",
                items: &["x", "y"]
            }]))
        );
        assert_eq!(fixed.grants_topic, Some(Grant::ChooseOne(&["a", "b"])));
    }
}

#[cfg(test)]
mod special_possession_test {
    use stonetop::fixed::{CanBe, EmptyFull, Resource, SpecialPossessionFixed};
    use stonetop::keys::SpecialPossessionKey;

    use crate::schema::SpecialPossession;

    fn parse(json5_source: &str) -> SpecialPossession {
        json5::from_str(json5_source).unwrap_or_else(|e| panic!("{e:#}"))
    }

    #[test]
    fn defaults_come_across_as_empty() {
        let fixed =
            parse(r#"{ name: "Sacred pouch", description: "<p>Carries things.</p>" }"#).to_fixed();
        assert_eq!(
            fixed,
            SpecialPossessionFixed {
                key: SpecialPossessionKey::SacredPouch,
                name: "Sacred pouch",
                description: "<p>Carries things.</p>",
                resource: None,
                pick: &[],
            }
        );
    }

    #[test]
    fn key_prefix_and_resource_and_pick_come_across() {
        let fixed = parse(
            r#"{
                name: "Weapons of War",
                keyPrefix: "Ph",
                description: "<p>Big weapons.</p>",
                resource: { hold: "Ammo", canBe: 3, start: "full" },
                pick: ["sword", "axe"],
            }"#,
        )
        .to_fixed();
        assert_eq!(fixed.key, SpecialPossessionKey::PhWeaponsOfWar);
        assert_eq!(
            fixed.resource,
            Some(Resource { hold: "Ammo", can_be: CanBe::Max(3), start: EmptyFull::Full })
        );
        assert_eq!(fixed.pick, &["sword", "axe"]);
    }
}

#[cfg(test)]
mod backstory_test {
    use stonetop::fixed::{BackstoryItem, TaggedRow};
    use stonetop::keys::BackstoryKey;

    use crate::schema::Backstory;

    fn parse(json5_source: &str) -> Backstory {
        json5::from_str(json5_source).unwrap_or_else(|e| panic!("{e:#}"))
    }

    #[test]
    fn every_item_kind_comes_across() {
        let fixed = parse(
            r#"{
                name: "Your Sacred Pouch",
                list: [
                    { text: "Some text." },
                    { choices: ["a", "b"] },
                    { tag: "Tag", items: ["x", "y"] },
                ],
            }"#,
        )
        .to_fixed();
        assert_eq!(fixed.key, BackstoryKey::YourSacredPouch);
        assert_eq!(fixed.list[0], BackstoryItem::Text("Some text."));
        assert_eq!(fixed.list[1], BackstoryItem::Choices(&["a", "b"]));
        assert_eq!(
            fixed.list[2],
            BackstoryItem::ChoiceRow(TaggedRow { tag: "Tag", items: &["x", "y"] })
        );
    }
}
