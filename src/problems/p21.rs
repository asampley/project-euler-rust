use crate::numbers::factors::FactorIter;

pub fn run() {
    println!("{}", (1..10000).filter(|&n| is_amicable(n)).sum::<u64>())
}

fn is_amicable(n: u64) -> bool {
    let factor_sum_1 = FactorIter::new(n).filter(|x| x < &n).sum();

    if factor_sum_1 == n {
        false
    } else { 
        n == FactorIter::new(factor_sum_1).filter(|x| x < &factor_sum_1).sum()
    }
}
