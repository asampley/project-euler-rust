//! # Number spiral diagonals
//!
//! Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is
//! formed as follows:
//!
//! ```text
//! 21 22 23 24 25
//! 20  7  8  9 10
//! 19  6  1  2 11
//! 18  5  4  3 12
//! 17 16 15 14 13
//! ```
//!
//! It can be verified that the sum of the numbers on the diagonals is 101.
//!
//! What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same
//! way?
//!
//!
//! ## Implementation notes
//!
//! Starting from 1, we can see that we skip one number four times, then three numbers four times,
//! and so on incrementing the number of skips by two every four times. Given a square of size n by
//! n we should therefore be able to skip straight to all the additions, resulting in a runtime of
//! O(sqrt(n)).
//!
//! Looking even more closely, though, we can simplify each square shell relative to the square
//! number it contains. It can easily be seen that each shell ends with the square of consecutive
//! odd numbers. If we label the shells starting from 0, and skip the zeroth shell as it only
//! contains a single number, each shell adds the following to the total sum:
//!
//! > `1 + ∑ (4 * (2 * shell + 1)² - 12 * shell)`
//!
//! This can be justified by the following logic, each shell ends with the next power of an odd,
//! and `2 * shell + 1` generates consective odd numbers. There are 4 corners to each shell, so we
//! add the power 4 times. However, this is too large, and we must remove the increment between
//! each number to adjust. The increments go up by 2 with each shell, and start at 2 for shell one.
//! For the first number in the shell we must remove the increment 3 times, then 2 times for the
//! next, and finally one more time, leading to 6 removals of the increment.
//!
//! > `6 * increment = 6 * 2 * shell = 12 * shell`.
//!
//! For the total we sum the shells from 1 to r = ⌊n / 2⌋, plus one more for the zeroth shell, where n
//! is the side length. By converting the sum to a proper quadratic and breaking it up, we can come up
//! with a very concise formula.
//!
//! > `(16 * r³ + 26 * r) / 3 + 10 * r² + 1`

pub fn run() {
	println!("{}", diagonal_sum(1001));
}

fn diagonal_sum(spiral_side: u64) -> u64 {
	let r = spiral_side / 2;
	let r2 = r * r;
	let r3 = r2 * r;

	(16 * r3 + 26 * r) / 3 + 10 * r2 + 1
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn example() {
		assert_eq!(101, diagonal_sum(5))
	}

	#[test]
	fn solution() {
		assert_eq!(669171001, diagonal_sum(1001))
	}
}
