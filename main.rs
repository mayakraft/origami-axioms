mod vector;
mod line;
mod axioms;
use vector::Vector;
use line::Line;

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
	let ax1 = axioms::axiom1(u, v);
	let ax2 = axioms::axiom2(u, v);
	let (ax3a, ax3b) = axioms::axiom3(l, m);
	println!("axiom 1 ({}, {}) {}", ax1.u.x, ax1.u.y, ax1.d);
	println!("axiom 2 ({}, {}) {}", ax2.u.x, ax2.u.y, ax2.d);
	println!("axiom 3a ({}, {}) {}", ax3a.u.x, ax3a.u.y, ax3a.d);
	println!("axiom 3b ({}, {}) {}", ax3b.u.x, ax3b.u.y, ax3b.d);
}

fn find_points (points: &mut Vec<Vector>) -> Vec<Line> {
	let mut round = Vec::new();
	for i in 0..points.len() - 1 {
		for j in (i + 1)..points.len() {
			round.push(axioms::axiom2(&points[i], &points[j]));
		}
	}
	return round;
}

fn main () {
	vector_tests();
	axiom_tests();
	let mut points: Vec<Vector> = Vec::new();
	points.push(Vector { x: 0.0, y: 0.0 });
	points.push(Vector { x: 1.0, y: 0.0 });
	points.push(Vector { x: 1.0, y: 1.0 });
	points.push(Vector { x: 0.0, y: 1.0 });
	let round = find_points(&mut points);
	println!("{:?}", round);
}

