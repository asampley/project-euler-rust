use crate::numbers::fibonacci::FibIter;

use num::{one, BigUint};

pub fn run() {
    let one_thousand_digits = BigUint::from(10_u8).pow(999);

    println!(
        "{}",
        FibIter::<BigUint>::new(one(), one())
            .enumerate()
            .find(|(_, x)| x >= &one_thousand_digits)
            .map(|(i, _)| i)
            .unwrap()
            + 1
    );
}
