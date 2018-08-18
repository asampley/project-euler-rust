mod fibonacci;
mod prime;

#[macro_use]
mod mods;
mods!(p1,p2,p3,p4,p5,p6,p7,p8,p9);

#[macro_use]
mod problems;

fn main() {
	print_problems!(p1,p2,p3,p4,p5,p6,p7,p8,p9);
}
