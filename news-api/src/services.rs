use crate::settings::Settings;
use anyhow::{Context, Result};
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;

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

pub struct Services {
    pub db_pool: DbPool,
}

impl Services {
    pub fn new(settings: &Settings) -> Result<Self> {
        let db_pool = DbPool::new(&settings.database.uri)?;

        Ok(Self { db_pool })
    }
}
