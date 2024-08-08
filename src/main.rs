//! # Segurança Computacional 2024.1 - Trabalho de Implementação
//! Henrique Rodrigues Rocha - 211036061

use std::env;

mod file;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = file::get_filename(&args);
    let (metadata, content) = file::read_file(&filename);

    println!("File size (in bytes): {}", metadata.len());
    println!("File content: {}", String::from_utf8(content).unwrap());
}
