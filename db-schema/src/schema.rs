// @generated automatically by Diesel CLI.

diesel::table! {
    article_tags (article_id, tag_id) {
        article_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    articles (id) {
        id -> Int4,
        author_id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        content -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    comments (id) {
        id -> Int4,
        article_id -> Int4,
        user_id -> Int4,
        parent_id -> Nullable<Int4>,
        content -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    likes (id) {
        id -> Int4,
        user_id -> Int4,
        article_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
    }
}

diesel::joinable!(article_tags -> articles (article_id));
diesel::joinable!(article_tags -> tags (tag_id));
diesel::joinable!(articles -> users (author_id));
diesel::joinable!(comments -> articles (article_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(likes -> articles (article_id));
diesel::joinable!(likes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(article_tags, articles, comments, likes, tags, users,);
