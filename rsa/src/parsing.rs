//! Codificação e decodificação em BASE64.

use base64::Engine;
use base64::engine::general_purpose::STANDARD;

/// Codifica para uma string BASE64.
pub fn to_base64(data: &[u8]) -> String {
    STANDARD.encode(data)
}

/// Decodifica uma string BASE64.
pub fn from_base64(data: &str) -> Vec<u8> {
    STANDARD.decode(data).unwrap()
}
