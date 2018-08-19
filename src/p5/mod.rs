use numbers::prime;
use std::cmp;
use std::collections::HashMap;

pub fn run() {
	println!("{}", smallest_divisible(20));
}

fn smallest_divisible(max: u64) -> u64 {
	let mut prime_cache = prime::PrimeCache::new();
	let mut prime_counts = HashMap::<u64,u32>::new();

	for x in 0..max+1 {
		let mut prime_factors = prime_cache.factors(x);
		if let Some(prime) = prime_factors.nth(0) {
			// count how many times this factor is used
			let count = (prime_factors.take_while(|p| p == &prime).count() + 1) as u32;
			// store the maximum count of this factor
			prime_counts.entry(prime).and_modify(|e| *e = cmp::max(count, *e)).or_insert(count);
		}
	}
	prime_counts.iter().map(|(prime, count)| prime.pow(*count)).product()
}
