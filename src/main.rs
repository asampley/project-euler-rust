#[macro_use]
mod macros;

mod numbers;
mod problems;

fn main() {
    for (i, problem) in problems::all_problems() {
        let start = std::time::Instant::now();
        print!("{}: ", i);
        problem();
        let elapsed = start.elapsed();
        println!("  Time taken: {}.{:06}s", elapsed.as_secs(), elapsed.subsec_micros());
    }
}
