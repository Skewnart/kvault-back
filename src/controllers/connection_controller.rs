use crate::{errors::db_error::DbError, repository::user_repository};
use actix_web::{HttpResponse, web::{self, ThinData}};
use actix_web::web::Data;
use deadpool_postgres::{Client, Pool};
use log::{info};
use crate::errors::app_request_error::AppRequestError;
use crate::models::authentication::token::Token;
use crate::models::authentication::user::{LoginDTO, RegisterDTO};
use crate::models::config::jwt_config::JwtConfig;

const ENDPOINT: &str = "/connection";

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope(ENDPOINT)
                .service(
                    web::resource("login")
                        .route(web::post().to(login))
                )
                .service(
                    web::resource("register")
                        .route(web::post().to(register))
                )
        );
}

async fn login(
    login_json: web::Json<LoginDTO>,
    ThinData(db_pool): ThinData<Pool>,
    jwt_config: Data<JwtConfig>
) -> Result<HttpResponse, AppRequestError> {
    info!("/POST login");

    let login_dto : LoginDTO = login_json.into_inner();
    let db_client: Client = db_pool.get().await
        .map_err(DbError::PoolError)
        .map_err(AppRequestError::InternalDbError)?;
    let user_id = user_repository::login(&db_client, login_dto).await?;

    if user_id == 0 {
        return Ok(HttpResponse::Ok().finish());
    }

    let token = Token::generate(user_id, jwt_config.ttl);
    let encoded_token = token.encode(jwt_config.sk.clone())
        .map_err(|_err| AppRequestError::InternalTokenError(_err.to_string()))?;

    Ok(HttpResponse::Ok().body(encoded_token))
}

async fn register(
    register_json: web::Json<RegisterDTO>,
    ThinData(db_pool): ThinData<Pool>
) -> Result<HttpResponse, AppRequestError> {
    info!("/POST register");

    let register_dto : RegisterDTO = register_json.into_inner();
    let db_client: Client = db_pool.get().await
        .map_err(DbError::PoolError)
        .map_err(AppRequestError::InternalDbError)?;
    
    let user_id = user_repository::register(&db_client, register_dto).await
        .map_err(AppRequestError::InternalDbError)?;

    Ok(HttpResponse::Created().body(user_id.to_string()))
}
