// use jsonwebtoken::{DecodingKey, EncodingKey};
//
// #[derive(Debug)]
// pub struct JwtKeys {
//     encoding: EncodingKey,
//     decoding: DecodingKey,
// }
//
// impl JwtKeys {
//     pub fn new() -> Self {
//         let secret : [u8; 16] = rand::random();
//
//         Self {
//             encoding: EncodingKey::from_secret(&secret),
//             decoding: DecodingKey::from_secret(&secret),
//         }
//     }
//
//     pub fn get_encoding(&self) -> &EncodingKey {
//         &self.encoding
//     }
// }