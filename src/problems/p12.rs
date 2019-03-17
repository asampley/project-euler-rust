use numbers::triangle::TriangleIter;
use numbers::factors::FactorIter;

pub fn run() {
	println!("{}", first_with_factors(500));
}

fn first_with_factors(min_factors: usize) -> u64 {
        let iter = TriangleIter::new();

        iter.filter(|t| FactorIter::new(*t).count() >= min_factors).nth(0).unwrap()
}
