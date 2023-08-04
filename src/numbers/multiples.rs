use num::{CheckedAdd, Zero};

/// Iterate over the infinite multiples of a number, stopping if the multiple cannot be represented
/// in the type `T`.
///
/// The multiples start with 0, and increment from there, so the first number will always be zero.
/// If this is undesirable, call [`Iterator::skip`]
pub struct MultipleIter<T> {
	n: T,
	next: Option<T>,
}

impl<T: Zero> MultipleIter<T> {
	pub fn new(n: T) -> Self {
		Self { n, next: Some(T::zero()) }
	}
}

impl<T: CheckedAdd> Iterator for MultipleIter<T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		let next = self.next.as_ref().and_then(|x| x.checked_add(&self.n));

		std::mem::replace(
			&mut self.next,
			next,
		)
	}
}
