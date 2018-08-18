use prime;

pub fn run() {
	println!("{}", largest_prime_factor(600_851_475_143));
}

fn largest_prime_factor(x: u64) -> u64 {
	let mut prime_cache = prime::PrimeCache::new();
	let mut prime_iter = prime_cache.iter_mut();

	let mut x_mut = x;

	loop {
		let prime = prime_iter.next().unwrap();
		while x_mut % prime == 0 {
			x_mut /= prime;
		}
		if x_mut == 1 {
			return prime;
		}
	}
}
