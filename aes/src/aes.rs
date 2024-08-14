//! Módulo responsável pela implementação do algoritmo [AES](https://pt.wikipedia.org/wiki/Advanced_Encryption_Standard).

use crate::constants::*;

type Block = [u8; 16];
type Key = [u8; 16];
type RoundKey = [u32; 4];

pub fn cipher(block: &Block, round_keys: &Vec<RoundKey>, rounds: usize) -> Block {
    let mut state = block.clone();

    add_round_key(&mut state, &round_keys[0]);

    for round in 1..rounds {
        sub_bytes(&mut state);
        shift_rows(&mut state);
        mix_columns(&mut state);
        add_round_key(&mut state, &round_keys[round]);
    }
    sub_bytes(&mut state);
    shift_rows(&mut state);
    add_round_key(&mut state, &round_keys[rounds]);

    state
}

fn key_expansion(key: &Key, rounds: usize) -> Vec<RoundKey> {
    let mut expanded_key: Vec<u32> = vec![0; 4 * (rounds + 1)];
    let mut temp = 0;

    for i in 0..4 {
        expanded_key[i] = u32::from_ne_bytes([key[4 * i], key[4 * i + 1], key[4 * i + 2], key[4 * i + 3]]);
    }
    for i in 4..(4 * (rounds + 1)) {
        temp = expanded_key[i - 1];

        if i % 4 == 0 {
            temp = sub_word(rot_word(temp)) ^ R_CON[i / 4] as u32;
        }
        expanded_key[i] = expanded_key[i - 4] ^ temp;
    }
    expanded_key.chunks(4).map(|chunk| { chunk.try_into().unwrap() }).collect()
}

fn rot_word(word: u32) -> u32 {
    let mut bytes: [u8; 4] = word.to_ne_bytes();
    let temp = bytes[0];

    for i in 0..3 {
        bytes[i] = bytes[i + 1];
    }
    bytes[3] = temp;

    u32::from_ne_bytes(bytes)
}

fn sub_word(word: u32) -> u32 {
    let mut bytes: [u8; 4] = word.to_ne_bytes();

    for i in 0..4 {
        bytes[i] = S_BOX[bytes[i] as usize];
    }
    u32::from_ne_bytes(bytes)
}

fn sub_bytes(state: &mut Block) {
    for i in 0..16 {
        state[i] = S_BOX[state[i] as usize];
    }
}

fn shift_rows(state: &mut Block) {
    let mut temp = state.clone();

    for i in 0..16 {
        temp[i] = state[(5 * i) % 16];
    }
    *state = temp;
}

fn mix_columns(state: &mut Block) {
    let mut temp = state.clone();

    for i in 0..4 {
        temp[4 * i] = MUL_2[state[4 * i] as usize] ^ MUL_3[state[4 * i + 1] as usize] ^ state[4 * i + 2] ^ state[4 * i + 3];
        temp[4 * i + 1] = state[4 * i] ^ MUL_2[state[4 * i + 1] as usize] ^ MUL_3[state[4 * i + 2] as usize] ^ state[4 * i + 3];
        temp[4 * i + 2] = state[4 * i] ^ state[4 * i + 1] ^ MUL_2[state[4 * i + 2] as usize] ^ MUL_3[state[4 * i + 3] as usize];
        temp[4 * i + 3] = MUL_3[state[4 * i] as usize] ^ state[4 * i + 1] ^ state[4 * i + 2] ^ MUL_2[state[4 * i + 3] as usize];
    }
    *state = temp;
}

fn add_round_key(state: &mut Block, round_key: &RoundKey) {
    let mut key_bytes: Vec<u8> = round_key.iter().flat_map(|w| { w.to_ne_bytes() }).collect();

    for i in 0..16 {
        state[i] ^= key_bytes[i];
    }
}
