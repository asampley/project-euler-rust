use crate::numbers::factorial;

pub fn run() {
    permutation(999_999, 10).iter().for_each(|p| print!("{}", p));
    println!();
}

fn permutation(n: u64, num_symbols: u64) -> Vec<u64> {
    let mut symbols = (0..num_symbols).collect::<Vec<_>>();

    let permutation_ids = (0..num_symbols)
        .rev()
        .map(|i| symbol(n, i));

    permutation_ids
        .map(|symbol_id| symbols.remove(symbol_id as usize))
        .collect::<Vec<_>>()
}

fn symbol(permutation: u64, digit: u64) -> u64 {
    (permutation / factorial::<u64>(digit)) % (digit + 1)
}
