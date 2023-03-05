
mod postgresql;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use postgresql::connect_postgres::PostgresPool;



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
  

    let db_pool = PostgresPool::new().await;
    let db_pool = match db_pool {
        Ok(pool) => {println!("Connected to database");

            pool.pool    },
        //throw error
        Err(e) => {
            println!("Error connecting to database: {}", e);
            std::process::exit(1);
        }
    };
    // do something with pool
   
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}