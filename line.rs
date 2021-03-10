use Vector;
use std::fmt;

const EPSILON: f64 = 0.00000001;

#[derive(Copy, Clone)]
pub struct Line {
	pub u: Vector,
	pub d: f64
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
	// implicit is this clips inside the unit square x/y: (0..1)
	// pub fn clip (&self) -> (bool, Segment) {

	// }
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

impl fmt::Debug for Line {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Line")
			.field("x", &self.u.x)
			.field("y", &self.u.y)
			.field("d", &self.d)
			.finish()
	}
}
