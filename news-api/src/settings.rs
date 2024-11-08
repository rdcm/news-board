use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: DbSettings,
    pub app: AppSettings,
}

#[derive(Debug, Deserialize)]
pub struct DbSettings {
    pub db_name: String,
    pub uri: String,
}

#[derive(Debug, Deserialize)]
pub struct AppSettings {
    pub host: String,
    pub port: i32,
}

impl AppSettings {
    pub fn get_sock_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
