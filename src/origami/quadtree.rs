use std::fmt;
use rabbit_ear as ear;
use self::ear::Vector;
use origami::CountPoint;
// use math::EPSILON;
const EPSILON: f64 = 1.0e-16;

// hardcode the dimensions of the quad
const QUAD_WIDTH: f64 = 1.0;
const QUAD_HEIGHT: f64 = 1.0;
const BUCKET_SIZE: usize = 1000;

// order of indices (+Y axis up, flip upside down if +Y is down)
//
// [     |     ]
// [  2  |  3  ]   ^
// [     |     ]   |
// [-----------]   y
// [     |     ]     x-->
// [  0  |  1  ]
// [     |     ]

// the quad at each level is calculated from top, left, and level
// level determines the width and height:
// 0: 1.0
// 1: 0.5
// 2: 0.25
// 3: 0.125
pub struct QuadTree {
	pub quadrants: Vec<QuadTree>,
	pub points: Vec<CountPoint>,
	pub level: u16,
	pub top: f64,
	pub left: f64
}

// this is hard-coded to a unit square with left corner at (0, 0)
pub fn make_tree () -> QuadTree { QuadTree {
	quadrants: vec![],
	points: vec![],
	level: 0, top: 0.0, left: 0.0
}}

fn make_node (level: u16, top: f64, left: f64) -> QuadTree {
	return QuadTree { quadrants: vec![], points: vec![], level, top, left };
}

fn dimensions (level: u16) -> (f64, f64) {
	let denom = 2.0_f64.powf(level as f64);
	return (QUAD_WIDTH / denom, QUAD_HEIGHT / denom);
}

// this makes the equal to the axis line on the + side
fn which_quad (point: &Vector, x_axis: f64, y_axis: f64) -> usize {
	let x_side = if point.x < x_axis as f64 {0} else {1};
	let y_side = if point.y < y_axis as f64 {0} else {2};
	return x_side + y_side;
}

fn split (node: &mut QuadTree) {
	// this is the next level's w and h
	let (w, h) = dimensions(node.level + 1);
	if w < EPSILON || h < EPSILON {
		println!("bucket dimensions below epsilon at level {}, suggest increase bucket size", node.level);
		return;
	}
	let y_axis = node.top + h;
	let x_axis = node.left + w;
	let tops = [node.top, node.top, y_axis, y_axis];
	let lefts = [node.left, x_axis, node.left, x_axis];
	for i in 0..4 {
		node.quadrants.push(make_node(node.level + 1, tops[i], lefts[i]));
	}
	// println!("pushing couted point...");
	for i in 0..node.points.len() {
		let index = which_quad(&node.points[i].0, x_axis, y_axis);
		node.quadrants[index].push_counted_point(node.points[i]);
	}
	// println!("  done pushing couted point");
	node.points.clear();
}

fn find_leaf<'a> (node: &'a mut QuadTree, point: &Vector) -> &'a mut QuadTree {
	if node.quadrants.len() == 0 { return node }
	// this is the next level's w and h
	let (w, h) = dimensions(node.level + 1);
	let x_axis = node.top + h;
	let y_axis = node.left + w;
	let index = which_quad(point, x_axis, y_axis);
	return find_leaf(&mut node.quadrants[index], point);
}

// if match found, returns true AND increments matched point counter
fn find_point_match (points: &mut Vec<CountPoint>, point: &Vector) -> bool {
	for i in 0..points.len() {
		if point.equivalent(points[i].0) {
			points[i].1 += 1;
			return true;
		}
	}
	return false;
}

fn flatten_recurse (node: &QuadTree, list: &mut Vec<CountPoint>) {
	for i in 0..node.points.len() { list.push(node.points[i]) }
	for i in 0..node.quadrants.len() {
		flatten_recurse(&node.quadrants[i], list);
	}
}

impl QuadTree {
	pub fn push_counted_point (&mut self, point: CountPoint) {
		let leaf = find_leaf(self, &point.0);
		if !find_point_match(&mut leaf.points, &point.0) {
			leaf.points.push(point);
		}
		if leaf.points.len() >= BUCKET_SIZE { split(leaf) }
	}
	// pub fn push (&mut self, point: &Vector) {
	pub fn push (&mut self, point: Vector) {
		let leaf = find_leaf(self, &point);
		if !find_point_match(&mut leaf.points, &point) {
			leaf.points.push((point, 1));
		}
		if leaf.points.len() >= BUCKET_SIZE { split(leaf) }
	}
	// if match found, returns true AND increment the matched point counter
	pub fn increment_match (&mut self, point: &Vector) -> bool {
		let leaf = find_leaf(self, &point);
		return find_point_match(&mut leaf.points, &point);
	}
	pub fn merge (&mut self, other: &QuadTree) {
		// println!("merging...");
		let list = other.flatten();
		for i in 0..list.len() {
			self.push_counted_point(list[i]);
		}
		// println!("done merging");
	}
	pub fn flatten (&self) -> Vec<CountPoint> {
		let mut list: Vec<CountPoint> = Vec::new();
		flatten_recurse(self, &mut list);
		return list;
	}
}

impl fmt::Debug for QuadTree {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("QuadTree")
			.field("level", &self.level)
			.field("(w, h)", &dimensions(self.level))
			.field("top", &self.top)
			.field("left", &self.left)
			.field("points", &self.points.len())
			.field("quadrants", &self.quadrants.len())
			.finish()
	}
}
