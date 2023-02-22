use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use std::process;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("pid:{}", process::id());
    HttpServer::new(|| App::new().service(hello).service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
