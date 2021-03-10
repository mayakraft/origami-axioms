mod vector;
mod line;
mod segment;
mod square;
mod axioms;
mod tests;
mod draw;
use vector::Vector;
use line::Line;
use segment::Segment;
use square::Square;
use tests::run_tests;
use draw::draw;

const UNIT_SQUARE: Square = Square {
	a: Line { u: Vector { x: 0.0 , y: 1.0 }, d: 0.0 },
	b: Line { u: Vector { x: 1.0 , y: 0.0 }, d: 1.0 },
	c: Line { u: Vector { x: 0.0 , y: 1.0 }, d: 1.0 },
	d: Line { u: Vector { x: 1.0 , y: 0.0 }, d: 0.0 }
};

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
				if line.equivalent(&lines[k].0) {
					lines[k].1 += 1;
					duplicate = true;
					// println!("found duplicate {:?} {:?}", lines[k].0, line);
					break;
				}
			}
			if duplicate { continue }
			// #2 check line for duplicate against current round
			//  - if t: increment line:count in current line
			//  - if f: add new entry to current round. with 0
			for k in 0..round.len() {
				if line.equivalent(&round[k].0) {
					round[k].1 += 1;
					duplicate = true;
					// println!("found duplicate {:?} {:?}", round[k].0, line);
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

fn compute_intersections (
	points: &mut Vec<(Vector, u64)>, // already existing intersection points
	new_lines: &mut Vec<(Line, u64)> // the newest set of lines
) -> Vec<(Vector, u64)> {
	let mut round: Vec<(Vector, u64)> = Vec::new();
	if new_lines.len() == 0 { return round }

	for i in 0..new_lines.len() - 1 {
		// println!("{}: {} new points", i, round.len());
		for j in (i + 1)..new_lines.len() {
			let (success, point) = new_lines[i].0.intersect(&new_lines[j].0);
			if !success { continue }
			if !UNIT_SQUARE.contains(&point) { continue }
			let mut duplicate = false;
			for k in 0..points.len() {
				if point.equivalent(&points[k].0) {
					points[k].1 += 1;
					duplicate = true;
					// println!("found duplicate {:?} {:?}", points[k].0, point);
					break;
				}
			}
			if duplicate { continue }
			for k in 0..round.len() {
				if point.equivalent(&round[k].0) {
					round[k].1 += 1;
					duplicate = true;
					// println!("found duplicate {:?} {:?}", round[k].0, point);
					break;
				}
			}
			if duplicate { continue }
			round.push((point, 1));
		}
	}
	return round;
}

fn main () {
	run_tests();

	let mut points: Vec<(Vector, u64)> = Vec::new();
	let mut lines: Vec<(Line, u64)> = Vec::new();
	points.push((Vector { x: 0.0, y: 0.0 }, 1));
	points.push((Vector { x: 1.0, y: 0.0 }, 1));
	points.push((Vector { x: 1.0, y: 1.0 }, 1));
	points.push((Vector { x: 0.0, y: 1.0 }, 1));

	// 1. compute all axioms for this round
	let mut round1_lines: Vec<(Line, u64)> = compute_axiom2(
		&mut points, &mut lines);
	// 2. compute new intersection points
	let mut round1_points: Vec<(Vector, u64)> = compute_intersections(
		&mut points, &mut round1_lines);
	// 3. merge points and lines from this new round
	points.append(&mut round1_points);
	lines.append(&mut round1_lines);

	// round 2!
	// 1. compute all axioms for this round
	let mut round2_lines: Vec<(Line, u64)> = compute_axiom2(
		&mut points, &mut lines);
	// 2. compute new intersection points
	let mut round2_points: Vec<(Vector, u64)> = compute_intersections(
		&mut points, &mut round2_lines);
	// 3. merge points and lines from this new round
	points.append(&mut round2_points);
	lines.append(&mut round2_lines);

	println!("..finished axioms round 2 ({} lines {} points)", lines.len(), points.len());

	// round 3
	// 1. compute all axioms for this round
	let mut round3_lines: Vec<(Line, u64)> = compute_axiom2(
		&mut points, &mut lines);
	// 2. compute new intersection points
	let mut round3_points: Vec<(Vector, u64)> = compute_intersections(
		&mut points, &mut round3_lines);
	// 3. merge points and lines from this new round
	points.append(&mut round3_points);
	lines.append(&mut round3_lines);

	println!("..finished axioms round 3 ({} lines {} points)", lines.len(), points.len());

	// round 4
	// 1. compute all axioms for this round
	let mut round4_lines: Vec<(Line, u64)> = compute_axiom2(
		&mut points, &mut lines);
	println!("  axiom done ({} lines)", round4_lines.len());
	// 2. compute new intersection points
	let mut round4_points: Vec<(Vector, u64)> = compute_intersections(
		&mut points, &mut round4_lines);
	// 3. merge points and lines from this new round
	println!("  intersections done");
	points.append(&mut round4_points);
	lines.append(&mut round4_lines);

	println!("..finished axioms round 4 ({} lines {} points)", lines.len(), points.len());

	let mut segments: Vec<Segment> = Vec::new();
	for i in 0..lines.len() {
		let (success, segment) = UNIT_SQUARE.clip(&lines[i].0);
		if success { segments.push(segment) }
	}

	println!("..lines clipped to segments");

	let marks: Vec<Vector> = points.iter()
		.map(|el| el.0)
		.collect::<Vec<Vector>>();

	println!("{} lines, {} segments, {} points", lines.len(), segments.len(), points.len());
	draw(&segments, &marks);

	println!("..svg saved");
}
