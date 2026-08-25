use regex::Regex;

use crate::playbook::{Background, Backstory, Move, Playbook, SpecialPossession};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NameType {
    pub name: String,
    pub typ: String,
}
pub trait NameKey: std::fmt::Debug {
    fn name_and_type(&self) -> NameType;
    fn name_type_and_debug(&self) -> (NameType, String) {
        (self.name_and_type(), format!("{self:?}"))
    }
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

macro_rules! impl_name_key {
    ($($t:ident),* $(,)?) => {
        $(impl NameKey for $t {
            fn name_and_type(&self) -> NameType {
                NameType {
                    name: enumable(&self.name),
                    typ: stringify!($t).to_string(),
                }
            }
        })*
    };
}

macro_rules! impl_name_key_with_override {
    ($($t:ident),* $(,)?) => {
        $(impl NameKey for $t {
            fn name_and_type(&self) -> NameType {
                let key = self.key.as_ref().unwrap_or(&self.name);
                NameType {
                    name: enumable(key),
                    typ: stringify!($t).to_string(),
                }
            }
        })*
    };
}

impl_name_key!(Playbook, Background, Backstory);
impl_name_key_with_override!(SpecialPossession, Move);
