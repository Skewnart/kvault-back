mod database;
mod models;

use actix_web::{
    App, Error, HttpResponse, HttpServer,
    web::{self, ThinData},
};
use std::env;

use confik::{Configuration as _, EnvSource};
use deadpool_postgres::{Client, Pool};
use tokio_postgres::NoTls;

use self::models::{errors::DbError, config::EnvConfig, db::{user::User}};

// TODO il faut mettre en place le logging avec les niveau de log
// TODO déplacer les actions dans des contrôleurs exprès (dossier controllers, models, services, resources)

pub async fn get_users(ThinData(db_pool): web::ThinData<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(DbError::PoolError)?;
    let users = database::get_users(&client).await?;

    Ok(HttpResponse::Ok().json(users))
}

pub async fn add_user(
    user: web::Json<User>,
    ThinData(db_pool): web::ThinData<Pool>,
) -> Result<HttpResponse, Error> {
    let user_info: User = user.into_inner();
    let client: Client = db_pool.get().await.map_err(DbError::PoolError)?;
    let new_user = database::add_user(&client, user_info).await?;

    Ok(HttpResponse::Ok().json(new_user))
}

// async fn index() -> impl Responder {
//     HttpResponse::Ok().body("Hello, World !")
// }

// async fn hello(name: web::Path<String>) -> impl Responder {
//     HttpResponse::Ok().body(format!("Hello {} !", &name))
// }

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
                    // .route("", web::get().to(index))
                    // .route("{name}", web::get().to(hello)),
        )
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
