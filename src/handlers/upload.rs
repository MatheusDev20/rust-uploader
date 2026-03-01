use axum::extract::{Multipart, State};
use uuid::Uuid;

use crate::entities::video::create_pending;
use crate::entities::video::mark_ready;
use crate::ffmpeg::{extract_thumbnail, transcode_to_mp4};
use crate::http::responses::{ApiResponse, bad_request, internal_error, ok};
use crate::{AppState, utils::constants::UPLOADS_FOLDER};

pub async fn upload_handler(
    State(AppState { uploader, db }): State<AppState>,
    mut multipart: Multipart,
) -> ApiResponse {
    let mut title: Option<String> = None;
    let mut video_file_bytes: Option<Vec<u8>> = None;
    let mut thumbnail_bytes: Option<Vec<u8>> = None;
    let mut source_ext: Option<String> = None;

    // Collect all multipart fields before acting on them.
    // Field order in a multipart body is not guaranteed — the client controls it —
    // so we can't assume "title" arrives before "file".
    while let Some(field) = multipart.next_field().await.unwrap() {
        // `match` is exhaustive pattern matching — like a `switch` in JS, but
        // the compiler forces you to handle every possible case (or use `_` as a catch-all).
        match field.name().unwrap_or("") {
            "title" => {
                title = Some(field.text().await.unwrap());
            }
            "file" => {
                // Extract the extension BEFORE consuming the field with .bytes().
                //
                // field.file_name() → Option<&str> borrowed from `field`, e.g. "video.mkv"
                // .rsplit('.').next() → splits "video.mkv" from the right, takes first: "mkv"
                // .to_lowercase() → converts to an owned String (borrow of `field` ends here)
                //
                // After this expression, `field` is no longer borrowed, so we can call
                // field.bytes() which consumes (moves) `field`.
                //
                // In JS: const ext = file.name?.split('.').pop()?.toLowerCase() ?? 'mp4'
                let ext = field
                    .file_name()
                    .and_then(|name| name.rsplit('.').next())
                    .map(|e| e.to_lowercase())
                    .unwrap_or_else(|| "mp4".to_string());

                source_ext = Some(ext);
                video_file_bytes = Some(field.bytes().await.unwrap().to_vec());
            }

            "thumbnail" => {
                thumbnail_bytes = Some(field.bytes().await.unwrap().to_vec());
            }

            _ => {}
        }
    }

    // `match` on an Option to extract the inner value or return early.
    // This is the idiomatic Rust alternative to null-checking: the type system
    // forces you to handle the `None` case — you can't accidentally use a null value.
    let data = match video_file_bytes {
        Some(b) => b,
        None => return bad_request("missing required field: file"),
    };

    // title is required — return 400 instead of silently defaulting.
    let title = match title {
        Some(t) => t,
        None => return bad_request("missing required field: title"),
    };
    let video_id = Uuid::new_v4();
    let ext = source_ext.unwrap_or_else(|| "mp4".to_string());

    // Source key uses the real extension — no more lying about the format.
    // e.g. "uploads/My Title/550e8400-e29b-41d4-a716-446655440000.mkv"
    let source_key = format!("{}/{}/{}.{}", UPLOADS_FOLDER, title, video_id, ext);

    // Processed key is always .mp4 — this is what the browser/player will use.
    // e.g. "uploads/My Title/550e8400-..._processed.mp4"
    let processed_key = format!("{}/{}/{}_processed.mp4", UPLOADS_FOLDER, title, video_id);

    // Phase 1: write the record BEFORE touching S3.
    // If the server crashes after this point but before Phase 2, the 'pending' record
    // survives — a background job can detect and retry or clean up orphaned uploads.
    if create_pending(&db, video_id, &source_key, &title)
        .await
        .is_err()
    {
        return internal_error("failed to create video record");
    }

    // Upload the original source file as-is.
    // If this fails, the 'pending' record stays in the DB as a trace of the attempt.
    if uploader.upload(&source_key, &data).await.is_err() {
        return internal_error("upload to S3 failed");
    }

    match thumbnail_bytes {
        Some(bytes) => {
            let thumb_key = format!("{}/{}/thumbnail.jpg", UPLOADS_FOLDER, title);
            if uploader.upload(&thumb_key, &bytes).await.is_err() {
                return internal_error("thumbnail upload to S3 failed");
            }
        }
        None => {
            let thumb_key = format!("{}/{}/thumbnail.jpg", UPLOADS_FOLDER, title);
            let thumb_bytes = extract_thumbnail(&data, &title, 2.0);

            match thumb_bytes {
                Ok(bytes) => {
                    if uploader.upload(&thumb_key, &bytes).await.is_err() {
                        return internal_error("thumbnail upload to S3 failed");
                    }
                }
                Err(_) => return internal_error("failed to extract thumbnail"),
            }
        }
    }

    // FIRE-AND-FORGET: spawn an independent background task for transcoding.
    //
    // `tokio::spawn` schedules this async block as a separate task on the Tokio
    // runtime. The HTTP handler returns immediately without waiting for it.
    //
    // In JS this is like calling an async function without `await`:
    //   transcodeAndMark(data, videoId)  // no await — runs in "background"
    //   return res.json({ status: "ok" })
    //
    // `async move { ... }`:
    //   - `async` creates a Future — a value that represents deferred work.
    //     Like a Promise, it doesn't run until something drives it (here: the runtime).
    //   - `move` transfers OWNERSHIP of the captured variables into the block.
    //     Without `move`, the block would borrow them — but the handler is about to
    //     return, dropping its locals. The task must OWN the data it needs.
    //     In JS closures capture by reference and the GC keeps values alive as long
    //     as something holds a reference. In Rust, you make ownership explicit and
    //     the compiler enforces it at compile time — no GC, no surprise lifetimes.
    tokio::spawn(async move {
        let id_str = video_id.to_string();

        // `spawn_blocking` runs a synchronous/blocking closure on a dedicated thread pool,
        // separate from the async executor threads.
        //
        // Why does this matter? Tokio runs async tasks on a small set of OS threads
        // (usually one per CPU core). If a task blocks a thread — e.g. by waiting for
        // ffmpeg to finish a long transcode — ALL other tasks on that thread are frozen.
        // `spawn_blocking` hands the blocking work off to a thread pool reserved for it,
        // keeping the async executor threads free to handle other requests.
        //
        // In JS this is like running CPU-intensive work in a Worker thread instead of
        // blocking the main event loop.
        //
        // Awaiting spawn_blocking gives Result<io::Result<Vec<u8>>, JoinError>:
        //   outer Result: did spawn_blocking itself fail? (e.g. thread panicked)
        //   inner Result: did transcode_to_mp4 return an error?
        let mp4_result = tokio::task::spawn_blocking(move || {
            // `data` and `ext` are moved into this closure.
            // The outer async block can't use them after this point.
            transcode_to_mp4(&data, &id_str, &ext)
        })
        .await;

        let mp4_bytes = match mp4_result {
            Ok(Ok(bytes)) => bytes,
            Ok(Err(e)) => {
                // Transcode failed — leave video as 'pending' for investigation/retry.
                eprintln!("[bg] transcode failed for {video_id}: {e}");
                return;
            }
            Err(e) => {
                // The blocking thread itself panicked — very unusual.
                eprintln!("[bg] spawn_blocking panicked for {video_id}: {e}");
                return;
            }
        };

        // Upload the processed MP4 — this is what clients will actually stream.
        if uploader.upload(&processed_key, &mp4_bytes).await.is_err() {
            eprintln!("[bg] processed MP4 upload failed for {video_id}");
            return;
        }

        // Phase 2: mark 'ready' ONLY after the processed MP4 exists in S3.
        // If this fails, the video stays 'pending' — safe to retry later.
        mark_ready(&db, video_id).await.ok();
    });

    // Return immediately — the video is still 'pending' in the DB.
    // It transitions to 'ready' once the background task finishes transcoding.
    ok("upload received, processing in background")
}
