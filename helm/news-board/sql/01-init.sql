CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(100) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    salt VARCHAR(32) NOT NULL
);

CREATE INDEX CONCURRENTLY idx_users_username ON users (username);
CREATE INDEX CONCURRENTLY idx_users_email ON users (email);

CREATE TABLE sessions (
    id SERIAL PRIMARY KEY,
    session_id VARCHAR(255) UNIQUE NOT NULL,
    user_id INT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX CONCURRENTLY idx_sessions_session_id ON sessions (session_id);
CREATE INDEX CONCURRENTLY idx_sessions_user_id ON sessions (user_id);

CREATE TABLE articles (
    id SERIAL PRIMARY KEY,
    author_id INTEGER NOT NULL REFERENCES users(id),
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX CONCURRENTLY idx_articles_author_id ON articles (author_id);
CREATE INDEX CONCURRENTLY idx_articles_created_at ON articles (created_at);
CREATE INDEX CONCURRENTLY idx_articles_title ON articles USING gin (to_tsvector('english', title));
CREATE INDEX CONCURRENTLY idx_articles_content ON articles USING gin (to_tsvector('english', content));

CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL
);

CREATE INDEX CONCURRENTLY idx_tags_name ON tags (name);

CREATE TABLE article_tags (
    article_id INTEGER NOT NULL REFERENCES articles(id),
    tag_id INTEGER NOT NULL REFERENCES tags(id),
    PRIMARY KEY (article_id, tag_id)
);

CREATE INDEX CONCURRENTLY idx_article_tags_article_id ON article_tags (article_id);
CREATE INDEX CONCURRENTLY idx_article_tags_tag_id ON article_tags (tag_id);

CREATE TABLE likes (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    article_id INTEGER NOT NULL REFERENCES articles(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (user_id, article_id)
);

CREATE INDEX CONCURRENTLY idx_likes_user_id ON likes (user_id);
CREATE INDEX CONCURRENTLY idx_likes_article_id ON likes (article_id);

CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    article_id INTEGER NOT NULL REFERENCES articles(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE SET NULL,
    parent_id INTEGER REFERENCES comments(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX CONCURRENTLY idx_comments_article_id ON comments (article_id);
CREATE INDEX CONCURRENTLY idx_comments_parent_id ON comments (parent_id);
CREATE INDEX CONCURRENTLY idx_comments_created_at ON comments (created_at);