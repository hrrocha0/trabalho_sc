use crate::{Key, RoundKey};
use crate::constants::{R_CON, S_BOX};

pub(crate) fn key_expansion(key: &Key, rounds: usize) -> Vec<RoundKey> {
    let mut expanded_key: Vec<u32> = vec![0; 4 * (rounds + 1)];
    let mut temp;

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
