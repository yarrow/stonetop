//! `stonetop/src/fixed/generated.rs` is exactly what `cargo xtask bake` would write from the
//! json5 now. A hand edit to the file, or a json5 change without a re-bake, goes red here.

use codegen::bake::{baked_source, generated_path};

#[test]
fn generated_file_is_fresh() {
    let path = generated_path();
    let on_disk = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("reading {}: {e}; run `cargo xtask bake`", path.display()));
    let expected = baked_source().unwrap_or_else(|e| panic!("{e:#}"));
    assert!(on_disk == expected, "{} is out of date: run `cargo xtask bake`", path.display());
}
