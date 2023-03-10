
mod postgresql;
mod handlers;

use std::env;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use postgresql::connect_postgres::PostgresPool;
use actix_cors::Cors;
use crate::{postgresql::seeding::seeding, handlers::app::init};



#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

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


    HttpServer::new(|| App::new().wrap(
        Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header(),
    ).configure(init))
    .bind(format!("0.0.0.0:{}",port))?
    .run()
    .await
}