//! `role="text"` over the six-tag HTML the descriptions are authored in, so that VoiceOver
//! speaks an element's text as one utterance instead of chunking it at every emphasis
//! change — "Echo, with", "bold", "inside".

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

/// Marks every element in `html` whose text must be heard as one utterance.
///
/// A paragraph takes the role itself: it has no role worth keeping. A list item does not,
/// because `role="text"` replaces `listitem` and would cost the list its item count and its
/// place in the rotor, so the run of text goes inside a `<span>` instead.
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
        .into_owned()
}

/// Marks a bare run of text that the view, not the string, supplies the element for: a
/// heading's words arrive without their `<h2>` around them, so there is no tag here to take a
/// role. The `<span>` goes inside whatever the view wraps this in, which is what a heading
/// needs anyway — `role="text"` on the `<h2>` itself would replace `heading` and cost the
/// document its rotor.
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
        "<ul><li>Gain <strong>+1 STR</strong></li></ul>",
        r#"<ul><li><span role="text">Gain <strong>+1 STR</strong></span></li></ul>"#;
        "a list item wraps its text rather than taking the role itself"
    )]
    #[test_case(
        "<ul><li>Choose <em>one</em><ul><li>Or <strong>both</strong></li></ul></li></ul>",
        concat!(
            r#"<ul><li><span role="text">Choose <em>one</em></span>"#,
            r#"<ul><li><span role="text">Or <strong>both</strong></span></li></ul>"#,
            "</li></ul>",
        );
        "a nested list stays outside its parent's span"
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

    /// Nothing without inline markup chunks, so nothing without inline markup is touched. The
    /// role is a fix for a specific VoiceOver behaviour, not decoration to spread over the
    /// document.
    #[test_case("<p>Plain prose, start to end.</p>"; "a paragraph of plain text")]
    #[test_case("<ul><li>A plain item</li><li>And another</li></ul>"; "plain list items")]
    #[test_case("<p>Prose with an <a href=\"/\">unrelated</a> tag.</p>"; "a tag that is not emphasis")]
    #[test_case(""; "nothing at all")]
    fn unmarked(html: &str) {
        assert_eq!(one_utterance(html), html);
    }
}
