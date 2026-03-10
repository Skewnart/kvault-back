use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "users")]
pub struct UserProfileDTO {
    pub username: String,
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
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UserType {
    USER,
    ADMIN,
}

impl UserType {
    pub fn from(user: String) -> Option<Self> {
        match user.as_str() {
            "USER" => Some(UserType::USER),
            "ADMIN" => Some(UserType::ADMIN),
            _ => None,
        }
    }

    pub fn is_admin(&self) -> bool {
        self == &UserType::ADMIN
    }
}
