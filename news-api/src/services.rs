use anyhow::{Context, Result};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct Services {
    pub db_pool: DbPool,
}

impl Services {
    pub fn new(db_uri: &str) -> Result<Self> {
        let manager = ConnectionManager::<PgConnection>::new(db_uri);
        let db_pool = Pool::builder()
            .build(manager)
            .context("[news-api] failed to build connection pool")?;

        Ok(Self { db_pool })
    }
}
