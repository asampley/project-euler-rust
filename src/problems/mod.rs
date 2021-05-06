pub_mods!(p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16, p17, p19, p20, p21, p22, p23, p24, p25, p26);

pub fn all_problems() -> impl Iterator<Item = (usize, &'static fn())> {
    vec![
        (1, &(p1::run as fn())),
        (2, &(p2::run as fn())),
        (3, &(p3::run as fn())),
        (4, &(p4::run as fn())),
        (5, &(p5::run as fn())),
        (6, &(p6::run as fn())),
        (7, &(p7::run as fn())),
        (8, &(p8::run as fn())),
        (9, &(p9::run as fn())),
        (10, &(p10::run as fn())),
        (11, &(p11::run as fn())),
        (12, &(p12::run as fn())),
        (13, &(p13::run as fn())),
        (14, &(p14::run as fn())),
        (15, &(p15::run as fn())),
        (16, &(p16::run as fn())),
        (17, &(p17::run as fn())),
        (19, &(p19::run as fn())),
        (20, &(p20::run as fn())),
        (21, &(p21::run as fn())),
        (22, &(p22::run as fn())),
        (23, &(p23::run as fn())),
        (24, &(p24::run as fn())),
        (25, &(p25::run as fn())),
        (26, &(p26::run as fn())),
    ].into_iter()
}
