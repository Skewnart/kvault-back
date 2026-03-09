use serde::{Deserialize, Serialize};
use std::fmt;
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

#[derive(Deserialize)]
pub struct InvitationInputDTO {
    pub duration_times: u8,
    pub duration_unit: DurationUnit,
}

#[derive(Deserialize)]
pub enum DurationUnit {
    Hours,
    Days,
}

impl fmt::Display for DurationUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DurationUnit::Hours => "Hours",
                DurationUnit::Days => "Days",
            }
        )
    }
}
