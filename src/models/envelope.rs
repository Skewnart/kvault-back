use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct EncodedDataDTO {
    pub enc_data: serde_json::Value,
}
