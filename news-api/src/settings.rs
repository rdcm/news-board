use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashSet;
use std::net::SocketAddr;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: DbSettings,
    pub app: AppSettings,
    pub auth: AuthSettings,
}

#[derive(Debug, Deserialize)]
pub struct AuthSettings {
    pub valid_token: String,
    pub secure_routes: String,
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

impl AuthSettings {
    pub fn get_secure_routes(&self) -> HashSet<String> {
        self.secure_routes
            .split(',')
            .map(|s| s.to_string())
            .collect()
    }
}

impl AppSettings {
    pub fn get_sock_address(&self) -> Result<SocketAddr> {
        format!("{}:{}", self.host, self.port)
            .parse()
            .context("[news-api] failed to parse socket address")
    }
}
