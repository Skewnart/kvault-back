use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EnvelopeDTO {
    pub envelope: serde_json::Value,
}
