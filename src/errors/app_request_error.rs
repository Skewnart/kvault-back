use crate::errors::db_error::DbError;
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use std::env::VarError;

#[derive(Debug, Display)]
pub enum AppRequestError {
    Unauthorized(String),
    Forbidden(String),
    InternalSerializationError(serde_json::error::Error),
    InternalTokenError(String),
    InternalDbError(DbError),
    InternalEnvVarError(VarError),
}

impl ResponseError for AppRequestError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AppRequestError::Unauthorized(ref err) => {
                HttpResponse::Unauthorized().body(err.to_string())
            }
            AppRequestError::Forbidden(ref err) => HttpResponse::Forbidden().body(err.to_string()),
            AppRequestError::InternalTokenError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            AppRequestError::InternalSerializationError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            AppRequestError::InternalDbError(ref err) => (*err).error_response(),
            AppRequestError::InternalEnvVarError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
        }
    }
}
