//! The server binary: one process serving one complete document per request.
//!
//! Configuration comes from the environment cargo-leptos sets (`LEPTOS_SITE_ROOT`,
//! `LEPTOS_SITE_ADDR`, and the rest), with the defaults in `stonetop/Cargo.toml`'s
//! `[package.metadata.leptos]`, so `cargo leptos watch` needs nothing else.

use leptos::prelude::get_configuration;

#[tokio::main]
async fn main() {
    let options = get_configuration(None).expect("reading the Leptos configuration").leptos_options;
    let address = options.site_addr;
    let app = stonetop::web::router(options);

    let listener =
        tokio::net::TcpListener::bind(address).await.expect("binding the listening socket");
    println!("Stonetop is listening on http://{address}");
    axum::serve(listener, app).await.expect("serving");
}
