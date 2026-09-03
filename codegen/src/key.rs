use std::any::type_name;
use std::fmt::Debug;
use std::str::FromStr;

use regex::Regex;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use stonetop::keys::{BackgroundKey, BackstoryKey, MoveKey, PlaybookKey, SpecialPossessionKey};

use crate::schema::{Background, Backstory, Move, Playbook, SpecialPossession};

/// A playbook or item whose name resolves to a variant of one of the key enums in
/// `stonetop/src/keys.rs`.
pub trait Key {
    type Key: FromStr + Debug;

    /// The item's name (with its `keyPrefix`, if any) shaped like a variant identifier.
    fn variant_name(&self) -> String;

    /// The item's key.
    ///
    /// # Panics
    ///
    /// If no variant is named `variant_name()`: the name in the json5 has changed, or a
    /// new item has been added, and `stonetop/src/keys.rs` has not been updated to match.
    fn key(&self) -> Self::Key {
        let name = self.variant_name();
        Self::Key::from_str(&name).unwrap_or_else(|_| {
            panic!(
                "no variant `{name}` in {}: add it to stonetop/src/keys.rs",
                type_name::<Self::Key>()
            )
        })
    }
}

/// Shape `phrase` like a variant identifier: HTML tags and parenthesised qualifiers are
/// dropped, apostrophes are removed, and the remaining words are run together in
/// `UpperCamelCase`.
fn enumable(phrase: &str) -> String {
    let untagged = Regex::new(r"<[^>]*>").unwrap().replace_all(phrase, "");
    let unqualified = Regex::new(r"\([^)]*\)").unwrap().replace_all(&untagged, "");
    let unquoted = unqualified.replace('\'', "");
    let spaced = Regex::new(r"\W").unwrap().replace_all(&unquoted, " ");
    let mut variant = String::with_capacity(spaced.len());
    let mut upper_next = true;
    for c in spaced.chars() {
        if c == ' ' {
            upper_next = true;
        } else if upper_next {
            variant.extend(c.to_uppercase());
            upper_next = false;
        } else {
            variant.push(c);
        }
    }
    variant
}

/// The key a name resolves to, for the names that appear in a `requirement` rather than as an
/// item's own `name`.
///
/// # Errors
///
/// If no variant is named `enumable(name)`.
fn key_from_name<K: FromStr, E: Error>(name: &str) -> Result<K, E> {
    let variant = enumable(name);
    K::from_str(&variant).map_err(|_| {
        E::custom(format!(
            "`{name}` shapes to `{variant}`, which is not a variant of {}: add it to stonetop/src/keys.rs",
            type_name::<K>()
        ))
    })
}

/// Deserialize a Move's name, as written in a `requirement`, into its `MoveKey`.
///
/// # Errors
///
/// If the name doesn't resolve to a `MoveKey` variant.
pub fn move_key<'de, D: Deserializer<'de>>(deserializer: D) -> Result<MoveKey, D::Error> {
    key_from_name(&String::deserialize(deserializer)?)
}

/// Deserialize a pair of Move names into their `MoveKey`s.
///
/// # Errors
///
/// If either name doesn't resolve to a `MoveKey` variant.
pub fn move_key_pair<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<(MoveKey, MoveKey), D::Error> {
    let (first, second) = <(String, String)>::deserialize(deserializer)?;
    Ok((key_from_name(&first)?, key_from_name(&second)?))
}

/// Deserialize a playbook's name, as written in the json5, into its `PlaybookKey`.
///
/// # Errors
///
/// If the name doesn't resolve to a `PlaybookKey` variant.
pub fn playbook_key<'de, D: Deserializer<'de>>(deserializer: D) -> Result<PlaybookKey, D::Error> {
    key_from_name(&String::deserialize(deserializer)?)
}

macro_rules! impl_key {
    ($($t:ident => $k:ident),* $(,)?) => {
        $(impl Key for $t {
            type Key = $k;
            fn variant_name(&self) -> String {
                enumable(&self.name)
            }
        })*
    };
}

macro_rules! impl_key_with_prefix {
    ($($t:ident => $k:ident),* $(,)?) => {
        $(impl Key for $t {
            type Key = $k;
            fn variant_name(&self) -> String {
                let prefix = self.key_prefix.as_deref().unwrap_or("");
                format!("{prefix}{}", enumable(&self.name))
            }
        })*
    };
}

impl Key for Playbook {
    type Key = PlaybookKey;
    fn variant_name(&self) -> String {
        enumable(&self.name)
    }
}

impl_key!(Background => BackgroundKey, Backstory => BackstoryKey);
impl_key_with_prefix!(SpecialPossession => SpecialPossessionKey, Move => MoveKey);

#[cfg(test)]
mod test {
    use super::enumable;

    #[test]
    fn words_run_together() {
        assert_eq!(enumable("Raised by Wolves"), "RaisedByWolves");
    }

    #[test]
    fn apostrophes_vanish_and_punctuation_splits() {
        assert_eq!(enumable("Fear & Anger"), "FearAnger");
        assert_eq!(enumable("Danu's Grace"), "DanusGrace");
    }

    #[test]
    fn tags_and_parenthesised_qualifiers_are_dropped() {
        assert_eq!(enumable("Sacred pouch (<em>magical</em>)"), "SacredPouch");
    }
}
