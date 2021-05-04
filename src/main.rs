#[macro_use]
mod macros;

mod numbers;
mod problems;

fn_print_problems!{print_problems, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16}

fn main() {
    print_problems();
}
