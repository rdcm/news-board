use crate::infrastructure::{create_article, get_articles_page};
use crate::mappers::into_model;
use crate::news::news_service_server::NewsService;
use crate::news::*;
use crate::services::Services;
use diesel::internal::derives::multiconnection::chrono::NaiveDateTime;
use tonic::{Code, Request, Response, Status};

#[tonic::async_trait]
impl NewsService for Services {
    async fn get_articles(
        &self,
        request: Request<GetArticlesRequest>,
    ) -> Result<Response<GetArticlesResponse>, Status> {
        let req = request.into_inner();
        let timestamp =
            NaiveDateTime::parse_from_str(req.last_timestamp.as_str(), "%Y-%m-%d %H:%M:%S%.6f")
                .ok();

        match get_articles_page(&self.db_pool, timestamp, req.page_size) {
            Ok(articles) => Ok(Response::new(GetArticlesResponse {
                articles: articles.into_iter().map(into_model).collect(),
            })),
            Err(err) => Err(Status::new(Code::Internal, err.to_string())),
        }
    }

    async fn create_article(
        &self,
        request: Request<CreateArticleRequest>,
    ) -> Result<Response<CreatedArticleResponse>, Status> {
        let req = request.into_inner();

        match create_article(
            &self.db_pool,
            1, // TODO remove hardcode, after adding auth
            req.title.as_str(),
            req.content.as_str(),
            req.tags,
        ) {
            Ok(id) => Ok(Response::new(CreatedArticleResponse { article_id: id })),
            Err(_) => Err(Status::new(Code::Unknown, "101")),
        }
    }
}
