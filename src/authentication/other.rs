// use std::fmt;
//
// #[derive(Debug)]
// pub struct AuthorizationError {
//     pub message: String,
//     pub status_code: u16,
// }
//
// impl AuthorizationError {
//     pub fn new(message: impl Into<String>) -> Self {
//         Self {
//             message: message.into(),
//             status_code: 403,
//         }
//     }
//
//     pub fn unauthorized(message: impl Into<String>) -> Self {
//         AuthorizationError::with_status(message, 401)
//     }
//
//     pub fn with_status(message: impl Into<String>, status_code: u16) -> Self {
//         Self {
//             message: message.into(),
//             status_code,
//         }
//     }
// }
//
// impl fmt::Display for AuthorizationError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.message)
//     }
// }
//
// impl std::error::Error for AuthorizationError {}
//
// pub fn extract_bearer_token(authorization: Option<&str>) -> Result<&str, AuthorizationError> {
//     let auth_header = authorization.ok_or_else(|| {
//         AuthorizationError::with_status("Le header Authorization est manquant", 401)
//     })?;
//
//     if !auth_header.starts_with("Bearer ") {
//         return Err(AuthorizationError::unauthorized(
//             "Le header Authorization doit commencer par \"Bearer \""
//         ));
//     }
//
//     Ok(&auth_header[7..]) // Supprime le préfixe 'Bearer '
// }