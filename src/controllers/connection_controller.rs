use crate::errors::app_request_error::AppRequestError;
use crate::models::config::jwt_config::JwtConfig;
use crate::models::token::Token;
use crate::models::user::{LoginDTO, RegisterDTO};
use crate::{errors::db_error::DbError, repository::user_repository};
use actix_web::web::Data;
use actix_web::{
    HttpResponse,
    web::{self, ThinData},
};
use deadpool_postgres::{Client, Pool};
use log::info;

const ENDPOINT: &str = "/connection";

const ENDPOINT_LOGIN: &str = "/login";
const ENDPOINT_REGISTER: &str = "/register";

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ENDPOINT)
            .service(web::resource(ENDPOINT_LOGIN).route(web::post().to(login)))
            .service(web::resource(ENDPOINT_REGISTER).route(web::post().to(register))),
    );
}

async fn login(
    login_json: web::Json<LoginDTO>,
    ThinData(db_pool): ThinData<Pool>,
    jwt_config: Data<JwtConfig>,
) -> Result<HttpResponse, AppRequestError> {
    info!("/POST login");

    let login_dto: LoginDTO = login_json.into_inner();
    let db_client: Client = db_pool
        .get()
        .await
        .map_err(DbError::PoolError)
        .map_err(AppRequestError::InternalDbError)?;
    let user_id = user_repository::login(&db_client, login_dto).await?;

    if user_id == 0 {
        return Ok(HttpResponse::NoContent().finish());
    }

    let token = Token::generate(user_id, jwt_config.ttl);
    let encoded_token = token
        .encode(jwt_config.sk.clone())
        .map_err(|_err| AppRequestError::InternalTokenError(_err.to_string()))?;

    Ok(HttpResponse::Ok().body(encoded_token))
}

async fn register(
    register_json: web::Json<RegisterDTO>,
    ThinData(db_pool): ThinData<Pool>,
) -> Result<HttpResponse, AppRequestError> {
    info!("/POST register");

    let register_dto: RegisterDTO = register_json.into_inner();
    let db_client: Client = db_pool
        .get()
        .await
        .map_err(DbError::PoolError)
        .map_err(AppRequestError::InternalDbError)?;

    let user_id = user_repository::register(&db_client, register_dto)
        .await
        .map_err(AppRequestError::InternalDbError)?;

    Ok(HttpResponse::Created().body(user_id.to_string()))
}
