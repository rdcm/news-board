use anyhow::Context;
use config::{Config, Environment};
use dotenvy::dotenv;
use news_api::app_state::AppState;
use news_api::auth_generated::auth_service_server::AuthServiceServer;
use news_api::auth_interceptor::AuthInterceptor;
use news_api::news_generated::news_service_server::NewsServiceServer;
use news_api::reflection_middleware::ReflectionMiddlewareLayer;
use news_api::settings::Settings;
use std::sync::Arc;
use tonic::codegen::InterceptedService;
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

    let app_state = AppState::new(Arc::new(settings.clone()))?;
    let auth_interceptor = AuthInterceptor::new(app_state.clone());
    let reflection_layer = ReflectionMiddlewareLayer::default();

    let sock_addr = settings.app.get_sock_address()?;

    Server::builder()
        .layer(reflection_layer)
        .add_service(InterceptedService::new(
            NewsServiceServer::new(app_state.clone()),
            auth_interceptor.clone(),
        ))
        .add_service(InterceptedService::new(
            AuthServiceServer::new(app_state.clone()),
            auth_interceptor.clone(),
        ))
        .serve(sock_addr)
        .await?;

    Ok(())
}
