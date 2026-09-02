use std::fmt;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
#[derive(Clone, Copy, Display, EnumString, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Die {
    D4,
    D6,
    D8,
    D10,
    D12,
}
impl Die {
    #[must_use]
    pub fn step_up(&self) -> Self {
        use Die::*;
        match *self {
            D4 => D6,
            D6 => D8,
            D8 => D10,
            D10 => D12,
            D12 => D12,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn upstep() {
        assert_eq!(Die::D4.step_up(), Die::D6);
    }

    #[test]
    fn to_string() {
        assert_eq!(Die::D4.to_string(), "d4");
    }

    #[test]
    fn from_string() {
        assert_eq!(Die::from_str("d4"), Ok(Die::D4));
    }

    #[test]
    fn no_uppercase() {
        assert!(Die::from_str("D4").is_err());
    }

    #[test]
    fn compare() {
        assert!(Die::D6 < Die::D10);
    }
}
