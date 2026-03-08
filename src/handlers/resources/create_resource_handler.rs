use axum::extract::State;
use sqlx::PgPool;

pub async fn new_resource_handler(State(_db): State<PgPool>) {}