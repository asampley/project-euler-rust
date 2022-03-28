//! # Lattice paths
//!
//! Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.
//!
//! How many such routes are there through a 20×20 grid?

use num::integer::binomial;

pub fn run() {
    println!("{}", lattice_paths(20));
}

fn lattice_paths(n: u64) -> u64 {
    binomial(n * 2, n)
}
