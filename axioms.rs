use Vector;
use Line;

pub fn axiom1 (a: &Vector, b: &Vector) -> Line {
	let u: Vector = b.subtract(a).rotate90().normalize();
	let d: f64 = a.add(b).dot(&u) / 2.0;
	return Line { u: u, d: d };
}

pub fn axiom2 (a: &Vector, b: &Vector) -> Line {
	let u: Vector = b.subtract(a).normalize();
	let d: f64 = a.add(b).dot(&u) / 2.0;
	return Line { u: u, d: d };
}

pub fn axiom3 (a: &Line, b: &Line) -> (Line, Line) {
	let l: Line = Line { u: Vector { x: 1.0, y: 0.0 }, d: a.d };
	let m: Line = Line { u: Vector { x: 0.0, y: 1.0 }, d: b.d };
	return (l, m);
}
