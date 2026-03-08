use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct Resource {
    pub id: Uuid,
    pub title: String,
    pub slug: Option<String>,
    pub resource_type: String,
    pub source_type: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub tags: Vec<String>,
    pub published_at: Option<DateTime<Utc>>,
}
