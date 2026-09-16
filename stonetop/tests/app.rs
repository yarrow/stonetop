//! What the app answers with, asserted at the app seam: the real router, real requests, and
//! the response parsed as HTML. See `tests/seam/mod.rs`.
#![cfg(feature = "ssr")]

mod seam;

use stonetop::fixed::setting_overview;

/// The overview's own shape decides the view's: one H2 per section, one H3 per
/// subsection, in the order the content has them. Asserted against the content rather than
/// against a fixed list, so adding a section to the json5 doesn't need this test edited.
#[tokio::test]
async fn the_setting_documents_headings_are_the_overviews_own_nesting() {
    let overview = setting_overview();
    let mut expected = vec![(1, overview.title.to_string())];
    for section in overview.sections {
        expected.push((2, section.heading.to_string()));
        for subsection in section.subsections {
            expected.push((3, subsection.heading.to_string()));
        }
    }

    let response = seam::get("/setting").await;
    assert!(response.status.is_success(), "GET /setting answered {}", response.status);
    assert_eq!(response.document().headings(), expected);
}

/// Rotor-by-heading is only navigation if the levels step one at a time.
#[tokio::test]
async fn the_setting_document_skips_no_heading_level() {
    let headings = seam::get("/setting").await.document().headings();
    let mut previous = 0;
    for (level, heading) in &headings {
        assert!(
            *level <= previous + 1,
            "{heading:?} is an h{level} under an h{previous}, a skipped level"
        );
        previous = *level;
    }
    assert_eq!(headings.iter().filter(|(level, _)| *level == 1).count(), 1, "not exactly one h1");
}

/// The first thing VoiceOver speaks on load has to say where the reader is.
#[tokio::test]
async fn the_setting_views_title_is_the_overviews_title() {
    assert_eq!(seam::get("/setting").await.document().title(), setting_overview().title);
}

/// The bodies are the json5's, inserted as HTML: the words arrive, and the markup with them.
#[tokio::test]
async fn the_setting_document_carries_the_overviews_prose_as_markup() {
    let response = seam::get("/setting").await;
    assert!(
        response.text().contains("an isolated village near the edge of the known world"),
        "the premise is missing from /setting"
    );
    assert!(
        response.text().contains("<strong>Stonetop</strong>"),
        "the bodies' markup did not survive; it reached the document as text"
    );
}

/// The root is not a blank document, and it is the way to the overview.
#[tokio::test]
async fn the_home_view_links_to_the_setting_overview() {
    let response = seam::get("/").await;
    assert!(response.status.is_success(), "GET / answered {}", response.status);
    assert_eq!(response.document().links_labelled("Setting overview"), ["/setting"]);
}

/// A URL that names nothing is a dead end that says so, not a crash and not a framework's
/// own error document.
#[tokio::test]
async fn an_unknown_route_is_a_plain_not_found_document() {
    let response = seam::get("/no-such-thing").await;
    assert_eq!(response.status, axum::http::StatusCode::NOT_FOUND);
    let document = response.document();
    assert_eq!(document.headings(), [(1, "Not found".to_string())]);
    assert_eq!(document.links_labelled("Stonetop's home"), ["/"]);
}

/// Neither view holds a control, so neither needs the client half to read: with JavaScript
/// off, what the server wrote is the whole document. That is true as long as nothing on these
/// two views is an `#[island]`, which is what this asserts. The day one appears, this goes red,
/// and whether the view still reads without JavaScript becomes a real question again.
#[tokio::test]
async fn neither_view_depends_on_the_client_half_to_be_read() {
    for path in ["/", "/setting"] {
        let response = seam::get(path).await;
        assert!(
            !response.text().contains("<leptos-island"),
            "{path} holds an island, so it no longer reads the same with JavaScript off"
        );
    }
}

/// The client half is served, and it carries no content.
///
/// The URLs are taken from the document rather than written down here, so this asks the
/// question a browser asks: does what the document sends for actually arrive? A 404 on the
/// WASM leaves a document that renders perfectly and is completely inert, with nothing in it to
/// say so, and the file name is settled at compile time, so writing the name into the test
/// would test the test. That the WASM holds no baked content is the whole point of keeping the
/// content behind `ssr`; in islands mode it holds because a `#[component]` never reaches the
/// client, and the grep is what proves it still does.
///
/// Ignored because it needs a cargo-leptos build behind it. `hooks/pre-push` runs the ignored
/// tests.
#[tokio::test]
#[ignore = "needs `cargo leptos build` to have put the client half under target/site/pkg"]
async fn the_client_half_the_document_asks_for_is_served_and_carries_no_baked_content() {
    let document = seam::get("/setting").await.document();
    let script = only(document.attrs_of("link[rel=\"modulepreload\"]", "href"), "a module preload");
    let wasm = only(document.attrs_of("link[type=\"application/wasm\"]", "href"), "a WASM preload");

    for url in [&script, &wasm] {
        let response = seam::get(url).await;
        assert!(
            response.status.is_success(),
            "the document asks for {url}, and the app answers {}",
            response.status
        );
        assert!(!response.bytes.is_empty(), "{url} arrived empty");
    }

    let baked = b"isolated village near the edge of the known world";
    let bytes = seam::get(&wasm).await.bytes;
    assert!(
        !bytes.windows(baked.len()).any(|window| window == baked),
        "{wasm} carries baked content: it should never have been compiled for wasm32"
    );
}

/// The one thing `found`, described as `what`, holds.
fn only(found: Vec<String>, what: &str) -> String {
    let [one] = <[String; 1]>::try_from(found)
        .unwrap_or_else(|found| panic!("the document has {} of {what}, not one", found.len()));
    one
}
