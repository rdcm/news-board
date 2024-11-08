use anyhow::{Context, Result};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: DbSettings,
    pub app: AppSettings,
}

#[derive(Debug, Deserialize)]
pub struct DbSettings {
    pub uri: String,
}

#[derive(Debug, Deserialize)]
pub struct AppSettings {
    pub host: String,
    pub port: i32,
}

impl AppSettings {
    pub fn get_sock_address(&self) -> Result<SocketAddr> {
        format!("{}:{}", self.host, self.port)
            .parse()
            .context("[news-api] failed to parse socket address")
    }
}
