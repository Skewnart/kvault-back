use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::Row;
use crate::{errors::db_error::DbError, models::db::user::User};
use crate::errors::app_error::{AppError};
use crate::models::authentication::user::{LoginDTO};

pub async fn get_user(client: &Client) -> Result<Vec<User>, DbError> {
    let stmt = include_str!("./sql/users/select_all.sql");
    let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&stmt).await?;

    let results = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>();

    Ok(results)
}

pub async fn add_user(client: &Client, user_info: User) -> Result<User, DbError> {
    let _stmt = include_str!("./sql/users/insert.sql");
    let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&_stmt).await?;

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(user_info.password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();

    client
        .query(
            &stmt,
            &[
                &user_info.email,
                &user_info.first_name,
                &user_info.last_name,
                &user_info.username,
                &hashed_password
            ],
        )
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(DbError::NotFound) // more applicable for SELECTs
}

pub async fn try_login(client: &Client, login_dto: LoginDTO) -> Result<i64, AppError> {
    let stmt = include_str!("./sql/users/login.sql");
    let stmt = client.prepare(&stmt).await
        .map_err(|_err| AppError::InternalServerError(_err.to_string()))?;

    let rows = client
        .query(&stmt, &[
            &login_dto.username
        ])
        .await
        .map_err(|_err| AppError::InternalServerError(_err.to_string()))?;

    let row = rows
        .iter()
        .collect::<Vec<&Row>>()
        .pop()
        .ok_or(AppError::NotFound)?;

    if login_dto.password.len() == 0 {
        return Ok(0);
    }

    let password_valid = PasswordHash::new(row.get(1))
        .and_then(|parsed_hash| {
            Argon2::default().verify_password(login_dto.password.as_bytes(), &parsed_hash)
        })
        .map_or(false, |_| true);

    if !password_valid {
        return Err(AppError::Unauthorized.into());
    }

    Ok(row.get(0))
}