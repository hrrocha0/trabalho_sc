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
//! Utilize o seguinte comando para mais informações:
//!
//! ```shell
//! aes.exe --help
//! ```

use clap::{Parser, Subcommand};
use common::file;

#[derive(Parser)]
#[command(about = "Criptografia simétrica com AES-128 e CTR")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Gera uma chave aleatória de 128 bits
    #[command(visible_alias = "gen")]
    GenerateKey,

    /// Cifra um arquivo
    #[command(visible_alias = "enc")]
    Encrypt {
        /// Caminho do arquivo de entrada
        #[arg(value_name = "INPUT")]
        input_path: String,

        /// Caminho do arquivo de saída
        #[arg(value_name = "OUTPUT")]
        output_path: String,

        /// Chave de 128 bits
        key: u128,

        /// Vetor inicial (offset) de 128 bits
        offset: u128,

        /// Quantidade de rounds
        #[arg(default_value_t = 10)]
        rounds: usize,
    },

    /// Decifra um arquivo
    #[command(visible_alias = "dec")]
    Decrypt {
        /// Caminho do arquivo de entrada
        #[arg(value_name = "INPUT")]
        input_path: String,

        /// Caminho do arquivo de saída
        #[arg(value_name = "OUTPUT")]
        output_path: String,

        /// Chave de 128 bits
        key: u128,

        /// Vetor inicial (offset) de 128 bits
        offset: u128,

        /// Quantidade de rounds
        #[arg(default_value_t = 10)]
        rounds: usize,
    },
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::GenerateKey => {
            let key: u128 = rand::random();

            println!("Chave gerada com sucesso!");
            println!();
            println!("Chave: {}", key);
        }
        Commands::Encrypt { input_path, output_path, key, offset, rounds } => {
            let key = key.clone();
            let offset = offset.clone();
            let rounds = rounds.clone();

            let input = file::read_file(input_path).unwrap();
            let output = aes::ctr::encrypt(&input, key, offset, rounds);

            file::write_file(output_path, &output).expect("Não foi possível escrever no arquivo de saída");
            println!("Arquivo cifrado com sucesso!");
        }
        Commands::Decrypt { input_path, output_path, key, offset, rounds } => {
            let key = key.clone();
            let offset = offset.clone();
            let rounds = rounds.clone();

            let input = file::read_file(input_path).expect("Não foi possível ler o arquivo de entrada");
            let output = aes::ctr::decrypt(&input, key, offset, rounds);

            file::write_file(output_path, &output).expect("Não foi possível escrever no arquivo de saída");
            println!("Arquivo decifrado com sucesso!");
        }
    }
}
