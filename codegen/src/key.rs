use std::any::type_name;
use std::fmt::{Debug, Write as _};
use std::str::FromStr;

use regex::Regex;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use stonetop::keys::{
    BackgroundKey, BackstoryKey, GizmoKey, MoveKey, PlaybookKey, SpecialPossessionKey,
};

use crate::schema::{Background, Backstory, Gizmo, Move, Playbook, SpecialPossession};

/// A playbook, item, or gizmo whose name resolves to a variant of one of the key enums in
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

/// Shape `phrase` like a variant identifier: HTML tags and parenthesised materials are
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

/// Deserialize a Special Possession's name, as written in a `grantsPossession`, into its
/// `SpecialPossessionKey`.
///
/// # Errors
///
/// If the name doesn't resolve to a `SpecialPossessionKey` variant.
pub fn possession_key<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<SpecialPossessionKey, D::Error> {
    key_from_name(&String::deserialize(deserializer)?)
}

/// Deserialize a pair of Special Possession names into their `SpecialPossessionKey`s.
///
/// # Errors
///
/// If either name doesn't resolve to a `SpecialPossessionKey` variant.
pub fn possession_key_pair<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<(SpecialPossessionKey, SpecialPossessionKey), D::Error> {
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

impl Key for Gizmo {
    type Key = GizmoKey;

    /// The name, then the material unless it is iron, then `PiercingN` for the starred
    /// upgrade: `Battleaxe`, `BattleaxePiercing1`, `BattleaxeBronze`, `LongSpearFineSteel`.
    fn variant_name(&self) -> String {
        let mut variant = enumable(&self.name);
        if let Some(material) = self.material.as_deref().filter(|q| !q.eq_ignore_ascii_case("iron"))
        {
            variant.push_str(&enumable(material));
        }
        if let Some(piercing) = self.piercing {
            let _ = write!(variant, "Piercing{piercing}");
        }
        variant
    }
}

/// The key a gizmo reference's target resolves to. A target is the gizmo's name with its
/// material if it has one, so it shapes to the variant the same way the gizmo's own name does:
/// `Lantern`, `Long spear, fine steel`, `Cuirass, boiled leather`.
///
/// # Errors
///
/// If no variant is named `enumable(target)`.
pub fn gizmo_key(target: &str) -> Result<GizmoKey, String> {
    let variant = enumable(target);
    GizmoKey::from_str(&variant).map_err(|_| {
        format!("`{{{target}}}` shapes to `{variant}`, which is not a GizmoKey: check gear.json5 and stonetop/src/keys.rs")
    })
}

impl_key!(Background => BackgroundKey, Backstory => BackstoryKey);
impl_key_with_prefix!(SpecialPossession => SpecialPossessionKey, Move => MoveKey);

#[cfg(test)]
mod test {
    use stonetop::keys::GizmoKey;

    use super::{Key, enumable, gizmo_key};
    use crate::schema::Gizmo;

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

    fn gizmo(json5_source: &str) -> Gizmo {
        json5::from_str(json5_source).unwrap_or_else(|e| panic!("{e:#}"))
    }

    #[test]
    fn a_gizmos_variant_drops_iron_and_keeps_other_materials_and_the_piercing_upgrade() {
        assert_eq!(
            gizmo(r#"{ name: "Spear", material: "iron", slots: 1 }"#).key(),
            GizmoKey::Spear
        );
        assert_eq!(
            gizmo(r#"{ name: "Battleaxe", material: "bronze", piercing: 2, slots: 1 }"#).key(),
            GizmoKey::BattleaxeBronzePiercing2
        );
        assert_eq!(
            gizmo(r#"{ name: "Empty book", material: "fine vellum", slots: 1 }"#).key(),
            GizmoKey::EmptyBookFineVellum
        );
        assert_eq!(gizmo(r#"{ name: "Block & tackle", slots: 1 }"#).key(), GizmoKey::BlockTackle);
    }

    #[test]
    fn a_reference_target_resolves_like_a_name_with_its_material() {
        assert_eq!(gizmo_key("Lantern"), Ok(GizmoKey::Lantern));
        assert_eq!(gizmo_key("Long spear, fine steel"), Ok(GizmoKey::LongSpearFineSteel));
        assert_eq!(gizmo_key("Sword"), Ok(GizmoKey::Sword));
        assert!(gizmo_key("Sword, iron").unwrap_err().contains("SwordIron"));
    }
}
