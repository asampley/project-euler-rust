automod::dir!(pub "src/problems");

use paste::paste;

/// expand to each problem module to an index and run function,
///
/// e.g. `p!(3)` becomes `(3, p3::run as fn())`
macro_rules! p {
	( $x:tt ) => {
		paste! {
			($x, [<p $x>]::run as fn())
		}
	};
}

pub fn all_problems() -> impl Iterator<Item = (usize, fn())> {
	[
		p!(1),
		p!(2),
		p!(3),
		p!(4),
		p!(5),
		p!(6),
		p!(7),
		p!(8),
		p!(9),
		p!(10),
		p!(11),
		p!(12),
		p!(13),
		p!(14),
		p!(15),
		p!(16),
		p!(17),
		p!(18),
		p!(19),
		p!(20),
		p!(21),
		p!(22),
		p!(23),
		p!(24),
		p!(25),
		p!(26),
		p!(27),
		p!(28),
		p!(29),
		p!(30),
	]
	.into_iter()
}
