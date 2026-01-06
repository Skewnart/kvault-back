use std::error::Error;
use actix_web::{FromRequest, HttpMessage, HttpRequest};
use actix_web::dev::Payload;
use base64::Engine;
use base64::engine::general_purpose;
use chrono::Utc;
use futures_util::future::{err, ok};
use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::errors::app_request_error::AppRequestError;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Token {
    pub sub: i64,
    pub token_uuid: String,
    pub exp: i64,
    pub iat: i64,
    pub nbf: i64,
}

impl Token {

    pub fn generate(user_id: i64, ttl: i64) -> Self {
        let now = Utc::now();

        Self {
            sub: user_id,
            token_uuid: Uuid::new_v4().to_string(),
            exp: (now + chrono::Duration::seconds(ttl)).timestamp(),
            iat: now.timestamp(),
            nbf: now.timestamp()
        }
    }

    pub fn encode(&self, secret_key: String) -> Result<String, Box<dyn Error>> {

        let bytes_secret_key = general_purpose::STANDARD.decode(secret_key)?;
        let decoded_secret_key = String::from_utf8(bytes_secret_key)?;
        let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256);

        Ok(jsonwebtoken::encode(
            &header,
            &self,
            &EncodingKey::from_rsa_pem(decoded_secret_key.as_bytes())?,
        )?)
    }

    pub fn extract_bearer(authorization: Option<&str>) -> Result<&str, AppRequestError> {
        let auth_header = authorization.ok_or_else(|| {
            AppRequestError::Unauthorized("Le header Authorization est manquant".to_string())
        })?;

        if !auth_header.starts_with("Bearer ") {
            Err(AppRequestError::Unauthorized(
                "Le header Authorization doit commencer par \"Bearer \"".to_string()
            ))?;
        }

        Ok(&auth_header[7..]) // Supprime le préfixe 'Bearer '
    }

    pub fn decode(token_str: &str, public_key: String) -> Result<Self, Box<dyn Error>> {
        let bytes_public_key = general_purpose::STANDARD.decode(public_key)?;
        let decoded_public_key = String::from_utf8(bytes_public_key)?;

        let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);

        let decoded = jsonwebtoken::decode::<Self>(
            token_str,
            &DecodingKey::from_rsa_pem(decoded_public_key.as_bytes())?,
            &validation,
        )?;

        Ok(decoded.claims)
    }

    pub fn verify(self) -> Result<Self, AppRequestError> {

        let now = Utc::now();

        if self.exp < now.timestamp() {
            Err(AppRequestError::Unauthorized("Le token est expiré.".to_string()))?;
        }
        if self.nbf > now.timestamp() {
            Err(AppRequestError::Unauthorized("Le token n'est pas encore valide.".to_string()))?;
        }

        Ok(self)
    }
}

impl FromRequest for Token {
    type Error = actix_web::Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.extensions().get::<Token>() {
            Some(token) => ok(token.clone()),
            None => err(actix_web::error::ErrorBadRequest("Token should be here"))
        }
    }
}