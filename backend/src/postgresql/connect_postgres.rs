use config::Config;
use crate::postgresql::config::PostGresConfig;
use deadpool_postgres::Pool;
use tokio_postgres::Error;
use tokio_postgres::NoTls;

pub struct PostgresPool {
    pub pool: Pool,
}

impl PostgresPool {
     pub async fn new() -> Result<Self, Error> {
        let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();
        let config: PostGresConfig = config_.try_deserialize().unwrap();
        let pool = config.pg.create_pool(None, NoTls).unwrap();
        Ok(Self { pool })
    }
}

