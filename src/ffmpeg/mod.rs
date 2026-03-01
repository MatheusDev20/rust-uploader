// src/ffmpeg/mod.rs
//
// This module wraps ffmpeg as a subprocess and exposes Rust functions
// for video processing operations. Each function is a free function
// (not a method on a struct) because there is no shared state — no
// configuration that lives between calls. In JS you might still use
// a class for grouping, but in Rust a module IS the grouping unit.
// A zero-state struct with methods adds boilerplate without benefit.

use std::fs;
use std::io;
use std::process::{Command, Stdio};

// WHY DOES THIS FUNCTION ACCEPT BYTES INSTEAD OF A FILE PATH?
//
// In our upload flow the video arrives over HTTP as a multipart field,
// so the handler already has it as Vec<u8> in memory. Accepting &[u8]
// (a borrowed slice — like a read-only view into a Vec<u8>) means:
//
//   - No ownership transfer — we borrow the data, not consume it.
//   - The caller can keep using the same buffer if needed.
//   - No coupling to the file system at the API boundary.
//
// The trade-off: ffmpeg cannot process MP4 from a pipe because the
// MP4 container stores its metadata (moov atom) at the END of the file.
// ffmpeg needs to seek backwards to read it, which is impossible on a
// pipe. So we must write to a real temp file anyway — accepting bytes
// adds one extra memory-to-disk copy compared to accepting a path.
//
// For this project (HTTP upload → process → store) &[u8] is the right
// choice. If the file were already on disk (e.g., from a previous step),
// passing a &Path would save that one write.
//
// In JS this would look like:
//   async function extractAudio(videoBytes: Buffer, id: string): Promise<Buffer>
pub fn extract_audio(video_bytes: &[u8], id: &str) -> io::Result<Vec<u8>> {
    // `std::env::temp_dir()` returns the OS temp directory, e.g. /tmp on Linux.
    // This is equivalent to `os.tmpdir()` in Node.js.
    let tmp = std::env::temp_dir();

    // `PathBuf::join` appends a path segment — like path.join() in Node.js.
    // The format!() macro works like template literals: `${id}.mp4` in JS.
    let in_path = tmp.join(format!("{}.mp4", id));
    let out_path = tmp.join(format!("{}.wav", id));

    // Write the raw bytes to a temp file so ffmpeg can seek over them.
    // `?` propagates the error to the caller — equivalent to:
    //   if (err) return Promise.reject(err)  — but at compile time enforced.
    fs::write(&in_path, video_bytes)?;

    // Spawn ffmpeg as a child process.
    // `Command` is Rust's equivalent of child_process.spawn() in Node.js.
    let child = Command::new("ffmpeg")
        .args([
            "-hide_banner",
            "-loglevel", "error",        // suppress all output except errors
            "-i", in_path.to_str().unwrap(), // input file
            "-vn",                       // drop the video stream, keep only audio
            "-ac", "1",                  // downmix to mono (1 channel)
            "-ar", "16000",              // resample to 16 kHz — standard for speech models
            "-y",                        // overwrite output file if it already exists
            out_path.to_str().unwrap(),  // output file
        ])
        .stdout(Stdio::null())   // discard stdout — we only care about the file it writes
        .stderr(Stdio::piped())  // capture stderr so we can include it in error messages
        .spawn()?;               // `spawn` starts the process; `?` returns early on failure

    // `wait_with_output()` blocks until the child exits and collects all
    // piped I/O. In Node.js this is like awaiting the 'close' event and
    // reading the accumulated chunks from the stream.
    let output = child.wait_with_output()?;

    // Always clean up the input file, even if ffmpeg failed.
    // `.ok()` discards the Result — equivalent to a try/catch that does nothing.
    // We don't want a cleanup failure to mask the real error.
    fs::remove_file(&in_path).ok();

    if !output.status.success() {
        // Clean up partial output before returning
        fs::remove_file(&out_path).ok();

        // Build a human-readable error from ffmpeg's stderr.
        // `from_utf8_lossy` replaces invalid UTF-8 sequences with '?' instead
        // of panicking — defensive parsing, equivalent to Buffer.toString('utf8')
        // with error handling.
        // `.into_owned()` converts the Cow<str> into a String we can own.
        let msg = String::from_utf8_lossy(&output.stderr).into_owned();
        return Err(io::Error::new(io::ErrorKind::Other, msg));
    }

    // Read the output file into memory, then clean it up.
    // `fs::read` returns Vec<u8> — the owned equivalent of a Node.js Buffer.
    let audio_bytes = fs::read(&out_path)?;
    fs::remove_file(&out_path).ok();

    // `Ok(...)` wraps the success value in the Result type.
    // In JS this is the resolved value of a Promise.
    Ok(audio_bytes)
}

// WHY `at_secs: f32` INSTEAD OF A HARDCODED VALUE?
//
// Thumbnails are often picked at 2–3 s: early enough to skip black leader
// frames (common in many recordings), late enough to show real content.
// Accepting the timestamp as a parameter lets the caller tune it per video

