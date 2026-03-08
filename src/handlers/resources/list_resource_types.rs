use axum::{Json, extract::State, response::Response, response::IntoResponse };
use reqwest::StatusCode;
use sqlx::PgPool;
use crate::{entities::resource_types::{ResourceType, list}};


#[derive(serde::Serialize)]
pub struct ListResourceTypeResponse {
  pub data: Vec<ResourceType>,
}
pub async fn list_resource_types(State(db): State<PgPool>,) -> Response {
    let result = list(&db).await;

    match result {
      Ok(resource_types) =>  (StatusCode::OK, Json(ListResourceTypeResponse { data: resource_types })).into_response(),
      Err(_) =>   (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ListResourceTypeResponse { data: vec![] }),
            )
                .into_response()
    }
}