#[macro_use]
macro_rules! pub_mods {
	( $( $x:ident ),* ) => ( $(pub mod $x;)* );
}

#[macro_use]
macro_rules! fn_print_problems {
	( $fn: ident, $( $x: ident ),* ) => {
        fn $fn() {
            use problems::{$($x),*};

            $(
                let start = std::time::Instant::now();
                print!("{}: ", stringify!($x));
                $x::run();
                let elapsed = start.elapsed();
                println!("  Time taken: {}.{:06}", elapsed.as_secs(), elapsed.subsec_micros());
            )*
        }
	}
}
