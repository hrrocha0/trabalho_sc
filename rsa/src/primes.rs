// Geração de pseudoprimos.

use num_bigint::{BigUint, RandBigInt};

/// Gera um número pseudoprimo aleatório de 1024 bits.
pub(crate) fn random_pseudoprime() -> BigUint {
    let mut rng = rand::thread_rng();

    loop {
        let n = rng.gen_biguint(1024);

        if miller_rabin(&n) {
            return n;
        }
    }
}

/// Verifica se um número é pseudoprimo com o teste de Miller-Rabin.
fn miller_rabin(n: &BigUint) -> bool {
    let zero = BigUint::from(0u8);
    let one = BigUint::from(1u8);
    let two = BigUint::from(2u8);

    let n_minus_one = n - &one;

    let mut s = zero.clone();
    let mut d = n_minus_one.clone();

    while &d / &two == zero.clone() {
        d /= &two;
        s += &one;
    }
    let mut rng = rand::thread_rng();

    'outer: for _ in 0..40 {
        let a = rng.gen_biguint_range(&two, n);
        let mut x = a.modpow(&d, n);

        if &x == &one || &x == &n_minus_one {
            continue;
        }
        let mut r = zero.clone();

        while &r < &s {
            x = x.modpow(&two, n);

            if &x == &n_minus_one {
                continue 'outer;
            }
            r += &one;
        }
        return false;
    }
    true
}
