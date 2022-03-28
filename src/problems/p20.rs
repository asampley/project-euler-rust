//! # Factorial digit sum
//!
//! n! means n × (n − 1) × ... × 3 × 2 × 1
//!
//! For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
//! and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
//!
//! Find the sum of the digits in the number 100!

use crate::numbers::factorial;

use num::BigUint;

pub fn run() {
    println!(
        "{}",
        factorial::<u64, BigUint>(100)
            .to_radix_be(10)
            .into_iter()
            .map(|i| i as u32)
            .sum::<u32>()
    );
}
