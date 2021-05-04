use num::BigUint;

pub fn run() {
    println!("{}", sum_of_digits(BigUint::from(2_u8).pow(1000)));
}

fn sum_of_digits(n: BigUint) -> u32 {
    n.to_str_radix(10)
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum()
}
