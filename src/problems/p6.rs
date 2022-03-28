pub fn run() {
    println!("{}", difference(1, 100));
}

fn difference(min: u64, max: u64) -> u64 {
    square_of_sum(min, max) - sum_of_squares(min, max)
}

fn sum_of_squares(min: u64, max: u64) -> u64 {
    (min..max + 1).map(|x| x * x).sum()
}

fn square_of_sum(min: u64, max: u64) -> u64 {
    let x: u64 = (min..max + 1).sum();
    x * x
}
