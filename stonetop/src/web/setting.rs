//! The Setting overview as one document: the title is the H1 and the document's `<title>`, each
//! section an H2, each map subsection an H3. Heading level comes from the nesting alone, as
//! it does in the Markdown renderer this document is the sibling of.
//!
//! A body is inserted as HTML rather than parsed into components. The json5 is authored in
//! six tags and nothing else, checked by the Markdown converter's panic, so there is nothing
//! for a parse to decide; and the spoken-form transforms have to run over the text first
//! either way.

use leptos::prelude::*;

use super::document::Document;
use crate::fixed::{SettingSection, SettingSubsection, setting_overview};
use crate::render::spoken::speak_slots;

/// The whole overview, in the order the content has it.
pub fn view(options: LeptosOptions) -> impl IntoView {
    let overview = setting_overview();
    // One spoken form, used twice: what VoiceOver says on load and what it says at the top of
    // the document have to be the same words.
    let title = speak_slots(overview.title);
    view! {
        <Document options title=title.clone()>
            <h1 inner_html=title></h1>
            {overview.sections.iter().map(section).collect_view()}
        </Document>
    }
}

fn section(section: &'static SettingSection) -> impl IntoView {
    view! {
        <h2 inner_html=speak_slots(section.heading)></h2>
        <div inner_html=speak_slots(section.html)></div>
        {section.subsections.iter().map(subsection).collect_view()}
    }
}

fn subsection(subsection: &'static SettingSubsection) -> impl IntoView {
    view! {
        <h3 inner_html=speak_slots(subsection.heading)></h3>
        <div inner_html=speak_slots(subsection.html)></div>
    }
}
