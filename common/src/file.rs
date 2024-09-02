//! Leitura e escrita em arquivos.

use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::Path;

/// Lê um arquivo, se existir.
pub fn read_file(path: &str) -> Result<Vec<u8>, Error> {
    let path = Path::new(path);
    let mut file = File::open(&path)?;

    let metadata = file.metadata()?;
    let mut content = vec![0; metadata.len() as usize];

    file.read(&mut content)?;

    Ok(content)
}

/// Escreve em um arquivo, se existir, e cria o arquivo, caso contrário.
pub fn write_file(path: &str, content: &Vec<u8>) -> Result<(), Error> {
    let path = Path::new(path);
    let mut file = File::create(&path)?;

    file.write(&content)?;

    Ok(())
}
