//! # Multiples of 3 or 5
//!
//!  If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
//!
//! Find the sum of all the multiples of 3 or 5 below 1000.
use std::ops::Range;

pub fn run() {
    println!("{}", sum_of_multiples(1..1000));
}

fn sum_of_multiples(r: Range<u32>) -> u32 {
    r.filter(|i| good(*i)).sum()
}

fn good(i: u32) -> bool {
    i % 3 == 0 || i % 5 == 0
}
