//! The axum app. Every route answers with one complete view, rendered on the server and
//! finished before a byte is sent: nothing may swap in underneath VoiceOver mid-utterance.
//! There is no client-side router, so a change of view is a real navigation; what the browser
//! gets is the WASM half, which hydrates controls and nothing else.

mod document;
mod home;
mod not_found;
mod setting;

use axum::Router;
use axum::body::Body;
use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::response::Response;
use axum::routing::get;
use leptos::prelude::{IntoView, LeptosOptions};

/// The whole app over `options`: the view routes, the files cargo-leptos builds under the
/// site pkg directory (the WASM half among them), and a plain not-found document for anything
/// else. Tests build this and send requests straight to it, with no network.
pub fn router(options: LeptosOptions) -> Router {
    // Rendering a view spawns the future that builds it, onto a global executor that
    // `leptos_axum` initialises only along its own routing paths. We take none of them: there
    // is no client-side router, so the routes below are plain axum. Idempotent — the error a
    // second call returns is the one we drop.
    let _ = any_spawner::Executor::init_tokio();

    Router::new()
        .route("/", get(home_route))
        .route("/setting", get(setting_route))
        .route_service(
            &leptos_axum::site_pkg_dir_service_route_path(&options),
            leptos_axum::site_pkg_dir_service(&options),
        )
        .fallback(not_found_route)
        .with_state(options)
}

async fn home_route(State(options): State<LeptosOptions>, request: Request<Body>) -> Response {
    render(options, request, home::view).await
}

async fn setting_route(State(options): State<LeptosOptions>, request: Request<Body>) -> Response {
    render(options, request, setting::view).await
}

/// Everything else. The not-found view takes the same rendering path as the two real ones, so
/// that the answer to a typo is a document like any other and not a framework's error page.
async fn not_found_route(State(options): State<LeptosOptions>, request: Request<Body>) -> Response {
    let mut response = render(options, request, not_found::view).await;
    *response.status_mut() = StatusCode::NOT_FOUND;
    response
}

/// `view` as one complete document. `render_app_async` is Leptos's async rendering mode: it
/// resolves everything the view awaits and only then writes the document, which is the one
/// SSR mode that doesn't fight a screen reader.
async fn render<V: IntoView + 'static>(
    options: LeptosOptions,
    request: Request<Body>,
    view: fn(LeptosOptions) -> V,
) -> Response {
    leptos_axum::render_app_async(move || view(options.clone()))(request).await
}
