//! # Special Pythagorean triplet
//!
//! A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
//!
//! > a² + b² = c²
//!
//! For example, 3² + 4² = 9 + 16 = 25 = 5².
//!
//! There exists exactly one Pythagorean triplet for which a + b + c = 1000.
//!
//! Find the product abc.

pub fn run() {
    match pythagorean_product(1000) {
        Some(i) => println!("{}", i),
        None => println!("None"),
    };
}

fn pythagorean_product(sum: u64) -> Option<u64> {
    for a in 1..sum {
        for b in a + 1..sum - a {
            let c = 1000 - a - b;
            if a * a + b * b == c * c {
                return Some(a * b * c);
            }
        }
    }
    None
}
