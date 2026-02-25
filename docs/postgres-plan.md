# Plan: Adding PostgreSQL with `sqlx`

## Why `sqlx`?

There are a few crates for database access in Rust:

| Crate | Style | Async-native |
|---|---|---|
| `sqlx` | Raw SQL + compile-time checks | Yes |
| `diesel` | ORM / query builder | No (diesel-async exists) |
| `tokio-postgres` | Low-level driver | Yes |

**`sqlx` is the best fit** here because it's async-first (compatible with Tokio), integrates cleanly with Axum's state pattern, and has a superpower: **compile-time query verification** — the compiler checks your SQL against a real database at build time. That's something JavaScript simply can't do.

---

## Step 1 — Add dependencies to `Cargo.toml`

```toml
sqlx = { version = "0.8", features = ["postgres", "runtime-tokio", "uuid", "migrate"] }
```

- `postgres` — enables the Postgres driver
- `runtime-tokio` — tells sqlx to use Tokio's async runtime (not async-std)
- `uuid` — allows sqlx to map UUID columns directly to Rust's `Uuid` type (you already use it)
- `migrate` — enables running `.sql` migration files programmatically

---

## Step 2 — Add `DATABASE_URL` to `.env`

```
DATABASE_URL=postgres://user:password@localhost:5432/mini_tube
```

This is the same pattern as `process.env.DATABASE_URL` in Node.js.

---

## Step 3 — Create a `PgPool` and inject it into `AppState`

`PgPool` is a **connection pool** — a reusable set of open database connections shared across all requests. Think of it like a pool of pre-established connections (similar to `pg.Pool` in Node's `pg` library).

In Rust, `PgPool` is `Clone + Send + Sync`, which means Axum can safely share it across all async handler threads — it becomes a field on `AppState`.

```rust
pub struct AppState {
    uploader: S3Uploader,
    db: PgPool,          // <-- new field
}
```

In `main()`:
```rust
let db = PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();
let state = AppState { uploader, db };

```

---

## Step 4 — Create a `db/` module and migration files

```
src/
  db/
    mod.rs       ← declares the module, exports types
    video.rs     ← DB queries for videos (insert, fetch, etc.)
migrations/
  0001_create_videos.sql
```

The migration SQL would look like:
```sql
CREATE TABLE videos (
    id UUID PRIMARY KEY,
    s3_key TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

Run migrations either with `sqlx migrate run` (CLI) or programmatically at startup with `sqlx::migrate!()`.

---

## Step 5 — Use the pool in a handler

After uploading to S3, save the video record to Postgres:

```rust
async fn upload(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Json<UploadResponse> {
    // ... existing S3 upload logic ...

    sqlx::query!(
        "INSERT INTO videos (id, s3_key) VALUES ($1, $2)",
        video_id,
        video_key
    )
    .execute(&state.db)
    .await
    .unwrap();
}
```

The `sqlx::query!()` macro is where the compile-time magic happens — it validates the SQL and the Rust types against your database **at compile time**, catching mismatches before the code even runs.

---

## Summary of what changes

| What | Action |
|---|---|
| `Cargo.toml` | Add `sqlx` |
| `.env` | Add `DATABASE_URL` |
| `src/main.rs` | Create `PgPool`, add to `AppState` |
| `src/db/` | New module with query functions |
| `migrations/` | SQL migration files |
