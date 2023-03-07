
mod postgresql;
mod handlers;

use std::env;
use actix_web::{ App, HttpServer};
use postgresql::connect_postgres::PostgresPool;

use crate::{postgresql::seeding::seeding, handlers::app::init};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
   
    let postgres_pool = PostgresPool::new().await;

    let db_con = match postgres_pool {
        Ok(pool) => pool.con,
        Err(e) => {
            println!("Error connecting to database: {}", e);
            std::process::exit(1);
        }
    };
 

    // seeding the data only in dev mode 
    if cfg!(debug_assertions) {
        seeding(db_con).await;
    }
    

  
    let port = env::var("PORT").unwrap();
    let port = port.parse::<u16>().unwrap();
    println!("Listening on : http://localhost:{}", port);


    HttpServer::new(|| App::new().configure(init))

    .bind(format!("0.0.0.0:{}",port))?
    .run()
    .await
}