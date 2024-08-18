use super::*;
use crate::{Block, Key};

pub fn cipher(input: &Vec<u8>, key: &Key, initial_vector: &Block, rounds: usize) -> Vec<u8> {
    let mut ciphertext: Vec<u8> = vec![];
    let mut ciphered_blocks: Vec<Vec<u8>> = vec![];

    let mut blocks: Vec<Vec<u8>> = vec![];
    let mut temp: Vec<u8> = vec![];
    let mut i = 0;

    while i < input.len() {
        temp.push(input[i]);
        i += 1;

        if i % 16 == 0 {
            blocks.push(temp.clone());
            temp.clear();
        }
    }
    if !temp.is_empty() {
        blocks.push(temp.clone());
    }
    let round_keys = key_scheduling::key_expansion(&key, rounds);

    for (i, block) in blocks.iter().enumerate() {
        let mut ciphered_block = block.clone();
        let offset = u128::from_be_bytes(initial_vector.clone()) + i as u128;
        let counter = super::cipher(&offset.to_be_bytes(), &round_keys, rounds);

        for j in 0..block.len() {
            ciphered_block[j] ^= counter[j];
        }
        ciphered_blocks.push(ciphered_block);
    }
    for mut ciphered_block in ciphered_blocks {
        ciphertext.append(&mut ciphered_block);
    }
    ciphertext
}
