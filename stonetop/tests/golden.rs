//! Renders each playbook, and the Setting overview, to Markdown and compares it with its
//! golden file under `tests/golden/`. The golden is the example of the correct
//! linearisation; when it and the renderer disagree, one of them is wrong, and the diff is
//! where that gets decided.
#![cfg(feature = "ssr")]

use std::fs;
use std::path::Path;

use similar::TextDiff;
use stonetop::fixed::setting_overview;
use stonetop::keys::PlaybookKey;
use stonetop::render::{render_markdown, render_setting_markdown};
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
    let rendered = render_markdown(key.fixed_part());
    assert_matches_golden(&rendered, file, &format!("render_markdown({key:?})"));
}

/// The Setting overview, likewise.
#[test]
fn the_setting_overview_renders_as_its_golden_file() {
    let rendered = render_setting_markdown(setting_overview());
    assert_matches_golden(&rendered, "setting.md", "render_setting_markdown(setting_overview())");
}

/// Panics with a unified line diff if `rendered` is not byte for byte `tests/golden/<file>`.
/// `label` names the call that produced `rendered`, for the diff's header.
fn assert_matches_golden(rendered: &str, file: &str, label: &str) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/golden").join(file);
    let golden =
        fs::read_to_string(&path).unwrap_or_else(|e| panic!("reading {}: {e}", path.display()));
    if rendered != golden {
        let diff = TextDiff::from_lines(&golden, rendered)
            .unified_diff()
            .context_radius(3)
            .header(&format!("tests/golden/{file}"), label)
            .to_string();
        panic!("{label} does not render as tests/golden/{file}:\n{diff}");
    }
}
