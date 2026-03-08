use super::ExtractorError;

pub struct PdfExtractor;

impl PdfExtractor {
    pub async fn extract(&self, _url: &str) -> Result<String, ExtractorError> {
        // TODO: fetch PDF bytes from URL and extract raw text.
        // Candidates: `pdf-extract` crate (wraps pdftotext) or `lopdf` for pure Rust.
        Err(ExtractorError::ExtractionFailed(
            "PDF extraction not yet implemented".to_string(),
        ))
    }
}
