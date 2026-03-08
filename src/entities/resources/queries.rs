use sqlx::PgPool;
use uuid::Uuid;

use crate::http::responses::CreateResourceError;

pub async fn create_resource(
    pool: &PgPool,
    id: Uuid,
    title: &str,
    slug: &Option<String>,
    resource_type: &str,
    source_type: &str,
    summary: &Option<String>,
    description: &Option<String>,
    url: &Option<String>,
    tags: &[String],
) -> Result<(), CreateResourceError> {

    sqlx::query(
        "INSERT INTO resources (id, title, slug, resource_type, source_type, summary, description, url, tags)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",

    )
    .bind(id)
    .bind(title)
    .bind(slug)
    .bind(resource_type)
    .bind(source_type)
    .bind(summary)
    .bind(description)
    .bind(url)
    .bind(tags)
    .execute(pool)
    .await?;

    Ok(())
}

