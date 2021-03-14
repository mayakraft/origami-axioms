use std::fmt;

const EPSILON: f64 = 1.0e-8;

#[derive(Copy, Clone)]
pub struct Vector {
	pub x: f64,
	pub y: f64
}

#[derive(Copy, Clone)]
pub struct Line {
	pub u: Vector,
	pub d: f64
}

#[derive(Copy, Clone)]
pub struct Segment {
	pub a: Vector,
	pub b: Vector
}

impl Vector {
	pub fn magnitude (&self) -> f64 { (self.x * self.x + self.y * self.y).sqrt() }
	pub fn magnitude_squared (&self) -> f64 { self.x * self.x + self.y * self.y }
	pub fn normalize (&self) -> Vector {
		let mut m = self.magnitude();
		if m < EPSILON { m = 1.0; }
		return Vector { x: self.x / m, y: self.y / m };
	}
	pub fn scale (&self, t: f64) -> Vector {
		Vector { x: self.x * t, y: self.y * t }
	}
	pub fn add (&self, u: &Vector) -> Vector {
		Vector { x: self.x + u.x, y: self.y + u.y }
	}
	pub fn subtract (&self, u: &Vector) -> Vector {
		Vector { x: self.x - u.x, y: self.y - u.y }
	}
	pub fn dot (&self, u: &Vector) -> f64 { self.x * u.x + self.y * u.y }
	pub fn determinant (&self, u: &Vector) -> f64 { self.x * u.y - self.y * u.x }
	pub fn flip (&self) -> Vector { Vector { x: -self.x, y: -self.y } }
	pub fn rotate90 (&self) -> Vector { Vector { x: -self.y, y: self.x } }
	pub fn rotate270 (&self) -> Vector { Vector { x: self.y, y: -self.x } }
	pub fn is_degenerate (&self) -> bool { (self.x.abs() + self.y.abs()) < EPSILON }
	pub fn equivalent (&self, u: &Vector) -> bool {
		self.x - u.x < EPSILON &&
		self.x - u.x > -EPSILON &&
		self.y - u.y < EPSILON &&
		self.y - u.y > -EPSILON
	}
	pub fn is_parallel (&self, u: &Vector) -> bool {
		(1.0 - self.normalize().dot(&u.normalize()).abs()) < EPSILON
	}
    // pub fn midpoint (&self, u: &Vector) -> Vector {
    //     self.add(u).scale(0.5)
    // }
	pub fn midpoint (&self, u: &Vector) -> Vector {
		Vector { x: (self.x + u.x) / 2.0, y: (self.y + u.y) / 2.0 }
	}
	// fn lerp (&self, u: &Vector, t: f64) -> Vector {
	// 	let s = 1.0 - t;
	// 	return Vector { x: self.x * s + u.x * t, y: self.y * s + u.y * t };
	// }
	pub fn distance_to (&self, u: &Vector) -> f64 {
		Vector { x: self.x - u.x, y: self.y - u.y }.magnitude()
	}
}

impl Line {
	pub fn intersect (&self, l: &Line) -> (bool, Vector) {
		let det = self.u.determinant(&l.u);
		if det.abs() < EPSILON {
			return (false, Vector { x: 0.0, y: 0.0 })
		}
		let x = self.d * &l.u.y - l.d * self.u.y;
		let y = l.d * self.u.x - self.d * &l.u.x;
		return (true, Vector { x: x / det, y: y / det });
	}
	pub fn equivalent (&self, l: &Line) -> bool {
		// check if lines are parallel
		(self.u.dot(&l.u.rotate90()).abs() < EPSILON) &&
		// instead of simply comparing the .d values,
		// scale the incoming by the dot prod of both .u normals
		// this allows (1,0) and (-1,0) to be treated the same
		((self.d - l.d * self.u.dot(&l.u)).abs() < EPSILON)
	}
// 	fn bisect_lines2 (&self, &l: Line) -> (Line, Line) {
// 		const determinant = cross2(vectorA, vectorB);
// 		const dotProd = dot(vectorA, vectorB);
// 		const bisects = determinant > -epsilon
// 			? [counter_clockwise_bisect2(vectorA, vectorB)]
// 			: [clockwise_bisect2(vectorA, vectorB)];
// 		bisects[1] = determinant > -epsilon
// 			? rotate90(bisects[0])
// 			: rotate270(bisects[0]);
// 		const numerator = (originB[0] - originA[0]) * vectorB[1] - vectorB[0] * (originB[1] - originA[1]);
// 		const t = numerator / determinant;
// 		const normalized = [vectorA, vectorB].map(vec => normalize(vec));
// 		const isParallel = Math.abs(cross2(...normalized)) < epsilon;
// 		const origin = isParallel
// 			? midpoint(originA, originB)
// 			: [originA[0] + vectorA[0] * t, originA[1] + vectorA[1] * t];
// 		const solution = bisects.map(vector => ({ vector, origin }));
// 		if (isParallel) { delete solution[(dotProd > -epsilon ? 1 : 0)]; }
// 		return solution;
// 	}
}

impl fmt::Debug for Vector {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Vector")
			.field("x", &self.x)
			.field("y", &self.y)
			.finish()
	}
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

impl fmt::Debug for Segment {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Segment")
			.field("x1", &self.a.x)
			.field("y1", &self.a.y)
			.field("x2", &self.b.x)
			.field("y2", &self.b.y)
			.finish()
	}
}
