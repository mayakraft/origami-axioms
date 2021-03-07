use std::fmt;

const EPSILON: f64 = 0.00000001;

struct Vector {
	x: f64,
	y: f64
}

impl fmt::Debug for Vector {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Vector")
			.field("x", &self.x)
			.field("y", &self.y)
			.finish()
	}
}

impl Vector {
	/// @param {number[]} one vector, n-dimensions
	/// @returns {number} one scalar
	fn magnitude (&self) -> f64 {
		(self.x * self.x + self.y * self.y).sqrt()
	}
	/// @param {number[]} one vector, n-dimensions
	/// @returns {number} one scalar
	// fn magnitude_squared (&self) -> f64 {
	// 	self.x * self.x + self.y * self.y
	// }
	/// @param {number[]} one vector, n-dimensions
	/// @returns {number[]} one vector
	fn normalize (&self) -> Vector {
	  let mut m = self.magnitude();
	  if m < EPSILON { m = 1.0; }
	  return Vector { x: self.x / m, y: self.y / m };
	}
	/// @param {number[]} one vector, n-dimensions
	/// @param {number} one scalar
	/// @returns {number[]} one vector
	// fn scale (&self, t: f64) -> Vector {
	// 	Vector { x: self.x * t, y: self.y * t }
	// }
	///
	/// @param {number[]} one vector, n-dimensions
	/// @param {number[]} one vector, n-dimensions
	/// @returns {number[]} one vector, dimension matching first parameter
	///
	fn add (&self, u: &Vector) -> Vector {
		Vector { x: self.x + u.x, y: self.y + u.y }
	}
	///
	/// @param {number[]} one vector, n-dimensions
	/// @param {number[]} one vector, n-dimensions
	/// @returns {number[]} one vector, dimension matching first parameter
	///
	fn subtract (&self, u: &Vector) -> Vector {
		Vector { x: self.x - u.x, y: self.y - u.y }
	}
	/// @param {number[]} one vector, n-dimensions
	/// @param {number[]} one vector, n-dimensions
	/// @returns {number} one scalar
	fn dot (&self, u: &Vector) -> f64 {
		self.x * u.x + self.y * u.y
	}
	/// @param {number[]} one vector, n-dimensions
	/// @param {number[]} one vector, n-dimensions
	/// @returns {number} one vector, dimension matching first parameter
	// fn midpoint (&self, u: &Vector) -> Vector {
	// 	Vector { x: (self.x + u.x) / 2.0, y: (self.y + u.y) / 2.0 }
	// }
	/// @param {number[]} one vector, n-dimensions
	/// @param {number[]} one vector, n-dimensions
	/// @param {number} scalar between 0 and 1
	/// @returns {number[]} one vector, dimensions matching first parameter
	// fn lerp (&self, u: &Vector, t: f64) -> Vector {
	// 	let s = 1.0 - t;
	// 	return Vector { x: self.x * s + u.x * t, y: self.y * s + u.y * t };
	// }
	/// @description technically cross product in 2D is undefined,
	///  this returns the determinant of the matrix of the 2 vectors
	/// @param {number[]} one 2D vector
	/// @param {number[]} one 2D vector
	/// @returns {number} one scalar; the determinant; the magnitude of the vector
	fn determinant (&self, u: &Vector) -> f64 {
		self.x * u.y - self.y * u.x
	}
	/// @param {number[]} one 2D vector
	/// @param {number[]} one 2D vector
	/// @returns {number[]} one 2D vector
	// fn distance (&self, u: &Vector) -> f64 {
	// 	Vector { x: self.x - u.x, y: self.y - u.y }.magnitude()
	// }
	/// @param {number[]} one vector, n-dimensions
	/// @returns {number[]} one vector
	fn flip (&self) -> Vector {
		Vector { x: -self.x, y: -self.y }
	}
	/// @param {number[]} one 2D vector
	/// @returns {number[]} one 2D vector, counter-clockwise rotation
	fn rotate90 (&self) -> Vector {
		Vector { x: -self.y, y: self.x }
	}
	/// @param {number[]} one 2D vector
	/// @returns {number[]} one 2D vector, counter-clockwise rotation
	fn rotate270 (&self) -> Vector {
		Vector { x: self.y, y: -self.x }
	}
	/// @param {number[]} one vector, n-dimensions
	/// @returns boolean
	fn is_degenerate (&self) -> bool {
		(self.x.abs() + self.y.abs()) < EPSILON
	}

