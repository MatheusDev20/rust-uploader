# Mini-Tube Code Walkthrough

A step-by-step explanation of the Rust code in `src/main.rs`.

## 1. Imports

```rust
use axum::{routing::get, Json, Router};
use serde::Serialize;
```

`use` is like `import` in Python/JS. The `::` here means "reach inside this module/crate and grab something". So `axum::{routing::get, Json, Router}` means: "from the `axum` crate, import `get` (which lives inside `routing`), `Json`, and `Router`".

## 2. The Response Struct

```rust
#[derive(Serialize)]
struct PingResponse {
    status: &'static str,
    message: &'static str,
}
```

- `struct` is like a class/object shape — it defines what fields the data has.
- `#[derive(Serialize)]` is an **attribute macro** — it auto-generates code that lets this struct be converted to JSON. You don't write the serialization logic yourself.
- `&'static str` means "a reference to a string that lives for the entire program" (string literals like `"ok"` are always `'static`).

## 3. The Handler Function

```rust
async fn ping() -> Json<PingResponse> {
    Json(PingResponse {
        status: "ok",
        message: "pong",
    })
}
```

This is the function that runs when someone hits `/ping`. It returns a `Json<PingResponse>` — Axum automatically sets the `Content-Type: application/json` header and serializes the struct to JSON.

## 4. The Main Function

```rust
#[tokio::main]
async fn main() {
    let app = Router::new().route("/ping", get(ping));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
```

- `#[tokio::main]` — **Tokio is not a variable**, it's an **async runtime** (a library/crate listed in `Cargo.toml`). Rust doesn't have a built-in async runtime like Node.js does, so you need one. This attribute macro transforms your `async fn main()` into a regular `fn main()` that starts the tokio runtime under the hood.
- `Router::new()` — creates a new router, then `.route("/ping", get(ping))` registers the `ping` handler on `GET /ping`.
- `.await` — just like JS, you `await` async operations.
- `.unwrap()` — means "if this fails, crash the program". It's a shortcut for error handling (fine for now, not great for production).

## The `::` Operator vs `.` Operator

`::` is **not** for calling methods on objects. Rust has two separate operators:

| Operator | Name | Example | Meaning |
|----------|------|---------|---------|
| `::` | **Path separator** | `Router::new()` | Access something **associated** with a type/module (like static methods or nested modules) |
| `.` | **Dot operator** | `app.route(...)` | Call a method **on an instance** |

Think of it this way:

- `::` is like accessing a **static method** or a **namespace** — `Router::new()` is like `Router.new()` in Ruby or `Router.New()` in C#.
- `.` is for calling methods on an **existing value** — `listener.bind(...)` operates on a specific listener instance.

So when you see `tokio::net::TcpListener::bind(...)`, read it as: "in the `tokio` crate, inside the `net` module, on the `TcpListener` type, call the associated function `bind`".
