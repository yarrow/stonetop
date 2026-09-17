//! The app seam: build the real axum app and send it requests, with no network and no
//! listening socket. A test here drives the app the way a browser does and asserts on the
//! document that comes back, which is what a screen reader would be reading.
//!
//! `LeptosOptions` is built here rather than read from the environment, because the
//! environment is what cargo-leptos sets for the server binary and the tests have no
//! cargo-leptos around them. The site root has to be the real one all the same: it is where
//! the app looks for the WASM half.

use std::path::PathBuf;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt as _;
use leptos::prelude::LeptosOptions;
use scraper::{Html, Selector};
use tower::ServiceExt as _;

/// What cargo-leptos writes the client half into, as an absolute path: `site-root` in
/// `stonetop/Cargo.toml` is relative to the workspace root, and a test's working directory
/// is the package.
fn site_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("target/site")
}

/// The same options the binary gets from cargo-leptos's environment. The output name is read
/// from the compile-time environment rather than written down, because that is the same value
/// `leptos` itself compiles into the file names it asks the browser for; see
/// `.cargo/config.toml`. A build where it is unset fails here rather than serving an inert
/// document.
fn options() -> LeptosOptions {
    LeptosOptions::builder()
        .output_name(env!("LEPTOS_OUTPUT_NAME"))
        .site_root(site_root().to_string_lossy().into_owned())
        .site_pkg_dir("pkg")
        .build()
}

/// The response to a GET of `path`.
pub async fn get(path: &str) -> Response {
    let request = Request::builder().uri(path).body(Body::empty()).expect("building the request");
    let response = stonetop::web::router(options())
        .oneshot(request)
        .await
        .expect("the app is infallible, so a response always comes back");
    let status = response.status();
    let bytes = response.into_body().collect().await.expect("reading the response body").to_bytes();
    Response { status, bytes: bytes.to_vec() }
}

/// One response, as a test reads it.
pub struct Response {
    pub status: StatusCode,
    pub bytes: Vec<u8>,
}

impl Response {
    /// The body as text. Panics if it isn't UTF-8, which for a document means the test asked
    /// for the wrong thing.
    pub fn text(&self) -> &str {
        std::str::from_utf8(&self.bytes).expect("the response body is not text")
    }

    /// The body parsed as HTML.
    pub fn document(&self) -> Document {
        Document(Html::parse_document(self.text()))
    }
}

/// A parsed document, with the questions a screen-reader-first test asks of one.
pub struct Document(Html);

impl Document {
    /// The `<title>`: VoiceOver's first utterance on load.
    pub fn title(&self) -> String {
        self.text_of("title").into_iter().next().expect("the document has no <title>")
    }

    /// Every heading in reading order, as its level and its text.
    pub fn headings(&self) -> Vec<(u8, String)> {
        let selector = selector("h1, h2, h3, h4, h5, h6");
        self.0
            .select(&selector)
            .map(|element| {
                let level = element.value().name()[1..].parse().expect("a heading level");
                (level, text(element))
            })
            .collect()
    }

    /// The text of every element matching `css`, in reading order.
    pub fn text_of(&self, css: &str) -> Vec<String> {
        self.0.select(&selector(css)).map(text).collect()
    }

    /// The value of `attribute` on every element matching `css`, in reading order.
    pub fn attrs_of(&self, css: &str, attribute: &str) -> Vec<String> {
        self.0
            .select(&selector(css))
            .filter_map(|element| element.attr(attribute))
            .map(str::to_string)
            .collect()
    }

    /// Every element VoiceOver would chunk: one holding inline markup that is not marked to
    /// speak as one utterance, given as its text. An empty answer is the passing one.
    ///
    /// A paragraph takes `role="text"` itself, having no role to lose. A list item and a
    /// heading both have one worth keeping — `listitem` carries the list's item count, and
    /// `heading` is the rotor — so their text goes inside a marked `<span>` instead, and the
    /// element itself must carry no role at all.
    pub fn chunking_elements(&self) -> Vec<String> {
        let span = selector(r#"span[role="text"]"#);
        self.0
            .select(&selector("p, li, h1, h2, h3, h4, h5, h6"))
            .filter(|element| emphasised(*element))
            .filter(|element| {
                let marked = if element.value().name() == "p" {
                    element.attr("role") == Some("text")
                } else {
                    // `role="text"` on the `<li>` itself would replace `listitem`.
                    element.attr("role").is_none() && element.select(&span).next().is_some()
                };
                !marked
            })
            .map(|element| text(element))
            .collect()
    }

    /// The `href` of every link whose text is `label`.
    pub fn links_labelled(&self, label: &str) -> Vec<String> {
        self.0
            .select(&selector("a[href]"))
            .filter(|element| text(*element) == label)
            .map(|element| element.attr("href").unwrap_or_default().to_string())
            .collect()
    }
}

/// Whether `element` holds inline markup of its own. Emphasis is a *direct* child in the
/// six-tag grammar the content is authored in, so this deliberately does not descend: a
/// nested list's emphasis belongs to the nested item, not to this one.
fn emphasised(element: scraper::ElementRef<'_>) -> bool {
    element
        .children()
        .filter_map(scraper::ElementRef::wrap)
        .any(|child| matches!(child.value().name(), "strong" | "em"))
}

fn text(element: scraper::ElementRef<'_>) -> String {
    element.text().collect::<String>().split_whitespace().collect::<Vec<_>>().join(" ")
}

fn selector(css: &str) -> Selector {
    Selector::parse(css).unwrap_or_else(|e| panic!("{css} is not a selector: {e}"))
}
