use crate::errors::app_request_error::AppRequestError;
use crate::errors::db_error::DbError;
use crate::middlewares::authentication_middleware::AuthenticationMiddleware;
use crate::models::token::Token;
use crate::repository::envelope_repository;
use actix_web::{
    HttpResponse,
    web::{self, ThinData},
};
use deadpool_postgres::{Client, Pool};
use log::info;

const ENDPOINT: &str = "/envelope";

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource(ENDPOINT)
            .wrap(AuthenticationMiddleware)
            .route(web::get().to(get)),
    );
}

async fn get(
    ThinData(db_pool): ThinData<Pool>,
    token: Token,
) -> Result<HttpResponse, AppRequestError> {
    info!("/GET envelope");

    let client: Client = db_pool
        .get()
        .await
        .map_err(DbError::from)
        .map_err(AppRequestError::InternalDbError)?;
    let envelope = envelope_repository::get_by_user_id(&client, token.infos.user_id)
        .await
        .map_err(AppRequestError::InternalDbError)?;

    Ok(HttpResponse::Ok().json(envelope))
}
