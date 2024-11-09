use crate::schema::comments;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types::{Array, Integer, Nullable, Text, Timestamp};

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = articles)]
pub struct ArticleEntry {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Integer)]
    pub author_id: i32,
    #[diesel(sql_type = Text)]
    pub title: String,
    #[diesel(sql_type = Text)]
    pub content: String,
    #[diesel(sql_type = Timestamp)]
    pub created_at: NaiveDateTime,
    #[diesel(sql_type = Array<Text>)]
    pub tags: Vec<String>,
}

#[derive(QueryableByName)]
pub struct ArticleId {
    #[diesel(sql_type = Integer)]
    pub id: i32,
}

#[derive(Queryable, QueryableByName, Selectable, Debug)]
#[diesel(table_name = comments)]
pub struct CommentEntry {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Integer)]
    pub article_id: i32,
    #[diesel(sql_type = Nullable<Integer>)]
    pub user_id: Option<i32>,
    #[diesel(sql_type = Nullable<Integer>)]
    pub parent_id: Option<i32>,
    #[diesel(sql_type = Text)]
    pub content: String,
    #[diesel(sql_type = Timestamp)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = comments)]
pub struct NewCommentEntry<'a> {
    pub article_id: i32,
    pub user_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub content: &'a str,
}
