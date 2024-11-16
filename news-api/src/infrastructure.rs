use crate::services::DbPool;
use anyhow::Result;
use db_schema::models::{ArticleEntry, ArticleId};
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
            array_agg(tags.name) AS tags
        FROM articles
            LEFT JOIN article_tags ON articles.id = article_tags.article_id
            LEFT JOIN tags ON tags.id = article_tags.tag_id
        WHERE articles.created_at < $1
        GROUP BY articles.id, articles.created_at
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
            array_agg(tags.name) AS tags
        FROM articles
            LEFT JOIN article_tags ON articles.id = article_tags.article_id
            LEFT JOIN tags ON tags.id = article_tags.tag_id
        WHERE articles.id = $1
        GROUP BY articles.id
        LIMIT 1
    "#,
    )
    .bind::<Integer, _>(article_id)
    .get_result::<ArticleEntry>(conn)?;

    Ok(article)
}
