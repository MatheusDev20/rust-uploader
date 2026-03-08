use axum::{
    extract::{FromRequest, Request, rejection::JsonRejection},
    http::StatusCode,
    Json,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use serde_json::json;

// A drop-in replacement for axum::Json that returns a structured JSON error
// on 422 instead of a plain text string.
//
// In JS frameworks like Express, validation errors usually come back as JSON automatically.
// In Axum, the default rejection is plain text — this wrapper fixes that.
//
// Usage: replace `Json(body): Json<T>` with `JsonBody(body): JsonBody<T>` in handlers.
pub struct JsonBody<T>(pub T);

impl<T, S> FromRequest<S> for JsonBody<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<T>::from_request(req, state).await {
            Ok(Json(value)) => Ok(JsonBody(value)),
            Err(rejection) => {
                let status = match rejection {
                    JsonRejection::JsonDataError(_) => StatusCode::UNPROCESSABLE_ENTITY,
                    JsonRejection::JsonSyntaxError(_) => StatusCode::BAD_REQUEST,
                    JsonRejection::MissingJsonContentType(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };

                let body = json!({
                    "status": "error",
                    "message": rejection.body_text(),
                });

                Err((status, Json(body)).into_response())
            }
        }
    }
}
