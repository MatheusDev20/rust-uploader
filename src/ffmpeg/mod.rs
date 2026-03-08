// src/ffmpeg/mod.rs

use std::fs;
use std::io;
use std::process::{Command, Stdio};
pub fn extract_thumbnail(video_bytes: &[u8], id: &str, at_secs: f32) -> io::Result<Vec<u8>> {
    let tmp = std::env::temp_dir();
    let in_path  = tmp.join(format!("{}_th.mp4", id));
    let out_path = tmp.join(format!("{}_th.jpg", id));

    fs::write(&in_path, video_bytes)?;
    let at_str  = format!("{:.1}", at_secs);
    let in_str  = in_path.to_str().unwrap();
    let out_str = out_path.to_str().unwrap();

    let child = Command::new("ffmpeg")
        .args([
            "-hide_banner",
            "-loglevel", "error",
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
