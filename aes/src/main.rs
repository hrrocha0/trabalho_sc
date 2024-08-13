//! # Segurança Computacional 2024.1 - Trabalho de Implementação
//! Henrique Rodrigues Rocha - 211036061
//!
//! ## Parte 1 - Cifra de bloco e modo de operação CTR

mod aes;
mod constants;

use common::file;
use common::file::FileData;
use std::env;
use std::io::Error;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let filename = file::get_filename(&args)?;
    let FileData { metadata, content } = file::read_file(&filename)?;

    println!("File size: {}", metadata.len());
    println!("File content: {:?}", content.clone());

    Ok(())
}
