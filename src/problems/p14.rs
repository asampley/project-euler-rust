use std::collections::HashMap;

pub fn run() {
	let mut cache = CollatzCache::new();
	println!("{}", longest_chain_start(1_000_000 - 1, &mut cache));
}

struct CollatzCache {
	cache: HashMap<u64, usize>
}

impl CollatzCache {
	fn new() -> Self {
		Self { cache: HashMap::new() }
	}

	fn collatz_count(&mut self, n: u64) -> usize {
		match self.cache.get(&n) {
			Some(&c) => c,
			None => {
				match n {
					0 | 1 => 1,
					n => {
						let next = 1 + self.collatz_count(CollatzCache::collatz_next(n));
						*self.cache.entry(n).or_insert(next)
					}
				}
			}
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
	(1..=max_start)
		.fold(0, |acc, x| if cache.collatz_count(acc) > cache.collatz_count(x) { acc } else { x })
}
