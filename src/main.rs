use actix_web::{get, web, App, HttpServer, Responder};

#[get("/api")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/api/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(hello))
        .bind(("0.0.0.0", 5001))?
        .run()
        .await
}
