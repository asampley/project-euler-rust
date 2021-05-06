pub_mods!(p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16, p17, p19, p20, p21, p22, p23, p24);

pub fn all_problems() -> impl Iterator<Item = (usize, &'static fn())> {
    let mut problems = Vec::new();

    problems.push((1, &(p1::run as fn())));
    problems.push((2, &(p2::run as fn())));
    problems.push((3, &(p3::run as fn())));
    problems.push((4, &(p4::run as fn())));
    problems.push((5, &(p5::run as fn())));
    problems.push((6, &(p6::run as fn())));
    problems.push((7, &(p7::run as fn())));
    problems.push((8, &(p8::run as fn())));
    problems.push((9, &(p9::run as fn())));
    problems.push((10, &(p10::run as fn())));
    problems.push((11, &(p11::run as fn())));
    problems.push((12, &(p12::run as fn())));
    problems.push((13, &(p13::run as fn())));
    problems.push((14, &(p14::run as fn())));
    problems.push((15, &(p15::run as fn())));
    problems.push((16, &(p16::run as fn())));
    problems.push((17, &(p17::run as fn())));
    problems.push((19, &(p19::run as fn())));
    problems.push((20, &(p20::run as fn())));
    problems.push((21, &(p21::run as fn())));
    problems.push((22, &(p22::run as fn())));
    problems.push((23, &(p23::run as fn())));
    problems.push((24, &(p24::run as fn())));

    problems.into_iter()
}
