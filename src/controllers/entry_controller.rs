use crate::errors::app_request_error::AppRequestError;
use crate::errors::db_error::DbError;
use crate::middlewares::authentication_middleware::AuthenticationMiddleware;
use crate::models::envelope::EncodedDataDTO;
use crate::models::token::Token;
use crate::repository::entry_repository;
use actix_web::{
    HttpResponse,
    web::{self, ThinData},
};
use deadpool_postgres::{Client, Pool};
use log::info;

const ENDPOINT: &str = "/entry";

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ENDPOINT)
            .wrap(AuthenticationMiddleware)
            .service(web::resource("").route(web::post().to(post_one)))
            .service(
                web::scope("/{id}").service(
                    web::resource("")
                        .route(web::get().to(get_one))
                        .route(web::put().to(put_one))
                        .route(web::delete().to(delete_one)),
                ),
            ),
    );
}

async fn get_one(
    ThinData(db_pool): ThinData<Pool>,
    token: Token,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppRequestError> {
    info!("/GET entry/{id}");

    let client: Client = db_pool
        .get()
        .await
        .map_err(DbError::from)
        .map_err(AppRequestError::InternalDbError)?;

    let id = id.into_inner();
    let entry = entry_repository::get(&client, id, token.sub)
        .await
        .map_err(AppRequestError::InternalDbError)?;

    Ok(HttpResponse::Ok().json(entry))
}

async fn post_one(
    enc_string_json: web::Json<EncodedDataDTO>,
    token: Token,
    ThinData(db_pool): ThinData<Pool>,
) -> Result<HttpResponse, AppRequestError> {
    info!("/POST entry/new");

    let enc_string_dto: EncodedDataDTO = enc_string_json.into_inner();
    let db_client: Client = db_pool
        .get()
        .await
        .map_err(DbError::PoolError)
        .map_err(AppRequestError::InternalDbError)?;

    let entry_id = entry_repository::insert(&db_client, enc_string_dto, token.sub)
        .await
        .map_err(AppRequestError::InternalDbError)?;

    Ok(HttpResponse::Created().body(entry_id.to_string()))
}

async fn put_one(
    enc_string_json: web::Json<EncodedDataDTO>,
    ThinData(db_pool): ThinData<Pool>,
    token: Token,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppRequestError> {
    info!("/PUT entry/{id}");

    let client: Client = db_pool
        .get()
        .await
        .map_err(DbError::from)
        .map_err(AppRequestError::InternalDbError)?;

    let enc_string_dto: EncodedDataDTO = enc_string_json.into_inner();
    let id = id.into_inner();
    entry_repository::update(&client, enc_string_dto, id, token.sub)
        .await
        .map_err(AppRequestError::InternalDbError)?;

    Ok(HttpResponse::NoContent().finish())
}

async fn delete_one(
    ThinData(db_pool): ThinData<Pool>,
    token: Token,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppRequestError> {
    info!("/DELETE entry/{id}");

    let client: Client = db_pool
        .get()
        .await
        .map_err(DbError::from)
        .map_err(AppRequestError::InternalDbError)?;

    let id = id.into_inner();
    entry_repository::delete(&client, id, token.sub)
        .await
        .map_err(AppRequestError::InternalDbError)?;

    Ok(HttpResponse::NoContent().finish())
}
