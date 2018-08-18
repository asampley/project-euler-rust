pub struct PrimeCache {
	primes: Vec<u64>,
}
pub struct PrimeIter<'a> {
	cache: &'a mut PrimeCache,
	i: usize,
}
	
impl PrimeCache {
	pub fn new() -> PrimeCache {
		PrimeCache { primes: vec![2,3] }
	}

	pub fn last(&self) -> &u64 {
		self.primes.last().unwrap()
	}

	pub fn is_prime(&mut self, x: u64) -> bool {
		while *self.last() < x {
			self.next_prime();
		}
		match self.primes.binary_search(&x) {
			Ok(_) => true,
			_     => false,
		}
	}

	pub fn next_prime(&mut self) -> u64 {
		// take the first number with no prime factors
		let prime = (*self.last()+1..).filter(
			// take all primes
			|x| self.primes.iter() // take all primes
				// that are less than sqrt(x) (+ 1 to keep float errors away)
				.take_while(|y| **y < ((*x as f64).sqrt() as u64) + 1) // that are less than sqrt(x)
				.all(|y| *x % *y != 0) // and make sure x is divisible by none of them
		).nth(0).unwrap();
		self.primes.push(prime);
		prime
	}
	
	pub fn iter_mut(&mut self) -> PrimeIter {
		PrimeIter { cache: self, i: 0 }
	}
}

impl<'a> Iterator for PrimeIter<'a> {
	type Item = u64;
	fn next(&mut self) -> Option<u64> {
		self.i += 1;
		while self.cache.primes.len() < self.i {
			self.cache.next_prime();
		}
		Some(*self.cache.primes.get(self.i - 1).unwrap())
	}
}
