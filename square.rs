use Vector;
use Line;
use Segment;

const EPSILON: f64 = 0.00000001;

#[derive(Copy, Clone)]
pub struct Square {
	pub a: Line,
	pub b: Line,
	pub c: Line,
	pub d: Line
}

// todo: this is currently hard-coded to a unit square
impl Square {
	pub fn contains (&self, p: &Vector) -> bool {
		p.x >= 0.0 && p.x <= 1.0 &&
		p.y >= 0.0 && p.y <= 1.0
	}
	pub fn clip (&self, l: &Line) -> (bool, Segment) {
		let mut intersects: [(bool, Vector); 4] = [
			self.a.intersect(&l),
			self.b.intersect(&l),
			self.c.intersect(&l),
			self.d.intersect(&l)
		];
		for i in 0..intersects.len() {
			intersects[i].0 = intersects[i].0 && self.contains(&intersects[i].1);
		}
		// filter out results
		let mut results: Vec<Vector> = Vec::new();
		for i in 0..intersects.len() {
			if intersects[i].0 {
				// results.push(intersects[i].1);
				results.push(Vector {
					x: intersects[i].1.x,
					y: intersects[i].1.y
				});
			}
		}
		if results.len() < 2 {
			return (false,
				Segment {a:Vector {x:0.0, y:0.0}, b:Vector {x:0.0, y:0.0}});
		}
		// sort along line
		let origin = l.u.scale(l.d);
		let vector = l.u.rotate90();
		let mut ts: Vec<f64> = Vec::new();
		for i in 0..results.len() {
			ts.push(results[i].subtract(&origin).dot(&vector));
		}
		let mut smallest = ts[0];
		let mut largest = ts[0];
		for i in 1..ts.len() {
			if ts[i] < smallest { smallest = ts[i] }
			if ts[i] > largest { largest = ts[i] }
		}
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

