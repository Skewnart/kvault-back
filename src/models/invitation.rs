use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use tokio_pg_mapper_derive::PostgresMapper;
use uuid::Uuid;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "invitations")]
pub struct InvitationOutputDTO {
    pub guid: Uuid,
    pub ends_at: SystemTime,
    pub is_active: bool,
    pub invited_username: Option<String>,
}
