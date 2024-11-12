use crate::services::DbPool;
use anyhow::{Context, Result};
use db_schema::models::{ArticleEntry, ArticleId};
use db_schema::schema::{article_tags, articles, tags};
use diesel::internal::derives::multiconnection::chrono::{NaiveDateTime, Utc};
use diesel::sql_types::{Array, Int4, Int8, Integer, Text, Timestamp};
use diesel::{
    sql_query, Connection, ExpressionMethods, PgConnection, QueryDsl,
    QueryResult, RunQueryDsl,
};

pub fn create_article(
    db_pool: &DbPool,
    author_id: i32,
    title: &str,
    content: &str,
    tag_names: Vec<String>,
) -> Result<ArticleId> {
    let conn = &mut db_pool
        .get()
        .context("[news-api] failed to retrieve db connection")?;

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
    .bind::<Int4, _>(author_id)
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
    let conn = &mut db_pool
        .get()
        .context("[news-api] failed retrieve db connection")?;

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
    conn: &mut PgConnection,
    article_id: i32,
    title: &str,
    content: &str,
    tag_names: Vec<String>,
) -> QueryResult<()> {
    conn.transaction(|conn| {
        diesel::update(articles::table.find(article_id))
            .set((articles::title.eq(title), articles::content.eq(content)))
            .execute(conn)?;

        diesel::delete(article_tags::table.filter(article_tags::article_id.eq(article_id)))
            .execute(conn)?;

        for tag_name in tag_names {
            let tag_id = tags::table
                .select(tags::id)
                .filter(tags::name.eq(tag_name))
                .first::<i32>(conn)?;

            diesel::insert_into(article_tags::table)
                .values((
                    article_tags::article_id.eq(article_id),
                    article_tags::tag_id.eq(tag_id),
                ))
                .execute(conn)?;
        }

        Ok(())
    })
}

pub fn delete_article(db_pool: &DbPool, author_id: i32, article_id: i32) -> Result<()> {
    let conn = &mut db_pool
        .get()
        .context("[news-api] failed retrieve db connection")?;

    let query = format!(
        r#"
        DO $$
        DECLARE
            target_article_id INTEGER := {};
            target_author_id INTEGER := {};
        BEGIN
            CREATE TEMP TABLE tags_ids_to_delete AS
            SELECT tag_id FROM article_tags
            WHERE article_tags.article_id = target_article_id;

            DELETE FROM articles
            WHERE articles.id = target_article_id AND articles.author_id = target_author_id;

            DELETE FROM tags
            WHERE tags.id IN (SELECT tag_id FROM tags_ids_to_delete)
              AND NOT EXISTS (
                SELECT 1 FROM article_tags WHERE article_tags.tag_id = tags.id
            );

            DROP TABLE tags_ids_to_delete;
        END $$;
        "#,
        article_id, author_id
    );

    sql_query(query).execute(conn)?;

    Ok(())
}

pub fn get_article(db_pool: &DbPool, article_id: i32) -> Result<ArticleEntry> {
    let conn = &mut db_pool
        .get()
        .context("[news-api] failed retrieve db connection")?;

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