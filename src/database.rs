use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::models::{errors::DbError, db::{user::User}};

pub async fn get_users(client: &Client) -> Result<Vec<User>, DbError> {
    let stmt = include_str!("../resources/sql/users/select_all.sql");
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
    let _stmt = include_str!("../resources/sql/users/insert.sql");
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
