#[macro_use]
macro_rules! mods {
	( $( $x:ident ),* ) => ( $(mod $x;)* );
}

#[macro_use]
macro_rules! pub_mods {
	( $( $x:ident ),* ) => ( $(pub mod $x;)* );
}

#[macro_use]
macro_rules! print_problems {
	( $( $x: ident ),* ) => {
		$(
                        let start = std::time::Instant::now();
			print!("{}: ", stringify!($x));
			$x::run();
                        let elapsed = start.elapsed();
                        println!("  Time taken: {}.{:06}", elapsed.as_secs(), elapsed.subsec_micros());
		)*
	}
}
