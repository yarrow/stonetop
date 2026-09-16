//! The not-found view. A URL that names nothing is a dead end that says so in words, never a
//! stack trace and never a framework's own error document.

use leptos::prelude::*;

use super::document::Document;

/// What a URL naming nothing answers with.
pub fn view(options: LeptosOptions) -> impl IntoView {
    view! {
        <Document options title="Not found — Stonetop">
            <h1>"Not found"</h1>
            <p>"There is nothing at this address. " <a href="/">"Stonetop's home"</a> " is a place to start."</p>
        </Document>
    }
}
