use Vector;
use std::fmt;

pub struct Segment {
	pub a: Vector,
	pub b: Vector
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
