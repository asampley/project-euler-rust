//! Each repeating decimal can be rewritten as as something over 10^n - 1 (several 9s).
//! The exception seems to be if the denominator shares any factors with 10.
//!
//! It would take more mathematics to prove this is correct, but it looks like if all
//! 2s and 5s are removed from the denominator, the lenght of the repetition can be
//! determined by iterating through all powers of 10 minus 1.

use num::{BigUint, One, Zero};

pub fn run() {
    println!(
        "{}",
        (1..1000)
            .map(|x| (cycle_length(x), x))
            .max()
            .map(|(_, x)| x)
            .unwrap()
    );
}

fn cycle_length(mut n: u64) -> u32 {
    while n % 2 == 0 {
        n /= 2;
    }

    while n % 5 == 0 {
        n /= 5;
    }

    (1..)
        .map(|i| (i, BigUint::from(10_u64).pow(i) - BigUint::one()))
        .find(|(_, p)| p % n == BigUint::zero())
        .map(|(i, _)| i)
        .unwrap()
}
