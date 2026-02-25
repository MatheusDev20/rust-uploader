  
use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::routing::{post};
use crate::handlers::upload::upload_handler;

use crate::{AppState, utils::constants::MAX_SIZE};


pub fn init_routes(state: AppState) -> Router  {
  Router::new()
      .route("/upload", post(upload_handler))
      .layer(DefaultBodyLimit::max(MAX_SIZE))
      .with_state(state)
}