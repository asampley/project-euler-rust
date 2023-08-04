automod::dir!(pub "src/problems");

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
		(28, run!(p28)),
		(29, run!(p29)),
	]
	.into_iter()
}
