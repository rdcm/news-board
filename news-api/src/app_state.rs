use crate::settings::Settings;
use anyhow::{Context, Result};
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use std::sync::Arc;

pub struct DbPool {
    db_pool: Pool<ConnectionManager<PgConnection>>,
}

impl DbPool {
    pub fn new(db_uri: &str) -> Result<Self> {
        let manager = ConnectionManager::<PgConnection>::new(db_uri);
        let db_pool = Pool::builder()
            .build(manager)
            .context("[news-api] failed to build connection pool")?;

        Ok(Self { db_pool })
    }

    pub fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>> {
        let conn = self
            .db_pool
            .get()
            .context("[news-api] failed to retrieve db connection")?;

        Ok(conn)
    }
}

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<DbPool>,
    pub settings: Arc<Settings>,
}

impl AppState {
    pub fn new(settings: Arc<Settings>) -> Result<Self> {
        let db_pool = Arc::new(DbPool::new(&settings.database.uri)?);

        Ok(Self { db_pool, settings })
    }
}
