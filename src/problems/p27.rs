//! # Quadratic Primes
//!
//! Euler discovered the remarkable quadratic formula:
//!
//! > n² + n + 41
//!
//! It turns out that the formula will produce 40 primes for the consecutive integer values 0 <= n <= 39. However, when n = 40, 40² + 40 + 41 = 40(40 + 1) + 41 is divisible by 41, and certainly when n = 41, 41² + 41 + 41 is clearly divisible by 41.
//!
//! The incredible formula n² - 79n + 1601 was discovered, which produces 80 primes for the consecutive values 0 <= n <= 79. The product of the coefficients, −79 and 1601, is −126479.
//!
//! Considering quadratics of the form:
//!
//! > n² + an + b, where |a| < 1000 and |b| <= 1000
//! >
//! > where |n| is the modulus/absolute value of n
//! > e.g. |11| = 11 and |-4| = 4
//!
//! Find the product of the coefficients a and b, for the quadratic expression that produces the maximum number of primes for consecutive values of n, starting with n = 0.
use itertools::Itertools;

use crate::numbers::prime::PrimeCache;

pub fn run() {
	println!("{}", coefficient_product(999, 1000));
}

fn coefficient_product(a_range: i64, b_range: i64) -> i64 {
	let mut cache = PrimeCache::new();

	(-a_range..a_range)
		.cartesian_product(-b_range..=b_range)
		.max_by_key(|(a, b)| prime_count(&mut cache, *a, *b))
		.map(|(a, b)| a * b)
		.unwrap()
}

fn prime_count(cache: &mut PrimeCache, a: i64, b: i64) -> usize {
	(0_i64..)
		.take_while(|n| match n * n + a * n + b {
			x if x >= 0 => cache.is_prime(x as u64),
			_ => false,
		})
		.count()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn example() {
		assert_eq!(40, prime_count(&mut PrimeCache::new(), 1, 41))
	}

	#[test]
	fn solution() {
		assert_eq!(-59231, coefficient_product(999, 1000))
	}
}
