//! # Sum square difference
//!
//! The sum of the squares of the first ten natural numbers is,
//!
//! > 1² + 2² + ... + 10² = 385
//!
//! The square of the sum of the first ten natural numbers is,
//!
//! > (1 + 2 + ... + 10)² = 55² = 3025
//!
//! Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 - 385 = 2640.
//!
//! Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

pub fn run() {
	println!("{}", difference(1, 100));
}

fn difference(min: u64, max: u64) -> u64 {
	square_of_sum(min, max) - sum_of_squares(min, max)
}

fn sum_of_squares(min: u64, max: u64) -> u64 {
	(min..max + 1).map(|x| x * x).sum()
}

fn square_of_sum(min: u64, max: u64) -> u64 {
	let x: u64 = (min..max + 1).sum();
	x * x
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn example() {
		assert_eq!(385, sum_of_squares(1, 10));
		assert_eq!(3025, square_of_sum(1, 10));
		assert_eq!(2640, difference(1, 10));
	}

	#[test]
	fn solution() {
		assert_eq!(25164150, difference(1, 100))
	}
}
