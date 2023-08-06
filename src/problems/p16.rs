//! # Power digit sum
//!
//! 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
//!
//! What is the sum of the digits of the number 2¹⁰⁰⁰?

use num::BigUint;

use crate::numbers::digits::Digits;

pub fn run() {
	println!("{}", sum_of_digits(BigUint::from(2_u8).pow(1000)));
}

fn sum_of_digits(n: BigUint) -> BigUint {
	n.digits(10).sum()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn example() {
		assert_eq!(BigUint::from(26_u16), sum_of_digits(BigUint::from(2_u8).pow(15)))
	}

	#[test]
	fn solution() {
		assert_eq!(BigUint::from(1366_u16), sum_of_digits(BigUint::from(2_u8).pow(1000))) }
}
