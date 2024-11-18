use crate::news_generated::Article;
use db_schema::models::ArticleEntry;

pub fn into_article(article_entry: ArticleEntry) -> Article {
    Article {
        id: article_entry.id,
        author_username: article_entry.author_username,
        title: article_entry.title,
        content: article_entry.content,
        created_at: article_entry.created_at.to_string(),
        tags: article_entry.tags,
    }
}

pub fn into_articles(article_entry: Vec<ArticleEntry>) -> Vec<Article> {
    article_entry.into_iter().map(into_article).collect()
}
