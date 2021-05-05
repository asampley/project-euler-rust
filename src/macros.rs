#[macro_use]
macro_rules! pub_mods {
	( $( $x:ident ),* ) => ( $(pub mod $x;)* );
}
