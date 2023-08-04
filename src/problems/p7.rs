//! # 10001st prime
//!
//! By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
//!
//! What is the 10 001st prime number?

use crate::numbers::prime;

pub fn run() {
	println!("{}", prime_i(10_000));
}

fn prime_i(i: usize) -> u64 {
	prime::PrimeCache::new().iter_mut().nth(i).unwrap()
}
