//! # Summation of primes
//!
//! The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//!
//! Find the sum of all the primes below two million.

use crate::numbers::prime;

pub fn run() {
	println!("{}", sum_of_primes(2_000_000));
}

fn sum_of_primes(max: u64) -> u64 {
	prime::PrimeCache::new()
		.iter_mut()
		.take_while(|p| *p < max)
		.sum()
}
