//! Converts the six HTML tags the json5 descriptions use (`p`, `strong`, `em`, `ul`, `ol`,
//! `li`) into Markdown, and `&amp;` into `&`. Anything else is unsupported and we panic.
//!
//! Text is copied through unchanged, so Markdown-special characters in the descriptions (a
//! paragraph beginning `2. `, or a `*` in running text) are not escaped.

use std::fmt;

use regex_lite::{Matches, Regex};
use std::iter::Peekable;
use std::sync::LazyLock;

use Tag::*;
use Token::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tag {
    Strong,
    Em,
    P,
    Li,
    Ul,
    Ol,
}

impl Tag {
    fn name(self) -> &'static str {
        match self {
            P => "p",
            Ul => "ul",
            Ol => "ol",
            Li => "li",
            Strong => "strong",
            Em => "em",
        }
    }

    fn from_name(name: &str) -> Option<Tag> {
        match name {
            "p" => Some(P),
            "ul" => Some(Ul),
            "ol" => Some(Ol),
            "li" => Some(Li),
            "strong" => Some(Strong),
            "em" => Some(Em),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum Token<'h> {
    Text(&'h str),
    Open(Tag),
    Close(Tag),
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Text(text) => write!(f, "text {text:?}"),
            Open(tag) => write!(f, "<{}>", tag.name()),
            Close(tag) => write!(f, "</{}>", tag.name()),
        }
    }
}

/// Markdown for `html`: `<p>` paragraphs and `<ul>` and `<ol>` lists preceded by a blank line if
/// there is any preceding text, `<strong>` text surrounded by "**", and `<em>` text by "*". Each
/// `<li>`'s text is preceded by "- " for unordered lists and by "1. ", "2. ", etc for ordered lists.
///
/// # Panics
///
/// On any tag other than the six, on tags that don't nest properly, and on text following a
/// nested list within its `<li>`.
#[must_use]
pub fn html_to_markdown(html: &str) -> String {
    Parser::new(html).markdown()
}

// Every character is `&`, `<`, or neither, and the regex alternation has a branch for each (`&` and
// `</?` match on their own), so the tokens always cover the whole input with no gaps.
const TOKEN: &str = r"&\w+;|&|</?\w[^<>]*>?|</?|[^&<]+";
static TOKENIZER: LazyLock<Regex> = LazyLock::new(|| Regex::new(TOKEN).unwrap());

struct Parser<'h> {
    html: &'h str,
    tokens: Peekable<Matches<'static, 'h>>,
    markdown: String,
}

impl<'h> Parser<'h> {
    fn new(html: &'h str) -> Self {
        Parser { html, tokens: TOKENIZER.find_iter(html).peekable(), markdown: String::new() }
    }

