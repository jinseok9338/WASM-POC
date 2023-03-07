
use dotenv::dotenv;
use std::env;
use crate::postgresql::config::PostGresConfig;

use tokio_postgres::Error;
use tokio_postgres::NoTls;
use deadpool_postgres::Object;
pub struct PostgresPool {
    pub con: Object,
}

impl PostgresPool {
     pub async fn new() -> Result<Self, Error> {

        dotenv().ok();       
        let pg_user = env::var("POSTGRES_USER").unwrap(); 
        let pg_pass = env::var("POSTGRES_PASSWORD").unwrap();
        let pg_host = env::var("POSTGRES_HOST").unwrap();
        let pg_port = env::var("POSTGRES_PORT").unwrap_or("5432".to_string());

    
        let mut config: PostGresConfig = PostGresConfig::default();
        
      
        config.pg.user = Some(pg_user);
        config.pg.password = Some(pg_pass);
        config.pg.host = Some(pg_host);
        config.pg.port = Some(pg_port.parse().unwrap());
        config.pg.dbname = Some("postgres".to_string());

        let pool = config.pg.create_pool(None, NoTls).unwrap();
        let con = pool.get().await.unwrap();

        Ok(Self { con })
    }

// after making db pool available to the app, we can use it to create a connection to the database

}


