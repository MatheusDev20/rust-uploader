pub mod html;
pub mod pdf;
pub mod video;

use html::HtmlExtractor;
use pdf::PdfExtractor;
use video::VideoExtractor;

#[derive(Debug)]
pub enum ExtractorError {
    FetchFailed(String),
    ExtractionFailed(String),
    UnsupportedType(String),
}

// Enum dispatch avoids `dyn Trait` complexity with async methods.
// Adding a new extractor = add a variant here + a new file.
// In JS this would be a map of { [resourceType]: extractorFn }.
pub enum Extractor {
    Html(HtmlExtractor),
    Pdf(PdfExtractor),
    Video(VideoExtractor),
}

impl Extractor {
    pub async fn extract(&self, url: &str) -> Result<String, ExtractorError> {
        match self {
            Extractor::Html(e) => e.extract(url).await,
            Extractor::Pdf(e) => e.extract(url).await,
            Extractor::Video(e) => e.extract(url).await,
        }
    }
}

// Factory: pick the right extractor based on resource_type.
pub fn for_resource_type(resource_type: &str) -> Result<Extractor, ExtractorError> {
    match resource_type {
        "api_documentation" => Ok(Extractor::Html(HtmlExtractor)),
        "internal_docs" => Ok(Extractor::Pdf(PdfExtractor)),
        "video" => Ok(Extractor::Video(VideoExtractor)),
        other => Err(ExtractorError::UnsupportedType(other.to_string())),
    }
}

// Pick the Postgres text-search language config based on source_type.
// This drives stemming and stop-word filtering in tsvector.
pub fn language_for(source_type: &str) -> &'static str {
    match source_type {
        "zendesk_official_docs" => "english",
        _ => "portuguese",
    }
}
