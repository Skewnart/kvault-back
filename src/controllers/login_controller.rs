use crate::{errors::db_error::DbError, repository::user_repository};
use actix_web::{
    Error, HttpResponse,
    web::{self, ThinData},
};
use actix_web::web::Data;
use deadpool_postgres::{Client, Pool};
use log::{debug, info};
use crate::models::authentication::jwt_keys::JwtKeys;
use crate::models::authentication::token::Token;
use crate::models::authentication::user::{LoginDTO, LoginUsernameDTO};

const ENDPOINT: &str = "/login";

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource(ENDPOINT)
                .route(web::post().to(login))
        ).service(
            web::resource(ENDPOINT.to_string() + "/check")
                .route(web::post().to(check_username))
        );
}

async fn check_username(
    login_json: web::Json<LoginUsernameDTO>,
    ThinData(db_pool): ThinData<Pool>
) -> Result<HttpResponse, Error> {
    info!("/POST check_username");

    let login_username_dto : LoginUsernameDTO = login_json.into_inner();
    let db_client: Client = db_pool.get().await.map_err(DbError::PoolError)?;
    let username_exists = user_repository::check_login(&db_client, login_username_dto).await?;

    Ok(HttpResponse::Ok().json(username_exists))
}

async fn login(
    login_json: web::Json<LoginDTO>,
    ThinData(db_pool): ThinData<Pool>,
    jwt_keys: Data<JwtKeys>
) -> Result<HttpResponse, Error> {
    info!("/POST login");

    let login_dto : LoginDTO = login_json.into_inner();
    let db_client: Client = db_pool.get().await.map_err(DbError::PoolError)?;
    let user_id = user_repository::try_login(&db_client, login_dto).await?;

    let token = Token::generate(user_id);
    let encoded_token = token.encode(jwt_keys.get_encoding())?;

    //Ici rajouter le header Authorization avec le user
    debug!("login user id: {}", user_id);
    debug!("encoded token: {}", encoded_token);

    Ok(HttpResponse::Ok().body(encoded_token))
}