// WHY f32 AND NOT u32?
//
// f32 allows sub-second precision (e.g. 1.5) at the cost of potential
// floating-point edge cases. For 1-5s thumbnails, any imprecision is
// irrelevant — we're talking about microsecond drift at most. u32 would be
// simpler but unnecessarily restrictive.
//
// ABOUT THE TEMP FILE NAMES:
//
// Both functions in this module use `id` to name their temp files.
// If they were called concurrently with the same `id`, they'd race on the
// same path. We avoid that by using distinct suffixes per operation:
//   extract_audio     → {id}.mp4   (input)  / {id}.wav   (output)
//   extract_thumbnail → {id}_th.mp4 (input) / {id}_th.jpg (output)
// The caller (our upload handler) uses a UUID per video, so in practice
// the same id is never reused concurrently — but defensive naming costs nothing.
pub fn extract_thumbnail(video_bytes: &[u8], id: &str, at_secs: f32) -> io::Result<Vec<u8>> {
    let tmp = std::env::temp_dir();
    let in_path  = tmp.join(format!("{}_th.mp4", id));
    let out_path = tmp.join(format!("{}_th.jpg", id));

    fs::write(&in_path, video_bytes)?;

    // Pre-format dynamic values into owned Strings BEFORE building the args array.
    //
    // Why? Rust array literals require every element to have the same type.
    // String literals are `&str`. Dynamic values like `at_secs.to_string()` produce
    // `String` — a different type. We can't mix them in `[...]` directly.
    //
    // `.as_str()` converts `&String` → `&str`, making all elements the same type.
    // The String must be kept alive in a variable; a temporary (`&format!(...)`)
    // would be dropped before the array is used — the compiler rejects that.
    //
    // In JS this is invisible: you'd just write `${atSecs}` in a template literal.
    // In Rust, string slices are views into memory that must outlive their usage.
    let at_str  = format!("{:.1}", at_secs);         // e.g. "2.0"
    let in_str  = in_path.to_str().unwrap();
    let out_str = out_path.to_str().unwrap();

    let child = Command::new("ffmpeg")
        .args([
            "-hide_banner",
            "-loglevel", "error",
            // Placing -ss BEFORE -i is "input seek":
            // ffmpeg jumps to the nearest keyframe at or before `at_secs`,
            // then decodes forward to the exact frame. This is O(1) for large
            // files — it does NOT decode every frame from the beginning.
            //
            // Placing -ss AFTER -i would be "output seek": ffmpeg decodes and
            // discards every frame from 0 up to `at_secs` — O(n), much slower.
            "-ss", at_str.as_str(),
            "-i", in_str,
            "-vframes", "1",    // extract exactly one frame, then stop
            "-q:v",    "2",     // JPEG quality scale: 1 (best) → 31 (worst)
                                // 2 gives visually lossless quality at ~50-100 KB
            "-y",               // overwrite output if it exists
            out_str,
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()?;

    let output = child.wait_with_output()?;
    fs::remove_file(&in_path).ok();

    if !output.status.success() {
        fs::remove_file(&out_path).ok();
        let msg = String::from_utf8_lossy(&output.stderr).into_owned();
        return Err(io::Error::new(io::ErrorKind::Other, msg));
    }

    let jpeg_bytes = fs::read(&out_path)?;
    fs::remove_file(&out_path).ok();

    Ok(jpeg_bytes)
}

// WHY TRANSCODE INSTEAD OF STORING THE ORIGINAL?
//
// Browsers support a limited set of video codecs via the <video> tag.
// H.264 + AAC inside an MP4 container is the universal baseline — it plays
// in every browser without plugins. Formats like MKV or AVI may use codecs
// (e.g. HEVC, Vorbis) that many browsers reject entirely.
//
// We also set `-movflags +faststart`: by default, MP4 stores its metadata
// (the "moov atom") at the END of the file. The browser must download the
// entire file before it can start playing. `+faststart` reorganizes the file
// so the moov atom is at the BEGINNING — the browser can play while still
// downloading. This is the HTTP progressive download trick YouTube uses for
// low-res previews.
//
// `src_ext` is the source file's original extension (e.g. "mkv", "webm").
// We use it to name the temp input file so ffmpeg knows what demuxer to use.
// ffmpeg detects format by file extension when the content-type header isn't
// available on disk — passing the wrong extension would make it guess wrong.
//
// In JS: async function transcodeToMp4(bytes: Buffer, id: string, srcExt: string): Promise<Buffer>
pub fn transcode_to_mp4(video_bytes: &[u8], id: &str, src_ext: &str) -> io::Result<Vec<u8>> {
    let tmp = std::env::temp_dir();

    // Use a `_tc` suffix to avoid colliding with extract_audio (`{id}.mp4`)
    // or extract_thumbnail (`{id}_th.mp4`) temp files when called concurrently
    // with the same id.
    let in_path = tmp.join(format!("{}_tc.{}", id, src_ext));
    let out_path = tmp.join(format!("{}_tc.mp4", id));

    fs::write(&in_path, video_bytes)?;

    let in_str = in_path.to_str().unwrap();
    let out_str = out_path.to_str().unwrap();

    let child = Command::new("ffmpeg")
        .args([
            "-hide_banner",
            "-loglevel", "error",
            "-i", in_str,
            "-c:v", "libx264",         // H.264: the most compatible video codec
            "-c:a", "aac",             // AAC: the standard audio codec for MP4
            "-movflags", "+faststart", // move moov atom to front — enables streaming
            "-y",                      // overwrite output if it exists
            out_str,
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()?;

    let output = child.wait_with_output()?;
    fs::remove_file(&in_path).ok();

    if !output.status.success() {
        fs::remove_file(&out_path).ok();
        let msg = String::from_utf8_lossy(&output.stderr).into_owned();
        return Err(io::Error::new(io::ErrorKind::Other, msg));
    }

    let mp4_bytes = fs::read(&out_path)?;
    fs::remove_file(&out_path).ok();

    Ok(mp4_bytes)
}
