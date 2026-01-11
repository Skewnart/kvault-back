use crate::errors::db_error::DbError;
use crate::models::folder::{FolderOutputDTO, InsertFolderInputDTO, FolderDetailOutputDTO, UpdateFolderInputDTO};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_all_by_user_id(
    client: &Client,
    user_id: i64,
) -> Result<Vec<FolderOutputDTO>, DbError> {
    let _stmt = include_str!("sql/folder/get_all_by_user_id.sql");
    let _stmt = client.prepare(_stmt).await?;

    let folders = client
        .query(&_stmt, &[&user_id])
        .await?
        .iter()
        .map(|row| FolderOutputDTO::from_row_ref(row).unwrap())
        .collect::<Vec<FolderOutputDTO>>();

    Ok(folders)
}

pub async fn get_one_by_id_user_id(
    client: &Client,
    folder_id: i64,
    user_id: i64,
) -> Result<FolderDetailOutputDTO, DbError> {
    let _stmt = include_str!("sql/folder/get_one_by_id_user_id.sql");
    let _stmt = client.prepare(_stmt).await?;

    let folders = client
        .query(&_stmt, &[&folder_id, &user_id])
        .await?
        .iter()
        .map(|row| FolderDetailOutputDTO::from_row_ref(row).unwrap())
        .collect::<Vec<FolderDetailOutputDTO>>()
        .pop()
        .ok_or(DbError::NotFound)?;

    Ok(folders)
}

pub async fn insert(
    client: &Client,
    insert_folder_dto: InsertFolderInputDTO,
    user_id: i64,
) -> Result<i64, DbError> {
    let _stmt = include_str!("./sql/folder/insert.sql");
    let _stmt = client.prepare(_stmt).await?;

    client
        .query(&_stmt, &[&insert_folder_dto.name, &user_id])
        .await?
        .iter()
        .map(|row| row.get(0))
        .collect::<Vec<i64>>()
        .pop()
        .ok_or(DbError::NotFound)
}

pub async fn update(
    client: &Client,
    update_folder_dto: UpdateFolderInputDTO,
    id: i64,
    user_id: i64,
) -> Result<(), DbError> {
    let _stmt = include_str!("./sql/folder/update.sql");
    let _stmt = client.prepare(_stmt).await?;

    client
        .query(&_stmt, &[&update_folder_dto.name, &id, &user_id])
        .await?
        .iter()
        .map(|row| row.get(0))
        .collect::<Vec<i64>>()
        .pop()
        .ok_or(DbError::NotFound)?;

    Ok(())
}

pub async fn delete(client: &Client, id: i64, user_id: i64) -> Result<(), DbError> {
    let _stmt = include_str!("./sql/folder/delete.sql");
    let _stmt = client.prepare(_stmt).await?;

    client
        .query(&_stmt, &[&id, &user_id])
        .await?
        .iter()
        .map(|row| row.get(0))
        .collect::<Vec<i64>>()
        .pop()
        .ok_or(DbError::NotFound)?;

    Ok(())
}
