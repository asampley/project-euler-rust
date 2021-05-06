pub struct FactorIter {
	number: u64,
	last: u64,
	temp: Option<u64>,
}

impl FactorIter {
	pub fn new(number: u64) -> Self {
		FactorIter {
			number,
			last: 0,
			temp: None,
		}
	}
}

impl Iterator for FactorIter {
	type Item = u64;

	fn next(&mut self) -> Option<Self::Item> {
		if self.temp.is_some() {
			return self.temp.take();
		} else {
			let factor = (self.last + 1..)
				.take_while(|i| i * i <= self.number)
				.filter(|i| self.number % i == 0)
				.nth(0);

			match factor {
				Some(f) => {
					self.last = f;
                    self.temp = match self.number / f {
                        f_2 if f_2 == f => None,
                        f_2 => Some(f_2),
                    }
				}
				None => (),
			}

			return factor;
		}
	}
}
