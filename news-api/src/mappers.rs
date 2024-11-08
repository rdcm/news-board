use crate::news::Article;
use db_schema::models::ArticleEntry;

pub fn into_model(article_entry: ArticleEntry) -> Article {
    Article {
        title: article_entry.title,
        content: article_entry.content,
        created_at: article_entry.created_at.to_string(),
        tags: Vec::new(), // TODO join tags
    }
}
