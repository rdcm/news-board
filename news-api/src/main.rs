use news_api::news::news_server::{News, NewsServer};
use news_api::news::{GetNewsRequest, NewsResponse};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
pub struct NewsService;

#[tonic::async_trait]
impl News for NewsService {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let news_service = NewsService::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(NewsServer::new(news_service))
        .serve(addr)
        .await?;

    Ok(())
}
