use rabbit_ear as ear;
use self::ear::Line;

const BUCKET: usize = 10000;
const BUCKET_F: f64 = 10000.0;

// fn binary_search (&Vec<Line>

pub struct LineContainer {
	pub buckets: Vec<Vec<(Line, u64)>>
}

fn point_to_index (line: &Line) -> usize {
	let mut i: usize = (line.d.abs() /2_f64.sqrt() * BUCKET_F).floor() as usize;
	// println!("p_2_ln {} {}", i, line.d);
	if i >= BUCKET { i = 9999 }
	return i;
}

pub fn make_line_container () -> LineContainer {
	let mut buckets: Vec<Vec<(Line, u64)>> = Vec::new();
	for _i in 0..BUCKET {
		let row: Vec<(Line, u64)> = Vec::new();
		buckets.push(row);
	}
	return LineContainer { buckets };
}

// fn duplicate_line_check (line: &Line, lines: &mut Vec<(Line, u64)>) -> bool {
// 	for k in 0..lines.len() {
// 		if line.equivalent(lines[k].0) {
// 			lines[k].1 += 1;
// 			return true;
// 		}
// 	}
// 	return false;
// }

impl LineContainer {
	pub fn push (&mut self, line: &Line) {
		let idx = point_to_index(line);
		self.buckets[idx].push((*line, 1));
	}
	// return true if match found. false if no match
	pub fn increment_match (&mut self, line: &Line) -> bool {
		let idx = point_to_index(line);
		let bucket = &mut self.buckets[idx];
		for i in 0..bucket.len() {
			if line.equivalent(bucket[i].0) {
				bucket[i].1 += 1;
				return true;
			}
		}
		return false;
	}
	pub fn merge (&mut self, t: &mut LineContainer) {
		for i in 0..self.buckets.len() {
			self.buckets[i].append(&mut t.buckets[i]);
		}
	}
	pub fn flatten (&self) -> Vec<(Line, u64)> {
		let mut list: Vec<(Line, u64)> = Vec::new();
		for i in 0..self.buckets.len() {
			for j in 0..self.buckets[i].len() {
				list.push(self.buckets[i][j]);
			}
		}
		return list;
	}
	// pub fn flatten_filter (&self, count: u64) -> Vec<(Line, u64)> {
	// 	let mut list: Vec<(Line, u64)> = Vec::new();
	// 	for i in 0..self.buckets.len() {
	// 		for j in 0..self.buckets[i].len() {
	// 			if self.buckets[i][j].1 >= count {
	// 				list.push(self.buckets[i][j]);
	// 			}
	// 		}
	// 	}
	// 	return list;
	// }
	pub fn len (&self) -> usize {
		let mut count: usize = 0;
		for i in 0..self.buckets.len() {
			count += self.buckets[i].len();
		}
		return count;
	}
	// pub fn filter_by_count (&self, count: u64) -> GridVec {
	// 	let mut tree: GridVec = make_tree();
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

