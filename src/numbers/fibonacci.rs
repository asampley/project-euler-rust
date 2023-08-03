use std::ops::AddAssign;

pub struct FibIter<T> {
	f0: T,
	f1: T,
	state: IterState,
}

enum IterState {
	First,
	Second,
	Next,
}

impl<T> FibIter<T> {
	pub fn new(f0: T, f1: T) -> Self {
		FibIter {
			f0,
			f1,
			state: IterState::First,
		}
	}
}

impl<T> Iterator for FibIter<T>
where
	T: Clone,
	for<'a> T: AddAssign<&'a T>,
{
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		Some(match self.state {
			IterState::First => {
				self.state = IterState::Second;
				self.f0.clone()
			}
			IterState::Second => {
				self.state = IterState::Next;
				self.f1.clone()
			}
			IterState::Next => {
				std::mem::swap(&mut self.f1, &mut self.f0);
				self.f1 += &self.f0;
				self.f1.clone()
			}
		})
	}
}
