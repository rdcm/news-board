use crate::app_state::DbPool;
use anyhow::{anyhow, Result};
use db_schema::models::{ArticleEntry, ArticleId, UserEntry, UserIdEntry};
use diesel::internal::derives::multiconnection::chrono::{NaiveDateTime, Utc};
use diesel::sql_types::{Array, Int8, Integer, Text, Timestamp};
use diesel::{sql_query, RunQueryDsl};

pub fn create_article(
    db_pool: &DbPool,
    author_id: i32,
    title: &str,
    content: &str,
    tag_names: Vec<String>,
) -> Result<ArticleId> {
    let conn = &mut db_pool.get_connection()?;

    let article_id = sql_query(
        r#"
        WITH inserted_article AS (
            INSERT INTO articles (author_id, title, content)
                VALUES ($1, $2, $3)
                RETURNING id
        ),
        inserted_tags AS (
             INSERT INTO tags (name)
                 SELECT unnest($4::text[])
                 ON CONFLICT (name) DO UPDATE
                     SET name = EXCLUDED.name
                 RETURNING id, name
         ),
         article_tag_associations AS (
             INSERT INTO article_tags (article_id, tag_id)
                 SELECT inserted_article.id, inserted_tags.id
                 FROM inserted_article, inserted_tags
         )
        SELECT inserted_article.id
        FROM inserted_article;
    "#,
    )
    .bind::<Integer, _>(author_id)
    .bind::<Text, _>(title)
    .bind::<Text, _>(content)
    .bind::<Array<Text>, _>(tag_names)
    .get_result::<ArticleId>(conn)?;

    Ok(article_id)
}

pub fn get_articles_page(
    db_pool: &DbPool,
    last_timestamp: Option<NaiveDateTime>,
    page_size: i64,
) -> Result<Vec<ArticleEntry>> {
    let conn = &mut db_pool.get_connection()?;

    let timestamp = last_timestamp.unwrap_or_else(|| Utc::now().naive_utc());
    let articles = sql_query(
        r#"
        SELECT
            articles.id,
            articles.author_id,
            articles.title,
            articles.content,
            articles.created_at,
            array_agg(tags.name) AS tags,
            users.username AS author_username
        FROM articles
            LEFT JOIN article_tags ON articles.id = article_tags.article_id
            LEFT JOIN tags ON tags.id = article_tags.tag_id
            LEFT JOIN users ON users.id = articles.author_id
        WHERE articles.created_at < $1
        GROUP BY articles.id, articles.created_at, users.username
        ORDER BY articles.created_at DESC
        LIMIT $2
    "#,
    )
    .bind::<Timestamp, _>(timestamp)
    .bind::<Int8, _>(page_size)
    .load::<ArticleEntry>(conn)?;

    Ok(articles)
}

pub fn update_article(
    db_pool: &DbPool,
    author_id: i32,
    article_id: i32,
    title: &str,
    content: &str,
    tag_names: Vec<String>,
) -> Result<()> {
    let conn = &mut db_pool.get_connection()?;

    sql_query(
        r#"
        WITH updated_article AS (
            UPDATE articles
            SET title = $1, content = $2
            WHERE id = $4 AND author_id = $3
            RETURNING id
        ),
        existing_tags AS (
            SELECT t.id, t.name
            FROM tags t
            JOIN article_tags at ON t.id = at.tag_id
            WHERE at.article_id = $4
        ),
        new_tags AS (
            INSERT INTO tags (name)
            SELECT unnest($5::text[])
            ON CONFLICT (name) DO UPDATE SET name = EXCLUDED.name
            RETURNING id, name
        ),
        tags_to_add AS (
            SELECT nt.id, nt.name
            FROM new_tags nt
            LEFT JOIN existing_tags et ON nt.name = et.name
            WHERE et.id IS NULL
        ),
        tags_to_remove AS (
            SELECT et.id, et.name
            FROM existing_tags et
            LEFT JOIN new_tags nt ON et.name = nt.name
            WHERE nt.id IS NULL
        ),
        deleted_article_tags AS (
            DELETE FROM article_tags
            WHERE article_id = $4 AND tag_id IN (SELECT id FROM tags_to_remove)
        ),
        inserted_article_tags AS (
            INSERT INTO article_tags (article_id, tag_id)
            SELECT $4, id FROM tags_to_add
            ON CONFLICT DO NOTHING
        ),
        deleted_orphaned_tags AS (
            DELETE FROM tags
            WHERE id IN (SELECT id FROM tags_to_remove)
              AND NOT EXISTS (
                  SELECT 1 FROM article_tags WHERE tag_id = tags.id AND article_id != $4
              )
        )
        SELECT 1;
        "#,
    )
    .bind::<Text, _>(title)
    .bind::<Text, _>(content)
    .bind::<Integer, _>(author_id)
    .bind::<Integer, _>(article_id)
    .bind::<Array<Text>, _>(tag_names)
    .execute(conn)?;

    Ok(())
}

