# Stonetop

A screen-reader-first web app for playing *Stonetop*. Content is adapted from *Stonetop* by
Jeremy Strandberg under CC BY-SA 4.0; see [`LICENSE.md`](LICENSE.md).

## Running it locally

```sh
cargo leptos watch
```

That builds both halves — the axum server and the WASM client — and serves them at
<http://127.0.0.1:3000>, rebuilding on a change. `cargo leptos build` builds without serving,
and `--release` builds both halves optimised. It needs cargo-leptos and the WASM target:

```sh
cargo binstall cargo-leptos          # or `cargo install cargo-leptos --locked`
rustup target add wasm32-unknown-unknown
```

Views so far: `/` and `/setting`.

**Build the server with cargo-leptos, not with `cargo build`.** The two halves have to agree
on where the WASM is, and cargo-leptos is what tells them. A server binary built any other way
serves a document that renders perfectly and never loads its client half.

## Tests

```sh
cargo nextest run --workspace --features stonetop/codegen,stonetop/ssr
```

`--all-features` is not a valid configuration: `stonetop`'s `ssr` and `hydrate` features are
the server and the client, and turning both on runs the client's code during server rendering.
[`hooks/pre-commit`](hooks/pre-commit) and [`hooks/pre-push`](hooks/pre-push) are what CI would
be; `pre-push` also builds with cargo-leptos and runs the `#[ignore]`d tests, which need that
build behind them.

## Regenerating the baked content

```sh
cargo xtask bake
```

The playbooks and the Setting overview are transcribed as json5 under `codegen/json5/` and
baked into `stonetop/src/fixed/generated.rs`, which is checked in.
