use math::Vector;

// hardcode the dimensions of the quad
const QUAD_WIDTH: f32 = 1.0;
const QUAD_HEIGHT: f32 = 1.0;

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
	pub quadrants: Vec<&QuadTree>,
	pub points: Vec<(&Vector, u16)>,
	// pub point: &Vector,
	pub level: u16,
	pub top: f32,
	pub left: f32
}

pub fn make_tree (level: u16, top: f32, left: f32) -> QuadTree {
	return QuadTree { quadrants: vec![], points: vec![], level, top, left };
}

fn dimensions (level: u16) -> (f32, f32) {
	let denom = 2.0_f32.powf(level as f32);
	return (QUAD_WIDTH / denom, QUAD_HEIGHT / denom);
}

// this makes the equal to the axis line on the + side
fn which_quad (point: &Vector, x_axis: f32, y_axis: f32) -> usize {
	let x_side = if point.x < x_axis as f64 {0} else {1};
	let y_side = if point.y < y_axis as f64 {0} else {2};
	return x_side + y_side;
}

fn split (node: &QuadTree) {
	// this is the next level's w and h
	let (w, h) = dimensions(node.level + 1);
	let tops = [node.top, node.top, node.top + h, node.top + h];
	let lefts = [node.left, node.left + w, node.left, node.left + w];
	for i in 0..4 {
		node.quadrants.push(&make_tree(node.level + 1, tops[i], lefts[i]));
	}
	for i in 0..node.points.len() {
		node.push(node.points[0]);
	}
	node.points.clear();
}
fn find_leaf (node: &QuadTree, point: &Vector) -> &QuadTree {
	if node.quadrants.len() == 0 { return node }
	// this is the next level's w and h
	let (w, h) = dimensions(node.level + 1);
	let x_axis = node.top + h;
	let y_axis = node.left + w;
	let index = which_quad(point, x_axis, y_axis);
	return find_leaf(node.quadrants[index], point);
}
// if match found, returns true AND increments matched point counter
fn find_point_match (points: &Vec<(&Vector, u16)>, point: &Vector) -> bool {
	for i in 0..points.len() {
		if point.equivalent(points[i].0) {
			points[i].1 += 1;
			return true;
		}
	}
	return false;
}

impl QuadTree {
	// pub fn push (&mut self, point: &Vector) {
	pub fn push (&mut self, point: (&Vector, u16)) {
		let leaf = find_leaf(self, point.0);
		if !find_point_match(&leaf.points, point.0) {
			leaf.points.push(point);
		}
		if leaf.quadrants.len() >= 4 { split(leaf); }
	}
}
