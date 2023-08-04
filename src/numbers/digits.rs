use duplicate::duplicate_item;

use num::Integer;

pub trait Digits<R> {
	type Iter;

	fn digits(self, radix: R) -> Self::Iter;
}

impl Digits<u32> for &num::BigUint {
	type Iter = <Vec<u8> as IntoIterator>::IntoIter;

	fn digits(self, radix: u32) -> Self::Iter {
		self.to_radix_le(radix).into_iter()
	}
}

impl Digits<u32> for &num::BigInt {
	type Iter = <Vec<u8> as IntoIterator>::IntoIter;

	fn digits(self, radix: u32) -> Self::Iter {
		self.to_radix_le(radix).1.into_iter()
	}
}

#[duplicate_item(
	Int;
	[u8]; [u16]; [u32]; [u64]; [u128]; [usize];
	[i8]; [i16]; [i32]; [i64]; [i128]; [isize];
)]
impl Digits<Int> for Int {
	type Iter = DigitIter<Int>;

	fn digits(self, radix: Int) -> Self::Iter {
		DigitIter::new(self, radix)
	}
}

/// Iterator over the digits of a number, from least significant digit to greatest.
pub struct DigitIter<T> {
	number: T,
	base: T,
}

impl<T> DigitIter<T> {
	pub fn new(number: T, base: T) -> Self {
		Self { number, base }
	}
}

impl<T: Integer> Iterator for DigitIter<T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		if self.number.is_zero() {
			None
		} else {
			let digit;

			(self.number, digit) = self.number.div_rem(&self.base);

			Some(digit)
		}
	}
}
