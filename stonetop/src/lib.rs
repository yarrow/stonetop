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

// The three features are three builds, and no two of them go together. `codegen` derives the
// strum tables and the bake machinery; `ssr` is the server, with the baked content behind it;
// `hydrate` is the WASM half, which must carry neither. `ssr` and `hydrate` together compile
// and then misbehave in silence — the islands run during server rendering, outside any
// reactive tracking context — so the combination is refused here rather than left to be
// noticed. This is also why the hooks name feature sets instead of `--all-features`.
#[cfg(all(feature = "hydrate", feature = "ssr"))]
compile_error!("`ssr` and `hydrate` are the server and the client: build one or the other");
#[cfg(all(feature = "hydrate", feature = "codegen"))]
compile_error!("`codegen` bakes content and derives the strum tables: it never goes to the WASM");

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
#[cfg(feature = "ssr")]
pub mod web;

/// The WASM half's entry point, called by the script `HydrationScripts` puts in every
/// document. Islands mode means it hydrates the controls the server marked as islands and
/// leaves the rest of the document exactly as the server wrote it, so the client never needs
/// the baked content the document was walked out of. There are no controls yet: this is the
/// client half proving it is real and empty.
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}
