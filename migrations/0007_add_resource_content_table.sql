CREATE TABLE resource_contents (
    id UUID PRIMARY KEY,

    resource_id UUID NOT NULL UNIQUE REFERENCES resources(id) ON DELETE CASCADE,

    raw_text TEXT,

    language  TEXT NOT NULL DEFAULT 'portuguese',

    search_vector TSVECTOR,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()

);

CREATE INDEX idx_resource_contents_search      ON resource_contents USING GIN (search_vector);
CREATE INDEX idx_resource_contents_resource_id ON resource_contents (resource_id);