//! The frame every document shares.

use leptos::prelude::*;

/// One complete document. `title` becomes the `<title>`, which is VoiceOver's first utterance
/// on load and so has to say where the reader is; `children` is the view itself, inside the
/// one `<main>`.
#[component]
pub fn Document(
    options: LeptosOptions,
    #[prop(into)] title: String,
    children: Children,
) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <title>{title}</title>
                <AutoReload options=options.clone() />
                <HydrationScripts options islands=true />
            </head>
            <body>
                <main>{children()}</main>
            </body>
        </html>
    }
}
