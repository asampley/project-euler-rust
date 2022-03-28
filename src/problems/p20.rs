use crate::numbers::factorial;

use num::BigUint;

pub fn run() {
    println!(
        "{}",
        factorial::<u64, BigUint>(100)
            .to_radix_be(10)
            .into_iter()
            .map(|i| i as u32)
            .sum::<u32>()
    );
}
