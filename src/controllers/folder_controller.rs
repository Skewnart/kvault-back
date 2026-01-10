use crate::errors::app_request_error::AppRequestError;
use crate::errors::db_error::DbError;
use crate::middlewares::authentication_middleware::AuthenticationMiddleware;
use crate::models::folder::{InsertFolderDTO, UpdateFolderDTO};
use crate::models::token::Token;
use crate::repository::folder_repository;
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
                    .route(web::post().to(post_one)),
            )
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

    let folders = folder_repository::get_all_by_user_id(&client, token.sub)
        .await
        .map_err(AppRequestError::InternalDbError)?;

    Ok(HttpResponse::Ok().json(folders))
}

async fn post_one(
    insert_folder_json: web::Json<InsertFolderDTO>,
    token: Token,
    ThinData(db_pool): ThinData<Pool>,
) -> Result<HttpResponse, AppRequestError> {
    info!("/POST folder");

    let insert_folder_dto: InsertFolderDTO = insert_folder_json.into_inner();
    let db_client: Client = db_pool
        .get()
        .await
        .map_err(DbError::PoolError)
        .map_err(AppRequestError::InternalDbError)?;

    let folder_id = folder_repository::insert(&db_client, insert_folder_dto, token.sub)
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
    let folder = folder_repository::get_one_by_id_user_id(&client, id, token.sub)
        .await
        .map_err(AppRequestError::InternalDbError)?;

    Ok(HttpResponse::Ok().json(folder))
}

async fn put_one(
    update_folder_json: web::Json<UpdateFolderDTO>,
    ThinData(db_pool): ThinData<Pool>,
    token: Token,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppRequestError> {
    info!("/PUT folder/{id}");

    let client: Client = db_pool
        .get()
        .await
        .map_err(DbError::from)
        .map_err(AppRequestError::InternalDbError)?;

    let update_folder_dto: UpdateFolderDTO = update_folder_json.into_inner();
    let id = id.into_inner();
    folder_repository::update(&client, update_folder_dto, id, token.sub)
        .await
        .map_err(AppRequestError::InternalDbError)?;

    Ok(HttpResponse::Ok().finish())
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

    Ok(HttpResponse::Ok().finish())
}
