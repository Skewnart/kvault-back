use crate::{errors::db_error::DbError, repository::user_repository};
use actix_web::{HttpResponse, web::{self, ThinData}};
use deadpool_postgres::{Client, Pool};
use log::{info};
use crate::errors::app_request_error::AppRequestError;
use crate::middlewares::authentication_middleware::AuthenticationMiddleware;
use crate::models::authentication::token::Token;

const ENDPOINT: &str = "/profile";

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource(ENDPOINT)
            .wrap(AuthenticationMiddleware)
            .route(web::get().to(get))
    );
}

async fn get(ThinData(db_pool): ThinData<Pool>, token: Token) -> Result<HttpResponse, AppRequestError> {
    info!("/GET profile");

    let client: Client = db_pool.get().await
        .map_err(DbError::from)
        .map_err(AppRequestError::InternalDbError)?;
    let user = user_repository::get_by_id(&client, token.sub).await
        .map_err(AppRequestError::InternalDbError)?;

    Ok(HttpResponse::Ok().json(user))
}
