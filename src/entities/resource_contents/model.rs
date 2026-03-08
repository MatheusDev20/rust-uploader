use uuid::Uuid;

// `search_vector` is excluded — sqlx has no native TSVECTOR type.
// We write it via raw SQL (`to_tsvector(...)`) and let Postgres own it.
#[derive(sqlx::FromRow)]
pub struct ResourceContent {
    pub id: Uuid,
    pub resource_id: Uuid,
    pub raw_text: String,
    pub language: String,
}
