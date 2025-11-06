use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, From};

#[derive(Debug, Display, From)]
pub enum CommonErrors {
    RuntimeError(String),
}

impl ResponseError for CommonErrors {
    fn error_response(&self) -> HttpResponse {
        match self {
            CommonErrors::RuntimeError(err) => {
                HttpResponse::InternalServerError().body(err.clone())
            }
        }
    }
}
