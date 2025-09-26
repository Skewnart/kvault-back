use actix_web::{
    App, Error, HttpResponse, HttpServer, Responder,
    web::{self, ThinData},
};
use std::env;

use confik::{Configuration as _, EnvSource};
use deadpool_postgres::{Client, Pool};
use tokio_postgres::NoTls;

mod config;
mod db;
mod errors;
mod models;

use crate::config::EnvConfig;

use self::{errors::DbError, models::User};

// TODO il faut mettre en place le logging avec les niveau de log
// TODO Il faut absolument comprendre toute la mise en place de PG (tout est pris "à la base" des exemples du repo d'actix)
// TODO donc ça comprend aussi la récupération des env et la mise en place de pg

pub async fn get_users(ThinData(db_pool): web::ThinData<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(DbError::PoolError)?;
    let users = db::get_users(&client).await?;

    Ok(HttpResponse::Ok().json(users))
}

pub async fn add_user(
    user: web::Json<User>,
    ThinData(db_pool): web::ThinData<Pool>,
) -> Result<HttpResponse, Error> {
    let user_info: User = user.into_inner();
    let client: Client = db_pool.get().await.map_err(DbError::PoolError)?;
    let new_user = db::add_user(&client, user_info).await?;

    Ok(HttpResponse::Ok().json(new_user))
}

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
        App::new()
            .app_data(web::ThinData(pool.clone()))
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/users")
                            .route(web::get().to(get_users))
                            .route(web::post().to(add_user)),
                    )
                    .route("", web::get().to(index))
                    .route("{name}", web::get().to(hello)),
        )
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
