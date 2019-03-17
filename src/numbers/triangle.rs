pub struct TriangleIter {
	num: u64,
	i: u64,
}

impl TriangleIter {
	pub fn new() -> TriangleIter {
		TriangleIter { num: 1, i: 1 }
	}
}

impl Iterator for TriangleIter {
	type Item=u64;

	fn next(&mut self) -> Option<u64> {
		self.i += 1;
		self.num += self.i;
		Some(self.num)
	}
}