    fn take(&mut self) -> Option<Token<'h>> {
        let snippet = self.tokens.next()?.as_str();
        Some(self.token_of(snippet))
    }

    fn peek(&mut self) -> Option<Token<'h>> {
        let snippet = self.tokens.peek()?.as_str();
        Some(self.token_of(snippet))
    }

    fn token_of(&self, snippet: &'h str) -> Token<'h> {
        let html = self.html;
        if let Some(rest) = snippet.strip_prefix('<') {
            if rest.is_empty() {
                panic!("Found an orphan less than (<) sign in {html}");
            }
            let (is_close, name) = match rest.strip_prefix('/') {
                Some(name) => (true, name),
                None => (false, rest),
            };
            let Some(name) = name.strip_suffix('>') else {
                panic!("Unexpected tag beginning `{snippet}` in {html}");
            };
            let Some(tag) = Tag::from_name(name) else {
                panic!("Unexpected tag `{snippet}` in {html}");
            };
            return if is_close { Close(tag) } else { Open(tag) };
        }
        match snippet {
            "&amp;" => Text("&"),
            "&" => Text(snippet), // Also panic on lone & once we've cleaned the json5 files.
            _ if snippet.starts_with('&') => panic!("Unexpected entity {snippet} in {html}"),
            _ => Text(snippet),
        }
    }

    fn markdown(mut self) -> String {
        if matches!(self.peek(), Some(Text(_) | Open(Strong | Em))) {
            // Text with emphasis, not wrapped in any tag.
            while let Some(token) = self.take() {
                match token {
                    Text(snippet) => self.markdown.push_str(snippet),
                    Open(tag @ (Strong | Em)) => self.span(tag),
                    _ => panic!("Unexpected tag {token} in {}", self.html),
                }
            }
        } else {
            // A sequence of paragraphs and lists.
            while let Some(token) = self.take() {
                match token {
                    Open(P) => {
                        self.ensure_block_separation();
                        self.span(P);
                    }
                    Open(tag @ (Ul | Ol)) => {
                        self.ensure_block_separation();
                        self.list(tag, 0);
                    }
                    Text(snippet) => panic!(
                        "Unexpected text (`{snippet}`) outside of a paragraph or list element in {}",
                        self.html
                    ),
                    _ => panic!("Unexpected tag {token} in {}", self.html),
                }
            }
        }
        self.markdown
    }

    /// A `<p>`, `<strong>`, or `<em>` whose opening tag has just been taken: its text and the
    /// emphasis allowed inside it, with `<strong>` text between "**" and `<em>` text between "*".
    fn span(&mut self, current: Tag) {
        let (stars, allowed): (&str, &[Tag]) = match current {
            P => ("", &[Strong, Em]),
            Strong => ("**", &[Em]),
            Em => ("*", &[]),
            _ => unreachable!("{} has no inline content", Open(current)),
        };
        self.markdown.push_str(stars);
        while let Some(token) = self.take() {
            match token {
                Close(tag) if tag == current => {
                    self.markdown.push_str(stars);
                    return;
                }
                Text(snippet) => self.markdown.push_str(snippet),
                Open(tag @ (Strong | Em)) if allowed.contains(&tag) => self.span(tag),
                Open(tag) => {
                    panic!("Found {} inside {} in {}", Open(tag), Open(current), self.html)
                }
                Close(tag) => panic!("Found {} with no {} in {}", Close(tag), Open(tag), self.html),
            }
        }
        panic!("Expected {}, but didn't find it in {}", Close(current), self.html);
    }

    fn ensure_block_separation(&mut self) {
        if !self.markdown.is_empty() {
            self.markdown.push_str("\n\n");
        }
    }

    /// A `<ul>` or `<ol>` whose opening tag has just been taken. Its items are indented by
    /// `indent` spaces.
    fn list(&mut self, current: Tag, indent: usize) {
        let mut item_number = 0;
        while let Some(token) = self.take() {
            match token {
                Open(Li) => {
                    item_number += 1;
                    let marker = match current {
                        Ul => "- ".to_string(),
                        Ol => format!("{item_number}. "),
                        _ => unreachable!("{} is not a list tag", Open(current)),
                    };
                    self.list_item(&marker, item_number > 1, indent);
                }
                Close(tag) if tag == current => return,
                Text(snippet) => panic!(
                    "Unexpected text (`{snippet}`) outside of a paragraph or list element in {}",
                    self.html
                ),
                _ => panic!("Unexpected tag {token} in {}", self.html),
            }
        }
        panic!("Expected {}, but didn't find it in {}", Close(current), self.html);
    }

    /// A `<li>` whose opening tag has just been taken: `marker` follows `indent` spaces, then
    /// the item's text, with any nested list on the lines after it.
    fn list_item(&mut self, marker: &str, follows_another_item: bool, indent: usize) {
        if follows_another_item {
            self.markdown.push('\n');
        }
        self.markdown.push_str(&" ".repeat(indent));
        self.markdown.push_str(marker);

        while let Some(token) = self.take() {
            match token {
                Open(tag @ (Ul | Ol)) => {
                    self.markdown.push('\n');
                    self.list(tag, indent + marker.len());
                    // Anything after the nested list would be glued onto its last item.
                    match self.take() {
                        Some(Close(Li)) => return,
                        Some(token) => {
                            panic!("Found {token} after a nested list in {}", self.html)
                        }
                        None => break,
                    }
                }
                Text(snippet) => self.markdown.push_str(snippet),
                Open(tag @ (Strong | Em)) => self.span(tag),
                Close(Li) => return,
                _ => panic!("Unexpected tag {token} in {}", self.html),
            }
        }
        panic!("Expected {}, but didn't find it in {}", Close(Li), self.html);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("dog<and>cat</and>" => panics "Unexpected tag `<and>`"; "unknown tag")]
    #[test_case("dog and <p>cat</p>" => panics "Unexpected tag <p>";
        "block tag in string that didn't start with one")]
    #[test_case("dog</em>" => panics "Unexpected tag </em>"; "unmatched end tag")]
    #[test_case("<p><ul><li>foo</li></ul></p>" => panics "Found <ul> inside <p>";
        "block tag inside another block tag")]
    #[test_case("<li>line</li>" => panics "Unexpected tag <li>"; "orphan <li>")]
    #[test_case("<em>line" => panics "Expected </em>, but didn't find it"; "unclosed tag")]
    #[test_case("<ul>WRONG<li>ok</li></ul>" => panics "Unexpected text (`WRONG`)";
        "text after a <ul>")]
    #[test_case("<ol>WRONG<li>ok</li></ol>" => panics "Unexpected text (`WRONG`)";
        "text after a <ol>")]
    #[test_case("<p>first</p>UN-TAGGED<p>second</p>" => panics "Unexpected text (`UN-TAGGED`)";
        "Text not within a tag")]
    #[test_case("<em>This <strong>won't</strong> work</em>" => panics "Found <strong> inside <em>";
        "strong inside em")]
    #[test_case("<ul><li>a</li></ol>" => panics "Unexpected tag </ol>"; "ol closing ul")]
    #[test_case("<ul><li>a</li>" => panics "Expected </ul>, but didn't find it"; "unclosed ul")]
    #[test_case("<ul><li>a" => panics "Expected </li>, but didn't find it"; "unclosed li")]
    #[test_case("<ul><li>two<ul><li>foo</li></ul>tail</li></ul>"
        => panics "Found text \"tail\" after a nested list";
        "text after a nested list")]
    #[test_case("<ul><li>two<ul><li>foo</li></ul><strong>tail</strong></li></ul>"
        => panics "Found <strong> after a nested list";
        "emphasis after a nested list")]
    fn bad_tags_panic(bad: &str) {
        let _ = html_to_markdown(bad);
    }

    #[test_case("vanilla", "vanilla";
        "plain text is unchanged")]
    #[test_case("", "";
        "empty text is unchanged")]
    #[test_case("dog&amp;cat", "dog&cat";
        "amp HTML entity becomes ampersand")]
    #[test_case("dog&cat", "dog&cat";
        "single ampersand goes through as an ampersand")]
    #[test_case("<em>foo</em> and bar", "*foo* and bar";
        "Open and closing em tags turn into one asterisk")]
    #[test_case("<strong>foo</strong> and bar", "**foo** and bar";
        "Open and closing strong tags turn into two asterisks")]
    #[test_case("<strong><em>foo</em></strong> and bar", "***foo*** and bar";
        "Open and closing strong-em tags turn into three asterisks")]
    #[test_case("<strong><em>foo</em></strong>, <strong>bar</strong> and <em>baz</em>",
        "***foo***, **bar** and *baz*";
        "Of course we can have all three kinds of emphasis in one string")]
    #[test_case("<p>abc</p><p>def</p>", "abc\n\ndef";
        "paragraphs are separated by blank lines")]
    #[test_case("<ul><li>foo</li><li>bar</li><li>baz</li></ul>",
        "- foo\n- bar\n- baz";
        "Unordered lists have items marked by a leading dash")]
    #[test_case("<ol><li>foo</li><li>bar</li><li>baz</li></ol>",
        "1. foo\n2. bar\n3. baz";
        "Ordered lists have items numbered from one")]
    #[test_case("<ol><li>one<ul><li>foo</li></ul></li></ol>", "1. one\n   - foo";
            "lists can be embedded in other lists")]
    #[test_case(
            "<ol><li>one</li><li>two<ul><li>foo</li><li>bar</li></ul></li><li>three</li></ol>",
            "1. one\n2. two\n   - foo\n   - bar\n3. three";
            "longer lists can be embedded in other lists")]
    #[test_case(
            "<ul><li>one</li><li>two<ol><li>foo</li><li>bar</li></ol></li><li>three</li></ul>",
            "- one\n- two\n  1. foo\n  2. bar\n- three";
            "Ordered lists can be embedded in unordered lists")]
    #[test_case(
            "<ol><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li></ol>",
            "1. \n2. \n3. \n4. \n5. \n6. \n7. \n8. \n9. \n10. ";
            "We can count to 10")]
    #[test_case(
            "<ol><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li>abc<ul><li>Foo</li></ul></li><li>def<ul><li>Bar</li></ul></li></ol>",
            "1. \n2. \n3. \n4. \n5. \n6. \n7. \n8. \n9. abc\n   - Foo\n10. def\n    - Bar";
            "Embedded lists get the right indentation")]
    fn parsed(html: &str, markdown: &str) {
        assert_eq!(html_to_markdown(html), markdown);
    }

    #[test]
    #[should_panic(expected = "Found <strong> inside <em>")]
    fn the_foxs_cheap_shot_move_had_an_embedded_strong_that_wouldnt_have_worked_in_html_either() {
        let _ = html_to_markdown(
            "<p>When you <strong><em>Ambush with a <strong>hand</strong> weapon</em></strong>, you have advantage on your damage roll.</p>",
        );
    }

    #[test]
    fn the_fixed_cheap_shot_move_is_cumbersome_but_works() {
        let html = "<p>When you <strong><em>Ambush with a</em></strong> <strong>hand</strong> <strong><em>weapon</em></strong>, you have advantage on your damage roll.</p>";
        let markdown = "When you ***Ambush with a*** **hand** ***weapon***, you have advantage on your damage roll.";
        assert_eq!(html_to_markdown(html), markdown);
    }

    // The Heavy's Formidable, from `heavy.json5`, and its lines in `markdown/word-slots/heavy.md`.
    #[test]
    fn an_unordered_list_between_paragraphs_reads_as_the_heavys_formidable() {
        let html = "<p>When you <strong><em>wade into battle</em></strong>, you can choose to roll +CHA: <strong>on a 10+</strong>, both; <strong>on a 7-9</strong>, pick 1:</p><ul><li>Lesser foes will quail, hesitate, or flee before you.</li><li>Doughty foes will focus on you, seeing you as the greatest threat.</li></ul><p><strong>On a 6-</strong>, pick 1 but ask the GM what you've missed.</p>";
        let markdown = "\
When you ***wade into battle***, you can choose to roll +CHA: **on a 10+**, both; **on a 7-9**, pick 1:

- Lesser foes will quail, hesitate, or flee before you.
- Doughty foes will focus on you, seeing you as the greatest threat.

**On a 6-**, pick 1 but ask the GM what you've missed.";
        assert_eq!(html_to_markdown(html), markdown);
    }

    /// The Heavy's Introductions, from `heavy.json5`, and its lines in
    /// `markdown/word-slots/heavy.md`.
    #[test]
    fn a_nested_list_indents_by_the_parent_markers_width_as_in_the_heavys_introductions() {
        let html = "<p>Wait here for everyone else. When everyone's ready, take turns introducing your characters. When <strong><em>someone reveals something and you want to know more</em></strong>, ask them about it. When <strong><em>someone asks you a question</em></strong>, answer it truthfully.</p><ol><li>On your first turn, <strong>introduce yourself</strong> by name, pronouns, background, origin, and appearance.</li><li>On your second turn, <strong>describe your special possessions</strong> and how you contribute to the village (beyond working the fields).</li><li>On your third turn, <strong>tell us about your history of violence</strong>, and what keeps you up at night.</li><li>On your next turn, <strong>answer one of the following</strong>, naming one or more NPCs who live in Stonetop.<ul><li>Who is your closest kin?</li><li>Who is your lover/spouse/betrothed?</li><li>Who most needs/deserves your protection?</li><li>Whose forgiveness do you strive to earn?</li></ul></li><li>Go around again. Answer another question from 4, or pass. When everyone has passed, go on.</li><li>On your next turn, <strong>ask your fellow PCs one of these</strong>. When others ask you, answer as you like.<ul><li>Which one of you once dragged me home, bleeding and unconscious?</li><li>Which one of you can I trust to always have my back?</li><li>Which one of you has stayed my hand?</li><li>Which one of you has traded blows with me?</li></ul></li><li>Go around again. Ask another question from 6, or pass. When everyone has passed, go on.</li><li>Add your home to the steading playbook. When everyone is done, let spring break forth!</li></ol>";
        let markdown = "\
Wait here for everyone else. When everyone's ready, take turns introducing your characters. When ***someone reveals something and you want to know more***, ask them about it. When ***someone asks you a question***, answer it truthfully.

1. On your first turn, **introduce yourself** by name, pronouns, background, origin, and appearance.
2. On your second turn, **describe your special possessions** and how you contribute to the village (beyond working the fields).
3. On your third turn, **tell us about your history of violence**, and what keeps you up at night.
4. On your next turn, **answer one of the following**, naming one or more NPCs who live in Stonetop.
   - Who is your closest kin?
   - Who is your lover/spouse/betrothed?
   - Who most needs/deserves your protection?
   - Whose forgiveness do you strive to earn?
5. Go around again. Answer another question from 4, or pass. When everyone has passed, go on.
6. On your next turn, **ask your fellow PCs one of these**. When others ask you, answer as you like.
   - Which one of you once dragged me home, bleeding and unconscious?
   - Which one of you can I trust to always have my back?
   - Which one of you has stayed my hand?
   - Which one of you has traded blows with me?
7. Go around again. Ask another question from 6, or pass. When everyone has passed, go on.
8. Add your home to the steading playbook. When everyone is done, let spring break forth!";
        assert_eq!(html_to_markdown(html), markdown);
    }

    #[test]
    fn em_inside_strong_is_ok() {
        assert_eq!(html_to_markdown("<strong>a <em>b</em> c</strong>"), "**a *b* c**");
    }

    /// The one entity in the json5, in the Blessed's Mastiffs; a bare "&" passes through.
    #[test]
    fn an_ampersand_entity_is_an_ampersand() {
        assert_eq!(
            html_to_markdown("Instinct: to bark &amp; threaten; 1-slot block & tackles"),
            "Instinct: to bark & threaten; 1-slot block & tackles"
        );
    }

    #[test]
    #[should_panic(expected = "Unexpected entity &nbsp; in <p>a&nbsp;b</p>")]
    fn any_other_entity_is_a_bug() {
        let _ = html_to_markdown("<p>a&nbsp;b</p>");
    }

    #[test]
    #[should_panic(expected = "Unexpected tag `<br>` in <p>line<br>break</p>")]
    fn any_other_tag_is_a_bug() {
        let _ = html_to_markdown("<p>line<br>break</p>");
    }

    #[test]
    #[should_panic(expected = "Unexpected tag `<p class=\"x\">`")]
    fn a_tag_with_attributes_is_a_bug() {
        let _ = html_to_markdown("<p class=\"x\">text</p>");
    }

    #[test]
    #[should_panic(expected = "Unexpected tag beginning `<p text`")]
    fn an_unclosed_tag() {
        let _ = html_to_markdown("<p text</p>");
    }

    #[test]
    #[should_panic(expected = "Unexpected tag beginning `</`")]
    fn an_unclosed_closing_tag() {
        let _ = html_to_markdown("<p>text</");
    }

    #[test]
    #[should_panic(expected = "Found an orphan less than (<) sign")]
    fn a_lone_less_than_is_a_bug() {
        let _ = html_to_markdown("0 < 1");
    }

    #[test]
    #[should_panic(expected = "Found </strong> with no <strong>")]
    fn crossed_emphasis_is_a_bug() {
        let _ = html_to_markdown("<strong><em>text</strong></em>");
    }

    #[test]
    #[should_panic(expected = "Expected </p>, but didn't find it")]
    fn an_unclosed_paragraph_is_a_bug() {
        let _ = html_to_markdown("<p>text");
    }

    #[test]
    #[should_panic(expected = "Unexpected text (`stray`) outside of a paragraph or list element")]
    fn text_between_list_items_is_a_bug() {
        let _ = html_to_markdown("<ul><li>a</li>stray<li>b</li></ul>");
    }
}
