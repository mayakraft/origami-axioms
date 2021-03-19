use std::fmt;
// use std::iter::FromIterator;

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

#[derive(Clone)]
pub struct Rect {
	pub sides: Vec<Line>
}

// prefer to use this constructor when making a rect, or at least,
// it's very important that the "sides" are Lines with normals
// that point outwards. needed for the "contains" method.
pub fn make_square () -> Rect {
	Rect {
		sides: vec![
			Line { u: Vector { x: 0.0 , y: 1.0 }, d: 0.0 },
			Line { u: Vector { x: 1.0 , y: 0.0 }, d: 1.0 },
			Line { u: Vector { x: 0.0 , y: 1.0 }, d: 1.0 },
			Line { u: Vector { x: 1.0 , y: 0.0 }, d: 0.0 }
		]
	}
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
		self.x - u.x < EPSILON && self.x - u.x > -EPSILON &&
		self.y - u.y < EPSILON && self.y - u.y > -EPSILON
	}
	pub fn is_parallel (&self, u: &Vector) -> bool {
		(1.0 - self.normalize().dot(&u.normalize()).abs()) < EPSILON
	}
	pub fn midpoint (&self, u: &Vector) -> Vector {
		Vector { x: (self.x + u.x) / 2.0, y: (self.y + u.y) / 2.0 }
	}
	pub fn distance_to (&self, u: &Vector) -> f64 {
		Vector { x: self.x - u.x, y: self.y - u.y }.magnitude()
	}
	// fn lerp (&self, u: &Vector, t: f64) -> Vector {
	// 	let s = 1.0 - t;
	// 	return Vector { x: self.x * s + u.x * t, y: self.y * s + u.y * t };
	// }
}

impl Line {
	pub fn intersect (&self, l: &Line) -> (bool, Vector) {
		let det = self.u.determinant(&l.u);
		if det.abs() < EPSILON {
			return (false, Vector { x: 0.0, y: 0.0 });
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
    pub fn reflectVector (&self, p: &Vector) -> Vector {
        let v1 = self.u.scale(self.d);
        let rot90 = self.u.rotate90();
        let v2 = rot90.scale(p.dot(&rot90));
        let projection = v1.add(&v2);
        return projection.add(&projection.subtract(&p));
    }
    pub fn reflectSegment (&self, s: &Segment) -> Segment {
        Segment {
            a: self.reflectVector(&s.a),
            b: self.reflectVector(&s.b)
        }
    }
}

impl Segment {
    // given we already know these two segments are collinear
    // check if they also overlap
    pub fn quick_overlap (&self, b: &Segment) -> bool {
        // seg a
        let vec_a = self.b.subtract(&self.a);
        let mag_a = vec_a.magnitude();
        let norm_a = vec_a.normalize();
        // seg b
        let vec_b = b.b.subtract(&b.a);
        let mag_b = vec_b.magnitude();
        let norm_b = vec_b.normalize();
        // project the other other segment's points onto its vector
        let aa = norm_b.dot(&self.a.subtract(&b.a));
        let ab = norm_b.dot(&self.b.subtract(&b.a));
        let ba = norm_a.dot(&b.a.subtract(&self.a));
        let bb = norm_a.dot(&b.b.subtract(&self.a));
        let t1 = aa >= 0.0 && aa <= mag_b;
        let t2 = ab >= 0.0 && ab <= mag_b;
        let t3 = ba >= 0.0 && ba <= mag_a;
        let t4 = bb >= 0.0 && bb <= mag_a;
        return t1 || t2 || t3 || t4;
    }
}

impl Rect {
	// todo: this is currently hard-coded to a unit square
	pub fn contains (&self, p: &Vector) -> bool {
		p.x >= 0.0 && p.x <= 1.0 &&
		p.y >= 0.0 && p.y <= 1.0
	}
	pub fn clip (&self, l: &Line) -> (bool, Segment) {
		// intersect line with every side, include inside the tuple
		// whether the intersection point is inside the polygon
		let results: Vec<Vector> = self.sides.iter()
			.map(|line| line.intersect(&l))
			.map(|el| (el.0, self.contains(&el.1), el.1))
			.filter(|el| el.0 && el.1)
			.map(|el| el.2)
			.collect();
		if results.len() < 2 {
			return (false,
				Segment {a:Vector {x:0.0, y:0.0}, b:Vector {x:0.0, y:0.0}});
		}
		// sort along line
		let origin = l.u.scale(l.d);
		let vector = l.u.rotate90();
		let ts: Vec<f64> = results.iter()
			.map(|pt| pt.subtract(&origin).dot(&vector))
			.collect();
		let smallest = *ts.iter().fold(&ts[0], |a, b| if b < a {b} else {a});
		let largest = *ts.iter().fold(&ts[0], |a, b| if b > a {b} else {a});
        // if the two points are the same the segment is degenerate
        if (smallest - largest).abs() < EPSILON {
			return (false,
				Segment {a:Vector {x:0.0, y:0.0}, b:Vector {x:0.0, y:0.0}});
        }
		return (true, Segment {
			a: origin.add(&vector.scale(smallest)),
			b: origin.add(&vector.scale(largest))
		});
	}
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
