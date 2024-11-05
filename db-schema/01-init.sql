CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(100) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL
);

CREATE INDEX idx_users_username ON users (username);
CREATE INDEX idx_users_email ON users (email);

CREATE TABLE news (
    id SERIAL PRIMARY KEY,
    author_id INTEGER NOT NULL REFERENCES users(id),
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_news_author_id ON news (author_id);
CREATE INDEX idx_news_created_at ON news (created_at);
CREATE INDEX idx_news_title ON news USING gin (to_tsvector('english', title));
CREATE INDEX idx_news_content ON news USING gin (to_tsvector('english', content));

CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL
);

CREATE INDEX idx_tags_name ON tags (name);

CREATE TABLE news_tags (
    news_id INTEGER NOT NULL REFERENCES news(id) ON DELETE CASCADE,
    tag_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (news_id, tag_id)
);

CREATE INDEX idx_news_tags_news_id ON news_tags (news_id);
CREATE INDEX idx_news_tags_tag_id ON news_tags (tag_id);

CREATE TABLE likes (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    news_id INTEGER NOT NULL REFERENCES news(id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (user_id, news_id)
);

CREATE INDEX idx_likes_user_id ON likes (user_id);
CREATE INDEX idx_likes_news_id ON likes (news_id);

CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    news_id INTEGER NOT NULL REFERENCES news(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE SET NULL,
    parent_id INTEGER REFERENCES comments(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_comments_news_id ON comments (news_id);
CREATE INDEX idx_comments_parent_id ON comments (parent_id);
CREATE INDEX idx_comments_created_at ON comments (created_at);
