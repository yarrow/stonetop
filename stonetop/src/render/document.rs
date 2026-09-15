//! What every Markdown renderer shares: the document under construction, and the path each
//! piece of content takes from the json5's HTML to spoken Markdown.

use super::markdown::html_to_markdown;
use super::spoken::speak_slots;

/// The document under construction: blocks separated by blank lines. A heading, a
/// paragraph, and a whole list are each one block.
#[derive(Default)]
pub(super) struct Document {
    blocks: Vec<String>,
}

impl Document {
    pub(super) fn block(&mut self, block: impl Into<String>) {
        self.blocks.push(block.into());
    }

    /// One block of lines, such as a list.
    pub(super) fn lines(&mut self, lines: impl IntoIterator<Item = String>) {
        self.block(lines.into_iter().collect::<Vec<_>>().join("\n"));
    }

    /// The whole document, ending in one newline.
    pub(super) fn finish(self) -> String {
        let mut text = self.blocks.join("\n\n");
        text.push('\n');
        text
    }
}

/// Content text as spoken Markdown: resource placeholders are the caller's business, since
/// only a possession has one; here slot diamonds become words and the HTML becomes Markdown.
pub(super) fn speak(text: &str) -> String {
    html_to_markdown(&speak_slots(text))
}
