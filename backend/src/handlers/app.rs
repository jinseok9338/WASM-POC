use std::env;

use actix_web::{web, HttpResponse, Responder, HttpRequest, http::header};

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

async fn login_request(_req: HttpRequest ) -> impl Responder {
   // send get request to  kauth.kakao.com/oauth/authorize?client_id=${REST_API_KEY}&redirect_uri=${REDIRECT_URI}&response_type=code
   // get REST_API_KEY from env variable
   // get REDIRECT_URI from env variable
   dotenv::dotenv().ok();
   let redirect_uri = env::var("REDIRECT_URI").unwrap();
   let rest_api_key = env::var("REST_API_KEY").unwrap();

    let url = format!("https://kauth.kakao.com/oauth/authorize?client_id={}&redirect_uri={}&response_type=code", rest_api_key, redirect_uri);
    // send get request to url
    let res = reqwest::get(url)
    .await.unwrap()
    .text()
    .await.unwrap();

    HttpResponse::Ok().body(res)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(hello))
            .route(web::post().to(echo))
    );
    cfg.service(
        web::resource("/login")
            .route(web::get().to(login_request))
    );

    cfg.route("/hey", web::get().to(manual_hello));
}