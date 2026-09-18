//! `role="text"` over the six-tag HTML the descriptions are authored in, so that VoiceOver
//! speaks an element's text as one utterance instead of chunking it at every emphasis
//! change — "Echo, with", "bold", "inside". Also `role="none"` on every unordered list, so
//! that VoiceOver does not stop at each item's bullet before reading the item's text. And a
//! spoken label on each word the US English voice gets wrong, which the `role="text"` around
//! it lets VoiceOver substitute into the utterance rather than pause on.

use std::borrow::Cow;
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

/// Each word the US English voice mispronounces, exactly as it appears in the content, and
/// how to spell it so the voice says it right. "Stonetop" is "ston-eh-top", and the hyphen
/// gives the synthesizer the syllable break. Capitalised "Sane" is the surname "Sané", and
/// the lowercase spelling dodges the name dictionary; lowercase "sane" in the content is
/// already said right, so it is not listed. "Fae" is said as "Fey".
const MISPRONOUNCED: [(&str, &str); 3] =
    [("Stonetop", "Stone-top"), ("Sane", "sane"), ("Fae", "Fey")];

/// The listed words as one alternation at word boundaries, so that a single pass labels them
/// all and a label the pass has just written is never matched again.
static MISPRONOUNCED_WORD: LazyLock<Regex> = LazyLock::new(|| {
    let words: Vec<&str> = MISPRONOUNCED.iter().map(|(word, _)| *word).collect();
    Regex::new(&format!(r"\b({})\b", words.join("|"))).expect("valid regex")
});

/// Every mispronounced word wrapped in a `<span aria-label>` that spells out how to say it.
/// The span carries no role of its own; it is the enclosing `role="text"` that lets the
/// label substitute into the utterance instead of splitting it.
fn label_pronunciations(html: &str) -> Cow<'_, str> {
    MISPRONOUNCED_WORD.replace_all(html, |caps: &Captures| {
        let word = &caps[0];
        let (_, said) = MISPRONOUNCED
            .iter()
            .find(|(listed, _)| *listed == word)
            .expect("the regex is built from the list");
        format!(r#"<span aria-label="{said}">{word}</span>"#)
    })
}

/// We want VoiceOver to read the tag within paragraphs and list elements as one thing, but
/// VoiceOver wants to pause at each change of emphasis (`<strong>`, etc). If a paragraph has any
/// emphasis tags, we use `<p role="text">`. For a list item, we use `<li><span role="text">` —
/// otherwise VoiceOver won't treat it as a list. But we actually want *unordered* lists to be
/// treated as text, not a list, because the stop at each bullet isn't worth it. So we use
/// `<ul role="none">`, which the list items will inherit. (The numbers introducing ordered list
/// items, which occur only in a playbook's Introductions section, are worth keeping as they
/// help make sure the players are in synch.)
pub fn one_utterance(html: &str) -> String {
    let html = label_pronunciations(html);
    let paragraphs = PARAGRAPH.replace_all(&html, |caps: &Captures| {
        let content = &caps[1];
        if has_inline_markup(content) {
            format!(r#"<p role="text">{content}</p>"#)
        } else {
            caps[0].to_string()
        }
    });
    LIST_ITEM_TEXT
        .replace_all(&paragraphs, |caps: &Captures| {
            let (content, ended_by) = (&caps[1], &caps[2]);
            if has_inline_markup(content) {
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
    let run = label_pronunciations(run);
    if has_inline_markup(&run) {
        format!(r#"<span role="text">{run}</span>"#)
    } else {
        run.into_owned()
    }
}

/// Whether `content` holds inline markup, which is what makes VoiceOver chunk it. `<strong>`
/// and `<em>` are the only inline tags the six-tag grammar has; the `<span>` is ours, from
/// [`label_pronunciations`], and an inline span carrying ARIA splits a paragraph just as
/// emphasis does.
fn has_inline_markup(content: &str) -> bool {
    content.contains("<strong>") || content.contains("<em>") || content.contains("<span")
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
        "<p>Welcome to Stonetop.</p>",
        r#"<p role="text">Welcome to <span aria-label="Stone-top">Stonetop</span>.</p>"#;
        "Stonetop is labelled with its syllable break, and the paragraph carries the role"
    )]
    #[test_case(
        "<p>Sane is the name of the thing.</p>",
        r#"<p role="text"><span aria-label="sane">Sane</span> is the name of the thing.</p>"#;
        "capitalised Sane is labelled lowercase, which the voice says right"
    )]
    #[test_case(
        "<p>The Fae are near.</p>",
        r#"<p role="text">The <span aria-label="Fey">Fae</span> are near.</p>"#;
        "Fae is labelled with the spelling the voice says right"
    )]
    #[test_case(
        "<p>A sane choice.</p>",
        "<p>A sane choice.</p>";
        "lowercase sane is already said right and is left alone"
    )]
    #[test_case(
        "<ol><li>Go to Stonetop</li></ol>",
        r#"<ol><li><span role="text">Go to <span aria-label="Stone-top">Stonetop</span></span></li></ol>"#;
        "a list item with a labelled word wraps its text, keeping the item"
    )]
    fn pronounced(html: &str, expected: &str) {
        assert_eq!(one_utterance(html), expected);
    }

    #[test_case(
        "The <em>Steplands</em>",
        r#"<span role="text">The <em>Steplands</em></span>"#;
        "a heading's run with emphasis is wrapped"
    )]
    #[test_case("The Steplands", "The Steplands"; "a heading's run of plain text is left alone")]
    #[test_case(
        "Welcome to Stonetop",
        r#"<span role="text">Welcome to <span aria-label="Stone-top">Stonetop</span></span>"#;
        "a heading's run with a labelled word is wrapped"
    )]
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
