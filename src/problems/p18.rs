//! # Maximum path sum I
//!
//! By starting at the top of the triangle below and moving to adjacent numbers on the row below,
//! the maximum total from top to bottom is 23.
//!
//! > 3\
//! > 7 4\
//! > 2 4 6\
//! > 8 5 9 3\
//!
//! That is, 3 + 7 + 4 + 9 = 23.
//!
//! Find the maximum total from top to bottom of the triangle below:
//!
//! > 75\
//! > 95 64\
//! > 17 47 82\
//! > 18 35 87 10\
//! > 20 04 82 47 65\
//! > 19 01 23 75 03 34\
//! > 88 02 77 73 07 63 67\
//! > 99 65 04 28 06 16 70 92\
//! > 41 41 26 56 83 40 80 70 33\
//! > 41 48 72 33 47 32 37 16 94 29\
//! > 53 71 44 65 25 43 91 52 97 51 14\
//! > 70 11 33 28 77 73 17 78 39 68 17 57\
//! > 91 71 52 38 17 14 91 43 58 50 27 29 48\
//! > 63 66 04 68 89 53 67 30 73 16 69 87 40 31\
//! > 04 62 98 27 23 09 70 98 73 93 38 53 60 04 23\
//!
//! **NOTE:** As there are only 16384 routes, it is possible to solve this problem by trying every
//! route. However, Problem 67, is the same challenge with a triangle containing one-hundred rows;
//! it cannot be solved by brute force, and requires a clever method! ;o)

use core::cmp::max;

pub fn run() {
	let triangle = Triangle {
		rows: vec![
			vec![75],
			vec![95, 64],
			vec![17, 47, 82],
			vec![18, 35, 87, 10],
			vec![20, 04, 82, 47, 65],
			vec![19, 01, 23, 75, 03, 34],
			vec![88, 02, 77, 73, 07, 63, 67],
			vec![99, 65, 04, 28, 06, 16, 70, 92],
			vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
			vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
			vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
			vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
			vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
			vec![63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
			vec![04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23],
		],
	};

	println!("{}", max_path_sum(&triangle));
}

#[derive(Debug)]
pub struct Triangle<T> {
	rows: Vec<Vec<T>>,
}

impl<T> Triangle<T> {
	pub fn get_mut(&mut self, index: &(usize, usize)) -> Option<&mut T> {
		self.rows.get_mut(index.0).and_then(|r| r.get_mut(index.1))
	}

	pub fn get(&self, index: &(usize, usize)) -> Option<&T> {
		self.rows.get(index.0).and_then(|r| r.get(index.1))
	}

	pub fn root(&self) -> Option<&T> {
		self.get(&(0, 0))
	}

	pub fn rows(&self) -> usize {
		self.rows.len()
	}
}

impl<T: Clone> Triangle<T> {
	pub fn with_rows(row_count: usize, value: T) -> Self {
		let mut rows = Vec::with_capacity(row_count);

		for r in 0..row_count {
			let mut row = Vec::with_capacity(r + 1);
			row.resize(r + 1, value.clone());

			rows.push(row);
		}

		Self { rows }
	}
}

fn max_path_sum(triangle: &Triangle<u64>) -> u64 {
	let mut max_path_triangle = Triangle::with_rows(triangle.rows(), 0);

	for row in (0..triangle.rows()).rev() {
		for col in 0..=row {
			let max_path = triangle.get(&(row, col)).unwrap()
				+ match (
					max_path_triangle.get(&(row + 1, col)),
					max_path_triangle.get(&(row + 1, col + 1)),
				) {
					(Some(a), Some(b)) => *max(a, b),
					_ => 0,
				};

			*max_path_triangle.get_mut(&(row, col)).unwrap() = max_path;
		}
	}

	*max_path_triangle.root().unwrap()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn example() {
		let triangle = Triangle {
			rows: vec![
				vec![3],
				vec![7, 4],
				vec![2, 4, 6],
				vec![8, 5, 9, 3],
			]
		};

		assert_eq!(23, max_path_sum(&triangle))
	}

	#[test]
	fn solution() {
		let triangle = Triangle {
			rows: vec![
				vec![75],
				vec![95, 64],
				vec![17, 47, 82],
				vec![18, 35, 87, 10],
				vec![20, 04, 82, 47, 65],
				vec![19, 01, 23, 75, 03, 34],
				vec![88, 02, 77, 73, 07, 63, 67],
				vec![99, 65, 04, 28, 06, 16, 70, 92],
				vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
				vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
				vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
				vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
				vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
				vec![63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
				vec![04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23],
			],
		};

		assert_eq!(1074, max_path_sum(&triangle))
	}
}
