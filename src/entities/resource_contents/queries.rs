use sqlx::PgPool;
use uuid::Uuid;

// Inserts the extracted text and computes the tsvector in one shot.
// `to_tsvector($4::regconfig, $3)` casts the language string to a Postgres
// text-search config (e.g. 'portuguese'::regconfig) so stemming and stop-words
// are applied correctly for that language.
pub async fn insert_content(
    pool: &PgPool,
    id: Uuid,
    resource_id: Uuid,
    raw_text: &str,
    language: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO resource_contents (id, resource_id, raw_text, language, search_vector)
         VALUES ($1, $2, $3, $4, to_tsvector($4::regconfig, $3))",
    )
    .bind(id)
    .bind(resource_id)
    .bind(raw_text)
    .bind(language)
    .execute(pool)
    .await?;

    Ok(())
}

// Result shape for a search hit — joins resources with resource_contents.
// `rank: f32` maps to Postgres float4 returned by ts_rank().
#[derive(sqlx::FromRow, serde::Serialize)]
pub struct ResourceSearchResult {
    pub id: Uuid,
    pub title: String,
    pub resource_type: String,
    pub source_type: String,
    pub url: Option<String>,
    pub summary: Option<String>,
    pub tags: Vec<String>,
    pub rank: f32,
}

// Full-text search across resource_contents, ranked by relevance.
//
// `plainto_tsquery` converts a plain string like "onboarding dev" into a tsquery
// without requiring the caller to know tsquery syntax — same as a search box input.
//
// `ts_rank` scores each match so the most relevant results come first.
// `language` drives stemming: 'portuguese' stems "conduzindo" → "conduz",
// 'english' stems "running" → "run". Defaults to 'portuguese' if not provided.
pub async fn search(
    pool: &PgPool,
    query: &str,
    language: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<ResourceSearchResult>, sqlx::Error> {
    sqlx::query_as::<_, ResourceSearchResult>(
        "SELECT
            r.id, r.title, r.resource_type, r.source_type,
            r.url, r.summary, r.tags,
            ts_rank(rc.search_vector, plainto_tsquery($1::regconfig, $2)) AS rank
         FROM resources r
         JOIN resource_contents rc ON rc.resource_id = r.id
         WHERE rc.search_vector @@ plainto_tsquery($1::regconfig, $2)
         ORDER BY rank DESC
         LIMIT $3 OFFSET $4",
    )
    .bind(language)
    .bind(query)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}
