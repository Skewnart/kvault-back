use crate::errors::db_error::DbError;
use crate::models::invitation::InvitationOutputDTO;
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_all(client: &Client) -> Result<Vec<InvitationOutputDTO>, DbError> {
    let _stmt = include_str!("sql/invitations/get_all.sql");
    let _stmt = client.prepare(_stmt).await?;

    let invitations = client
        .query(&_stmt, &[])
        .await?
        .iter()
        .map(|row| InvitationOutputDTO::from_row_ref(row).unwrap())
        .collect::<Vec<InvitationOutputDTO>>();

    Ok(invitations)
}
