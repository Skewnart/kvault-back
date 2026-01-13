use crate::errors::db_error::DbError;
use crate::models::entry::{EntryDetailOutputDTO, EntryOutputDTO};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_all_by_folder_id_user_id(
    client: &Client,
    folder_id: i64,
    user_id: i64,
) -> Result<Vec<EntryOutputDTO>, DbError> {
    let _stmt = include_str!("sql/entry/get_all_by_folder_id_user_id.sql");
    let _stmt = client.prepare(_stmt).await?;

    let folders = client
        .query(&_stmt, &[&folder_id, &user_id])
        .await?
        .iter()
        .map(|row| EntryOutputDTO::from_row_ref(row).unwrap())
        .collect::<Vec<EntryOutputDTO>>();

    Ok(folders)
}

pub async fn get_one_by_id_user_id(
    client: &Client,
    id: i64,
    user_id: i64,
) -> Result<EntryDetailOutputDTO, DbError> {
    let _stmt = include_str!("sql/entry/get_one_by_id_user_id.sql");
    let _stmt = client.prepare(_stmt).await?;

    let entry = client
        .query(&_stmt, &[&id, &user_id])
        .await?
        .iter()
        .map(|row| EntryDetailOutputDTO::from_row_ref(row).unwrap())
        .collect::<Vec<EntryDetailOutputDTO>>()
        .pop()
        .ok_or(DbError::NotFound)?;

    Ok(entry)
}
