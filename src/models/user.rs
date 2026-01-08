use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "users")] // singular 'user' is a keyword..
pub struct UserProfileDto {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterDTO {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}
