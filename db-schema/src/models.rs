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

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = users)]
pub struct UserEntry {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub username: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub email: Option<String>,
    #[diesel(sql_type = Text)]
    pub password_hash: String,
    #[diesel(sql_type = Text)]
    pub salt: String,
}

#[derive(QueryableByName, Clone)]
pub struct UserIdEntry {
    #[diesel(sql_type = Integer)]
    pub id: i32,
}
