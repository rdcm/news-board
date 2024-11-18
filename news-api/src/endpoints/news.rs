use crate::app_state::AppState;
use crate::infrastructure::{
    create_article, delete_article, get_article, get_articles_page, update_article,
};
use crate::mappers::{into_article, into_articles};
use crate::news_generated::news_service_server::NewsService;
use crate::news_generated::*;
use crate::utils::{get_user_id, parse_timestamp};
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl NewsService for AppState {
    async fn get_article(
        &self,
        request: Request<GetArticleRequest>,
    ) -> Result<Response<GetArticleResponse>, Status> {
        let req = request.into_inner();

        let article = get_article(&self.db_pool, req.article_id)
            .map_err(|_| Status::failed_precondition("Article not found"))?;

        Ok(Response::new(GetArticleResponse {
            article: Some(into_article(article)),
        }))
    }

    async fn get_articles(
        &self,
        request: Request<GetArticlesRequest>,
    ) -> Result<Response<GetArticlesResponse>, Status> {
        let req = request.into_inner();
        let timestamp = parse_timestamp(&req.last_timestamp);

        let article_page = get_articles_page(&self.db_pool, timestamp, req.page_size)
            .map_err(|_| Status::failed_precondition("Getting page error"))?;

        Ok(Response::new(GetArticlesResponse {
            articles: into_articles(article_page),
        }))
    }

    async fn create_article(
        &self,
        request: Request<CreateArticleRequest>,
    ) -> Result<Response<CreatedArticleResponse>, Status> {
        let user_id = get_user_id(&request)?;
        let req = request.into_inner();

        let article_id = create_article(
            &self.db_pool,
            user_id.value,
            req.title.as_str(),
            req.content.as_str(),
            req.tags,
        )
        .map_err(|_| Status::failed_precondition("Creating article failed"))?;

        Ok(Response::new(CreatedArticleResponse {
            article_id: article_id.id,
        }))
    }

    async fn delete_article(
        &self,
        request: Request<DeleteArticleRequest>,
    ) -> Result<Response<DeleteArticleResponse>, Status> {
        let user_id = get_user_id(&request)?;
        let req = request.into_inner();

        delete_article(&self.db_pool, user_id.value, req.article_id)
            .map_err(|_| Status::failed_precondition("Deleting article failed"))?;

        Ok(Response::new(DeleteArticleResponse {}))
    }

    async fn update_article(
        &self,
        request: Request<UpdateArticleRequest>,
    ) -> Result<Response<UpdateArticleResponse>, Status> {
        let user_id = get_user_id(&request)?;
        let req = request.into_inner();

        update_article(
            &self.db_pool,
            user_id.value,
            req.article_id,
            &req.title,
            &req.content,
            req.tags,
        )
        .map_err(|_| Status::failed_precondition("Updating article failed"))?;

        Ok(Response::new(UpdateArticleResponse {}))
    }
}
