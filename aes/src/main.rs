//! # Segurança Computacional 2024.1 - Trabalho de Implementação
//! Henrique Rodrigues Rocha - 211036061
//!
//! ## Parte 1 - Cifra de bloco e modo de operação CTR
//!
//! Implementação do algoritmo de criptografia simétrica
//! [AES-128](https://en.wikipedia.org/wiki/Advanced_Encryption_Standard) com o modo de operação
//! [CTR](https://en.wikipedia.org/wiki/Block_cipher_mode_of_operation#Counter_(CTR)).
//!
//! ### Uso
//!
//! Devem ser fornecidos os seguintes argumentos:
//! * \<input\> - o caminho do arquivo de entrada
//! * \<output\> - o caminho do arquivo de saída
//! * \<key\> - a chave de 128 bits codificada como um inteiro sem sinal
//! * \<iv\> - o vetor inicial (offset) codificado como um inteiro sem sinal
//! * \<rounds\> - a quantidade de rounds, opcional, 10 por padrão
//!
//! Por exemplo:
//!
//! ```shell
//! aes plaintext.txt ciphertext.txt 0123456789abcdef 0000000000000000 12
//! ```

use common::file;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = file::read_file(args.get(1).expect("O arquivo de entrada deve ser especificado.")).content;

    let key: u128 = args.get(3)
        .expect("A chave deve ser especificada.")
        .parse::<u128>()
        .expect("A chave deve ser um inteiro sem sinal de 128 bits.");

    let offset: u128 = args.get(4)
        .expect("O vetor inicial deve ser especificado.")
        .parse()
        .expect("O bloco deve ser um inteiro sem sinal de 128 bits");

    let rounds: usize = args.get(5)
        .unwrap_or(&String::new())
        .parse()
        .unwrap_or(10);

    let output = aes::ctr::cipher(&input, key, offset, rounds);

    file::write_file(args.get(2).expect("O arquivo de saída deve ser especificado."), &output);
}
