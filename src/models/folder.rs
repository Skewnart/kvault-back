use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "folders")]
pub struct FolderOutputDTO {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "folders")]
pub struct FolderDetailOutputDTO {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct InsertFolderInputDTO {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateFolderInputDTO {
    pub name: String,
}
