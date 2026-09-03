//! Write the baked Fixed content to `stonetop/src/fixed/generated.rs`. Run as `cargo xtask bake`.

use anyhow::{Context, Result};

use codegen::bake::{baked_source, generated_path};

fn main() -> Result<()> {
    let source = baked_source()?;
    let path = generated_path();
    std::fs::write(&path, source).with_context(|| format!("writing {}", path.display()))?;
    println!("wrote {}", path.display());
    Ok(())
}
