use crate::errors::app_request_error::AppRequestError;
use crate::errors::db_error::DbError;
use crate::models::user::UserProfileDTO;
use crate::models::user::{LoginDTO, RegisterDTO};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::Row;

pub async fn get_by_id(client: &Client, user_id: i64) -> Result<UserProfileDTO, DbError> {
    let _stmt = include_str!("sql/users/get_by_id.sql");
    let _stmt = client.prepare(_stmt).await?;

    let user = client
        .query(&_stmt, &[&user_id])
        .await?
        .iter()
        .map(|row| UserProfileDTO::from_row_ref(row).unwrap())
        .collect::<Vec<UserProfileDTO>>()
        .pop()
        .ok_or(DbError::NotFound)?;

    Ok(user)
}

pub async fn login(client: &Client, login_dto: LoginDTO) -> Result<i64, AppRequestError> {
    let _stmt = include_str!("./sql/users/login.sql");
    let _stmt = client
        .prepare(_stmt)
        .await
        .map_err(DbError::from)
        .map_err(AppRequestError::InternalDbError)?;

    let rows = client
        .query(&_stmt, &[&login_dto.username])
        .await
        .map_err(DbError::PGError)
        .map_err(AppRequestError::InternalDbError)?;

    let row = rows
        .iter()
        .collect::<Vec<&Row>>()
        .pop()
        .ok_or(DbError::NotFound)
        .map_err(AppRequestError::InternalDbError)?;

    if login_dto.password.is_empty() {
        return Ok(0);
    }

    let password_valid = PasswordHash::new(row.get(1))
        .and_then(|parsed_hash| {
            Argon2::default().verify_password(login_dto.password.as_bytes(), &parsed_hash)
        })
        .is_ok_and(|_| true);

    if !password_valid {
        Err(AppRequestError::Unauthorized(
            "Mot de passe non valide.".to_string(),
        ))?;
    }

    Ok(row.get(0))
}

pub async fn register(client: &Client, register_dto: RegisterDTO) -> Result<i64, DbError> {
    let _stmt = include_str!("./sql/users/insert.sql");
    let _stmt = client.prepare(_stmt).await?;

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(register_dto.password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();

    client
        .query(
            &_stmt,
            &[
                &register_dto.username,
                &hashed_password,
                &register_dto.first_name,
                &register_dto.last_name,
            ],
        )
        .await?
        .iter()
        .map(|row| row.get(0))
        .collect::<Vec<i64>>()
        .pop()
        .ok_or(DbError::NotFound)
}
