mod vector;
mod line;
mod segment;
mod square;
mod axioms;
use vector::Vector;
use line::Line;
use segment::Segment;
use square::Square;

fn vector_tests () {
	let sqrt2 = (2.0_f64).sqrt();
	let v: Vector = Vector { x: 1.2, y: -0.8 };
	let u: Vector = Vector { x: 2.0, y: 2.0 };
	let l = Line { u: Vector { x: 1.0, y: 0.0 }, d: 1.0 };
	let m = Line { u: Vector { x: -sqrt2, y: sqrt2 }, d: 1.0 };
	// let m = Line { u: Vector { x: 0.0, y: 1.0 }, d: 1.0 };
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
	let (success, intersect) = l.intersect(&m);
	let equivalent: bool = u.equivalent(&v);
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
	println!("equivalent {}", equivalent);
	println!("intersect {} {} {}", success, intersect.x, intersect.y);
}

fn line_tests () {
	let a = Line {
		u: Vector { x: 0.7071067811865475, y: 0.7071067811865475},
		d: 0.7071067811865475
	};
	let b = Line {
		u: Vector { x: 1.0, y: 0.0},
		d: 0.5
	};
	// let equivalent: bool = a.equivalent(&b);
	let equivalent: bool = b.equivalent(&a);
	println!("are these equivalent? {}", equivalent);

	// make sure these should be duplicate
	// test if they are duplicate
	// duplicate test Line { x: -1.0, y: 0.0, d: -0.5 } Line { x: 1.0, y: 0.0, d: 0.5 }

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

// fn compute_axiom1 (
// 	points: &mut Vec<(Vector, u64)>,
// 	lines: &mut Vec<(Line, u64)>,
// ) -> Vec<Line> {
// 	let mut round: Vec<(Line, u64)> = Vec::new();
// 	for i in 0..points.len() - 1 {
// 		for j in (i + 1)..points.len() {
// 			let line = axioms::axiom1(&points[i], &points[j]);
// 			// #1 check line for duplicate against printed lines
// 			//  - if t: increment line:count in printed line
// 			//  - if f: test #2
// 			// #2 check line for duplicate against current round
// 			//  - if t: increment line:count in current line
// 			//  - if f: add new entry to current round. with 0
// 			round.push((line, 0));
// 		}
// 	}
// 	return round;
// }

fn compute_axiom2 (
	points: &mut Vec<(Vector, u64)>,
	lines: &mut Vec<(Line, u64)>
) -> Vec<(Line, u64)> {
	let mut round: Vec<(Line, u64)> = Vec::new();
	for i in 0..points.len() - 1 {
		for j in (i + 1)..points.len() {
			// println!("{} {}", i, j);
			let line: Line = axioms::axiom2(&points[i].0, &points[j].0);
			let mut duplicate = false;
			// #1 check line for duplicate against printed lines
			//  - if t: increment line:count in printed line
			//  - if f: test #2
			for k in 0..lines.len() {
				if lines[k].0.equivalent(&line) {
					lines[k].1 += 1;
					duplicate = true;
					break;
				}
			}
			if duplicate { continue }
			// #2 check line for duplicate against current round
			//  - if t: increment line:count in current line
			//  - if f: add new entry to current round. with 0
			for k in 0..round.len() {
				if round[k].0.equivalent(&line) {
					round[k].1 += 1;
					duplicate = true;
					break;
				}
			}
			if duplicate { continue }
			round.push((line, 1));
		}
	}
	return round;
}

// fn compute_axiom3 (
// 	points: &mut Vec<(Vector, u64)>,
// 	lines: &mut Vec<(Line, u64)>,
// ) -> Vec<Line> {
// 	let mut round: Vec<(Line, u64)> = Vec::new();
// 	for i in 0..lines.len() - 1 {
// 		for j in (i + 1)..lines.len() {
// 			let line = axioms::axiom1(&points[i], &points[j]);
// 			// #1 check line for duplicate against printed lines
// 			//  - if t: increment line:count in printed line
// 			//  - if f: test #2
// 			// #2 check line for duplicate against current round
// 			//  - if t: increment line:count in current line
// 			//  - if f: add new entry to current round. with 0
// 			round.push((line, 0));
// 		}
// 	}
// 	return round;
// }

fn main () {
	vector_tests();
	axiom_tests();
	line_tests();
	let mut points: Vec<(Vector, u64)> = Vec::new();
	let mut lines: Vec<(Line, u64)> = Vec::new();
	points.push((Vector { x: 0.0, y: 0.0 }, 1));
	points.push((Vector { x: 1.0, y: 0.0 }, 1));
	points.push((Vector { x: 1.0, y: 1.0 }, 1));
	points.push((Vector { x: 0.0, y: 1.0 }, 1));
	let mut round1_lines = compute_axiom2(&mut points, &mut lines);

	// for i in 0..4 {
	// 	println!("{}", i);
	// }

	// after all axioms are computed this round
	// 1. compute new intersection points
	// 2. merge lines from last round
	let mut round1_points: Vec<(Vector, u64)> = Vec::new();

	for i in 0..round1_lines.len() - 1 {
		for j in (i + 1)..round1_lines.len() {
			let (success, point) = round1_lines[i].0.intersect(&round1_lines[j].0);
			if !success { continue }
			let mut duplicate = false;
			for k in 0..points.len() {
				if points[k].0.equivalent(&point) {
					points[k].1 += 1;
					duplicate = true;
					break;
				}
			}
			if duplicate { continue }
			for k in 0..round1_points.len() {
				if round1_points[k].0.equivalent(&point) {
					round1_points[k].1 += 1;
					duplicate = true;
					break;
				}
			}
			if duplicate { continue }
			round1_points.push((point, 1));
		}
	}

	// merge points and lines from last round
	points.append(&mut round1_points);
	lines.append(&mut round1_lines);

	println!("{:?}", lines);
	let mut round2_lines = compute_axiom2(&mut points, &mut lines);
	lines.append(&mut round2_lines);
	println!("{:?}", lines);


	let unit_square: Square = Square {
		a: Line { u: Vector { x: 0.0 , y: 1.0 }, d: 0.0 },
		b: Line { u: Vector { x: 1.0 , y: 0.0 }, d: 1.0 },
		c: Line { u: Vector { x: 0.0 , y: 1.0 }, d: 1.0 },
		d: Line { u: Vector { x: 1.0 , y: 0.0 }, d: 0.0 }
	};


	// let mut segments: Vec<Segment> = Vec::new();
	// for i in 0..lines.len() {
	// 	let (success, segment) = unit_square.clip(&lines[i].0);
	// 	if success { segments.push(segment) }
	// }
	// println!("{:?}", segments);	
}
