use crate::{errors::db_error::DbError, repository::user_repository};
use actix_web::{HttpResponse, web::{self, ThinData}};
use actix_web::web::Data;
use deadpool_postgres::{Client, Pool};
use log::{info};
use crate::errors::app_error::AppError;
use crate::models::authentication::jwt_keys::JwtKeys;
use crate::models::authentication::token::Token;
use crate::models::authentication::user::{LoginDTO};

const ENDPOINT: &str = "/login";

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource(ENDPOINT)
                .route(web::post().to(login)));
}

async fn login(
    login_json: web::Json<LoginDTO>,
    ThinData(db_pool): ThinData<Pool>,
    jwt_keys: Data<JwtKeys>
) -> Result<HttpResponse, AppError> {
    info!("/POST login");

    let login_dto : LoginDTO = login_json.into_inner();
    let db_client: Client = db_pool.get().await
        .map_err(DbError::PoolError)
        .map_err(AppError::InternalDbError)?;
    let user_id = user_repository::try_login(&db_client, login_dto).await?;

    if user_id == 0 {
        return Ok(HttpResponse::Ok().finish());
    }

    let token = Token::generate(user_id);
    let encoded_token = token.encode(jwt_keys.get_encoding())
        .map_err(|_err| AppError::InternalServerError(_err.to_string()))?;

    Ok(HttpResponse::Ok().body(encoded_token))
}
