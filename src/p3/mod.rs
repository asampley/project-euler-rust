use prime;

pub fn run() {
	println!("{}", largest_prime_factor(600_851_475_143));
}

fn largest_prime_factor(x: u64) -> u64 {
	let mut prime_cache = prime::PrimeCache::new();

	prime_cache.factors(x).last().unwrap()
}
