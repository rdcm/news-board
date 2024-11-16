use anyhow::Context;
use config::{Config, Environment};
use dotenvy::dotenv;
use news_api::app_state::AppState;
use news_api::auth_interceptor::AuthInterceptor;
use news_api::news::news_service_server::NewsServiceServer;
use news_api::reflection_middleware::ReflectionMiddlewareLayer;
use news_api::settings::Settings;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    _ = dotenv();

    let config = Config::builder()
        .add_source(Environment::with_prefix("NEWS_API").separator("__"))
        .build()
        .context("[news-api] [config] Failed to build config from env variables")?;

    let settings: Settings = config
        .try_deserialize()
        .context("[news-api] [config] Failed to deserialize config")?;

    let app_state = AppState::new(&settings)?;
    let auth_interceptor = AuthInterceptor::new(&settings);
    let server = NewsServiceServer::with_interceptor(app_state, auth_interceptor);

    let sock_addr = settings.app.get_sock_address()?;

    Server::builder()
        .layer(ReflectionMiddlewareLayer::default())
        .add_service(server)
        .serve(sock_addr)
        .await?;

    Ok(())
}
