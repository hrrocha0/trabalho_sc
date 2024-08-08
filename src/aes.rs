//! Módulo responsável pela implementação do algoritmo [AES](https://pt.wikipedia.org/wiki/Advanced_Encryption_Standard).

pub(crate) const BLOCK_SIZE: usize = 16;   // Tamanho do bloco em bytes
pub(crate) const KEY_LENGTH: usize = 16;   // Comprimento da chave em bytes

/// Cifra um bloco de 128 bits com o algoritmo AES.
#[allow(unused)]
pub fn encrypt(block: &[u8; BLOCK_SIZE], key: &[u8; KEY_LENGTH], rounds: usize) -> [u8; BLOCK_SIZE] {
    todo!()
}

/// Decifra um bloco de 128 bits com o algoritmo AES.
#[allow(unused)]
pub fn decrypt(block: &[u8; BLOCK_SIZE], key: &[u8; KEY_LENGTH], rounds: usize) -> [u8; BLOCK_SIZE] {
    todo!()
}

/// Separa a entrada em blocos de 128 bits.
#[allow(unused)]
pub fn separate_blocks(content: &Vec<u8>) -> Vec<[u8; BLOCK_SIZE]> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_separate_blocks() {
        let content: Vec<u8> = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
        ];
        let expected: Vec<[u8; BLOCK_SIZE]> = vec![
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            [16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31],
        ];
        assert_eq!(expected, separate_blocks(&content));
    }
}
