use std::fmt;

const EPSILON: f64 = 0.00000001;

pub struct Vector {
	pub x: f64,
	pub y: f64
}

impl Vector {
	/// @param {number[]} one vector, n-dimensions
	/// @returns {number} one scalar
	pub fn magnitude (&self) -> f64 {
		(self.x * self.x + self.y * self.y).sqrt()
	}
	/// @param {number[]} one vector, n-dimensions
	/// @returns {number} one scalar
	// fn magnitude_squared (&self) -> f64 {
	// 	self.x * self.x + self.y * self.y
	// }
	/// @param {number[]} one vector, n-dimensions
	/// @returns {number[]} one vector
	pub fn normalize (&self) -> Vector {
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
	pub fn add (&self, u: &Vector) -> Vector {
		Vector { x: self.x + u.x, y: self.y + u.y }
	}
	///
	/// @param {number[]} one vector, n-dimensions
	/// @param {number[]} one vector, n-dimensions
	/// @returns {number[]} one vector, dimension matching first parameter
	///
	pub fn subtract (&self, u: &Vector) -> Vector {
		Vector { x: self.x - u.x, y: self.y - u.y }
	}
	/// @param {number[]} one vector, n-dimensions
	/// @param {number[]} one vector, n-dimensions
	/// @returns {number} one scalar
	pub fn dot (&self, u: &Vector) -> f64 {
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
	pub fn determinant (&self, u: &Vector) -> f64 {
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
	pub fn flip (&self) -> Vector {
		Vector { x: -self.x, y: -self.y }
	}
	/// @param {number[]} one 2D vector
	/// @returns {number[]} one 2D vector, counter-clockwise rotation
	pub fn rotate90 (&self) -> Vector {
		Vector { x: -self.y, y: self.x }
	}
	/// @param {number[]} one 2D vector
	/// @returns {number[]} one 2D vector, counter-clockwise rotation
	pub fn rotate270 (&self) -> Vector {
		Vector { x: self.y, y: -self.x }
	}
	/// @param {number[]} one vector, n-dimensions
	/// @returns boolean
	pub fn is_degenerate (&self) -> bool {
		(self.x.abs() + self.y.abs()) < EPSILON
	}

	// todo: should we use cross product to determine parallel?

	/// @param {number[]} one vector, n-dimensions
	/// @param {number[]} one vector, n-dimensions
	/// @returns boolean
	pub fn is_parallel (&self, u: &Vector) -> bool {
		(1.0 - self.normalize().dot(&u.normalize()).abs()) < EPSILON
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
