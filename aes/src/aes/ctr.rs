//! Módulo responsável por implementar o modo de operação CTR.

use super::{Block, Key};

pub fn cipher(plaintext: &Vec<u8>, key: &Key, offset: u128, rounds: usize) -> Vec<u8> {
    let mut ciphered_blocks: Vec<Block> = vec![];
    let mut temp = plaintext.clone();

    if (plaintext.len() % 16 != 0) {
        temp.append(&mut vec![0; 16 - (plaintext.len() % 16)]);
    }
    let blocks: Vec<Block> = temp
        .chunks(16)
        .map(|chunk| { chunk.try_into().unwrap() })
        .collect();

    let round_keys = super::key_expansion(&key, rounds);

    for (i, block) in blocks.iter().enumerate() {
        let mut ciphered_block = block.clone();
        let counter = super::cipher(&(offset + i as u128).to_ne_bytes(), &round_keys, rounds);

        for j in 0..16 {
            ciphered_block[j] ^= counter[j];
        }
        ciphered_blocks.push(ciphered_block);
    }
    ciphered_blocks.join(&0)
}

pub fn decipher(ciphertext: &Vec<u8>, key: &Key, offset: u128, rounds: usize) -> Vec<u8> {
    cipher(ciphertext, key, offset, rounds)
}
