use crate::authentication::other::{AuthInfo, AuthorizationError, ISSUER };
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde_json::Value;
use std::collections::HashMap;

pub struct JwtValidator {
    jwks: HashMap<String, DecodingKey>,
}

impl JwtValidator {
    pub fn new() -> Result<Self, AuthorizationError> {
        let jwks = Self::fetch_jwks()?;
        Ok(Self { jwks })
    }

    fn fetch_jwks() -> Result<HashMap<String, DecodingKey>, AuthorizationError> {
        // let response = reqwest::get(JWKS_URI).await.map_err(|e| {
        //     AuthorizationError::with_status(format!("Failed to fetch JWKS: {}", e), 401)
        // })?;
        //
        // let jwks: Value = response.json().await.map_err(|e| {
        //     AuthorizationError::with_status(format!("Failed to parse JWKS: {}", e), 401)
        // })?;

        let keys = HashMap::new();

        // if let Some(keys_array) = jwks["keys"].as_array() {
        //     for key in keys_array {
        //         if let (Some(kid), Some(kty), Some(n), Some(e)) = (
        //             key["kid"].as_str(),
        //             key["kty"].as_str(),
        //             key["n"].as_str(),
        //             key["e"].as_str(),
        //         ) {
        //             if kty == "RSA" {
        //                 if let Ok(decoding_key) = DecodingKey::from_rsa_components(n, e) {
        //                     keys.insert(kid.to_string(), decoding_key);
        //                 }
        //             }
        //         }
        //     }
        // }
        //
        // if keys.is_empty() {
        //     return Err(AuthorizationError::with_status("No valid keys found in JWKS", 401));
        // }

        Ok(keys)
    }

    pub fn validate_jwt(&self, token: &str) -> Result<AuthInfo, AuthorizationError> {
        let header = decode_header(token).map_err(|e| {
            AuthorizationError::with_status(format!("Invalid token header: {}", e), 401)
        })?;

        let kid = header.kid.ok_or_else(|| {
            AuthorizationError::with_status("Token missing kid claim", 401)
        })?;

        let key = self.jwks.get(&kid).ok_or_else(|| {
            AuthorizationError::with_status("Unknown key ID", 401)
        })?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_issuer(&[ISSUER]);
        validation.validate_aud = false; // We'll verify audience manually

        let token_data = decode::<Value>(token, key, &validation).map_err(|e| {
            AuthorizationError::with_status(format!("Invalid token: {}", e), 401)
        })?;

        let claims = token_data.claims;
        self.verify_payload(&claims)?;

        Ok(self.create_auth_info(claims))
    }

    fn verify_payload(&self, claims: &Value) -> Result<(), AuthorizationError> {
        // Vérifiez que la revendication d'audience correspond à votre indicateur de ressource API
        let audiences = match &claims["aud"] {
            Value::Array(arr) => arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>(),
            Value::String(s) => vec![s.as_str()],
            _ => vec![],
        };

        if !audiences.contains(&"https://your-api-resource-indicator") {
            return Err(AuthorizationError::new("Invalid audience"));
        }

        // Vérifiez les portées requises pour les ressources API globales
        let required_scopes = vec!["api:read", "api:write"]; // Remplacez par vos portées requises
        let scopes = claims["scope"]
            .as_str()
            .map(|s| s.split(' ').collect::<Vec<_>>())
            .unwrap_or_default();

        for required_scope in &required_scopes {
            if !scopes.contains(required_scope) {
                return Err(AuthorizationError::new("Insufficient scope"));
            }
        }

        Ok(())
    }

    fn create_auth_info(&self, claims: Value) -> AuthInfo {
        let scopes = claims["scope"]
            .as_str()
            .map(|s| s.split(' ').map(|s| s.to_string()).collect())
            .unwrap_or_default();

        let audience = match &claims["aud"] {
            Value::Array(arr) => arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect(),
            Value::String(s) => vec![s.clone()],
            _ => vec![],
        };

        AuthInfo::new(
            claims["sub"].as_str().unwrap_or_default().to_string(),
            claims["client_id"].as_str().map(|s| s.to_string()),
            claims["organization_id"].as_str().map(|s| s.to_string()),
            scopes,
            audience,
        )
    }
}