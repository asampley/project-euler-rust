use std::ops::Range;

pub fn run() {
	println!("{}", sum_of_multiples(1..1000));
}

fn sum_of_multiples(r: Range<u32>) -> u32 {
	r.filter(|i| good(*i)).sum()
}

fn good(i: u32) -> bool {
	i % 3 == 0 || i % 5 == 0
}
