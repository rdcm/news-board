use anyhow::Context;
use config::{Config, Environment};
use dotenvy::dotenv;
use news_api::news::news_service_server::NewsServiceServer;
use news_api::services::Services;
use news_api::settings::Settings;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    _ = dotenv();

    let settings = Config::builder()
        .add_source(Environment::with_prefix("NEWS_API").separator("__"))
        .build()
        .context("[news-api] [config] Failed to build config from env variables")?;

    let settings: Settings = settings
        .try_deserialize()
        .context("[news-api] [config] Failed to deserialize config")?;

    let services = Services::default();

    let sock_addr = settings
        .app
        .get_sock_address()
        .parse()
        .context("[news-api] failed to parse socket address")?;

    Server::builder()
        .add_service(NewsServiceServer::new(services))
        .serve(sock_addr)
        .await?;

    Ok(())
}
