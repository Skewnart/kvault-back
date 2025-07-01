use std::env;

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
    let mut args = env::args();
    args.next().expect("First argument is missing");

    let port = match args.next() {
        Some(port) => port.parse::<u16>().unwrap_or(5001),
        _ => 5001
    };

    println!("Port utilisé : {port}");

    HttpServer::new(|| App::new().service(index).service(hello))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
