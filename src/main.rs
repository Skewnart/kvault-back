mod controllers;
mod database;
mod models;

use actix_web::{App, HttpServer, web};
use std::env;

use confik::{Configuration as _, EnvSource};
use tokio_postgres::NoTls;

use self::controllers::user_controller;
use self::models::config::EnvConfig;

// TODO déplacer les actions dans des contrôleurs exprès (dossier controllers [x], models [x], errors [], resources [], repository [] (voir en-dessous))
// TODO dossier repository au lieu du fichier database.rs
// TODO récupérer le port d'écoute depuis la config au lieu juste du 5001 en paramètre
// TODO delete les warnings au build
// TODO il faut mettre en place le logging avec les niveaux de log
// TODO implémenter les tests ??

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut args = env::args();
    args.next().expect("First argument is missing");

    let port = match args.next() {
        Some(port) => port.parse::<u16>().unwrap_or(5001),
        _ => 5001,
    };

    println!("Port utilisé : {port}");

    dotenvy::dotenv().ok();

    let config = EnvConfig::builder()
        .override_with(EnvSource::new())
        .try_build()
        .unwrap();

    let pool = config.database.create_pool(None, NoTls).unwrap();

    // # Test de connexion à la base au démarrage
    println!("Test de connexion à la base PostgreSQL.");
    match pool.get().await {
        Ok(_) => println!("Connexion à la base PostgreSQL réussie."),
        Err(e) => {
            eprintln!("Erreur de connexion à la base PostgreSQL : {e}");
            std::process::exit(1);
        }
    }

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
