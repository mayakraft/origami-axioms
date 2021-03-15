use Vector;
use Line;
use Segment;

const EPSILON: f64 = 1.0e-8;

// broadly, this "Square" is actually an axis-aligned polygon. can be any rect.
#[derive(Clone)]
pub struct Square {
	pub sides: Vec<Line>
}

impl Square {
	// todo: this is currently hard-coded to a unit square
	pub fn contains (&self, p: &Vector) -> bool {
		p.x >= 0.0 && p.x <= 1.0 &&
		p.y >= 0.0 && p.y <= 1.0
	}
	pub fn clip (&self, l: &Line) -> (bool, Segment) {
		// intersect line with every side, include inside the tuple
		// whether the intersection point is inside the polygon
		let results: Vec<(Vector)> = self.sides.iter()
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
		return (true, Segment {
			a: origin.add(&vector.scale(smallest)),
			b: origin.add(&vector.scale(largest))
		});
	}
}

