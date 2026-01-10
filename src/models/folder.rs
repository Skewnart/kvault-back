use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "folder")]
pub struct AllFolderDTO {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "folder")]
pub struct SingleFolderDTO {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct InsertFolderDTO {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateFolderDTO {
    pub name: String,
}
