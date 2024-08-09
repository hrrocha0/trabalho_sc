//! Módulo responsável pela implementação do algoritmo [AES](https://pt.wikipedia.org/wiki/Advanced_Encryption_Standard).

pub(crate) const BLOCK_SIZE: usize = 4;   // Tamanho do bloco em words
pub(crate) const KEY_LENGTH: usize = 4;   // Comprimento da chave em words

/// Cifra um bloco de 128 bits com o algoritmo AES.
pub fn encrypt(input: &[u8; 4 * BLOCK_SIZE], output: &mut [u8; 4 * BLOCK_SIZE], key: &[u8; 4 * KEY_LENGTH], rounds: usize) {
    let mut round_keys: Vec<u32> = vec![0; BLOCK_SIZE * (rounds + 1)];
    let mut state: [u8; 4 * BLOCK_SIZE] = input.clone();

    key_expansion(key, &mut round_keys, rounds);
    add_round_key(&mut state, *round_keys[0..BLOCK_SIZE]);

    for r in 1..rounds {
        sub_bytes(&mut state);
        shift_rows(&mut state);
        mix_columns(&mut state);
        add_round_key(&mut state, *round_keys[(r * BLOCK_SIZE)..((r + 1) * BLOCK_SIZE)])
    }
    sub_bytes(&mut state);
    shift_rows(&mut state);
    add_round_key(&mut state, *round_keys[(rounds * BLOCK_SIZE)..((rounds + 1) * BLOCK_SIZE)]);

    *output = state.clone();
}

/// Decifra um bloco de 128 bits com o algoritmo AES.
#[allow(unused)]
pub fn decrypt(input: &[u8; 4 * BLOCK_SIZE], output: &mut [u8; 4 * BLOCK_SIZE], key: &[u8; 4 * KEY_LENGTH], rounds: usize) {
    todo!()
}

/// Expande a chave em um vetor de chaves de rodada.
#[allow(unused)]
fn key_expansion(key: &[u8; 4 * KEY_LENGTH], round_keys: &mut Vec<u32>, rounds: usize) {
    let mut temp: u32 = 0;

    for i in 0..KEY_LENGTH {
        round_keys[i] = u32::from_ne_bytes(*key[(4 * i)..(4 * i + 4)]);
    }
    for i in KEY_LENGTH..(BLOCK_SIZE * (rounds + 1)) {
        temp = round_keys[i - 1];

        if temp % KEY_LENGTH as u32 == 0 {
            todo!();
        }
        round_keys[i] = round_keys[i - KEY_LENGTH] ^ temp;
    }
    todo!()
}

/// Implementa a etapa AddRoundKey do algoritmo AES.
#[allow(unused)]
fn add_round_key(state: &mut [u8; 4 * BLOCK_SIZE], round_key: &[u32; BLOCK_SIZE]) {
    todo!()
}

/// Implementa a etapa SubBytes do algoritmo AES.
#[allow(unused)]
fn sub_bytes(state: &mut [u8; 4 * BLOCK_SIZE]) {
    todo!()
}

/// Implementa a etapa ShiftRows do algoritmo AES.
#[allow(unused)]
fn shift_rows(state: &mut [u8; 4 * BLOCK_SIZE]) {
    todo!()
}

/// Implementa a etapa MixColumns do algoritmo AES.
#[allow(unused)]
fn mix_columns(state: &mut [u8; 4 * BLOCK_SIZE]) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_encrypt() {
        let input: [u8; 4 * BLOCK_SIZE] = [
            0x0, 0x1, 0x2, 0x3,
            0x4, 0x5, 0x6, 0x7,
            0x8, 0x9, 0xa, 0xb,
            0xc, 0xd, 0xe, 0xf,
        ];
        let key: [u8; 4 * KEY_LENGTH] = [
            0xf, 0xe, 0xd, 0xc,
            0xb, 0xa, 0x9, 0x8,
            0x7, 0x6, 0x5, 0x4,
            0x3, 0x2, 0x1, 0x0,
        ];
        let mut output: [u8; 4 * BLOCK_SIZE] = [0; 4 * BLOCK_SIZE];

        encrypt(&input, &mut output, &key, 10);
        todo!()
    }

    #[test]
    #[ignore]
    fn test_decrypt() {
        let input: [u8; 4 * BLOCK_SIZE] = [
            0x0, 0x1, 0x2, 0x3,
            0x4, 0x5, 0x6, 0x7,
            0x8, 0x9, 0xa, 0xb,
            0xc, 0xd, 0xe, 0xf,
        ];
        let key: [u8; 4 * KEY_LENGTH] = [
            0xf, 0xe, 0xd, 0xc,
            0xb, 0xa, 0x9, 0x8,
            0x7, 0x6, 0x5, 0x4,
            0x3, 0x2, 0x1, 0x0,
        ];
        let mut output: [u8; 4 * BLOCK_SIZE] = [0; 4 * BLOCK_SIZE];

        decrypt(&input, &mut output, &key, 10);
        todo!()
    }

    #[test]
    fn test_expand_key() {}
}
