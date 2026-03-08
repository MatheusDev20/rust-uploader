pub mod constants;

// Convert a human-readable title into a safe S3 key segment.
// "My Cool Video! (2026)" → "my-cool-video-2026"
//
// Rules:
//   - lowercase everything
//   - replace spaces and underscores with hyphens
//   - strip any character that isn't alphanumeric or a hyphen
//   - collapse consecutive hyphens into one
//
// In JS: title.toLowerCase().replace(/[\s_]+/g, '-').replace(/[^a-z0-9-]/g, '').replace(/-+/g, '-')
pub fn slugify(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| if c == ' ' || c == '_' { '-' } else { c })
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>()
        // collapse runs of hyphens: "my--video" → "my-video"
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
