CREATE TABLE comments (
    id         UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    article_id UUID         NOT NULL REFERENCES articles(id) ON DELETE CASCADE,
    author_id  UUID         NOT NULL REFERENCES users(id)    ON DELETE CASCADE,
    content    TEXT         NOT NULL,
    created_at TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_comments_article_id ON comments(article_id);
CREATE INDEX idx_comments_author_id  ON comments(author_id);
