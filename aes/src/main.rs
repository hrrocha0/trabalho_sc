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
//! ./aes <ENTRADA> <SAIDA> <CHAVE> <OFFSET> [ROUNDS]
//! ```

use clap::Parser;
use common::file;

/// Define os argumentos da linha de comando.
#[derive(Parser)]
struct Cli {
    /// Caminho do arquivo de entrada
    #[arg(value_name = "ENTRADA")]
    input: String,

    /// Caminho do arquivo de saída
    #[arg(value_name = "SAIDA")]
    output: String,

    /// Chave codificada como um inteiro sem sinal de 128 bits
    #[arg(value_name = "CHAVE")]
    key: u128,

    /// Vetor Inicial (offset) codificado como um inteiro sem sinal de 128 bits
    #[arg()]
    offset: u128,

    /// Número de rounds, 10 por padrão
    #[arg()]
    rounds: Option<usize>,
}

fn main() {
    let args = Cli::parse();

    let input = args.input;
    let output = args.output;
    let key = args.key;
    let offset = args.offset;
    let rounds = args.rounds.unwrap_or(10);

    let input_data = file::read_file(&input).expect("Não foi possível ler o arquivo de entrada");
    let output_data = aes::ctr::cipher(&input_data, key, offset, rounds);

    file::write_file(&output, &output_data).expect("Não foi possível escrever no arquivo de saída");

    println!("{} bytes cifrados/decifrados com sucesso!", input_data.len());
    println!();
    println!("Entrada: {}", &input);
    println!("Saída: {}", &output);
    println!("Chave: {:#032x}", key);
    println!("Offset: {:#032x}", offset);
    println!("Rounds: {}", rounds);
}
