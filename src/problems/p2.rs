use crate::numbers::fibonacci;

pub fn run() {
    println!("{}", sum_of_even(4_000_000));
}

fn sum_of_even(max: u64) -> u64 {
    fibonacci::FibIter::new(1, 2)
        .filter(|f| *f % 2 == 0)
        .take_while(|f| *f <= max)
        .sum()
}