	// todo: should we use cross product to determine parallel?

	/// @param {number[]} one vector, n-dimensions
	/// @param {number[]} one vector, n-dimensions
	/// @returns boolean
	fn is_parallel (&self, u: &Vector) -> bool {
		(1.0 - self.normalize().dot(&u.normalize()).abs()) < EPSILON
	}
}

struct Line {
	u: Vector,
	d: f64
}

impl fmt::Debug for Line {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Line")
			.field("x", &self.u.x)
			.field("y", &self.u.y)
			.field("d", &self.d)
			.finish()
	}
}

fn axiom1 (a: &Vector, b: &Vector) -> Line {
	let u: Vector = b.subtract(a).rotate90().normalize();
	let d: f64 = a.add(b).dot(&u) / 2.0;
	return Line { u: u, d: d };
}

fn axiom2 (a: &Vector, b: &Vector) -> Line {
	let u: Vector = b.subtract(a).normalize();
	let d: f64 = a.add(b).dot(&u) / 2.0;
	return Line { u: u, d: d };
}

fn axiom3 (a: &Line, b: &Line) -> (Line, Line) {
	let l: Line = Line { u: Vector { x: 1.0, y: 0.0 }, d: a.d };
	let m: Line = Line { u: Vector { x: 0.0, y: 1.0 }, d: b.d };
	return (l, m);
}

fn vector_tests () {
	let v: Vector = Vector { x: 1.2, y: -0.8 };
	let u: Vector = Vector { x: 2.0, y: 2.0 };
	let mag1: f64 = u.magnitude();
	let mag2: f64 = u.normalize().magnitude();
	let norm: Vector = u.normalize();
	let rot90: Vector = u.normalize().rotate90();
	let rot270: Vector = u.normalize().rotate270();
	let flip: Vector = v.flip();
	let dot: f64 = u.dot(&v);
	let determ: f64 = v.determinant(&u);
	let degenerate: bool = u.is_degenerate();
	let parallel: bool = u.is_parallel(&v);
	println!("magnitude {}", mag1);
	println!("magnitude {}", mag2);
	println!("normalized {} {}", norm.x, norm.y);
	println!("rotate 90 {} {}", rot90.x, rot90.y);
	println!("rotate 270 {} {}", rot270.x, rot270.y);
	println!("flip {} {}", flip.x, flip.y);
	println!("dot {}", dot);
	println!("determinant {}", determ);
	println!("degenerate {}", degenerate);
	println!("parallel {}", parallel);
}

fn axiom_tests () {
	let u: &Vector = &Vector { x: 2.0, y: 2.0 };
	let v: &Vector = &Vector { x: 1.2, y: -0.8 };
	let l: &Line = &Line { u: Vector { x: 1.0, y: 0.0 }, d: 1.0 };
	let m: &Line = &Line { u: Vector { x: 0.0, y: 1.0 }, d: 1.0 };
	let ax1 = axiom1(u, v);
	let ax2 = axiom2(u, v);
	let (ax3a, ax3b) = axiom3(l, m);
	println!("axiom 1 ({}, {}) {}", ax1.u.x, ax1.u.y, ax1.d);
	println!("axiom 2 ({}, {}) {}", ax2.u.x, ax2.u.y, ax2.d);
	println!("axiom 3a ({}, {}) {}", ax3a.u.x, ax3a.u.y, ax3a.d);
	println!("axiom 3b ({}, {}) {}", ax3b.u.x, ax3b.u.y, ax3b.d);
}

fn find_points () {
	let mut points = Vec::new();
	points.push(Vector { x: 0.0, y: 0.0 });
	points.push(Vector { x: 1.0, y: 0.0 });
	points.push(Vector { x: 1.0, y: 1.0 });
	points.push(Vector { x: 0.0, y: 1.0 });
	let mut round = Vec::new();
	for i in 0..points.len() - 1 {
		for j in (i + 1)..points.len() {
			round.push(axiom2(&points[i], &points[j]));
		}
	}
	println!("{:?}", round);
}

fn main () {
	vector_tests();
	axiom_tests();
	find_points();
}
