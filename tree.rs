use Vector;

const BUCKET: usize = 100;
const BUCKET_F: f64 = 100.0;

// #[derive(Copy, Clone)]
pub struct QuadTree {
	pub buckets: Vec<Vec<Vec<(Vector, u64)>>>
}

fn point_to_index (point: &Vector) -> (usize, usize) {
	let mut i: usize = (point.x * BUCKET_F).floor() as usize;
	if i == BUCKET { i = 99 }
	let mut j: usize = (point.y * BUCKET_F).floor() as usize;
	if j == BUCKET { j = 99 }
	return (i, j);
}

pub fn make_tree () -> QuadTree {
	let mut buckets: Vec<Vec<Vec<(Vector, u64)>>> = Vec::new();
	for i in 0..BUCKET {
		let row: Vec<Vec<(Vector, u64)>> = Vec::new();
		buckets.push(row);
		for _j in 0..BUCKET {
			let col: Vec<(Vector, u64)> = Vec::new();
			buckets[i].push(col);
		}
	}
	return QuadTree { buckets };
}

impl QuadTree {
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
	pub fn merge (&mut self, t: &mut QuadTree) {
		for i in 0..self.buckets.len() {
			for j in 0..self.buckets[i].len() {
				self.buckets[i][j].append(&mut t.buckets[i][j]);
			}
		}
	}
	pub fn flatten (&self) -> Vec<&Vector> {
		let mut list: Vec<&Vector> = Vec::new();
		for i in 0..self.buckets.len() {
			for j in 0..self.buckets[i].len() {
				for k in 0..self.buckets[i][j].len() {
					list.push(&self.buckets[i][j][k].0);
				}
			}
		}
		return list;
	}
	pub fn len (&self) -> usize {
		let mut count: usize = 0;
		for i in 0..self.buckets.len() {
			for j in 0..self.buckets[i].len() {
				count += self.buckets[i][j].len();
			}
		}
		return count;
	}
    pub fn filter_by_count (&self, count: u64) -> QuadTree {
        let mut tree: QuadTree = make_tree();
		for i in 0..self.buckets.len() {
			for j in 0..self.buckets[i].len() {
				for k in 0..self.buckets[i][j].len() {
                    if self.buckets[i][j][k].1 >= count {
                        tree.buckets[i][j].push(self.buckets[i][j][k]);
                    }
				}
			}
		}
        return tree;
    }
}

