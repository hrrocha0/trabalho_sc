//! # Segurança Computacional 2024.1 - Trabalho de Implementação
//! Henrique Rodrigues Rocha - 211036061

use std::env;

mod file;
mod aes;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = file::get_filename(&args);
    let (metadata, content) = file::read_file(&filename);
    // let key: [u8; KEY_LENGTH] = rand::random();
    // let plaintext_blocks: Vec<[u8; BLOCK_SIZE]> = aes::separate_blocks(&content);
    //
    // let ciphertext_blocks: Vec<[u8; BLOCK_SIZE]> = plaintext_blocks
    //     .iter()
    //     .map(|block| { aes::encrypt(block, &key, 10) })
    //     .collect();

    println!("File size: {}", metadata.len());
    println!("File content: {}", String::from_utf8(content.clone()).unwrap());
}
