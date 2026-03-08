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
    pub tags: Vec<String>,
    pub views: i32,
    pub thumbnail_url: Option<String>,
}
