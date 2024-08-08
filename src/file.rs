//! Módulo responsável por gerenciar a leitura e a escrita em arquivos.

use std::fs;
use std::fs::{File, Metadata};
use std::io::Read;
use std::path::Path;

/// Obtém o nome do arquivo dos argumentos da linha de comando.
pub fn get_filename(args: &Vec<String>) -> String {
    if args.len() == 1 {
        panic!("Nome do arquivo não foi especificado.")
    }
    args[1].clone()
}

/// Lê o conteúdo do arquivo em bytes.
pub fn read_file(filename: &String) -> (Metadata, Vec<u8>) {
    let path = Path::new(filename);
    let metadata = fs::metadata(path).expect("Não foi possível obter os metadados do arquivo");
    let mut file = File::open(path).expect("Não foi possível abrir o arquivo.");
    let mut content: Vec<u8> = vec![0; metadata.len() as usize];

    file.read(&mut content).expect("Não foi possível ler o arquivo.");
    (metadata, content)
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use std::str::from_utf8;

    use super::*;

    #[test]
    fn test_get_filename() {
        let args: Vec<String> = vec!["target\\debug\\trabalho_sc.exe", "tests\\hello_world.txt"]
            .iter()
            .map(<&str>::to_string)
            .collect();

        let filename = get_filename(&args);

        assert_eq!("tests\\hello_world.txt", filename.as_str());
    }

    #[test]
    #[should_panic]
    fn test_get_filename_for_invalid_input() {
        let args: Vec<String> = vec!["target\\debug\\trabalho_sc.exe"]
            .iter()
            .map(<&str>::to_string)
            .collect();

        let filename = get_filename(&args);
    }

    #[test]
    fn test_read_file() {
        let filename = String::from("tests\\hello_world.txt");
        let (metadata, content) = read_file(&filename);

        assert_eq!(12, metadata.len());
        assert_eq!(12, content.len());

        let text = String::from_utf8(content).unwrap();

        assert_eq!("Hello World!", text);
    }

    #[test]
    #[should_panic]
    fn test_read_file_for_invalid_input() {
        let filename = String::from("tests\\goodbye_world.txt");
        let (metadata, content) = read_file(&filename);
    }
}

