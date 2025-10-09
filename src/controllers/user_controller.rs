use crate::{database, errors::db_error::DbError, models::db::user::User};
use actix_web::{
    Error, HttpResponse,
    web::{self, ThinData},
};
use deadpool_postgres::{Client, Pool};

const ENDPOINT: &str = "/users";

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource(ENDPOINT)
            .route(web::get().to(get_users))
            .route(web::post().to(add_user)),
    );
}

async fn get_users(ThinData(db_pool): web::ThinData<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(DbError::PoolError)?;
    let users = database::get_users(&client).await?;

    Ok(HttpResponse::Ok().json(users))
}

async fn add_user(
    user: web::Json<User>,
    ThinData(db_pool): web::ThinData<Pool>,
) -> Result<HttpResponse, Error> {
    let user_info: User = user.into_inner();
    let client: Client = db_pool.get().await.map_err(DbError::PoolError)?;
    let new_user = database::add_user(&client, user_info).await?;

    Ok(HttpResponse::Ok().json(new_user))
}
