use crate::errors::db_error::DbError;
use crate::models::invitation::{InvitationDTO, InvitationInputDTO};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use uuid::Uuid;

pub async fn get_all(client: &Client) -> Result<Vec<InvitationDTO>, DbError> {
    let _stmt = include_str!("sql/invitations/get_all.sql");
    let _stmt = client.prepare(_stmt).await?;

    let invitations = client
        .query(&_stmt, &[])
        .await?
        .iter()
        .map(|row| InvitationDTO::from_row_ref(row).unwrap())
        .collect::<Vec<InvitationDTO>>();

    Ok(invitations)
}

pub async fn insert(
    client: &Client,
    insert_folder_dto: &InvitationInputDTO,
) -> Result<Uuid, DbError> {
    let _stmt = include_str!("./sql/invitations/insert.sql");
    let _stmt = client.prepare(_stmt).await?;

    client
        .query(
            &_stmt,
            &[
                &insert_folder_dto.duration_times.to_string(),
                &insert_folder_dto.duration_unit.to_string(),
            ],
        )
        .await?
        .iter()
        .map(|row| row.get(0))
        .collect::<Vec<Uuid>>()
        .pop()
        .ok_or(DbError::NotFound)
}

pub async fn update_user_id(client: &Client, user_id: i64, guid: Uuid) -> Result<(), DbError> {
    let _stmt = include_str!("./sql/invitations/update_user_id.sql");
    let _stmt = client.prepare(_stmt).await?;

    client
        .query(&_stmt, &[&user_id, &guid])
        .await?
        .iter()
        .map(|row| row.get(0))
        .collect::<Vec<Uuid>>()
        .pop()
        .ok_or(DbError::NotFound)?;

    Ok(())
}

pub async fn check_guid(client: &Client, guid: Uuid) -> Result<(), DbError> {
    let _stmt = include_str!("./sql/invitations/check_guid.sql");
    let _stmt = client.prepare(_stmt).await?;

    client
        .query(&_stmt, &[&guid])
        .await?
        .iter()
        .map(|row| row.get(0))
        .collect::<Vec<Uuid>>()
        .pop()
        .ok_or(DbError::NotFound)?;

    Ok(())
}
