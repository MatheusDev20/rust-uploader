CREATE TABLE resources (
    id UUID PRIMARY KEY,

    title TEXT NOT NULL,
    slug TEXT UNIQUE,

    status TEXT NOT NULL DEFAULT 'pending',

    resource_type TEXT NOT NULL,
    source_type TEXT NOT NULL,

    summary TEXT,
    description TEXT,

    url TEXT,

    tags TEXT[] NOT NULL DEFAULT '{}',

    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT resources_resource_type_check CHECK (
        resource_type IN (
            'video',
            'api_documentation',
            'internal_docs'
        )
    ),

    CONSTRAINT resources_source_type_check CHECK (
        source_type IN (
            'drozbase',
            'zendesk_official_docs',
            'self_stored_videos'
        )
    ),

    CONSTRAINT resources_status_check CHECK (
            status IN ('pending', 'processing', 'published', 'failed')
    )
);

CREATE INDEX idx_resources_resource_type ON resources (resource_type);
CREATE INDEX idx_resources_source_type ON resources (source_type);
CREATE INDEX idx_resources_tags_gin ON resources USING GIN (tags);
CREATE INDEX idx_resources_metadata_gin ON resources USING GIN (metadata);