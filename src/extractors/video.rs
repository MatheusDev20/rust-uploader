use super::ExtractorError;

pub struct VideoExtractor;

impl VideoExtractor {
    pub async fn extract(&self, _url: &str) -> Result<String, ExtractorError> {
        // TODO: send audio to a transcription service (Whisper API, AssemblyAI, etc.)
        // and return the transcript as plain text.
        Err(ExtractorError::ExtractionFailed(
            "video transcription not yet implemented".to_string(),
        ))
    }
}
