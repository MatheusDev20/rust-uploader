use axum::{Json, http::StatusCode};
use serde::{ser::SerializeStruct, Serialize};

pub struct UploadResponse {
    pub status: &'static str,
    pub message: &'static str,
}

impl Serialize for UploadResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("UploadResponse", 2)?;
        state.serialize_field("status", &self.status)?;
        state.serialize_field("message", &self.message)?;
        state.end()
    }
}

// Type alias for the handler return type.
// In JS this would be: type ApiResponse = [StatusCode, UploadResponse]
// Here it's a tuple — Axum reads the first element as the HTTP status code
// and the second as the response body. No need to build a Response object manually.
pub type ApiResponse = (StatusCode, Json<UploadResponse>);

// Helper constructors — one per HTTP status we actually use.
// Each takes the message and wraps it in the right tuple.
// The `status` field in the JSON body mirrors the HTTP status semantically,
// so callers don't need to think about both independently.

pub fn bad_request(message: &'static str) -> ApiResponse {
    (StatusCode::BAD_REQUEST, Json(UploadResponse { status: "error", message }))
}

pub fn internal_error(message: &'static str) -> ApiResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, Json(UploadResponse { status: "error", message }))
}

pub fn ok(message: &'static str) -> ApiResponse {
    (StatusCode::OK, Json(UploadResponse { status: "ok", message }))
}


// Domain Errors

#[derive(Debug)]
pub enum CreateVideoError {
    AlreadyExists,
    Database(sqlx::Error),
}

impl From<sqlx::Error> for CreateVideoError {
    fn from(err: sqlx::Error) -> Self {
        CreateVideoError::Database(err)
    }
}

#[derive(Debug)]
pub enum CreateResourceError {
    GenericError,
    Database(sqlx::Error)
}

impl From<sqlx::Error> for CreateResourceError {
    fn from(err: sqlx::Error) -> Self {
        CreateResourceError::Database(err)
    }
}
