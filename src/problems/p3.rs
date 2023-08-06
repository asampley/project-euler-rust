//! # Largest prime factor
//! The prime factors of 13195 are 5, 7, 13 and 29.
//!
//! What is the largest prime factor of the number 600851475143 ?

use crate::numbers::prime;

pub fn run() {
	println!("{}", largest_prime_factor(600_851_475_143));
}

fn largest_prime_factor(x: u64) -> u64 {
	let mut prime_cache = prime::PrimeCache::new();

	prime_cache.factors(x).last().unwrap()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn example() {
		assert_eq!(29, largest_prime_factor(13195))
	}

	#[test]
	fn solution() {
		assert_eq!(6857, largest_prime_factor(600_851_475_143))
	}
}
