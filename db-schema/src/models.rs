use crate::schema::{articles, comments};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types::{Integer, Nullable, Text, Timestamp};

#[derive(Queryable, Debug)]
#[diesel(table_name = articles)]
pub struct Article {
    pub id: i32,
    pub author_id: i32,
    pub title: String,
    pub content: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = articles)]
pub struct NewArticle<'a> {
    pub author_id: i32,
    pub title: &'a str,
    pub content: &'a str,
}

#[derive(Queryable, QueryableByName, Selectable, Debug)]
#[diesel(table_name = comments)]
pub struct Comment {
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
pub struct NewComment<'a> {
    pub article_id: i32,
    pub user_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub content: &'a str,
}
