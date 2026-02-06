use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use serde::Serialize;

#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

#[get("/hello/{name}")]
async fn hello(path: web::Path<String>) -> impl Responder {
    let response = HelloResponse {
        message: format!("Hello, {}!", path.into_inner()),
    };

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or_else(|_| String::from("8082"));
    let address = format!("127.0.0.1:{port}");

    println!("web_actix listening on http://{address}");

    HttpServer::new(|| App::new().service(health).service(hello))
        .bind(address)?
        .run()
        .await
}
