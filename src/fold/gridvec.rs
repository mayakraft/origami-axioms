use math::Vector;

const BUCKET: usize = 1000;
const BUCKET_MAX_I: usize = 999;
const BUCKET_F: f64 = 1000.0;

// #[derive(Copy, Clone)]
pub struct GridVec {
	pub buckets: Vec<Vec<Vec<(Vector, u64)>>>
}

fn point_to_index (point: &Vector) -> (usize, usize) {
	let mut i: usize = (point.x * BUCKET_F).floor() as usize;
	if i == BUCKET { i = BUCKET_MAX_I }
	let mut j: usize = (point.y * BUCKET_F).floor() as usize;
	if j == BUCKET { j = BUCKET_MAX_I }
	return (i, j);
}

pub fn make_grid () -> GridVec {
	let mut buckets: Vec<Vec<Vec<(Vector, u64)>>> = Vec::new();
	for i in 0..BUCKET {
		let row: Vec<Vec<(Vector, u64)>> = Vec::new();
		buckets.push(row);
		for _j in 0..BUCKET {
			let col: Vec<(Vector, u64)> = Vec::new();
			buckets[i].push(col);
		}
	}
	return GridVec { buckets };
}

impl GridVec {
	pub fn push (&mut self, point: &Vector) {
		let idx = point_to_index(point);
		self.buckets[idx.0][idx.1].push((*point, 1));
	}
	// return true if match found. false if no match
	pub fn increment_match (&mut self, point: &Vector) -> bool {
		let idx = point_to_index(point);
		let bucket = &mut self.buckets[idx.0][idx.1];
		for i in 0..bucket.len() {
			if point.equivalent(&bucket[i].0) {
				bucket[i].1 += 1;
				return true;
			}
		}
		return false;
	}
	pub fn merge (&mut self, t: &mut GridVec) {
		for i in 0..self.buckets.len() {
			for j in 0..self.buckets[i].len() {
				self.buckets[i][j].append(&mut t.buckets[i][j]);
			}
		}
	}
	pub fn flatten (&self) -> Vec<&(Vector, u64)> {
		let mut list: Vec<&(Vector, u64)> = Vec::new();
		for i in 0..self.buckets.len() {
			for j in 0..self.buckets[i].len() {
				for k in 0..self.buckets[i][j].len() {
					list.push(&self.buckets[i][j][k]);
				}
			}
		}
		return list;
	}
	// pub fn flatten_filter (&self, count: u64) -> Vec<&(Vector, u64)> {
	// 	let mut list: Vec<&(Vector, u64)> = Vec::new();
	// 	for i in 0..self.buckets.len() {
	// 		for j in 0..self.buckets[i].len() {
	// 			for k in 0..self.buckets[i][j].len() {
	// 				if self.buckets[i][j][k].1 >= count {
	// 					list.push(&self.buckets[i][j][k]);
	// 				}
	// 			}
	// 		}
	// 	}
	// 	return list;
	// }
	pub fn len (&self) -> usize {
		let mut count: usize = 0;
		for i in 0..self.buckets.len() {
			for j in 0..self.buckets[i].len() {
				count += self.buckets[i][j].len();
			}
		}
		return count;
	}
	// pub fn filter_by_count (&self, count: u64) -> GridVec {
	// 	let mut tree: GridVec = make_grid();
	// 	for i in 0..self.buckets.len() {
	// 		for j in 0..self.buckets[i].len() {
	// 			for k in 0..self.buckets[i][j].len() {
	// 				if self.buckets[i][j][k].1 >= count {
	// 					tree.buckets[i][j].push(self.buckets[i][j][k]);
	// 				}
	// 			}
	// 		}
	// 	}
	// 	return tree;
	// }
}
