//! Módulo responsável pela implementação do algoritmo
//! [RSA](https://en.wikipedia.org/wiki/RSA_(cryptosystem)).

use num_bigint::BigUint;

pub mod key_generation;

#[allow(unused)]
fn cipher(plaintext: &Vec<u8>, n: &BigUint, e: &BigUint) -> Vec<u8> {
    todo!()
}

#[allow(unused)]
fn decipher(ciphertext: &Vec<u8>, n: &BigUint, d: &BigUint) -> Vec<u8> {
    todo!()
}

#[allow(unused)]
fn oaep(message: &Vec<u8>, nonce: &BigUint) -> [u8; todo!()] {
    todo!()
}

#[allow(unused)]
pub fn sign(message: &Vec<u8>, n: &BigUint, d: &BigUint) -> [u8; todo!()] {
    todo!()
}

#[allow(unused)]
pub fn verify(message: &Vec<u8>, signature: &[u8; todo!()], n: &BigUint, e: &BigUint) -> bool {
    todo!()
}
