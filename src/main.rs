mod controllers;
mod errors;
mod middlewares;
mod models;
mod repository;

use actix_web::{App, HttpServer, web};

use confik::{Configuration as _, EnvSource};
use log::{error, info};
use tokio_postgres::NoTls;

use crate::middlewares::error_logger::ErrorLogger;

use self::controllers::user_controller;
use self::models::config::env_config::EnvConfig;

// TODO implémenter les tests ??

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let config = EnvConfig::builder()
        .override_with(EnvSource::new())
        .try_build()
        .expect("Configuration from .env file failed ");

    env_logger::init();

    let pool = config.database.create_pool(None, NoTls).unwrap();
    let port = config.server.port;

    info!("Test de connexion à la base PostgreSQL.");
    match pool.get().await {
        Ok(_) => info!("Connexion à la base PostgreSQL réussie."),
        Err(e) => {
            error!("Erreur de connexion à la base PostgreSQL : {e}");
            std::process::exit(1);
        }
    }

    info!("Port externe de l'application : {port}");

    HttpServer::new(move || {
        App::new().service(
            web::scope("/api")
                .wrap(ErrorLogger)
                .app_data(web::ThinData(pool.clone()))
                .configure(user_controller::configure),
        )
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
