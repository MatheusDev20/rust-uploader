
use sqlx::PgPool;

use crate::entities::resource_types::model::ResourceType;

pub async fn list (pool: &PgPool) -> Result<Vec<ResourceType>, sqlx::Error> {

  let resource_types = sqlx::query_as::<_, ResourceType>("SELECT * FROM resource_types")
      .fetch_all(pool)
      .await?;

    Ok(resource_types)
}