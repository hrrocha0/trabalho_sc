use num_bigint::BigUint;
use sha3::{Digest, Sha3_256};

/// Gera o hash SHA-3 de uma mensagem.
pub fn hash(message: &BigUint) -> BigUint {
    let mut hasher = Sha3_256::new();

    hasher.update(message.to_bytes_le());
    BigUint::from_bytes_le(hasher.finalize().as_slice())
}
