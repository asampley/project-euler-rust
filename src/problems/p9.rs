pub fn run() {
    match pythagorean_product(1000) {
        Some(i) => println!("{}", i),
        None => println!("None"),
    };
}

fn pythagorean_product(sum: u64) -> Option<u64> {
    for a in 1..sum {
        for b in a + 1..sum - a {
            let c = 1000 - a - b;
            if a * a + b * b == c * c {
                return Some(a * b * c);
            }
        }
    }
    None
}
