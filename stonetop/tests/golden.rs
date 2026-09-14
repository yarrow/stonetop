//! Renders each playbook to Markdown and compares it with its golden file under
//! `tests/golden/`. The golden is the example of the correct linearisation; when it and
//! the renderer disagree, one of them is wrong, and the diff is where that gets decided.
#![cfg(feature = "ssr")]

use std::fs;
use std::path::Path;

use similar::TextDiff;
use stonetop::keys::PlaybookKey;
use stonetop::render::render_markdown;
use test_case::test_case;

/// Fails with a unified line diff if `key`'s rendering differs from `tests/golden/<file>`.
#[test_case(PlaybookKey::TheBlessed, "blessed.md")]
#[test_case(PlaybookKey::TheFox, "fox.md")]
#[test_case(PlaybookKey::TheHeavy, "heavy.md")]
#[test_case(PlaybookKey::TheJudge, "judge.md")]
#[test_case(PlaybookKey::TheLightbearer, "lightbearer.md")]
#[test_case(PlaybookKey::TheMarshal, "marshal.md")]
#[test_case(PlaybookKey::TheRanger, "ranger.md")]
#[test_case(PlaybookKey::TheSeeker, "seeker.md")]
#[test_case(PlaybookKey::TheWouldBeHero, "would-be-hero.md")]
fn renders_as_its_golden_file(key: PlaybookKey, file: &str) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/golden").join(file);
    let golden =
        fs::read_to_string(&path).unwrap_or_else(|e| panic!("reading {}: {e}", path.display()));
    let rendered = render_markdown(key.fixed_part());
    if rendered != golden {
        let diff = TextDiff::from_lines(&golden, &rendered)
            .unified_diff()
            .context_radius(3)
            .header(&format!("tests/golden/{file}"), &format!("render_markdown({key:?})"))
            .to_string();
        panic!("{key:?} does not render as tests/golden/{file}:\n{diff}");
    }
}
