//! # Largest palindrome product
//! A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
//!
//! Find the largest palindrome made from the product of two 3-digit numbers.

pub fn run() {
	println!("{}", largest_palindrome_product(3))
}

fn palindromes(digits: u32) -> impl DoubleEndedIterator<Item = u64> {
	(0..10u64.pow(digits / 2 + digits % 2))
		.map(move |x| {
			x.to_string()
				.chars()
				.chain(x.to_string()[digits as usize % 2..].chars().rev())
				.collect()
		})
		.map(|x: String| x.parse::<u64>().unwrap())
}

fn is_product(x: u64, digits: u32) -> bool {
	for a in (10u64.pow(digits - 1)..10u64.pow(digits)).rev() {
		for b in (a..10u64.pow(digits)).rev() {
			if x == a * b {
				return true;
			}
		}
	}
	return false;
}

fn largest_palindrome_product(digits: u32) -> u64 {
	palindromes(digits * 2)
		.rev()
		.filter(|x| is_product(*x, digits))
		.nth(0)
		.unwrap()
}
