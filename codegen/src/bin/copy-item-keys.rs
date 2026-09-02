//! Write the client crate's copy of `item_keys.rs`: `codegen/src/item_keys.rs` with the
//! strum derives stripped, into `stonetop/src/item_keys.rs`. Normally run as
//! `cargo xtask copy-keys`, which also checks the result with rustfmt.
//!
//! Today the copy is a strip, because the client is the only consumer of the stonetop
//! copy. If stonetop becomes a single Leptos crate compiled for both server and client,
//! this becomes a rewrite instead: the server-only derives move behind
//! `#[cfg_attr(feature = "ssr", derive(...))]` rather than disappearing.

use std::path::Path;

use anyhow::{Context, Result};

use codegen::client_copy::client_copy_of_item_keys;

fn main() -> Result<()> {
    let copy = client_copy_of_item_keys()?;
    let output = Path::new(env!("CARGO_MANIFEST_DIR")).join("../stonetop/src/item_keys.rs");
    std::fs::write(&output, copy).with_context(|| format!("writing {}", output.display()))?;
    println!("wrote {}", output.display());
    Ok(())
}
