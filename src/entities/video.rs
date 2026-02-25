use sqlx::PgPool;
use uuid::Uuid;

// Phase 1: insert a record BEFORE the S3 upload.
// This way we always have a trace of the attempt, even if S3 fails later.
//
// `&PgPool` — a shared reference to the connection pool.
// PgPool is internally an Arc (atomic reference count), so cloning it is cheap
// and passing `&PgPool` is idiomatic — no ownership transfer needed.
//
// `Result<(), sqlx::Error>` — like `Promise<void>` in JS, but explicit about failure.
// `()` is the unit type: "success with no value".

pub async fn create_pending(
    pool: &PgPool,
    id: Uuid,
    source_key: &str,
    title: &str,
) -> Result<(), sqlx::Error> {
    // `sqlx::query()` is the runtime version — SQL is checked at runtime, not compile time.
    // The alternative, `sqlx::query!()`, verifies column types at compile time
    // but requires a live DATABASE_URL during `cargo build`. For now, the runtime
    // version keeps the build simpler.
    //
    sqlx::query(
        "INSERT INTO videos (id, source_key, owner_id, title, status)
         VALUES ($1, $2, $3, $4, 'pending')",
    )
    .bind(id)
    .bind(source_key)
    .bind(Uuid::nil()) // all-zeros UUID — placeholder until auth is implemented
    .bind(title)
    .execute(pool)
    .await?; // `?` propagates the error to the caller instead of panicking

    Ok(())
}

// Phase 2: mark the record ready once S3 confirms the upload succeeded.
// If this update fails (e.g. DB is temporarily down), the record stays 'pending'
// and can be reconciled later by a background job — nothing is lost.
pub async fn mark_ready(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE videos SET status = 'ready', updated_at = now() WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
