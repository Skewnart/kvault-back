use crate::errors::app_request_error::AppRequestError;
use crate::errors::db_error::DbError;
use crate::middlewares::authentication_middleware::AuthenticationMiddleware;

use crate::models::envelope::EncStringDTO;
use crate::models::token::Token;
use crate::repository::{folder_repository, user_repository};
use actix_web::{
    HttpResponse,
    web::{self, ThinData},
};
use deadpool_postgres::{Client, Pool};
use log::info;

const ENDPOINT: &str = "/folder";

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ENDPOINT)
            .wrap(AuthenticationMiddleware)
            .service(
                web::resource("")
                    .route(web::get().to(get_all))
                    .route(web::post().to(set_all)),
            )
            .service(web::resource("/new").route(web::post().to(post_one)))
            .service(
                web::resource("/{id}")
                    .route(web::get().to(get_one))
                    .route(web::put().to(put_one))
                    .route(web::delete().to(delete_one)),
            ),
    );
}

async fn get_all(
    ThinData(db_pool): ThinData<Pool>,
    token: Token,
) -> Result<HttpResponse, AppRequestError> {
    info!("/GET folder");

    let client: Client = db_pool
        .get()
        .await
        .map_err(DbError::from)
        .map_err(AppRequestError::InternalDbError)?;

    let enc_folders = user_repository::get_enc_folders_by_id(&client, token.sub)
        .await
        .map_err(AppRequestError::InternalDbError)?;

    Ok(HttpResponse::Ok().json(enc_folders))
}

async fn set_all(
    ThinData(db_pool): ThinData<Pool>,
    token: Token,
    enc_string_json: web::Json<EncStringDTO>,
) -> Result<HttpResponse, AppRequestError> {
    info!("/POST folder");

    let client: Client = db_pool
        .get()
        .await
        .map_err(DbError::from)
        .map_err(AppRequestError::InternalDbError)?;

    let enc_string_json = enc_string_json.into_inner();

    user_repository::update_enc_folders_by_id(&client, enc_string_json, token.sub)
        .await
        .map_err(AppRequestError::InternalDbError)?;

    Ok(HttpResponse::NoContent().finish())
}

async fn post_one(
    token: Token,
    ThinData(db_pool): ThinData<Pool>,
    enc_entries_json: web::Json<EncStringDTO>,
) -> Result<HttpResponse, AppRequestError> {
    info!("/POST folder/new");

    let db_client: Client = db_pool
        .get()
        .await
        .map_err(DbError::PoolError)
        .map_err(AppRequestError::InternalDbError)?;

    let enc_entries_json = enc_entries_json.into_inner();

    let folder_id = folder_repository::insert(&db_client, token.sub, &enc_entries_json)
        .await
        .map_err(AppRequestError::InternalDbError)?;

    Ok(HttpResponse::Created().body(folder_id.to_string()))
}

async fn get_one(
    ThinData(db_pool): ThinData<Pool>,
    token: Token,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppRequestError> {
    info!("/GET folder/{id}");

    let client: Client = db_pool
        .get()
        .await
        .map_err(DbError::from)
        .map_err(AppRequestError::InternalDbError)?;

    let id = id.into_inner();
    let enc_entries = folder_repository::get(&client, id, token.sub)
        .await
        .map_err(AppRequestError::InternalDbError)?;

    Ok(HttpResponse::Ok().json(enc_entries))
}

async fn put_one(
    ThinData(db_pool): ThinData<Pool>,
    token: Token,
    enc_string_json: web::Json<EncStringDTO>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppRequestError> {
    info!("/PUT folder/{id}");

    let client: Client = db_pool
        .get()
        .await
        .map_err(DbError::from)
        .map_err(AppRequestError::InternalDbError)?;

    let id = id.into_inner();
    let enc_string_json = enc_string_json.into_inner();

    folder_repository::update(&client, enc_string_json, id, token.sub)
        .await
        .map_err(AppRequestError::InternalDbError)?;

    Ok(HttpResponse::NoContent().finish())
}

async fn delete_one(
    ThinData(db_pool): ThinData<Pool>,
    token: Token,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppRequestError> {
    info!("/DELETE folder/{id}");

    let client: Client = db_pool
        .get()
        .await
        .map_err(DbError::from)
        .map_err(AppRequestError::InternalDbError)?;

    let id = id.into_inner();
    folder_repository::delete(&client, id, token.sub)
        .await
        .map_err(AppRequestError::InternalDbError)?;

    Ok(HttpResponse::NoContent().finish())
}
