//! Módulo responsável por implementar o modo de operação
//! [CTR](https://en.wikipedia.org/wiki/Block_cipher_mode_of_operation#Counter_(CTR)).

use super::*;

/// Cifra ou decifra utilizando o algoritmo AES-128 e o modo de operação CTR.
pub fn cipher(input: &Vec<u8>, key: u128, offset: u128, rounds: usize) -> Vec<u8> {
    let mut output: Vec<u8> = vec![];

    let round_keys = key_scheduling::key_expansion(key, rounds);

    for (i, chunk) in input.chunks(16).enumerate() {
        let mut bytes = chunk.to_vec();
        let counter: [u8; 16] = (offset + i as u128).to_be_bytes();

        for j in 0..bytes.len() {
            bytes[j] ^= super::cipher(&counter, &round_keys, rounds)[j];
        }
        output.append(&mut bytes);
    }
    output
}
