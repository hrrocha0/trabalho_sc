//! Módulo responsável por gerenciar a leitura e a escrita em arquivos.

use std::fs::{File, Metadata};
use std::io::Read;
use std::path::Path;

pub struct FileData {
    pub metadata: Metadata,
    pub content: Vec<u8>,
}

pub fn get_filename(args: &Vec<String>) -> String {
    if args.len() <= 1 {
        panic!("O nome do arquivo deve ser especificado.");
    }
    args[1].clone()
}

pub fn read_file(filename: &String) -> FileData {
    let path = Path::new(&filename);
    let mut file = File::open(&path).expect("Não foi possível abrir o arquivo.");

    let metadata = file.metadata().expect("Não foi possível ler os metatados do arquivo.");
    let mut content = vec![0; metadata.len() as usize];

    file.read(&mut content).expect("Não foi possível ler o arquivo.");

    FileData { metadata, content }
}
