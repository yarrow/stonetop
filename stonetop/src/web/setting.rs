//! The Setting overview as one document: the title is the H1 and the document's `<title>`, each
//! section an H2, each map subsection an H3. Heading level comes from the nesting alone, as
//! it does in the Markdown renderer this document is the sibling of.
//!
//! A body is inserted as HTML rather than parsed into components. The json5 is authored in
//! six tags and nothing else, checked by the Markdown converter's panic, so there is nothing
//! for a parse to decide; and the spoken-form transforms have to run over the text first
//! either way.
//!
//! A body goes through two transforms on the way in, in this order: [`speak_slots`] turns
//! print glyphs into words, then [`one_utterance`] marks what VoiceOver would otherwise read
//! in chunks. Only the second is peculiar to HTML; the Markdown renderer runs the first alone.
//! A heading takes [`one_utterance_run`] in place of the second, because its words arrive
//! without an element around them and its own `heading` role has to survive. The `<title>`
//! takes neither: it is not markup, and VoiceOver reads it as one utterance regardless.

use leptos::prelude::*;

use super::document::Document;
use crate::fixed::{SettingSection, SettingSubsection, setting_overview};
use crate::render::spoken::speak_slots;
use crate::render::text_role::{one_utterance, one_utterance_run};

/// The whole overview, in the order the content has it.
pub fn view(options: LeptosOptions) -> impl IntoView {
    let overview = setting_overview();
    // One spoken form, used twice: what VoiceOver says on load and what it says at the top of
    // the document have to be the same words.
    let title = speak_slots(overview.title);
    view! {
        <Document options title=title.clone()>
            <h1 inner_html=one_utterance_run(&title)></h1>
            {overview.sections.iter().map(section).collect_view()}
        </Document>
    }
}

fn section(section: &'static SettingSection) -> impl IntoView {
    view! {
        <h2 inner_html=one_utterance_run(&speak_slots(section.heading))></h2>
        <div inner_html=one_utterance(&speak_slots(section.html))></div>
        {section.subsections.iter().map(subsection).collect_view()}
    }
}

fn subsection(subsection: &'static SettingSubsection) -> impl IntoView {
    view! {
        <h3 inner_html=one_utterance_run(&speak_slots(subsection.heading))></h3>
        <div inner_html=one_utterance(&speak_slots(subsection.html))></div>
    }
}
