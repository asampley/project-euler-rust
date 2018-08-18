macro_rules! print_problems {
	( $( $x: ident ),* ) => {
		$(
			print!("{}: ", stringify!($x));
			$x::run();
		)*
	}
}
