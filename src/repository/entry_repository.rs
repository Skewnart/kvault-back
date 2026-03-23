use crate::errors::db_error::DbError;
use crate::models::envelope::EncodedDataDTO;
use deadpool_postgres::Client;

pub async fn get(client: &Client, id: i64, user_id: i64) -> Result<EncodedDataDTO, DbError> {
    let _stmt = include_str!("sql/entry/get.sql");
    let _stmt = client.prepare(_stmt).await?;

    let entry = client
        .query(&_stmt, &[&id, &user_id])
        .await?
        .iter()
        .map(|row| EncodedDataDTO {
            enc_data: row.get(0),
        })
        .collect::<Vec<EncodedDataDTO>>()
        .pop()
        .ok_or(DbError::NotFound)?;

    Ok(entry)
}

pub async fn insert(
    client: &Client,
    enc_string_dto: EncodedDataDTO,
    user_id: i64,
) -> Result<i64, DbError> {
    let _stmt = include_str!("./sql/entry/insert.sql");
    let _stmt = client.prepare(_stmt).await?;

    client
        .query(&_stmt, &[&enc_string_dto.enc_data, &user_id])
        .await?
        .iter()
        .map(|row| row.get(0))
        .collect::<Vec<i64>>()
        .pop()
        .ok_or(DbError::NotFound)
}

pub async fn update(
    client: &Client,
    update_entry_dto: EncodedDataDTO,
    id: i64,
    user_id: i64,
) -> Result<(), DbError> {
    let _stmt = include_str!("./sql/entry/update.sql");
    let _stmt = client.prepare(_stmt).await?;

    client
        .query(&_stmt, &[&update_entry_dto.enc_data, &id, &user_id])
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
