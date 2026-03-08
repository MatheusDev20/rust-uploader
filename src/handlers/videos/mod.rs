mod handler;
mod response;
pub mod upload;

// Re-export handlers so callers import from `handlers::videos` directly,
// without needing to know the internal file layout.
// Same idea as an index.ts barrel file in JS.
pub use handler::{get_video_handler, list_videos_handler, stream_handler};
pub use upload::upload_handler;
