use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, From};
use crate::errors::db_error::DbError;

#[derive(Debug, Display, From)]
pub enum AppError {
    Unauthorized,
    Forbidden,
    NotFound,
    InternalServerError(String),
    InternalDbError(DbError)
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AppError::Unauthorized => HttpResponse::Unauthorized().finish(),
            AppError::Forbidden => HttpResponse::Forbidden().finish(),
            AppError::NotFound => HttpResponse::NotFound().finish(),
            AppError::InternalServerError(ref err) => HttpResponse::InternalServerError().body(err.to_string()),
            AppError::InternalDbError(ref err) => HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}
