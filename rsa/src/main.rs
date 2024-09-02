//! # Segurança Computacional 2024.1 - Trabalho de Implementação
//! Henrique Rodrigues Rocha - 211036061
//!
//! ## Parte 2 - Gerador/Verificador de Assinaturas
//!
//! Implementação de um gerador e verificador de assinaturas [RSA](https://en.wikipedia.org/wiki/RSA_(cryptosystem)) em
//! arquivos.
//!
//! ### Uso
//!
//! Utilize o seguinte comando para mais informações:
//!
//! ```shell
//! rsa.exe --help
//! ```

mod primes;

use clap::{Parser, Subcommand};
use common::file;
use num_bigint::{BigUint, RandBigInt};
use rsa::parsing;

#[derive(Parser)]
#[command(about = "Assinaturas digitais com RSA")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Gera um par de chaves RSA
    #[command(visible_alias = "gen")]
    GenerateKeys {
        /// Caminho do arquivo onde será armazenada a chave pública
        #[arg(value_name = "PUBLIC_KEY")]
        public_key_path: String,

        /// Caminho do arquivo onde será armazenada a chave privada
        #[arg(value_name = "PRIVATE_KEY")]
        private_key_path: String,
    },

    /// Assina um arquivo
    Sign {
        /// Caminho do arquivo de entrada
        #[arg(value_name = "INPUT")]
        input_path: String,

        /// Caminho do arquivo de saída
        #[arg(value_name = "OUTPUT")]
        output_path: String,

        /// Caminho do arquivo onde está armazenada a chave pública
        #[arg(value_name = "PUBLIC_KEY")]
        public_key_path: String,

        /// Caminho do arquivo onde está armazenada a chave privada
        #[arg(value_name = "PRIVATE_KEY")]
        private_key_path: String,
    },

    /// Verifica a assinatura de um arquivo
    #[command(visible_alias = "vrfy")]
    Verify {
        /// Caminho do arquivo de entrada
        #[arg(value_name = "INPUT")]
        input_path: String,
    },
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::GenerateKeys { public_key_path, private_key_path } => {
            let mut public_key: Vec<u8> = vec![];
            let mut private_key: Vec<u8> = vec![];

            let p = primes::random_pseudoprime();
            let q = primes::random_pseudoprime();
            let (n, e, d) = rsa::gen_keys(&p, &q);

            public_key.append(&mut n.to_bytes_le());
            public_key.append(&mut e.to_bytes_le());
            public_key.append(&mut vec![0; 512 - public_key.len()]);

            private_key.append(&mut n.to_bytes_le());
            private_key.append(&mut d.to_bytes_le());
            private_key.append(&mut vec![0; 512 - private_key.len()]);


            file::write_file(public_key_path, &public_key).unwrap();
            file::write_file(private_key_path, &private_key).unwrap();

            println!("Chaves geradas com sucesso!");
        }
        Commands::Sign { input_path, output_path, public_key_path, private_key_path } => {
            let mut rng = rand::thread_rng();

            let input = file::read_file(input_path).unwrap();
            let public_key = file::read_file(public_key_path).unwrap();
            let private_key = file::read_file(private_key_path).unwrap();

            let message = BigUint::from_bytes_le(&input);
            let nonce = rng.gen_biguint(256);
            let n = BigUint::from_bytes_le(&private_key[..256]);
            let d = BigUint::from_bytes_le(&private_key[256..]);

            let signature = rsa::sign(&message, &nonce, (&n, &d)).to_bytes_le();
            let mut output_bytes = vec![];

            output_bytes.append(&mut public_key.clone());
            output_bytes.append(&mut signature.clone());
            output_bytes.append(&mut input.clone());

            let output = parsing::to_base64(&output_bytes);

            file::write_file(&output_path, &output.into_bytes()).unwrap();
            println!("Assinatura gerada com sucesso!");
        }
        Commands::Verify { input_path } => {
            let input = String::from_utf8(file::read_file(input_path).unwrap()).unwrap();
            let input_bytes = parsing::from_base64(&input);

            let public_key = &input_bytes[..512];
            let signature = &input_bytes[512..768];
            let plaintext = &input_bytes[768..];

            let message = BigUint::from_bytes_le(&plaintext);
            let signature = BigUint::from_bytes_le(&signature);
            let n = BigUint::from_bytes_le(&public_key[..256]);
            let e = BigUint::from_bytes_le(&public_key[256..]);

            let result = rsa::verify(&message, &signature, (&n, &e));

            println!("Verificação realizada com sucesso!");
            println!();

            if result {
                println!("Resultado: 1");
            } else {
                println!("Resultado: 0");
            }
        }
    }
}
