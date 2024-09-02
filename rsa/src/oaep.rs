//! Implementação do algoritmo OAEP.

use num_bigint::BigUint;
use crate::hashing;

/// Mascara uma mensagem com o algoritmo OAEP.
pub(crate) fn pad(message: &BigUint, nonce: &BigUint) -> BigUint {
    let mut masked_bytes: Vec<u8> = vec![];

    let x = message ^ hashing::hash(nonce);
    let y = nonce ^ hashing::hash(&x);

    masked_bytes.append(&mut x.to_bytes_le());
    masked_bytes.append(&mut y.to_bytes_le());

    BigUint::from_bytes_le(&masked_bytes)
}

/// Obtém a mensagem original de uma mensagem mascarada com OAEP.
pub(crate) fn unpad(masked: &BigUint) -> BigUint {
    let masked_bytes = masked.to_bytes_le();
    let x = BigUint::from_bytes_le(&masked_bytes[..32]);
    let y = BigUint::from_bytes_le(&masked_bytes[32..]);

    let nonce = y ^ hashing::hash(&x);
    let message = x ^ hashing::hash(&nonce);

    message
}
