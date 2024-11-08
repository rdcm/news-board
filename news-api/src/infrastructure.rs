use crate::services::DbPool;
use anyhow::{Context, Result};
use db_schema::models::{ArticleEntry, CommentEntry, NewCommentEntry};
use db_schema::schema::{article_tags, articles, comments, likes, tags};
use diesel::internal::derives::multiconnection::chrono::NaiveDateTime;
use diesel::sql_types::Integer;
use diesel::{
    sql_query, BoolExpressionMethods, Connection, ExpressionMethods, PgConnection, QueryDsl,
    QueryResult, RunQueryDsl,
};

pub fn create_article(
    db_pool: &DbPool,
    author_id: i32,
    title: &str,
    content: &str,
    tag_names: Vec<String>,
) -> Result<i32> {
    let conn = &mut db_pool
        .get()
        .context("[news-api] failed retrieve db connection")?;

    conn.transaction(|conn| {
        let article_id = diesel::insert_into(articles::table)
            .values((
                articles::title.eq(title),
                articles::content.eq(content),
                articles::author_id.eq(author_id),
            ))
            .returning(articles::id)
            .get_result::<i32>(conn)?;

        for tag in &tag_names {
            diesel::insert_into(tags::table)
                .values(tags::name.eq(tag))
                .on_conflict_do_nothing()
                .execute(conn)?;
        }

        for tag_name in tag_names {
            let tag_id = tags::table
                .select(tags::id)
                .filter(tags::name.eq(&tag_name))
                .first::<i32>(conn)?;

            diesel::insert_into(article_tags::table)
                .values((
                    article_tags::article_id.eq(article_id),
                    article_tags::tag_id.eq(tag_id),
                ))
                .execute(conn)?;
        }

        Ok(article_id)
    })
}

pub fn get_articles_page(
    db_pool: &DbPool,
    last_timestamp: Option<NaiveDateTime>,
    page_size: i64,
) -> Result<Vec<ArticleEntry>> {
    let conn = &mut db_pool
        .get()
        .context("[news-api] failed retrieve db connection")?;

    let mut query = articles::table
        .order(articles::created_at.desc())
        .limit(page_size)
        .into_boxed();

    if let Some(timestamp) = last_timestamp {
        query = query.filter(articles::created_at.lt(timestamp));
    }

    query.load(conn).context("[news-api] failed load articles")
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

pub fn delete_article(conn: &mut PgConnection, article_id: i32) -> QueryResult<usize> {
    diesel::delete(articles::table.find(article_id)).execute(conn)
}

pub fn get_article(conn: &mut PgConnection, article_id: i32) -> QueryResult<ArticleEntry> {
    articles::table.find(article_id).first(conn)
}

pub fn add_comment(
    conn: &mut PgConnection,
    article_id: i32,
    user_id: Option<i32>,
    content: &str,
    parent_id: Option<i32>,
) -> QueryResult<i32> {
    let new_comment = NewCommentEntry {
        article_id,
        user_id,
        parent_id,
        content,
    };

    diesel::insert_into(comments::table)
        .values(new_comment)
        .returning(comments::id)
        .get_result(conn)
}

pub fn update_comment(
    conn: &mut PgConnection,
    comment_id: i32,
    user_id: i32,
    content: &str,
) -> QueryResult<usize> {
    diesel::update(
        comments::table.filter(
            comments::id
                .eq(comment_id)
                .and(comments::user_id.eq(user_id)),
        ),
    )
    .set(comments::content.eq(content))
    .execute(conn)
}

pub fn remove_comment(
    conn: &mut PgConnection,
    comment_id: i32,
    user_id: i32,
) -> QueryResult<usize> {
    diesel::delete(
        comments::table.filter(
            comments::id
                .eq(comment_id)
                .and(comments::user_id.eq(user_id)),
        ),
    )
    .execute(conn)
}

pub fn get_comment_tree(
    conn: &mut PgConnection,
    article_id: i32,
) -> QueryResult<Vec<CommentEntry>> {
    sql_query(
        "
        WITH RECURSIVE comment_tree AS (
            SELECT * FROM comments WHERE news_id = $1 AND parent_comment_id IS NULL
            UNION ALL
            SELECT c.* FROM comments c
            JOIN comment_tree ct ON c.parent_comment_id = ct.id
        )
        SELECT * FROM comment_tree;
        ",
    )
    .bind::<Integer, _>(article_id)
    .load::<CommentEntry>(conn)
}

pub fn like_article(conn: &mut PgConnection, article_id: i32, user_id: i32) -> QueryResult<usize> {
    diesel::insert_into(likes::table)
        .values((likes::article_id.eq(article_id), likes::user_id.eq(user_id)))
        .on_conflict_do_nothing()
        .execute(conn)
}

pub fn unlike_article(
    conn: &mut PgConnection,
    article_id: i32,
    user_id: i32,
) -> QueryResult<usize> {
    diesel::delete(
        likes::table.filter(
            likes::article_id
                .eq(article_id)
                .and(likes::user_id.eq(user_id)),
        ),
    )
    .execute(conn)
}
