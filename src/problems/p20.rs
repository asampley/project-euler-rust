//! # Factorial digit sum
//!
//! n! means n × (n − 1) × ... × 3 × 2 × 1
//!
//! For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
//! and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
//!
//! Find the sum of the digits in the number 100!

use crate::numbers::{factorial, digits::Digits};

use num::BigUint;

pub fn run() {
	println!("{}", sum_of_factorial_digits(100));
}

fn sum_of_factorial_digits(n: u64) -> u32 {
	factorial::<u64, BigUint>(n)
		.digits(10)
		.rev()
		.map(|i| i as u32)
		.sum::<u32>()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn example() {
		assert_eq!(27, sum_of_factorial_digits(10))
	}

	#[test]
	fn solution() {
		assert_eq!(648, sum_of_factorial_digits(100))
	}
}
