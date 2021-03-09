use Vector;
use Line;
use Segment;

const EPSILON: f64 = 0.00000001;

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
			ts[i] = results[i].subtract(&origin).dot(&vector);
		}
		let mut smallest = 0;
		let mut largest = 0;
		for i in 0..ts.len() {
			if ts[i] < ts[smallest] { smallest = i }
			if ts[i] > ts[largest] { largest = i }
		}
		if (ts[smallest] - ts[largest]).abs() < EPSILON {
			return (false,
				Segment {a:Vector {x:0.0, y:0.0}, b:Vector {x:0.0, y:0.0}});
		}
		return (true, Segment {
			a: origin.add(&vector.scale(ts[smallest])),
			b: origin.add(&vector.scale(ts[largest]))
		});
	}
}
