//! # Segurança Computacional 2024.1 - Trabalho de Implementação
//! Henrique Rodrigues Rocha - 211036061
//!
//! ## Parte 1 - Cifra de bloco e modo de operação CTR

use std::env;
use aes::{Block, Key};
use common::file;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = file::read_file(args.get(1).expect("O arquivo de entrada deve ser especificado.")).content;

    let key: Key = args.get(3)
        .expect("A chave deve ser especificada.")
        .as_bytes()
        .try_into()
        .expect("A chave deve ser uma cadeia de 128 bits.");

    let initial_vector: Block = args.get(4)
        .expect("O vetor inicial deve ser especificado.")
        .as_bytes()
        .try_into()
        .expect("O bloco deve ser uma cadeia de 128 bits");

    let rounds: usize = args.get(5)
        .unwrap_or(&String::new())
        .parse()
        .unwrap_or(10);

    let output = aes::ctr::cipher(&input, &key, &initial_vector, rounds);

    file::write_file(args.get(2).expect("O arquivo de saída deve ser especificado."), &output);
}
