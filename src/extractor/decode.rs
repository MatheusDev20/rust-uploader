use std::process::{Command, Stdio};
use std::fs;

#[derive(Clone)]
pub struct AudioExtractor;

impl AudioExtractor {
    pub fn new() -> AudioExtractor {
        AudioExtractor
    }

    pub fn extract_audio(&self, input_bytes: &[u8], id: &str) -> std::io::Result<Vec<u8>> {
        let tmp = std::env::temp_dir();

        // MP4 stores its metadata (moov atom) at the END of the file by default.
        // When ffmpeg reads from a pipe it cannot seek backwards to find it → "no streams".
        // Writing to a real file lets ffmpeg seek freely, solving both the moov and WAV header issues.
        let in_path = tmp.join(format!("{}.mp4", id));
        let out_path = tmp.join(format!("{}.wav", id));

        fs::write(&in_path, input_bytes)?;

        let mut child = Command::new("ffmpeg")
            .args([
                "-hide_banner",
                "-loglevel", "error",
                "-i", in_path.to_str().unwrap(),
                "-vn",          // drop video stream
                "-ac", "1",     // mono
                "-ar", "16000", // 16kHz (standard for speech models)
                "-y",           // overwrite if exists
                out_path.to_str().unwrap(),
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()?;

        let output = child.wait_with_output()?;

        // Always clean up temp files, even on error
        fs::remove_file(&in_path).ok();

        if !output.status.success() {
            fs::remove_file(&out_path).ok();
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, err));
        }

        let audio_bytes = fs::read(&out_path)?;
        fs::remove_file(&out_path).ok();

        Ok(audio_bytes)
    }
}
