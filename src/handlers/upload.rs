

use axum::{Json, extract::{Multipart, State}};
use uuid::Uuid;

use crate::{AppState, http::responses::UploadResponse, utils::constants::UPLOADS_FOLDER};
use crate::entities::video::create_pending;
use crate::entities::video::mark_ready;

pub async fn upload_handler(
    State(AppState {uploader, db} ): State<AppState>,
    mut multipart: Multipart,
) -> Json<UploadResponse> {
    let mut title: Option<String> = None;
    let mut file_bytes: Option<Vec<u8>> = None;

    // Collect all multipart fields before acting on them.
    // Field order in a multipart body is not guaranteed — the client controls it —
    // so we can't assume "title" arrives before "file".
    while let Some(field) = multipart.next_field().await.unwrap() {
        // `match` is exhaustive pattern matching — like a `switch` in JS, but
        // the compiler forces you to handle every possible case (or use `_` as a catch-all).
        match field.name().unwrap_or("") {
            "title" => {
                // `.text()` reads the field as a UTF-8 string.
                // `Some(...)` wraps the value, signaling "this field was provided".
                title = Some(field.text().await.unwrap());
            }

            "file" => {
                // `.bytes()` collects the entire field into memory as raw bytes (Vec<u8>)
                file_bytes = Some(field.bytes().await.unwrap().to_vec());
            }
            _ => {} // ignore any other fields
        }
    }

    // `match` on an Option to extract the inner value or return early.
    // This is the idiomatic Rust alternative to null-checking: the type system

    // forces you to handle the `None` case — you can't accidentally use a null value.
    let data = match file_bytes {
        Some(b) => b,
        None => return Json(UploadResponse { status: "error", message: "no file field found in request" }),
    };

    // `unwrap_or_else` provides a fallback if the Option is None.
    // The closure `|| "Untitled".to_string()` is lazy — it only runs if needed.
    // This is cheaper than `unwrap_or("Untitled".to_string())`, which would allocate

    // the String eagerly even when title is already Some.
    let title = title.unwrap_or_else(|| "Untitled".to_string());

    let video_id = Uuid::new_v4();
    let video_key = format!("{}/{}.mp4", UPLOADS_FOLDER, video_id);

    // Phase 1: write the record BEFORE touching S3.
    // If the server crashes after this point but before Phase 2, the 'pending' record
    // survives — a background job can detect and retry or clean up orphaned uploads.
    // `.is_err()` checks the Result without consuming it. Equivalent to catching
    // a thrown error in JS: if (await createPending(...).catch(() => false)) { ... }
    if create_pending(&db, video_id, &video_key, &title).await.is_err() {
        return Json(UploadResponse { status: "error", message: "failed to create video record" });
    }

    // S3 upload — if this fails, the 'pending' record stays in the DB.
    // That's intentional: we know the upload was attempted and can investigate.
    if uploader.upload(&video_key, data).await.is_err() {
        return Json(UploadResponse { status: "error", message: "upload to S3 failed" });
    }

    // Phase 2: confirm success by updating the status to 'ready'.
    if mark_ready(&db, video_id).await.is_err() {
        return Json(UploadResponse { status: "error", message: "failed to mark video ready" });
    }

    Json(UploadResponse {
        status: "ok",
        message: "video uploaded",
    })
}