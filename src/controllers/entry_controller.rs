use crate::errors::app_request_error::AppRequestError;
use crate::errors::db_error::DbError;
use crate::middlewares::authentication_middleware::AuthenticationMiddleware;
use crate::models::entry::InsertEntryInputDTO;
use crate::models::token::Token;
use crate::repository::entry_repository;
use actix_web::{
    HttpResponse,
    web::{self, ThinData},
};
use deadpool_postgres::{Client, Pool};
use log::info;

const ENDPOINT: &str = "/entry";
const ENDPOINT_PASSWORD: &str = "/password";

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ENDPOINT)
            .wrap(AuthenticationMiddleware)
            .service(
                web::resource("")
                    .route(web::post().to(post_one)),
            )
            .service(
                web::scope("/{id}")
                    .service(web::resource("").route(web::get().to(get_one)))
                    .service(
                        web::resource(ENDPOINT_PASSWORD).route(web::get().to(get_one_password)),
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
    let entry = entry_repository::get_one_by_id_user_id(&client, id, token.sub)
        .await
        .map_err(AppRequestError::InternalDbError)?;

    Ok(HttpResponse::Ok().json(entry))
}

async fn get_one_password(
    ThinData(db_pool): ThinData<Pool>,
    token: Token,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppRequestError> {
    info!("/GET entry/{id}/password");

    let client: Client = db_pool
        .get()
        .await
        .map_err(DbError::from)
        .map_err(AppRequestError::InternalDbError)?;

    let id = id.into_inner();
    let entry = entry_repository::get_password_by_id_user_id(&client, id, token.sub)
        .await
        .map_err(AppRequestError::InternalDbError)?;

    Ok(HttpResponse::Ok().json(entry))
}

async fn post_one(
    insert_entry_json: web::Json<InsertEntryInputDTO>,
    token: Token,
    ThinData(db_pool): ThinData<Pool>,
) -> Result<HttpResponse, AppRequestError> {
    info!("/POST entry");

    let insert_entry_dto: InsertEntryInputDTO = insert_entry_json.into_inner();
    let db_client: Client = db_pool
        .get()
        .await
        .map_err(DbError::PoolError)
        .map_err(AppRequestError::InternalDbError)?;

    let entry_id = entry_repository::insert(&db_client, insert_entry_dto, token.sub)
        .await
        .map_err(AppRequestError::InternalDbError)?;

    Ok(HttpResponse::Created().body(entry_id.to_string()))
}
