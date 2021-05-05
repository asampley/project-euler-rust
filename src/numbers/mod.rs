use std::ops::MulAssign;

use num::One;

pub mod factors;
pub mod fibonacci;
pub mod prime;
pub mod triangle;

pub fn factorial<R>(n: u64) -> R where R: MulAssign<u64> + One,
{
    let mut result = num::one();

    for i in 1..n {
        result *= i;
    }

    result
}
