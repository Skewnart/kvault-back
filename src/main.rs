mod controllers;
mod errors;
mod models;
mod repository;

use actix_web::{App, HttpServer, web};

use confik::{Configuration as _, EnvSource};
use tokio_postgres::NoTls;

use self::controllers::user_controller;
use self::models::config::EnvConfig;

// TODO scinder les modèles de configuration exprès dans un module de configuration
// TODO delete les warnings du build
// TODO gérer les manques dans le .env ?
// TODO il faut mettre en place le logging avec les niveaux de log
// TODO implémenter les tests ??

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenvy::dotenv().ok();
    
    let config = EnvConfig::builder()
    .override_with(EnvSource::new())
    .try_build()
    .unwrap();

    let pool = config.database.create_pool(None, NoTls).unwrap();
    let port = config.server.port;

    // # Test de connexion à la base au démarrage
    println!("Test de connexion à la base PostgreSQL.");
    match pool.get().await {
        Ok(_) => println!("Connexion à la base PostgreSQL réussie."),
        Err(e) => {
            eprintln!("Erreur de connexion à la base PostgreSQL : {e}");
            std::process::exit(1);
        }
    }

    println!("Port utilisé : {port}");

    HttpServer::new(move || {
        App::new().service(
            web::scope("/api")
                .app_data(web::ThinData(pool.clone()))
                .configure(user_controller::configure),
        )
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
