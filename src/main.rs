mod controllers;
mod errors;
mod middlewares;
mod models;
mod repository;

use self::controllers::{connection_controller, infos_controller, profile_controller};
use self::models::config::env_config::EnvConfig;
use crate::controllers::{entry_controller, folder_controller};
use crate::middlewares::error_logger_middleware::ErrorLoggerMiddleware;
use actix_web::{App, HttpServer, web};
use confik::{Configuration as _, EnvSource};
use log::{error, info};
use tokio_postgres::NoTls;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let config = EnvConfig::builder()
        .override_with(EnvSource::new())
        .try_build()
        .expect("Configuration from .env file failed ");

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

    let jwt_config = web::Data::new(config.jwt);

    info!("Port externe de l'application : {port}");

    HttpServer::new(move || {
        App::new().service(
            web::scope("/api")
                .wrap(ErrorLoggerMiddleware)
                .app_data(web::ThinData(pool.clone()))
                .app_data(web::Data::clone(&jwt_config))
                .configure(connection_controller::configure)
                .configure(profile_controller::configure)
                .configure(folder_controller::configure)
                .configure(entry_controller::configure)
                .configure(infos_controller::configure),
        )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
