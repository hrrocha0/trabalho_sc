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

#[allow(unused)]
fn sub_bytes(state: &mut Block) {
    todo!()
}

#[allow(unused)]
fn shift_rows(state: &mut Block) {
    todo!()
}

#[allow(unused)]
fn mix_columns(state: &mut Block) {
    todo!()
}

#[allow(unused)]
fn add_round_key(state: &mut Block, round_key: &Key) {
    todo!()
}

#[allow(unused)]
fn key_expansion(key: &Key, rounds: usize) -> Vec<Key> {
    todo!()
}
