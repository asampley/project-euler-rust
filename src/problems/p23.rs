use crate::numbers::factors::FactorIter;

pub fn run() {
    println!("{}", non_abundant_sums());
}

fn non_abundant_sums() -> u64 {
    let mut abundant_numbers = Vec::new();
    let mut sum_of_abundant_numbers = [false; 28124];

    for n in 1..28124 {
        if NumberType::new(n) == NumberType::Abundant {
            abundant_numbers.push(n);
        }
    }

    for a in &abundant_numbers {
        for b in &abundant_numbers {
            if (a + b - 1) as usize >= sum_of_abundant_numbers.len() {
                break;
            }
            sum_of_abundant_numbers[(a + b - 1) as usize] = true;
        }
    }

    sum_of_abundant_numbers
        .iter()
        .enumerate()
        .filter(|(_, &a)| !a)
        .map(|(i, _)| i as u64 + 1)
        .sum()
}

#[derive(PartialEq, Eq)]
enum NumberType {
    Deficient,
    Perfect,
    Abundant,
}

impl NumberType {
    fn new(n: u64) -> Self {
        use NumberType::*;

        match FactorIter::new(n).filter(|&f| f < n).sum::<u64>() {
            x if x < n => Deficient,
            x if x == n => Perfect,
            _ => Abundant,
        }
    }
}
