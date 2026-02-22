# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build          # compile
cargo run            # run the server (listens on http://localhost:3000)
cargo check          # fast type-check without producing a binary
cargo clippy         # lint
cargo test           # run tests
cargo test <name>    # run a single test by name
```

## Architecture

A minimal Rust HTTP API server built with **Axum** (web framework), **Tokio** (async runtime), and **Serde** (JSON serialization).

- `src/main.rs` — entry point; sets up the Axum `Router`, binds TCP on `0.0.0.0:3000`, and serves it with Tokio.
- Routes are registered via `Router::new().route(path, method(handler))`.
- Handlers are `async fn`s that return Axum-compatible types (e.g., `Json<T>` where `T: Serialize`).
- Response structs derive `#[derive(Serialize)]` so Axum can auto-serialize them to JSON and set `Content-Type: application/json`.

Currently there is one endpoint: `GET /ping` → `{"status": "ok", "message": "pong"}`.

The project uses Rust edition 2024.


### More instructions

The final goal is build the system and also understand Rust as a language, its ecosystem, do not write any code without explaining the logic behind it, and explore rust features, comparing with another language (e.g, JavaScript).