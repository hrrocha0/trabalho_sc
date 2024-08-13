//! Módulo responsável pela implementação do algoritmo [AES](https://pt.wikipedia.org/wiki/Advanced_Encryption_Standard).

use crate::constants::*;

type Block = [u8; BLOCK_SIZE];
type Key = [u8; KEY_LENGTH];

pub fn encrypt(block: &Block, key: &Key, rounds: usize) -> Block {
    let round_keys = key_expansion(&key, rounds);
    let mut state = block.clone();

    add_round_key(&mut state, &round_keys[0]);

    for i in 1..rounds {
        sub_bytes(&mut state);
        shift_rows(&mut state);
        mix_columns(&mut state);
        add_round_key(&mut state, &round_keys[i]);
    }
    sub_bytes(&mut state);
    shift_rows(&mut state);
    add_round_key(&mut state, &round_keys[rounds]);

    state
}

#[allow(unused)]
pub fn decrypt(block: &Block, round_keys: &Vec<Key>, rounds: usize) -> Block {
    todo!()
}

fn sub_bytes(state: &mut Block) {
    for i in 0..BLOCK_SIZE {
        state[i] = S_BOX[state[i] as usize];
    }
}

fn shift_rows(state: &mut Block) {
    let mut temp = state.clone();

    for i in 0..BLOCK_SIZE {
        temp[i] = state[(5 * i) % BLOCK_SIZE];
    }
    *state = temp;
}

fn mix_columns(state: &mut Block) {
    let mut temp = state.clone();

    for i in 0..(BLOCK_SIZE / 4) {
        temp[4 * i] = MUL_2[state[4 * i] as usize] ^ MUL_3[state[4 * i + 1] as usize] ^ state[4 * i + 2] ^ state[4 * i + 3];
        temp[4 * i + 1] = state[4 * i] ^ MUL_2[state[4 * i + 1] as usize] ^ MUL_3[state[4 * i + 2] as usize] ^ state[4 * i + 3];
        temp[4 * i + 2] = state[4 * i] ^ state[4 * i + 1] ^ MUL_2[state[4 * i + 2] as usize] ^ MUL_3[state[4 * i + 3] as usize];
        temp[4 * i + 3] = MUL_3[state[4 * i] as usize] ^ state[4 * i + 1] ^ state[4 * i + 2] ^ MUL_2[state[4 * i + 3] as usize];
    }
    *state = temp;
}

fn add_round_key(state: &mut Block, round_key: &Key) {
    for i in 0..BLOCK_SIZE {
        state[i] ^= round_key[i];
    }
}

#[allow(unused)]
fn key_expansion(key: &Key, rounds: usize) -> Vec<Key> {
    todo!()
}
