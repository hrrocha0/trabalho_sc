use super::*;
use crate::{Block, Key};

pub fn cipher(input: &Vec<u8>, key: &Key, initial_vector: &Block, rounds: usize) -> Vec<u8> {
    let mut output: Vec<u8> = vec![];

    let round_keys = key_scheduling::key_expansion(&key, rounds);
    let offset = u128::from_be_bytes(initial_vector.clone());

    for (i, chunk) in input.chunks(16).enumerate() {
        let mut bytes = chunk.to_vec();
        let counter: Block = (offset + i as u128).to_be_bytes();

        for j in 0..bytes.len() {
            bytes[j] ^= super::cipher(&counter, &round_keys, rounds)[j];
        }
        output.append(&mut bytes);
    }
    output
}
