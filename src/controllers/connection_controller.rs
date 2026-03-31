use crate::errors::app_request_error::AppRequestError;
use crate::mapper::invitation_mapper::invitation_to_invitation_output;
use crate::middlewares::authentication_middleware::AuthenticationMiddleware;
use crate::models::config::jwt_config::JwtConfig;
use crate::models::invitation::InvitationInputDTO;
use crate::models::token::Token;
use crate::models::user::{LoginDTO, RegisterDTO, UserType};
use crate::repository::invitation_repository;
use crate::{errors::db_error::DbError, repository::user_repository};
use actix_web::web::Data;
use actix_web::{
    HttpResponse,
    web::{self, ThinData},
};
use deadpool_postgres::{Client, Pool};
use log::info;
use uuid::Uuid;

const ENDPOINT: &str = "/connection";

const ENDPOINT_LOGIN: &str = "/login";
const ENDPOINT_REGISTER: &str = "/register";
const ENDPOINT_REGISTER_INVITATIONS: &str = "/invitations";

// POST connection/login : Login
// GET connection/register/invitations : GET all invitations
// POST connection/register/invitations : new invitation
// POST connection/register/{guid} : use invitation

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ENDPOINT)
            .service(web::resource(ENDPOINT_LOGIN).route(web::post().to(login)))
            .service(
                web::scope(ENDPOINT_REGISTER)
                    .service(
                        web::resource(ENDPOINT_REGISTER_INVITATIONS)
                            .wrap(AuthenticationMiddleware)
                            .route(web::get().to(get_invitations))
                            .route(web::post().to(post_invitation)),
                    )
                    .service(web::resource("{guid}").route(web::post().to(register))),
            ),
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
    let (user_id, user_type) = user_repository::login(&db_client, login_dto).await?;

    if user_id == 0 {
        return Ok(HttpResponse::NoContent().finish());
    }

    let token = Token::generate(user_id, user_type, jwt_config.ttl);
    let encoded_token = token
        .encode(jwt_config.sk.clone())
        .map_err(|_err| AppRequestError::InternalTokenError(_err.to_string()))?;

    Ok(HttpResponse::Ok().body(encoded_token))
}

async fn get_invitations(
    ThinData(db_pool): ThinData<Pool>,
    token: Token,
) -> Result<HttpResponse, AppRequestError> {
    info!("/GET register/invitations");

    if !token.infos.user_type.is_admin() {
        return Err(AppRequestError::Forbidden(String::from(
            "User is not admin",
        )));
    }

    let db_client: Client = db_pool
        .get()
        .await
        .map_err(DbError::PoolError)
        .map_err(AppRequestError::InternalDbError)?;

    let invitations: Vec<_> = invitation_repository::get_all(&db_client)
        .await
        .map_err(AppRequestError::InternalDbError)?
        .into_iter()
        .map(invitation_to_invitation_output)
        .collect();

    let invitations =
        serde_json::to_string(&invitations).map_err(AppRequestError::InternalSerializationError)?;

    Ok(HttpResponse::Ok().body(invitations))
}

async fn post_invitation(
    ThinData(db_pool): ThinData<Pool>,
    token: Token,
    invitation_input: web::Json<InvitationInputDTO>,
) -> Result<HttpResponse, AppRequestError> {
    info!("/GET register/invitations");

    if !token.infos.user_type.is_admin() {
        return Err(AppRequestError::Forbidden(String::from(
            "User is not admin",
        )));
    }

    let db_client: Client = db_pool
        .get()
        .await
        .map_err(DbError::PoolError)
        .map_err(AppRequestError::InternalDbError)?;

    let invitation_input = invitation_input.into_inner();

    let invitation_uuid = invitation_repository::insert(&db_client, &invitation_input)
        .await
        .map_err(AppRequestError::InternalDbError)?;

    Ok(HttpResponse::Created().body(invitation_uuid.to_string()))
}

async fn register(
    register_json: web::Json<RegisterDTO>,
    ThinData(db_pool): ThinData<Pool>,
    guid: web::Path<Uuid>,
    jwt_config: Data<JwtConfig>,
) -> Result<HttpResponse, AppRequestError> {
    info!("/POST register");

    let db_client: Client = db_pool
        .get()
        .await
        .map_err(DbError::PoolError)
        .map_err(AppRequestError::InternalDbError)?;

    let guid: Uuid = guid.into_inner();
    invitation_repository::check_guid(&db_client, guid)
        .await
        .map_err(AppRequestError::InternalDbError)?;

    let register_dto: RegisterDTO = register_json.into_inner();
    let user_id = user_repository::register(&db_client, register_dto)
        .await
        .map_err(AppRequestError::InternalDbError)?;

    invitation_repository::update_user_id(&db_client, user_id, guid)
        .await
        .map_err(AppRequestError::InternalDbError)?;

    let token = Token::generate(user_id, UserType::User, jwt_config.ttl);
    let encoded_token = token
        .encode(jwt_config.sk.clone())
        .map_err(|_err| AppRequestError::InternalTokenError(_err.to_string()))?;

    Ok(HttpResponse::Created().body(encoded_token))
}
