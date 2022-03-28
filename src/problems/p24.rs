//! Each symbol can be calculated as the nth symbol remaining in the list in one equation,
//! so we can jump to the nth permutation by simply calculating the index of each symbol, and
//! removing it from the list. This repeats for each symbol until we have the permutation.
//!
//! We want the nth of the permutations of L. Each digit d will be the ith symbol
//! remaining in the list, using the equation `i = (n / d!) % (d + 1)`. Both n and d start at
//! 0, and digit 0 is the rightmost digit.
//!
//! As an example, L = {0, 1, 2}, n = 3, then for each of the digits:
//! * d = 0, i = 3 / 1 % 1 = 0
//! * d = 1, i = 3 / 1 % 2 = 1
//! * d = 2, i = 3 / 2 % 3 = 1
//!
//! We convert back to an answer by removing elements from the list, starting from the largest
//! value of d. You can see once each element is removed, we end up with the solution 120.
//! * d = 2, i = 1, L <= {0, 2}
//! * d = 1, i = 1, L <= {0}
//! * d = 0, i = 0, L <= {}

use crate::numbers::factorial;

pub fn run() {
    permutation(999_999, 10)
        .iter()
        .for_each(|p| print!("{}", p));
    println!();
}

fn permutation(n: u64, num_symbols: u64) -> Vec<u64> {
    let mut symbols = (0..num_symbols).collect::<Vec<_>>();

    let permutation_ids = (0..num_symbols).rev().map(|i| symbol(n, i));

    permutation_ids
        .map(|symbol_id| symbols.remove(symbol_id as usize))
        .collect::<Vec<_>>()
}

fn symbol(permutation: u64, digit: u64) -> u64 {
    (permutation / factorial::<_, u64>(digit)) % (digit + 1)
}
