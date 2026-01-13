use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "entry")]
pub struct EntryOutputDTO {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub is_favorite: bool,
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "entry")]
pub struct EntryDetailOutputDTO {
    pub name: String,
    pub description: String,
    pub is_favorite: bool,
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "entry")]
pub struct EntryPasswordOutputDTO {
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct InsertEntryInputDTO {
    pub name: String,
    pub description: String,
    pub password: String,
    pub is_favorite: bool,
    pub folder_id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateEntryInputDTO {
    pub name: String,
    pub description: String,
    pub password: String,
    pub is_favorite: bool,
}

#[derive(Serialize, Deserialize)]
pub struct MoveEntryInputDTO {
    pub folder_id: i64,
}
