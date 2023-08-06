//! # Longest Collatz sequence
//!
//! The following iterative sequence is defined for the set of positive integers:
//!
//! > n → n/2 (n is even)\
//! > n → 3n + 1 (n is odd)
//!
//! Using the rule above and starting with 13, we generate the following sequence:
//!
//! > 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
//!
//!
//! It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.
//!
//! Which starting number, under one million, produces the longest chain?
//!
//! **NOTE:** Once the chain starts the terms are allowed to go above one million.

use std::collections::HashMap;

pub fn run() {
	let mut cache = CollatzCache::new();
	println!("{}", longest_chain_start(1_000_000 - 1, &mut cache));
}

struct CollatzCache {
	cache: HashMap<u64, usize>,
}

impl CollatzCache {
	fn new() -> Self {
		Self {
			cache: HashMap::new(),
		}
	}

	fn collatz_count(&mut self, n: u64) -> usize {
		match self.cache.get(&n) {
			Some(&c) => c,
			None => match n {
				0 | 1 => 1,
				n => {
					let next = 1 + self.collatz_count(CollatzCache::collatz_next(n));
					*self.cache.entry(n).or_insert(next)
				}
			},
		}
	}

	fn collatz_next(n: u64) -> u64 {
		if n % 2 == 0 {
			n / 2
		} else {
			3 * n + 1
		}
	}
}

fn longest_chain_start(max_start: u64, cache: &mut CollatzCache) -> u64 {
	(1..=max_start).fold(0, |acc, x| {
		if cache.collatz_count(acc) > cache.collatz_count(x) {
			acc
		} else {
			x
		}
	})
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn example() {
		assert_eq!(10, CollatzCache::new().collatz_count(13))
	}

	#[test]
	fn solution() {
		assert_eq!(837799, longest_chain_start(1_000_000, &mut CollatzCache::new()))
	}
}
