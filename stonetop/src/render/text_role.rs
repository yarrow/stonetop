//! `role="text"` over the six-tag HTML the descriptions are authored in, so that VoiceOver
//! speaks an element's text as one utterance instead of chunking it at every emphasis
//! change — "Echo, with", "bold", "inside". Also `role="none"` on every unordered list, so
//! that VoiceOver does not stop at each item's bullet before reading the item's text.

use std::sync::LazyLock;

use regex_lite::{Captures, Regex};

/// One paragraph, opening tag to closing tag, with its content captured.
static PARAGRAPH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)<p>(.*?)</p>").expect("valid regex"));

/// A list item's run of text, which ends either at the item's own close or at a nested list.
/// The run is captured, and so is whichever of the two ended it, because the match consumes
/// it and the replacement has to put it back. A nested list must stay *outside* the `<span>`,
/// or the span closes across the list's tags and the markup is malformed; the grammar in
/// [`super::markdown`] allows nothing after a nested list, so the run before it is the whole
/// of the item's own text.
static LIST_ITEM_TEXT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)<li>(.*?)(<ul>|<ol>|</li>)").expect("valid regex"));

/// We want VoiceOver to read the tag within paragraphs and list elements as one thing, but
/// VoiceOver wants to pause at each change of emphasis (`<strong>`, etc). If a paragraph has any
/// emphasis tags, we use `<p role="text">`. For a list item, we use `<li><span role="text">` —
/// otherwise VoiceOver won't treat it as a list. But we actually want *unordered* lists to be
/// treated as text, not a list, because the stop at each bullet isn't worth it. So we use
/// `<ul role="none">`, which the list items will inherit. (The numbers introducing ordered list
/// items, which occur only in a playbook's Introductions section, are worth keeping as they
/// help make sure the players are in synch.)

pub fn one_utterance(html: &str) -> String {
    let paragraphs = PARAGRAPH.replace_all(html, |caps: &Captures| {
        let content = &caps[1];
        if has_emphasis(content) {
            format!(r#"<p role="text">{content}</p>"#)
        } else {
            caps[0].to_string()
        }
    });
    LIST_ITEM_TEXT
        .replace_all(&paragraphs, |caps: &Captures| {
            let (content, ended_by) = (&caps[1], &caps[2]);
            if has_emphasis(content) {
                format!(r#"<li><span role="text">{content}</span>{ended_by}"#)
            } else {
                caps[0].to_string()
            }
        })
        .replace("<ul>", r#"<ul role="none">"#)
}

/// Header text comes here without its `<h1>`, `<h2>`, etc.  We need to use `<span role="text">`
/// in headers for the same reason as ordered list items: We want VoiceOver to treat a header as
/// a header, just as we want VoicOver to treat an ordered list item as a list item.
pub fn one_utterance_run(run: &str) -> String {
    if has_emphasis(run) { format!(r#"<span role="text">{run}</span>"#) } else { run.to_string() }
}

/// Whether `content` holds inline markup, which is what makes VoiceOver chunk it. `<strong>`
/// and `<em>` are the only inline tags the six-tag grammar has.
fn has_emphasis(content: &str) -> bool {
    content.contains("<strong>") || content.contains("<em>")
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(
        "<p>Gain <strong>+1 STR</strong> now.</p>",
        r#"<p role="text">Gain <strong>+1 STR</strong> now.</p>"#;
        "a paragraph with emphasis carries the role"
    )]
    #[test_case(
        "<ol><li>Gain <strong>+1 STR</strong></li></ol>",
        r#"<ol><li><span role="text">Gain <strong>+1 STR</strong></span></li></ol>"#;
        "a list item wraps its text rather than taking the role itself"
    )]
    #[test_case(
        "<ul><li>A plain item</li><li>And another</li></ul>",
        r#"<ul role="none"><li>A plain item</li><li>And another</li></ul>"#;
        "an unordered list is not a list, whether or not its items hold emphasis"
    )]
    #[test_case(
        "<ul><li>Gain <strong>+1 STR</strong></li></ul>",
        r#"<ul role="none"><li><span role="text">Gain <strong>+1 STR</strong></span></li></ul>"#;
        "an unordered item with emphasis still wraps its text"
    )]
    #[test_case(
        "<ol><li>Choose <em>one</em><ul><li>Or <strong>both</strong></li></ul></li></ol>",
        concat!(
            r#"<ol><li><span role="text">Choose <em>one</em></span>"#,
            r#"<ul role="none"><li><span role="text">Or <strong>both</strong></span></li></ul>"#,
            "</li></ol>",
        );
        "a nested list stays outside its parent's span, and the ordered step keeps its number"
    )]
    fn marked(html: &str, expected: &str) {
        assert_eq!(one_utterance(html), expected);
    }

    #[test_case(
        "The <em>Steplands</em>",
        r#"<span role="text">The <em>Steplands</em></span>"#;
        "a heading's run with emphasis is wrapped"
    )]
    #[test_case("The Steplands", "The Steplands"; "a heading's run of plain text is left alone")]
    fn marked_run(run: &str, expected: &str) {
        assert_eq!(one_utterance_run(run), expected);
    }

    /// We don't add `role="text"` where it's not needed.
    #[test_case("<p>Plain prose, start to end.</p>"; "a paragraph of plain text")]
    #[test_case("<ol><li>A plain step</li><li>And another</li></ol>"; "plain ordered items")]
    #[test_case("<p>Prose with an <a href=\"/\">unrelated</a> tag.</p>"; "a tag that is not emphasis")]
    #[test_case(""; "nothing at all")]
    fn unmarked(html: &str) {
        assert_eq!(one_utterance(html), html);
    }
}
