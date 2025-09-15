use std::env;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, World !")
}

async fn hello(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {} !", &name))
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

    HttpServer::new(
        || App::new()
            .service(
                web::scope("/api")
                    .route("", web::get().to(index))
                    .route("{name}", web::get().to(hello))
            ))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
