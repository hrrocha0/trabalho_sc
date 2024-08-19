//! Módulo responsável pela implementação do algoritmo [AES](https://pt.wikipedia.org/wiki/Advanced_Encryption_Standard).
#![allow(dead_code)]

pub mod ctr;
mod constants;
mod key_scheduling;

use constants::*;

pub type Block = [u8; 16];
pub type Key = [u8; 16];
pub type RoundKey = [u32; 4];

fn cipher(block: &Block, round_keys: &Vec<RoundKey>, rounds: usize) -> Block {
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

fn decipher(block: &Block, round_keys: &Vec<RoundKey>, rounds: usize) -> Block {
    let mut state = block.clone();

    add_round_key(&mut state, &round_keys[rounds]);

    for round in (1..rounds).rev() {
        inv_shift_rows(&mut state);
        inv_sub_bytes(&mut state);
        add_round_key(&mut state, &round_keys[round]);
        inv_mix_columns(&mut state);
    }
    inv_shift_rows(&mut state);
    inv_sub_bytes(&mut state);
    add_round_key(&mut state, &round_keys[0]);

    state
}

fn add_round_key(state: &mut Block, round_key: &RoundKey) {
    let key_bytes: Vec<u8> = round_key.iter().flat_map(|w| { w.to_le_bytes() }).collect();

    for i in 0..16 {
        state[i] ^= key_bytes[i];
    }
}

fn sub_bytes(state: &mut Block) {
    for i in 0..16 {
        state[i] = S_BOX[state[i] as usize];
    }
}

fn shift_rows(state: &mut Block) {
    let mut temp = state.clone();

    for i in 0..16 {
        temp[i] = state[(i + 4 * (i % 4)) % 16];
    }
    *state = temp;
}

fn mix_columns(state: &mut Block) {
    let mut temp = state.clone();

    for i in 0..4 {
        temp[4 * i] = MUL_2[state[4 * i] as usize]
            ^ MUL_3[state[4 * i + 1] as usize]
            ^ state[4 * i + 2]
            ^ state[4 * i + 3];

        temp[4 * i + 1] = state[4 * i]
            ^ MUL_2[state[4 * i + 1] as usize]
            ^ MUL_3[state[4 * i + 2] as usize]
            ^ state[4 * i + 3];

        temp[4 * i + 2] = state[4 * i]
            ^ state[4 * i + 1]
            ^ MUL_2[state[4 * i + 2] as usize]
            ^ MUL_3[state[4 * i + 3] as usize];

        temp[4 * i + 3] = MUL_3[state[4 * i] as usize]
            ^ state[4 * i + 1]
            ^ state[4 * i + 2]
            ^ MUL_2[state[4 * i + 3] as usize];
    }
    *state = temp;
}

fn inv_sub_bytes(state: &mut Block) {
    for i in 0..16 {
        state[i] = INV_S_BOX[state[i] as usize];
    }
}

fn inv_shift_rows(state: &mut Block) {
    let mut temp = state.clone();

    for i in 0..16 {
        temp[i] = state[(i + 16 - 4 * (i % 4)) % 16];
    }
    *state = temp;
}

fn inv_mix_columns(state: &mut Block) {
    let mut temp = state.clone();

    for i in 0..4 {
        temp[4 * i] = MUL_14[state[4 * i] as usize]
            ^ MUL_11[state[4 * i + 1] as usize]
            ^ MUL_13[state[4 * i + 2] as usize]
            ^ MUL_9[state[4 * i + 3] as usize];

        temp[4 * i + 1] = MUL_9[state[4 * i] as usize]
            ^ MUL_14[state[4 * i + 1] as usize]
            ^ MUL_11[state[4 * i + 2] as usize]
            ^ MUL_13[state[4 * i + 3] as usize];

        temp[4 * i + 2] = MUL_13[state[4 * i] as usize]
            ^ MUL_9[state[4 * i + 1] as usize]
            ^ MUL_14[state[4 * i + 2] as usize]
            ^ MUL_11[state[4 * i + 3] as usize];

        temp[4 * i + 3] = MUL_11[state[4 * i] as usize]
            ^ MUL_13[state[4 * i + 1] as usize]
            ^ MUL_9[state[4 * i + 2] as usize]
            ^ MUL_14[state[4 * i + 3] as usize];
    }
    *state = temp;
}