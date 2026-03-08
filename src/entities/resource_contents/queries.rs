use sqlx::PgPool;
use uuid::Uuid;

// Inserts the extracted text and computes the tsvector in one shot.
// `to_tsvector($4::regconfig, $3)` casts the language string to a Postgres
// text-search config (e.g. 'portuguese'::regconfig) so stemming and stop-words
// are applied correctly for that language.
pub async fn insert_content(
    pool: &PgPool,
    id: Uuid,
    resource_id: Uuid,
    raw_text: &str,
    language: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO resource_contents (id, resource_id, raw_text, language, search_vector)
         VALUES ($1, $2, $3, $4, to_tsvector($4::regconfig, $3))",
    )
    .bind(id)
    .bind(resource_id)
    .bind(raw_text)
    .bind(language)
    .execute(pool)
    .await?;

    Ok(())
}
