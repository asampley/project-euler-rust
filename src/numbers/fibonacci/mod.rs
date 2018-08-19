pub struct FibIter {
	f0: u64,
	f1: u64,
	state: IterState,
}

enum IterState {
	First,
	Second,
	Next,
}

impl FibIter {
	pub fn new(f0: u64, f1: u64) -> FibIter {
		FibIter { f0, f1, state: IterState::First }
	}
}
 
impl Iterator for FibIter {
	type Item = u64;
	fn next(&mut self) -> Option<u64> {
		Some( match self.state {
			IterState::First => {
				self.state = IterState::Second;
				self.f0
			}
			IterState::Second => {
				self.state = IterState::Next;
				self.f1
			}
			IterState::Next => {
				let temp = self.f1;
				self.f1 = self.f0 + self.f1;
				self.f0 = temp;
				self.f1
			}
		})
	}
}
