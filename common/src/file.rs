//! Módulo responsável por gerenciar a leitura e a escrita em arquivos.

use std::fs::{File, Metadata};
use std::io::{Read, Write};
use std::path::Path;

pub struct FileData {
    pub metadata: Metadata,
    pub content: Vec<u8>,
}

pub fn read_file(filename: &str) -> FileData {
    let path = Path::new(filename);
    let mut file = File::open(&path).expect("Não foi possível abrir o arquivo.");

    let metadata = file.metadata().expect("Não foi possível ler os metatados do arquivo.");
    let mut content = vec![0; metadata.len() as usize];

    file.read(&mut content).expect("Não foi possível ler o arquivo.");

    FileData { metadata, content }
}

pub fn write_file(filename: &str, content: &Vec<u8>) {
    let path = Path::new(filename);
    let mut file = File::create(&path).expect("Não foi possível criar ou abrir o arquivo.");

    file.write(&content).expect("Não foi possível escrever no arquivo.");
}
