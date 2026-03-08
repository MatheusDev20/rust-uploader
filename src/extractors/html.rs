use scraper::{ElementRef, Html, Node, Selector};

use super::ExtractorError;

pub struct HtmlExtractor;

impl HtmlExtractor {
    pub async fn extract(&self, url: &str) -> Result<String, ExtractorError> {
        let html = reqwest::get(url)
            .await
            .map_err(|e| ExtractorError::FetchFailed(e.to_string()))?
            .text()
            .await
            .map_err(|e| ExtractorError::FetchFailed(e.to_string()))?;

        let document = Html::parse_document(&html);

        // Try progressively broader content areas.
        // `<main>` and `<article>` typically exclude nav/sidebar/footer.
        let candidates = ["main", "article", "[role='main']", "body"];

        for selector_str in &candidates {
            let Ok(selector) = Selector::parse(selector_str) else {
                continue;
            };

            if let Some(element) = document.select(&selector).next() {
                let text = extract_text(element);
                if !text.is_empty() {
                    return Ok(text);
                }
            }
        }

        Err(ExtractorError::ExtractionFailed(
            "no content found in page".to_string(),
        ))
    }
}

// Tags whose text content we never want — CSS rules, JS code, hidden elements.
const SKIP_TAGS: &[&str] = &["script", "style", "noscript", "head"];

// Recursively walk the element tree and collect text nodes,
// skipping any subtree rooted at a tag in SKIP_TAGS.
//
// This is necessary because scraper's built-in .text() iterator
// visits ALL text nodes including those inside <style> and <script>,
// which produces raw CSS/JS mixed into the extracted content.
//
// Think of it like a depth-first tree walk in JS:
//   function extractText(node) {
//     if (SKIP_TAGS.includes(node.tagName)) return ''
//     return [...node.childNodes].map(child =>
//       child.nodeType === Node.TEXT_NODE ? child.textContent : extractText(child)
//     ).join(' ')
//   }
fn extract_text(element: ElementRef) -> String {
    let mut parts: Vec<String> = Vec::new();

    for child in element.children() {
        match child.value() {
            // Text node — this is actual readable content, keep it.
            Node::Text(text) => {
                let trimmed = text.trim();
                if !trimmed.is_empty() {
                    parts.push(trimmed.to_string());
                }
            }

            // Element node — recurse unless it's a tag we want to skip.
            Node::Element(el) => {
                if SKIP_TAGS.contains(&el.name()) {
                    continue;
                }

                if let Some(child_ref) = ElementRef::wrap(child) {
                    let child_text = extract_text(child_ref);
                    if !child_text.is_empty() {
                        parts.push(child_text);
                    }
                }
            }

            // Comments, doctypes, etc. — ignore.
            _ => {}
        }
    }

    // Collapse all whitespace so the final string is clean.
    parts.join(" ").split_whitespace().collect::<Vec<_>>().join(" ")
}
