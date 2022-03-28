use crate::numbers::prime;

pub fn run() {
    println!("{}", prime_i(10_000));
}

fn prime_i(i: usize) -> u64 {
    prime::PrimeCache::new().iter_mut().nth(i).unwrap()
}
