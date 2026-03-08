
#[derive(sqlx::FromRow, serde::Serialize)]
pub struct ResourceType {
    pub id: i32,
    pub name: String,
    pub status: String,
    pub display: String
}