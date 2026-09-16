//! The home view: what this is, and the way on.

use leptos::prelude::*;

use super::document::Document;

/// What a stranger who lands on `/` meets: a sentence saying what this is, and the way in.
pub fn view(options: LeptosOptions) -> impl IntoView {
    view! {
        <Document options title="Stonetop">
            <h1>"Stonetop"</h1>
            <p>
                "A way to play " <em>"Stonetop"</em>
                " by ear: character sheets re-expressed as documents that read aloud. Nothing is
                 playable here yet. What there is to read is the "
                <a href="/setting">"Setting overview"</a> "."
            </p>
        </Document>
    }
}
