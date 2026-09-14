#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_macros, unused_mut, unused_variables)
)]

// The baked file in `fixed/generated.rs` names this crate's types by the paths their `Bake`
// derives give, `stonetop::fixed::…` and `stonetop::keys::…`, so that the same expressions
// would be valid from any crate that depends on `stonetop`. This alias lets the crate reach
// itself by that name.
#[cfg(feature = "ssr")]
extern crate self as stonetop;

//use anyhow::{Context, Result};
pub mod admin;
pub mod chargen;
pub mod die;
pub use die::Die;
pub mod fixed;
pub mod keys;
pub mod level_up;
pub mod play;
#[cfg(feature = "ssr")]
pub mod render;
pub mod state;
