use crate::errors::app_request_error::AppRequestError;
use crate::errors::db_error::DbError;
use crate::middlewares::authentication_middleware::AuthenticationMiddleware;
use crate::models::folder::{
    FolderDetailWithEntriesDTO, InsertFolderInputDTO, UpdateFolderInputDTO,
};
use crate::models::token::Token;
use crate::repository::{entry_repository, folder_repository};
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
            .service(
                web::resource("")
                    // .route(web::post().to(post_one)),
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(get_one))
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