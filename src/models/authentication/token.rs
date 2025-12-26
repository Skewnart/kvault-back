use std::error::Error;
use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};

static THIRTY_MINUTES: i64 = 60 * 30; // in seconds

#[derive(Serialize, Deserialize)]
pub struct Token {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user_id: i64
}

impl Token {
    pub fn generate(login: i64) -> Self {

        let now = Utc::now().timestamp_nanos_opt().unwrap_or(0i64);
        
        Self {
            iat: now,
            exp: now + THIRTY_MINUTES,
            user_id: login.clone()
        }
    }
    pub fn encode(&self, encoding_key: &EncodingKey) -> Result<String, Box<dyn Error>> {
        Ok(jsonwebtoken::encode(
            &Header::default(),
            &self,
            encoding_key
        )?)
    }
}
