use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{errors::db_error::DbError, models::db::user::User};
use crate::models::authentication::user::{LoginDTO, LoginUsernameDTO};

pub async fn get_user(client: &Client) -> Result<Vec<User>, DbError> {
    let stmt = include_str!("./sql/users/select_all.sql");
    let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();

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
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &user_info.email,
                &user_info.first_name,
                &user_info.last_name,
                &user_info.username,
            ],
        )
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(DbError::NotFound) // more applicable for SELECTs
}

pub async fn check_login(client: &Client, login_username_dto: LoginUsernameDTO) -> Result<bool, DbError> {
    let stmt = include_str!("./sql/users/check_username.sql");
    let stmt = client.prepare(&stmt).await?;

    Ok(
        client
        .query(&stmt, &[
            &login_username_dto.username
        ])
        .await?
        .len() > 0
    )
}

pub async fn try_login(client: &Client, login_dto: LoginDTO) -> Result<i64, DbError> {
    let stmt = include_str!("./sql/users/login.sql");
    let stmt = client.prepare(&stmt).await?;

    client
        .query(&stmt, &[
            &login_dto.username,
            &login_dto.password
        ])
        .await?
        .iter()
        .map(|row| row.get(0))
        .collect::<Vec<i64>>()
        .pop()
        .ok_or(DbError::NotFound)
}