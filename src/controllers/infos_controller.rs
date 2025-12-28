use actix_web::{HttpResponse, web};
use log::info;

use crate::errors::common_error::CommonErrors;

const ENDPOINT: &str = "/infos";
const ENDPOINT_VERSION: &str = "/version";

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ENDPOINT)
            .service(web::resource(ENDPOINT_VERSION).route(web::get().to(get_version))),
    );
}

async fn get_version() -> Result<HttpResponse, CommonErrors> {
    info!("/GET version");

    const CARGO_VERSION: &str = "CARGO_PKG_VERSION";

    let version =
        std::env::var(CARGO_VERSION).map_err(|err| CommonErrors::RuntimeError(err.to_string()))?;

    Ok(HttpResponse::Ok().body(version))
}
