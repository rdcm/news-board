use anyhow::{Context, Result};
use diesel::{Connection, PgConnection};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Services {
    pub db: Arc<Mutex<PgConnection>>,
}

impl Services {
    pub fn new(db_uri: &str) -> Result<Self> {
        let db = Arc::new(Mutex::new(
            PgConnection::establish(db_uri).context("[news-api] failed establish db connection")?,
        ));

        Ok(Self { db })
    }
}
