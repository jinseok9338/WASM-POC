

use actix_web::{web, HttpResponse, Responder, HttpRequest,};

 async fn hello(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

 async fn echo(_req: HttpRequest, body: web::Bytes) -> impl Responder {
    let req_body = String::from_utf8_lossy(&body).to_string();
    HttpResponse::Ok().body(req_body)
}
 async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}



pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(hello))
            .route(web::post().to(echo))
    );


    cfg.route("/hey", web::get().to(manual_hello));
}