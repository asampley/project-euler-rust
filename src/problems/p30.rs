//! # Digit Fifth Powers
//!
//! Surprisingly there are only three numbers that can be written as the sum of fourth powers of
//! their digits:
//!
//! > 1634 = 1⁴ + 6⁴ + 3⁴ + 4⁴
//! > 8208 = 8⁴ + 2⁴ + 0⁴ + 8⁴
//! > 9474 = 9⁴ + 4⁴ + 7⁴ + 4⁴
//!
//! As `1 = 1⁴` is not a sum it is not included.
//!
//! The sum of these numbers is `1634 + 8208 + 9474 = 19316`.
//!
//! Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.
//!
//! ## Implementation notes
//!
//! Observe that even using the maximum digit 9, which has the highest fifth power, numbers begin
//! to grow faster than each digit can provide. The largest sum for seven digits is less that the
//! smallest 7 digit number, `7 * 9⁵ = 413343 < 1000000`. Thus, we can limit our search to only 6
//! digit numbers. In fact, the largest 6 digit number that could even be a sum of its digits fifth
//! powers is when all digits are 9, and `6 * 9⁵ = 354294`, so we stop our search there.
//!
//! To keep constants out of the code, the actual upper bound is generated on the fly by finding
//! the first multiple of 9⁵ that is less than or equal to the corresponding power of 10 with one
//! more digit.
//!
//! The checking starts at 2, as per the removal of 1 mentioned above.

use crate::numbers::{
	digits::Digits,
	multiples::MultipleIter,
	powers::PowerIter,
};

pub fn run() {
	println!(
		"{}",
		(2..=upper_bound(5))
			.filter(|x| digit_power(*x, 5))
			.sum::<u32>()
	);
}

fn upper_bound(p: u32) -> u32 {
	PowerIter::new(10).zip(MultipleIter::new(9u32.pow(p)).skip(1))
		.take_while(|(p, m)| p <= m)
		.last()
		.unwrap()
		.1
}

fn digit_power(x: u32, p: u32) -> bool {
	let sum: u32 = x.digits(10).map(|d| d.pow(p)).sum();

	sum == x
}
