use std::ops::{AddAssign, MulAssign};

use num::One;

pub mod factors;
pub mod fibonacci;
pub mod prime;
pub mod triangle;

pub fn factorial<T, R>(n: T) -> R
where
    T: AddAssign<T> + Ord + One + Copy,
    R: MulAssign<T> + AddAssign<T> + One,
{
    let mut result = num::one();
    let mut factor = num::one();

    while factor < n {
        factor += num::one();
        result *= factor;
    }

    result
}
