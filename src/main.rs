use std::env;
use std::collections::HashSet;

#[macro_use]
mod macros;

mod numbers;
mod problems;

fn main() {
    let args: HashSet<usize> = env::args().skip(1).map(|s| s.parse().expect("Supplied argument not a number")).collect();

    if args.is_empty() {
        for (i, problem) in problems::all_problems() {
            run(i, problem);
        }
    } else {
        for (i, problem) in problems::all_problems().filter(|(i, _)| args.contains(i)) {
            run(i, problem);
        }
    }
}

fn run(i: usize, f: &fn()) {
    let start = std::time::Instant::now();
    print!("{}: ", i);
    f();
    let elapsed = start.elapsed();
    println!("  Time taken: {}.{:06}s", elapsed.as_secs(), elapsed.subsec_micros());
}
