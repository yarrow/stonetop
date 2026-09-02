use std::any::type_name;
use std::fmt::Debug;
use std::str::FromStr;

use regex::Regex;

use crate::item_keys::{ItemKey, PlaybookKey};
use crate::schema::{Background, Backstory, Move, Playbook, SpecialPossession};

/// A playbook or item whose name resolves to a variant of one of the key enums in
/// `item_keys.rs`.
pub trait Key {
    type Key: FromStr + Debug;

    /// The item's name (with its `keyPrefix`, if any) shaped like a variant identifier.
    fn variant_name(&self) -> String;

    /// The item's key.
    ///
    /// # Panics
    ///
    /// If no variant is named `variant_name()`: the name in the json5 has changed, or a
    /// new item has been added, and `item_keys.rs` has not been updated to match.
    fn key(&self) -> Self::Key {
        let name = self.variant_name();
        Self::Key::from_str(&name).unwrap_or_else(|_| {
            panic!(
                "no variant `{name}` in {}: add it to codegen/src/item_keys.rs",
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

macro_rules! impl_key {
    ($($t:ident),* $(,)?) => {
        $(impl Key for $t {
            type Key = ItemKey;
            fn variant_name(&self) -> String {
                enumable(&self.name)
            }
        })*
    };
}

macro_rules! impl_key_with_prefix {
    ($($t:ident),* $(,)?) => {
        $(impl Key for $t {
            type Key = ItemKey;
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

impl_key!(Background, Backstory);
impl_key_with_prefix!(SpecialPossession, Move);

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
