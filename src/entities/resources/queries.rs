use sqlx::PgPool;
use uuid::Uuid;

use crate::http::responses::CreateResourceError;

pub async fn create_resource(
    pool: &PgPool,
    id: Uuid,
    title: &str,
    resource_type: &str,
    source_type: &str,
    tags: &[String],
) -> Result<(), CreateResourceError> {
    sqlx::query(
        "INSERT INTO resources (id, title, resource_type, source_type, tags)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(id)
    .bind(title)
    .bind(resource_type)
    .bind(source_type)
    .bind(tags)
    .execute(pool)
    .await?;

    Ok(())
}
