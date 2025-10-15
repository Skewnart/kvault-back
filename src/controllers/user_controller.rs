use crate::{errors::db_error::DbError, models::db::user::User, repository::user_repository};
use actix_web::{
    Error, HttpResponse,
    web::{self, ThinData},
};
use deadpool_postgres::{Client, Pool};
use log::info;

const ENDPOINT: &str = "/users";

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource(ENDPOINT)
            .route(web::get().to(get_users))
            .route(web::post().to(add_user)),
    );
}

async fn get_users(ThinData(db_pool): web::ThinData<Pool>) -> Result<HttpResponse, Error> {
    info!("/GET users");

    let client: Client = db_pool.get().await.map_err(DbError::PoolError)?;
    let users = user_repository::get_user(&client).await?;

    Ok(HttpResponse::Ok().json(users))
}

async fn add_user(
    user: web::Json<User>,
    ThinData(db_pool): web::ThinData<Pool>,
) -> Result<HttpResponse, Error> {
    info!("/POST users");

    let user_info: User = user.into_inner();
    let client: Client = db_pool.get().await.map_err(DbError::PoolError)?;
    let new_user = user_repository::add_user(&client, user_info).await?;

    Ok(HttpResponse::Ok().json(new_user))
}
