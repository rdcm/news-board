use crate::infrastructure::create_article;
use crate::news::news_service_server::NewsService;
use crate::news::*;
use crate::services::Services;
use tonic::{Code, Request, Response, Status};

#[tonic::async_trait]
impl NewsService for Services {
    async fn get_news(
        &self,
        request: Request<GetNewsRequest>,
    ) -> Result<Response<NewsResponse>, Status> {
        let name = request.into_inner().name;
        let reply = NewsResponse {
            message: format!("Hello, {}!", name),
        };
        Ok(Response::new(reply))
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
            Err(err) => Err(Status::new(Code::Unknown, "101")),
        }
    }
}
