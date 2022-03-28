use num_enum::{FromPrimitive, UnsafeFromPrimitive};

pub fn run() {
    println!("{}", count_sundays());
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, UnsafeFromPrimitive, Debug)]
#[allow(dead_code)]
enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, UnsafeFromPrimitive, Debug)]
#[allow(dead_code)]
enum Month {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

impl Month {
    fn days(&self, year: u64) -> u8 {
        use Month::*;

        match self {
            Sep | Apr | Jun | Nov => 30,
            Feb => {
                if leap_year(year) {
                    29
                } else {
                    28
                }
            }
            _ => 31,
        }
    }
}

impl FromPrimitive for DayOfWeek {
    type Primitive = u8;

    fn from_primitive(n: Self::Primitive) -> Self {
        unsafe { Self::from_unchecked(n % 7) }
    }
}

impl FromPrimitive for Month {
    type Primitive = u8;

    fn from_primitive(n: Self::Primitive) -> Self {
        unsafe { Self::from_unchecked(n % 12) }
    }
}

fn count_sundays() -> u64 {
    let mut sundays = 0;
    let mut day_of_week = DayOfWeek::Tuesday; // Jan 1 1900 = Monday -> Jan 1 1901 was a Tuesday

    for year in 1901..=2000 {
        for month in (0..12).map(|n| Month::from_primitive(n)) {
            if day_of_week == DayOfWeek::Sunday {
                sundays += 1;
            }

            day_of_week = DayOfWeek::from_primitive(day_of_week as u8 + month.days(year));
        }
    }

    sundays
}

fn leap_year(year: u64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
