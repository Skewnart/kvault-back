use actix_web::{HttpResponse, web};
use log::info;
use crate::errors::app_request_error::AppRequestError;

const ENDPOINT: &str = "/infos";
const ENDPOINT_VERSION: &str = "/version";

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ENDPOINT)
            .service(web::resource(ENDPOINT_VERSION).route(web::get().to(get_version))),
    );
}

async fn get_version() -> Result<HttpResponse, AppRequestError> {
    info!("/GET version");

    const CARGO_VERSION: &str = "CARGO_PKG_VERSION";

    let version =
        std::env::var(CARGO_VERSION)
            .map_err(AppRequestError::InternalEnvVarError)?;

    Ok(HttpResponse::Ok().body(version))
}
