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
