//! Implementação do modo de operação CTR.

use super::*;

/// Cifra uma mensagem com os algoritmos AES-128 e CTR.
pub fn encrypt(plaintext: &Vec<u8>, key: u128, offset: u128, rounds: usize) -> Vec<u8> {
    let mut ciphertext: Vec<u8> = vec![];
    let round_keys = key_schedule::key_expansion(key, rounds);

    for (i, chunk) in plaintext.chunks(16).enumerate() {
        let mut bytes = chunk.to_vec();
        let counter: [u8; 16] = (offset + i as u128).to_be_bytes();

        for j in 0..bytes.len() {
            bytes[j] ^= super::encrypt(&counter, &round_keys, rounds)[j];
        }
        ciphertext.append(&mut bytes);
    }
    ciphertext
}

/// Decifra uma mensagem com os algoritmos AES-128 e CTR.
pub fn decrypt(ciphertext: &Vec<u8>, key: u128, offset: u128, rounds: usize) -> Vec<u8> {
    encrypt(ciphertext, key, offset, rounds)
}
