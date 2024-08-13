//! Módulo responsável por gerenciar a leitura e a escrita em arquivos.

use std::fs::{File, Metadata};
use std::io::{Error, ErrorKind, Read};
use std::path::Path;

pub struct FileData {
    pub metadata: Metadata,
    pub content: Vec<u8>,
}

pub fn get_filename(args: &Vec<String>) -> Result<String, Error> {
    if args.len() <= 1 {
        return Err(Error::new(ErrorKind::InvalidInput, "O nome do arquivo deve ser especificado."));
    }
    Ok(args[1].clone())
}

pub fn read_file(filename: &String) -> Result<FileData, Error> {
    let path = Path::new(&filename);
    let mut file = File::open(&path)?;

    let metadata = file.metadata()?;
    let mut content = vec![0; metadata.len() as usize];

    file.read(&mut content)?;

    Ok(FileData { metadata, content })
}
