//! # Reciprocal cycles
//!
//! A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions
//! with denominators 2 to 10 are given:
//!
//! | fraction | decimal |
//! |----------|---------|
//! | 1/2 | 0.5 |
//! | 1/3 | 0.(3) |
//! | 1/4 | 0.25 |
//! | 1/5 | 0.2 |
//! | 1/6 | 0.1(6) |
//! | 1/7 | 0.(142857) |
//! | 1/8 | 0.125 |
//! | 1/9 | 0.(1) |
//! | 1/10 | 0.1 |
//!
//! Where 0.1(6) means 0.166666..., and has a 1-digit recurring
//! cycle. It can be seen that 1/7 has a 6-digit recurring
//! cycle.
//!
//! Find the value of d < 1000 for which 1/d contains the
//! longest recurring cycle in its decimal fraction part.
//!
//!
//! ## Implementation notes
//!
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
