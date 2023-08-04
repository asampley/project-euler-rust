use num::{CheckedMul, One};

/// Iterate over the infinite powers of a number, stopping if the power cannot be represented in
/// the type `T`.
///
/// The powers start with 0, and increment by 1 from there, so the first number will always be one.
pub struct PowerIter<T> {
	n: T,
	next: Option<T>,
}

impl<T: One> PowerIter<T> {
	pub fn new(n: T) -> Self {
		Self { n, next: Some(T::one()) }
	}
}

impl<T: CheckedMul> Iterator for PowerIter<T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		let next = self.next.as_ref().and_then(|x| x.checked_mul(&self.n));

		std::mem::replace(
			&mut self.next,
			next,
		)
	}
}
