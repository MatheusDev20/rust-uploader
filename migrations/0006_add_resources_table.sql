CREATE TABLE resources (
    id UUID PRIMARY KEY,

    title TEXT NOT NULL,
    slug TEXT UNIQUE,

    resource_type TEXT NOT NULL,
    source_type TEXT NOT NULL,

    summary TEXT,
    description TEXT,

    url TEXT,

    tags TEXT[] NOT NULL DEFAULT '{}',

    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,

    published_at TIMESTAMPTZ,
    last_synced_at TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT resources_resource_type_check CHECK (
        resource_type IN (
            'video'
            'api_documentation'
        )
    ),

    CONSTRAINT resources_source_type_check CHECK (
        source_type IN (
            'internal_docs',
            'zendesk_official_docs',
            'videos'
        )
    )
);

CREATE INDEX idx_resources_resource_type ON resources (resource_type);
CREATE INDEX idx_resources_source_type ON resources (source_type);
CREATE INDEX idx_resources_published_at ON resources (published_at DESC);
CREATE INDEX idx_resources_tags_gin ON resources USING GIN (tags);
CREATE INDEX idx_resources_metadata_gin ON resources USING GIN (metadata);