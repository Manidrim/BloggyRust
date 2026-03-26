CREATE TABLE articles (
    id         UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title      VARCHAR(255) NOT NULL,
    slug       VARCHAR(255) NOT NULL UNIQUE,
    content    TEXT         NOT NULL,
    excerpt    TEXT,
    author_id  UUID         NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    published  BOOLEAN      NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_articles_slug      ON articles(slug);
CREATE INDEX idx_articles_author_id ON articles(author_id);
CREATE INDEX idx_articles_published ON articles(published);
