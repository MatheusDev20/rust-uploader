use sqlx::PgPool;
use uuid::Uuid;

use crate::http::responses::CreateVideoError;
use super::model::Video;

// Phase 1: insert a record BEFORE the S3 upload.
// This way we always have a trace of the attempt, even if S3 fails later.
//
// `&PgPool` — a shared reference to the connection pool.
// PgPool is internally an Arc (atomic reference count), so cloning it is cheap
// and passing `&PgPool` is idiomatic — no ownership transfer needed.
pub async fn create_pending(
    pool: &PgPool,
    id: Uuid,
    source_key: &str,
    title: &str,
    tags: &[String],
) -> Result<(), CreateVideoError> {
    if let Some(_row) = sqlx::query("SELECT id FROM videos WHERE title = $1")
        .bind(title)
        .fetch_optional(pool)
        .await?
    {
        return Err(CreateVideoError::AlreadyExists);
    }

    sqlx::query(
        "INSERT INTO videos (id, source_key, owner_id, title, status, tags)
         VALUES ($1, $2, $3, $4, 'pending', $5)",
    )
    .bind(id)
    .bind(source_key)
    .bind(Uuid::nil()) // all-zeros UUID — placeholder until auth is implemented
    .bind(title)
    .bind(tags)
    .execute(pool)
    .await?;

    Ok(())
}

// Store the S3 key of the thumbnail once it has been successfully uploaded.
// Called right after the thumbnail upload succeeds, before transcoding starts.
pub async fn set_thumbnail_url(
    pool: &PgPool,
    id: Uuid,
    thumbnail_url: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE videos SET thumbnail_url = $2, updated_at = now() WHERE id = $1",
    )
    .bind(id)
    .bind(thumbnail_url)
    .execute(pool)
    .await?;

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

// Fetch a paginated list of videos, newest first.
// `limit` caps how many rows are returned; `offset` skips the first N rows.
// This is the simplest form of pagination — same as SQL's LIMIT/OFFSET.
// In JS ORMs: Video.findAll({ limit, offset, order: [['created_at', 'DESC']] })
pub async fn list_videos(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<Video>, sqlx::Error> {
    sqlx::query_as::<_, Video>(
        "SELECT id, title, status, source_key, processed_key, tags, views, thumbnail_url
         FROM videos
         ORDER BY created_at DESC
         LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

// Fetch a single video by ID.
// Returns Option<Video>: None if the ID doesn't exist — like `findById` in JS ORMs.
// `query_as::<_, Video>` tells sqlx to map each row into a `Video` struct using FromRow.
pub async fn get_video(pool: &PgPool, id: Uuid) -> Result<Option<Video>, sqlx::Error> {
    sqlx::query_as::<_, Video>(
        "SELECT id, title, status, source_key, processed_key, tags, views, thumbnail_url
         FROM videos WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}
