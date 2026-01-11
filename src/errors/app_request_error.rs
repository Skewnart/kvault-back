use crate::errors::db_error::DbError;
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use std::env::VarError;

#[derive(Debug, Display)]
pub enum AppRequestError {
    Unauthorized(String),
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
            AppRequestError::InternalTokenError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            AppRequestError::InternalDbError(ref err) => {
                match err {
                    DbError::NotFound => HttpResponse::NotFound().finish(),
                    _ => HttpResponse::InternalServerError().body(err.to_string()),
                }
            }
            AppRequestError::InternalEnvVarError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
        }
    }
}
