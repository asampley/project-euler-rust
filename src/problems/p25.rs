//! # 1000-digit Fibonacci number
//!
//! The Fibonacci sequence is defined by the recurrence relation:
//!
//! > Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
//!
//! Hence the first 12 terms will be:
//!
//! | n  | Fn |
//! |----|----|
//! | 1 | 1 |
//! | 2 | 1 |
//! | 3 | 2 |
//! | 4 | 3 |
//! | 5 | 5 |
//! | 6 | 8 |
//! | 7 | 13 |
//! | 8 | 21 |
//! | 9 | 34 |
//! | 10 | 55 |
//! | 11 | 89 |
//! | 12 | 144 |
//!
//! The 12th term, F12, is the first term to contain three digits.
//!
//! What is the index of the first term in the Fibonacci sequence to contain 1000 digits?

use crate::numbers::fibonacci::FibIter;

use num::{one, BigUint};

pub fn run() {
    let one_thousand_digits = BigUint::from(10_u8).pow(999);

    println!(
        "{}",
        FibIter::<BigUint>::new(one(), one())
            .enumerate()
            .find(|(_, x)| x >= &one_thousand_digits)
            .map(|(i, _)| i)
            .unwrap()
            + 1
    );
}
