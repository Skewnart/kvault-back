use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EnvelopeDTO {
    pub envelope: serde_json::Value,
}

#[derive(Deserialize, Serialize)]
pub struct EncStringDTO {
    pub enc_string: String,
}