pub fn delete_article(db_pool: &DbPool, author_id: i32, article_id: i32) -> Result<()> {
    let conn = &mut db_pool.get_connection()?;

    sql_query(
        r#"
        WITH deleted_article AS (
            DELETE FROM articles
            WHERE articles.id = $1 AND articles.author_id = $2
            RETURNING id
        ),
        tags_to_delete AS (
            DELETE FROM article_tags
            WHERE article_id = $1
            RETURNING tag_id
        ),
        deleted_tags AS (
             DELETE FROM tags
             WHERE tags.id IN (SELECT tag_id FROM tags_to_delete)
                 AND NOT EXISTS (
                     SELECT 1 FROM article_tags
                     WHERE article_tags.tag_id = tags.id AND article_tags.article_id != $1
                 )
             RETURNING id
        )
        SELECT 1;
        "#,
    )
    .bind::<Integer, _>(article_id)
    .bind::<Integer, _>(author_id)
    .execute(conn)?;

    Ok(())
}

pub fn get_article(db_pool: &DbPool, article_id: i32) -> Result<ArticleEntry> {
    let conn = &mut db_pool.get_connection()?;

    let article = sql_query(
        r#"
        SELECT
            articles.id,
            articles.author_id,
            articles.title,
            articles.content,
            articles.created_at,
            array_agg(tags.name) AS tags,
            users.username AS author_username
        FROM articles
            LEFT JOIN article_tags ON articles.id = article_tags.article_id
            LEFT JOIN tags ON tags.id = article_tags.tag_id
            LEFT JOIN users ON users.id = articles.author_id
        WHERE articles.id = $1
        GROUP BY articles.id, users.username
        LIMIT 1
    "#,
    )
    .bind::<Integer, _>(article_id)
    .get_result::<ArticleEntry>(conn)?;

    Ok(article)
}

pub fn create_user(
    db_pool: &DbPool,
    username: &str,
    hashed_password: &str,
    salt: &str,
) -> Result<UserIdEntry> {
    let conn = &mut db_pool.get_connection()?;

    let user_id = sql_query(
        r#"
        INSERT INTO users (username, email, password_hash, salt)
        VALUES ($1, NULL, $2, $3)
        ON CONFLICT (username) DO NOTHING
        RETURNING id;
    "#,
    )
    .bind::<Text, _>(username)
    .bind::<Text, _>(hashed_password)
    .bind::<Text, _>(salt)
    .get_result::<UserIdEntry>(conn)?;

    Ok(user_id)
}

pub fn save_session_id(db_pool: &DbPool, user_id: i32, session_id: &str) -> Result<()> {
    let conn = &mut db_pool.get_connection()?;

    sql_query(
        r#"
        INSERT INTO sessions (session_id, user_id, created_at)
        VALUES ($1, $2, NOW())
        RETURNING id;
    "#,
    )
    .bind::<Text, _>(session_id)
    .bind::<Integer, _>(user_id)
    .execute(conn)?;

    Ok(())
}

pub fn get_user_by_username(db_pool: &DbPool, username: &str) -> Result<UserEntry> {
    let conn = &mut db_pool.get_connection()?;

    let user = sql_query(r#"SELECT * FROM users WHERE username = $1;"#)
        .bind::<Text, _>(username)
        .get_result::<UserEntry>(conn)?;

    Ok(user)
}

pub fn get_session_by_id(db_pool: &DbPool, session_id: &str) -> Result<UserIdEntry> {
    let conn = &mut db_pool.get_connection()?;

    let user_id = sql_query(r#"SELECT user_id AS id FROM sessions WHERE session_id = $1;"#)
        .bind::<Text, _>(session_id)
        .get_result::<UserIdEntry>(conn)
        .map_err(|_| anyhow!("session not found"))?;

    Ok(user_id)
}

pub fn delete_session(db_pool: &DbPool, session_id: &str) -> Result<()> {
    let conn = &mut db_pool.get_connection()?;

    sql_query(r#"DELETE FROM sessions WHERE session_id = $1;"#)
        .bind::<Text, _>(session_id)
        .execute(conn)
        .map_err(|_| anyhow!("Delete session may failed"))?;

    Ok(())
}
