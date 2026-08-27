#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_macros, unused_mut, unused_variables)
)]

//use anyhow::{Context, Result};
pub mod chargen;
pub mod item_keys;
pub mod level_up;
pub mod play;

use std::fmt;
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
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

impl fmt::Display for Die {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Die::*;
        let die = match *self {
            D4 => "d4",
            D6 => "d6",
            D8 => "d8",
            D10 => "d10",
            D12 => "d12",
        };
        write!(f, "{die}")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn upstep() {
        assert_eq!(Die::D4.step_up().to_string(), "d6");
    }

    #[test]
    fn compare() {
        assert!(Die::D6 < Die::D10);
    }
}
