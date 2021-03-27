use math::Vector;

// hardcode the dimensions of the quad
const QUAD_WIDTH: f32 = 1.0;
const QUAD_HEIGHT: f32 = 1.0;

// #[derive(Copy, Clone)]
pub struct QuadTree {
	pub root: QuadTreeNode
}

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
pub struct QuadTreeNode {
	pub quadrants: Vec<&QuadTreeNode>,
	// pub ul: &QuadTreeNode,
	// pub ur: &QuadTreeNode,
	// pub br: &QuadTreeNode,
	// pub bl: &QuadTreeNode,
	pub points: Vec<&Vector>,
	// pub point: &Vector,
	pub level: u16,
	pub top: f32,
	pub left: f32
}

fn dimensions (level: u16) -> (f32, f32) {
	let denom = 2.0_f32.powf(level as f32);
	return (QUAD_WIDTH / denom, QUAD_HEIGHT / denom);
}

fn which_quad (point: &Vector, x_axis: f32, y_axis: f32) -> u8 {
	let x_side = if point.x < x_axis {0} else {1};
	let y_side = if point.y < y_axis {0} else {2};
	return x_side + y_side;
}

pub fn make_tree (level: u16, top: f32, left: f32) -> QuadTree {
	return QuadTree { quadrants: vec![], points: vec![], level, top, left };
}

impl QuadTree {
	fn split (&mut self) {
		// this is the next level's w and h
		let (w, h) = dimensions(self.level + 1);
		let tops = [self.top, self.top, self.top + h, self.top + h];
		let lefts = [self.left, self.left + w, self.left, self.left + w];
		for i in 0..4 {
			quadrants.push(make_tree(self.level + 1, tops[i], lefts[i]));
		}
		let x_axis = self.top + h;
		let y_axis = self.left + w;

	}
	fn find_leaf (&mut self, point: &Vector) {

	}
	pub fn push (&mut self, point: &Vector) {
		find_leaf(point);
		let idx = point_to_index(point);
		self.buckets[idx.0][idx.1].push((*point, 1));
	}

}
