use crate::news_generated::Article;
use db_schema::models::ArticleEntry;

pub fn into_model(article_entry: ArticleEntry) -> Article {
    Article {
        id: article_entry.id,
        author_id: article_entry.author_id,
        title: article_entry.title,
        content: article_entry.content,
        created_at: article_entry.created_at.to_string(),
        tags: article_entry.tags,
    }
}
