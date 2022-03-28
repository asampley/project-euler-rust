//! # Number letter counts
//!
//! If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
//!
//! If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?
//!
//! **NOTE:** Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out numbers is in compliance with British usage.

pub fn run() {
    println!("{}", (1..=1000).map(|n| num_letters(n)).sum::<u64>());
}

/// spaces and dashes are not counted
fn num_letters(n: u64) -> u64 {
    match n {
        // single digits
        1 | 2 | 6 => 3,     // three letters
        0 | 4 | 5 | 9 => 4, // four letters
        3 | 7 | 8 => 5,     // five letters

        // unique double digits
        10 => 3,                // three letters
        11 | 12 => 6,           // six letters
        15 | 16 => 7,           // seven letters
        13 | 14 | 18 | 19 => 8, // eight letters
        17 => 9,                // nine letters

        // patterned double digits
        20..=39 | 80..=99 => {
            6 + match n % 10 {
                0 => 0,
                x => num_letters(x),
            }
        }
        40..=69 => {
            5 + match n % 10 {
                0 => 0,
                x => num_letters(x),
            }
        }
        70..=79 => {
            7 + match n % 10 {
                0 => 0,
                x => num_letters(x),
            }
        }

        // _ hundred and _
        100..=999 => {
            num_letters(n / 100)
                + 7
                + match n % 100 {
                    0 => 0,
                    x => 3 + num_letters(x),
                }
        }
        1000..=9999 => {
            num_letters(n / 1000)
                + 8
                + match n % 1000 {
                    0 => 0,
                    x => num_letters(x),
                }
        }
        _ => unimplemented!(),
    }
}
