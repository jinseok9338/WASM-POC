
mod postgresql;
use std::fs::read_to_string;
use std::env;
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
    let port = env::var("PORT").unwrap();
    let port = port.parse::<u16>().unwrap();
    println!("Listening on : http://localhost:{}", port);

   let db_pool= match db_pool {
        Ok(pool) => {println!("Connected to database!"); 
            pool.pool },
        //throw error
        Err(e) => {
            println!("Error connecting to database: {}", e);
            std::process::exit(1);
        }
    };

    // do something with pool
    let conn = db_pool.get().await;
    let conn = match conn {
        Ok(conn) => conn,
        Err(e) => {
            println!("Error getting connection from pool: {}", e);
            std::process::exit(1);
        }
    };

    //clear the database first 
    let clear = conn.batch_execute("DROP TABLE IF EXISTS users;").await;
    match clear {
        Ok(_) => println!("Cleared DataBase"),
        Err(e) => {
            println!("Error clearing database: {}", e);
            std::process::exit(1);
        }
    };
    // read seed.sql file
    let sql_file_content = read_to_string("seed.sql").unwrap();
    let seeding = conn.batch_execute(&sql_file_content).await;

    match seeding {
        Ok(_) => println!("Seeding the database"),
        Err(e) => {
            println!("Error seeding database: {}", e);
            std::process::exit(1);
        }
    };


    println!("Starting Web server");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(format!("0.0.0.0:{}",port))?
    .run()
    .await
}