//! Implementação do algoritmo RSA.

use num_bigint::{BigUint, RandBigInt};

mod oaep;
pub mod hashing;
pub mod parsing;

/// Gera os números (n, e, d) que compõem o par de chaves RSA.
pub fn gen_keys(p: &BigUint, q: &BigUint) -> (BigUint, BigUint, BigUint) {
    let one = BigUint::from(1u8);
    let two = BigUint::from(2u8);

    let n = p * q;
    let phi_n = (p - &one) * (q - &one);

    let mut rng = rand::thread_rng();

    loop {
        let e = rng.gen_biguint_range(&two, &phi_n);

        if gcd(&e, &phi_n) == one.clone() {
            let d = e.modinv(&phi_n).unwrap();
            return (n, e, d);
        }
    }
}

fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
    let zero = BigUint::from(0u8);

    if b == &zero {
        return a.clone();
    }
    gcd(b, &(a % b))
}

/// Gera a assinatura digital de uma mensagem.
pub fn sign(message: &BigUint, nonce: &BigUint, private_key: (&BigUint, &BigUint)) -> BigUint {
    let hashed = hashing::hash(message);
    let masked = oaep::pad(&hashed, nonce);

    encrypt(&masked, private_key)
}

/// Verifica a assinatura digital de uma mensagem.
pub fn verify(message: &BigUint, signature: &BigUint, public_key: (&BigUint, &BigUint)) -> bool {
    let masked = decrypt(signature, public_key);
    let hashed = oaep::unpad(&masked);

    hashing::hash(message) == hashed
}

/// Cifra uma mensagem com o algoritmo RSA.
fn encrypt(message: &BigUint, public_key: (&BigUint, &BigUint)) -> BigUint {
    let (n, e) = public_key;
    message.modpow(e, n)
}

/// Decifra uma mensagem com o algoritmo RSA.
fn decrypt(encrypted: &BigUint, private_key: (&BigUint, &BigUint)) -> BigUint {
    let (n, d) = private_key;
    encrypted.modpow(d, n)
}
