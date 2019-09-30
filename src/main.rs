#[macro_use]
mod macros;

mod numbers;
mod problems;

use problems::{p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p14};

fn main() {
    print_problems!(p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p14);
}
