//! # Non-abundant sums
//!
//! A perfect number is a number for which the sum of its proper divisors is exactly equal to the number. For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect number.
//!
//! A number n is called deficient if the sum of its proper divisors is less than n and it is called abundant if this sum exceeds n.
//!
//! As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be written as the sum of two abundant numbers is 24. By mathematical analysis, it can be shown that all integers greater than 28123 can be written as the sum of two abundant numbers. However, this upper limit cannot be reduced any further by analysis even though it is known that the greatest number that cannot be expressed as the sum of two abundant numbers is less than this limit.
//!
//! Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.

use crate::numbers::factors::FactorIter;

pub fn run() {
	println!("{}", non_abundant_sums());
}

fn non_abundant_sums() -> u64 {
	let mut abundant_numbers = Vec::new();
	let mut sum_of_abundant_numbers = [false; 28124];

	for n in 1..28124 {
		if NumberType::new(n) == NumberType::Abundant {
			abundant_numbers.push(n);
		}
	}

	for a in &abundant_numbers {
		for b in &abundant_numbers {
			if (a + b - 1) as usize >= sum_of_abundant_numbers.len() {
				break;
			}
			sum_of_abundant_numbers[(a + b - 1) as usize] = true;
		}
	}

	sum_of_abundant_numbers
		.iter()
		.enumerate()
		.filter(|(_, &a)| !a)
		.map(|(i, _)| i as u64 + 1)
		.sum()
}

#[derive(PartialEq, Eq)]
enum NumberType {
	Deficient,
	Perfect,
	Abundant,
}

impl NumberType {
	fn new(n: u64) -> Self {
		use NumberType::*;

		match FactorIter::new(n).filter(|&f| f < n).sum::<u64>() {
			x if x < n => Deficient,
			x if x == n => Perfect,
			_ => Abundant,
		}
	}
}
