//! The client crate's copy of `item_keys.rs`: the same enums, minus the strum derives the
//! client has no use for.
//!
//! The transform is textual and deliberately strict. `item_keys.rs` is hand-maintained and
//! rustfmt-formatted, so its shape is regular; anything outside that shape is an error
//! rather than a guess. The output is itself rustfmt-clean by construction, so the two
//! files can be compared byte for byte.

use anyhow::{Context, Result, bail, ensure};
use regex::Regex;

/// The derives the client keeps, in the order they appear in the source.
pub const CLIENT_DERIVES: &[&str] =
    &["Clone", "Copy", "Debug", "PartialEq", "Eq", "PartialOrd", "Ord", "Serialize", "Deserialize"];

/// The derives that come from strum and are stripped from the client copy.
pub const STRUM_DERIVES: &[&str] = &["Display", "EnumString", "EnumIter"];

const GENERATED_HEADER: &str = "\
//! Generated from `codegen/src/item_keys.rs` by `cargo xtask copy-keys`. Do not edit!
";

/// `source` (the text of `codegen/src/item_keys.rs`) rewritten for the client crate.
///
/// # Errors
///
/// If `source` is not in the shape this transform expects: no `use strum` line, a derive
/// that is neither a client nor a strum derive, or a derive list left empty.
pub fn client_copy(source: &str) -> Result<String> {
    let body = strip_header(source);
    let body = drop_strum_use(body)?;
    rewrite_derives(&body).map(|body| format!("{GENERATED_HEADER}\n{body}"))
}

/// `source` without its leading `//!` block and the blank line after it.
fn strip_header(source: &str) -> &str {
    let mut rest = source;
    while let Some(line_end) = rest.find('\n') {
        if !rest.starts_with("//!") {
            break;
        }
        rest = &rest[line_end + 1..];
    }
    rest.trim_start_matches('\n')
}

fn drop_strum_use(body: &str) -> Result<String> {
    let mut kept = Vec::new();
    let mut dropped = 0;
    for line in body.lines() {
        if line.starts_with("use strum::") {
            dropped += 1;
        } else {
            kept.push(line);
        }
    }
    ensure!(dropped == 1, "expected exactly one `use strum::` line, found {dropped}");
    Ok(kept.join("\n") + "\n")
}

fn rewrite_derives(body: &str) -> Result<String> {
    let derive = Regex::new(r"#\[derive\(([^)]*)\)\]").unwrap();
    let mut out = String::with_capacity(body.len());
    let mut last = 0;
    for m in derive.captures_iter(body) {
        let whole = m.get(0).unwrap();
        out.push_str(&body[last..whole.start()]);
        out.push_str(&client_derive(&m[1])?);
        last = whole.end();
    }
    out.push_str(&body[last..]);
    Ok(out)
}

/// The single-line client derive for a source derive list.
fn client_derive(list: &str) -> Result<String> {
    let mut kept = Vec::new();
    for ident in list.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        if CLIENT_DERIVES.contains(&ident) {
            kept.push(ident);
        } else if !STRUM_DERIVES.contains(&ident) {
            bail!("derive `{ident}` is neither a client derive nor a strum derive");
        }
    }
    ensure!(!kept.is_empty(), "derive list `{list}` has nothing the client keeps");
    Ok(format!("#[derive({})]", kept.join(", ")))
}

/// The client copy of this crate's own `item_keys.rs`.
///
/// # Errors
///
/// If the file can't be read or isn't in the expected shape.
pub fn client_copy_of_item_keys() -> Result<String> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/item_keys.rs");
    let source =
        std::fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    client_copy(&source).with_context(|| format!("transforming {}", path.display()))
}

#[cfg(test)]
mod test {
    use super::*;

    const SOURCE: &str = "\
//! Source of truth.
//!
//! Two paragraphs.

use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Display,
    EnumString,
    EnumIter,
)]
pub enum PlaybookKey {
    TheBlessed,
}
";

    #[test]
    fn header_use_and_derives_are_rewritten() {
        let expected = format!(
            "{GENERATED_HEADER}
use serde::{{Deserialize, Serialize}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PlaybookKey {{
    TheBlessed,
}}
"
        );
        assert_eq!(client_copy(SOURCE).unwrap(), expected);
    }

    #[test]
    fn unknown_derive_is_an_error() {
        let source = SOURCE.replace("EnumIter,\n)]", "EnumIter,\n    Hash,\n)]");
        assert!(client_copy(&source).unwrap_err().to_string().contains("`Hash`"));
    }

    #[test]
    fn missing_strum_use_is_an_error() {
        let source = SOURCE.replace("use strum::{Display, EnumIter, EnumString};\n", "");
        assert!(client_copy(&source).is_err());
    }

    #[test]
    fn the_real_file_transforms() {
        client_copy_of_item_keys().unwrap();
    }
}
