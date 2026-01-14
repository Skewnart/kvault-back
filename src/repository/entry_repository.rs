use crate::errors::db_error::DbError;
use crate::models::entry::{
    EntryDetailOutputDTO, EntryOutputDTO, EntryPasswordOutputDTO, InsertEntryInputDTO,
    MoveEntryInputDTO, UpdateEntryInputDTO,
};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_all_by_folder_id_user_id(
    client: &Client,
    folder_id: i64,
    user_id: i64,
) -> Result<Vec<EntryOutputDTO>, DbError> {
    let _stmt = include_str!("sql/entry/get_all_by_folder_id_user_id.sql");
    let _stmt = client.prepare(_stmt).await?;

    let entries = client
        .query(&_stmt, &[&folder_id, &user_id])
        .await?
        .iter()
        .map(|row| EntryOutputDTO::from_row_ref(row).unwrap())
        .collect::<Vec<EntryOutputDTO>>();

    Ok(entries)
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

pub async fn get_password_by_id_user_id(
    client: &Client,
    id: i64,
    user_id: i64,
) -> Result<EntryPasswordOutputDTO, DbError> {
    let _stmt = include_str!("sql/entry/get_password_by_id_user_id.sql");
    let _stmt = client.prepare(_stmt).await?;

    let entry = client
        .query(&_stmt, &[&id, &user_id])
        .await?
        .iter()
        .map(|row| EntryPasswordOutputDTO::from_row_ref(row).unwrap())
        .collect::<Vec<EntryPasswordOutputDTO>>()
        .pop()
        .ok_or(DbError::NotFound)?;

    Ok(entry)
}

pub async fn insert(
    client: &Client,
    insert_entry_dto: InsertEntryInputDTO,
    user_id: i64,
) -> Result<i64, DbError> {
    let _stmt = include_str!("./sql/entry/insert.sql");
    let _stmt = client.prepare(_stmt).await?;

    client
        .query(
            &_stmt,
            &[
                &insert_entry_dto.name,
                &insert_entry_dto.description,
                &insert_entry_dto.password,
                &insert_entry_dto.is_favorite,
                &insert_entry_dto.folder_id,
                &user_id,
            ],
        )
        .await?
        .iter()
        .map(|row| row.get(0))
        .collect::<Vec<i64>>()
        .pop()
        .ok_or(DbError::NotFound)
}

pub async fn update(
    client: &Client,
    update_entry_dto: UpdateEntryInputDTO,
    id: i64,
    user_id: i64,
) -> Result<(), DbError> {
    let _stmt = include_str!("./sql/entry/update.sql");
    let _stmt = client.prepare(_stmt).await?;

    client
        .query(
            &_stmt,
            &[
                &update_entry_dto.name,
                &update_entry_dto.description,
                &update_entry_dto.password,
                &update_entry_dto.is_favorite,
                &id,
                &user_id,
            ],
        )
        .await?
        .iter()
        .map(|row| row.get(0))
        .collect::<Vec<i64>>()
        .pop()
        .ok_or(DbError::NotFound)?;

    Ok(())
}

pub async fn move_from_folder(
    client: &Client,
    move_entry_dto: MoveEntryInputDTO,
    id: i64,
    user_id: i64,
) -> Result<(), DbError> {
    let _stmt = include_str!("./sql/entry/move_from_folder.sql");
    let _stmt = client.prepare(_stmt).await?;

    client
        .query(&_stmt, &[&move_entry_dto.folder_id, &id, &user_id])
        .await?
        .iter()
        .map(|row| row.get(0))
        .collect::<Vec<i64>>()
        .pop()
        .ok_or(DbError::NotFound)?;

    Ok(())
}

pub async fn delete(client: &Client, id: i64, user_id: i64) -> Result<(), DbError> {
    let _stmt = include_str!("./sql/entry/delete.sql");
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
