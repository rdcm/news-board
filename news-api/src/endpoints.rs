use crate::news::news_service_server::NewsService;
use crate::news::{GetNewsRequest, NewsResponse};
use crate::services::Services;
use tonic::{Request, Response, Status};

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
}
