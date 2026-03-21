use crate::errors::db_error::DbError;
use crate::models::envelope::EnvelopeDTO;
use deadpool_postgres::Client;

pub async fn get_by_user_id(client: &Client, user_id: i64) -> Result<EnvelopeDTO, DbError> {
    let _stmt = include_str!("sql/envelope/get.sql");
    let _stmt = client.prepare(_stmt).await?;

    let envelope = client
        .query(&_stmt, &[&user_id])
        .await?
        .iter()
        .map(|row| row.get(0))
        .collect::<Vec<serde_json::Value>>()
        .pop()
        .ok_or(DbError::NotFound)?;

    let envelope_dto = EnvelopeDTO { envelope };

    Ok(envelope_dto)
}
