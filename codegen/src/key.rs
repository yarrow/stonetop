use regex::Regex;

use crate::schema::{Background, Backstory, Move, Playbook, SpecialPossession};

/// A playbook item with a unique key shaped like a Rust enum variant.
pub trait Key {
    fn key(&self) -> String;
}

fn enumable(phrase: &str) -> String {
    let unquoted = phrase.replace('\'', "");
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
            fn key(&self) -> String {
                enumable(&self.name)
            }
        })*
    };
}

macro_rules! impl_key_with_prefix {
    ($($t:ident),* $(,)?) => {
        $(impl Key for $t {
            fn key(&self) -> String {
                let prefix = self.key_prefix.as_deref().unwrap_or("");
                format!("{prefix}{}", enumable(&self.name))
            }
        })*
    };
}

impl_key!(Playbook, Background, Backstory);
impl_key_with_prefix!(SpecialPossession, Move);
