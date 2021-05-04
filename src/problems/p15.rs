use num::integer::binomial;

pub fn run() {
    println!("{}", lattice_paths(20));
}

fn lattice_paths(n: u64) -> u64 {
    binomial(n * 2, n)
}
