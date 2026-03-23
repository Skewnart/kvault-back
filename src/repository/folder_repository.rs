use crate::errors::db_error::DbError;
use crate::models::envelope::EncodedDataDTO;
use deadpool_postgres::Client;

pub async fn get(client: &Client, folder_id: i64, user_id: i64) -> Result<EncodedDataDTO, DbError> {
    let _stmt = include_str!("sql/folder/get.sql");
    let _stmt = client.prepare(_stmt).await?;

    let folders = client
        .query(&_stmt, &[&folder_id, &user_id])
        .await?
        .iter()
        .map(|row| EncodedDataDTO {
            enc_data: row.get(0),
        })
        .collect::<Vec<EncodedDataDTO>>()
        .pop()
        .ok_or(DbError::NotFound)?;

    Ok(folders)
}

pub async fn update(
    client: &Client,
    enc_string_dto: EncodedDataDTO,
    id: i64,
    user_id: i64,
) -> Result<(), DbError> {
    let _stmt = include_str!("sql/folder/update.sql");
    let _stmt = client.prepare(_stmt).await?;

    client
        .query(&_stmt, &[&enc_string_dto.enc_data, &id, &user_id])
        .await?
        .iter()
        .map(|row| row.get(0))
        .collect::<Vec<i64>>()
        .pop()
        .ok_or(DbError::NotFound)?;

    Ok(())
}

pub async fn insert(
    client: &Client,
    user_id: i64,
    enc_entries_json: &EncodedDataDTO,
) -> Result<i64, DbError> {
    let _stmt = include_str!("./sql/folder/insert.sql");
    let _stmt = client.prepare(_stmt).await?;

    client
        .query(&_stmt, &[&user_id, &enc_entries_json.enc_data])
        .await?
        .iter()
        .map(|row| row.get(0))
        .collect::<Vec<i64>>()
        .pop()
        .ok_or(DbError::NotFound)
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
