use sqlx::PgPool;
use uuid::Uuid;

// `FromRow` lets sqlx automatically map a DB row to this struct.
// Each field name must match the column name in the SELECT — like an ORM's model in JS.
//
// `Option<String>` for processed_key because it's NULL until the background
// transcoding job finishes and calls mark_ready().
#[derive(sqlx::FromRow)]
pub struct Video {
    pub id: Uuid,
    pub title: String,
    pub status: String,
    pub source_key: String,
    pub processed_key: Option<String>,
}

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
// Now also stores the processed_key so the stream endpoint knows which S3 object to sign.
pub async fn mark_ready(
    pool: &PgPool,
    id: Uuid,
    processed_key: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE videos SET status = 'ready', processed_key = $2, updated_at = now() WHERE id = $1",
    )
    .bind(id)
    .bind(processed_key)
    .execute(pool)
    .await?;

    Ok(())
}

// Fetch a single video by ID.
// Returns Option<Video>: None if the ID doesn't exist — like `findById` in JS ORMs.
// `query_as::<_, Video>` tells sqlx to map each row into a `Video` struct using FromRow.
pub async fn get_video(pool: &PgPool, id: Uuid) -> Result<Option<Video>, sqlx::Error> {
    sqlx::query_as::<_, Video>(
        "SELECT id, title, status, source_key, processed_key FROM videos WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}
