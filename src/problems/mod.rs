pub mod p1;
pub mod p2;
pub mod p3;
pub mod p4;
pub mod p5;
pub mod p6;
pub mod p7;
pub mod p8;
pub mod p9;
pub mod p10;
pub mod p11;
pub mod p12;
pub mod p13;
pub mod p14;
pub mod p15;
pub mod p16;
pub mod p17;
pub mod p18;
pub mod p19;
pub mod p20;
pub mod p21;
pub mod p22;
pub mod p23;
pub mod p24;
pub mod p25;
pub mod p26;
pub mod p27;

macro_rules! run {
    ( $x:ident ) => {
        &($x::run as fn())
    };
}

pub fn all_problems() -> impl Iterator<Item = (usize, &'static fn())> {
    [
        (1, run!(p1)),
        (2, run!(p2)),
        (3, run!(p3)),
        (4, run!(p4)),
        (5, run!(p5)),
        (6, run!(p6)),
        (7, run!(p7)),
        (8, run!(p8)),
        (9, run!(p9)),
        (10, run!(p10)),
        (11, run!(p11)),
        (12, run!(p12)),
        (13, run!(p13)),
        (14, run!(p14)),
        (15, run!(p15)),
        (16, run!(p16)),
        (17, run!(p17)),
        (18, run!(p18)),
        (19, run!(p19)),
        (20, run!(p20)),
        (21, run!(p21)),
        (22, run!(p22)),
        (23, run!(p23)),
        (24, run!(p24)),
        (25, run!(p25)),
        (26, run!(p26)),
        (27, run!(p27)),
    ]
    .into_iter()
}
