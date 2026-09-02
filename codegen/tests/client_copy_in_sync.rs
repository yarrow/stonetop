//! `stonetop/src/item_keys.rs` is exactly what `cargo xtask copy-keys` would write from
//! `codegen/src/item_keys.rs`. A hand edit to either side goes red here.

use std::path::Path;

use codegen::client_copy::client_copy_of_item_keys;

#[test]
fn client_copy_in_sync() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../stonetop/src/item_keys.rs");
    let on_disk = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("reading {}: {e}", path.display()));
    let expected = client_copy_of_item_keys().unwrap_or_else(|e| panic!("{e:#}"));
    assert!(on_disk == expected, "{} is out of date: run `cargo xtask copy-keys`", path.display());
}
