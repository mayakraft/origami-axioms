mod primitives;
mod square;
mod axioms;
mod quadtree;
mod linecontainer;
mod tests;
mod draw;
use primitives::Vector;
use primitives::Line;
use primitives::Segment;
use quadtree::QuadTree;
use quadtree::make_tree;
use linecontainer::LineContainer;
use linecontainer::make_line_container;
use square::Square;
use tests::run_tests;
use draw::draw;


// fn duplicate_point_check (point: &Vector, points: &mut Vec<(Vector, u64)>) -> bool {
// 	for k in 0..points.len() {
// 		if point.equivalent(&points[k].0) {
// 			points[k].1 += 1;
// 			return true;
// 		}
// 	}
// 	return false;
// }

// fn compute_axiom1 (
// 	points: &Vec<(Vector, u64)>, // the previous round of points (build from this)
// 	lines: &mut Vec<(Line, u64)>, // the previous round of lines (build from this)
//     // lines: &mut LineContainer,
// 	new_lines: &mut Vec<(Line, u64)> // the current round (check for duplicates only)
// ) {
// 	for i in 0..points.len() - 1 {
// 		// println!("{}/{}: {} axiom 1", i, points.len(), new_lines.len());
// 		for j in (i + 1)..points.len() {
// 			let line: Line = axioms::axiom1(&points[i].0, &points[j].0);
// 			if duplicate_line_check(&line, lines) { continue }
// 			if duplicate_line_check(&line, new_lines) { continue }
// 			new_lines.push((line, 1));
// 		}
// 	}
// }

fn compute_axiom2 (
	points: &Vec<&(Vector, u64)>, // the previous round of points (build from this)
	// lines: &mut Vec<(Line, u64)>, // the previous round of lines (build from this)
	// new_lines: &mut Vec<(Line, u64)> // the current round (check for duplicates only)
    lines: &mut LineContainer,
    new_lines: &mut LineContainer,
) {
	for i in 0..points.len() - 1 {
		println!("{}/{}: {} axiom 2", i, points.len(), new_lines.len());
		for j in (i + 1)..points.len() {
			// let line: Line = axioms::axiom2(&points[i].0, &points[j].0);
			// if duplicate_line_check(&line, lines) { continue }
			// if duplicate_line_check(&line, new_lines) { continue }
			// new_lines.push((line, 1));
    		let line: Line = axioms::axiom2(&points[i].0, &points[j].0);
            if lines.increment_match(&line) { continue }
            if new_lines.increment_match(&line) { continue }
			new_lines.push(&line);
		}
	}
}

// fn compute_axiom3 (
// 	points: &Vec<(Vector, u64)>, // the previous round of points (build from this)
// 	lines: &mut Vec<(Line, u64)>, // the previous round of lines (build from this)
// 	new_lines: &mut Vec<(Line, u64)>, // the current round (check for duplicates only)
//     boundary: &Square
// ) {
// 	for i in 0..lines.len() - 1 {
// 		println!("{}/{}: {} axiom 3", i, points.len(), new_lines.len());
// 		for j in (i + 1)..lines.len() {
// 			let solutions = axioms::axiom3(&lines[i].0, &lines[j].0, boundary);
//             for k in 0..solutions.len() {
//                 if duplicate_line_check(&solutions[k], lines) { continue }
// 			    if duplicate_line_check(&solutions[k], new_lines) { continue }
// 			    new_lines.push((solutions[k], 1));
//             }
// 		}
// 	}
// }

// fn compute_axiom4 (
// 	points: &Vec<(Vector, u64)>, // the previous round of points (build from this)
// 	lines: &mut Vec<(Line, u64)>, // the previous round of lines (build from this)
// 	new_lines: &mut Vec<(Line, u64)>, // the current round (check for duplicates only)
//     boundary: &Square
// ) {
// }

// fn compute_axiom5 (
// 	points: &Vec<(Vector, u64)>, // the previous round of points (build from this)
// 	lines: &mut Vec<(Line, u64)>, // the previous round of lines (build from this)
// 	new_lines: &mut Vec<(Line, u64)>, // the current round (check for duplicates only)
//     boundary: &Square
// ) {
// }

// fn compute_axiom6 (
// 	points: &Vec<(Vector, u64)>, // the previous round of points (build from this)
// 	lines: &mut Vec<(Line, u64)>, // the previous round of lines (build from this)
// 	new_lines: &mut Vec<(Line, u64)>, // the current round (check for duplicates only)
//     boundary: &Square
// ) {
// }

// fn compute_axiom7 (
// 	points: &Vec<(Vector, u64)>, // the previous round of points (build from this)
// 	lines: &mut Vec<(Line, u64)>, // the previous round of lines (build from this)
// 	new_lines: &mut Vec<(Line, u64)>, // the current round (check for duplicates only)
//     boundary: &Square
// ) {
// }

fn compute_intersections (
	points: &mut QuadTree, // already existing intersection points
    old_lines: &mut Vec<&(Line, u64)>, // all lines from previous rounds
	new_lines: &mut Vec<&(Line, u64)>, // the newest set of lines
    polygon: &Square
) -> QuadTree {
	let mut round: QuadTree = make_tree();
    // concat new and old lines into one list "all_lines"
    let mut all_lines: Vec<&(Line, u64)> = Vec::new();
    for i in 0..new_lines.len() { all_lines.push(new_lines[i]) }
    for i in 0..old_lines.len() { all_lines.push(old_lines[i]) }
    // get intersections points comparing two arrays: new_lines to old_lines
    // this compares every new line to every new AND old line, but avoids
    // old lines getting compared to themselves again (which already happened)
	if new_lines.len() == 0 { return round }
	for i in 0..new_lines.len() - 1 {
        if round.len() > 1000000 { break; }
		println!("{}/{}: {} new points", i, new_lines.len(), round.len());
		for j in (i + 1)..all_lines.len() {
			let (success, point) = new_lines[i].0.intersect(&all_lines[j].0);
			if !success { continue }
			if !polygon.contains(&point) { continue }
			// if duplicate_point_check(&point, points) { continue }
			// if duplicate_point_check(&point, &mut round) { continue }
			if points.increment_match(&point) { continue }
			if round.increment_match(&point) { continue }
			round.push(&point); // automatically makes tuple (point, 1)
		}
	}
	return round;
}

fn compute_round (
    round: u16,
	point_quadtree: &mut QuadTree,
	// lines: &mut Vec<(Line, u64)>,
	lines: &mut LineContainer,
    boundary: &Square
	// compute_pts: bool
) {
    // all axioms will be built from function arguments points and lines
    // from the previous round (make points into Vector from the quadtree)
	// let points: Vec<(Vector, u64)> = if round < 2 {
        // point_quadtree.flatten()
    // } else {
    // let points = point_quadtree.flatten().iter()
    //         .filter(|tup| tup.1 > 1)
    //         .collect::<Vec<(Vector, u64)>>();
    // };
    // let points = point_quadtree.flatten_filter(if round < 2 { 0 } else { 1 });
    let points = point_quadtree.flatten_filter(round as u64);
    println!("round {}, {} points, using only {}", round, point_quadtree.len(), points.len());

	// new lines is all the lines made in THIS round
	// let mut new_lines: Vec<(Line, u64)> = Vec::new();
	let mut new_lines: LineContainer = make_line_container();
	// 1. compute all axioms for this round
	// compute_axiom1(&points, lines, &mut new_lines);
	compute_axiom2(&points, lines, &mut new_lines);
	// compute_axiom3(&points, lines, &mut new_lines, boundary);
	// compute_axiom4(&points, lines, &mut new_lines, boundary);
	// compute_axiom5(&points, lines, &mut new_lines, boundary);
	// compute_axiom6(&points, lines, &mut new_lines, boundary);
	// compute_axiom7(&points, lines, &mut new_lines, boundary);
	// todo: list more axioms
	// 2. compute new intersection points
	// let mut new_points: Vec<(Vector, u64)> = if compute_pts { compute_intersections(
	// 	points, &mut new_lines) } else { Vec::new() };
	// let mut new_points: Vec<(Vector, u64)> = compute_intersections(
	// 	points, &mut new_lines);

    let mut flat_lines = lines.flatten();
    let mut flat_new_lines = new_lines.flatten();
	let mut new_points: QuadTree = compute_intersections(
        // point_quadtree, lines, &mut new_lines, boundary);
        point_quadtree, &mut flat_lines, &mut flat_new_lines, boundary);
	// let mut new_points: QuadTree = if compute_pts { compute_intersections(points, &mut new_lines) }
	// 	else { make_tree() };
	// 3. merge points and lines from this new round
	// points.append(&mut new_points);
	point_quadtree.merge(&mut new_points);
	// lines.append(&mut new_lines);
	lines.merge(&mut new_lines);
}

fn main () {
	run_tests();

    // points is a special quad tree struct, but you can treat it like a Vec<Vector>
	let mut points = make_tree();
	points.push(&Vector { x: 0.0, y: 0.0 });
	points.push(&Vector { x: 1.0, y: 0.0 });
	points.push(&Vector { x: 1.0, y: 1.0 });
	points.push(&Vector { x: 0.0, y: 1.0 });
    
    // lines is (for now) simply a Vec of the Line tuple where spot 2 is #appearances
	// let mut lines: Vec<(Line, u64)> = Vec::new();
    // lines.push((Line { u: Vector { x: 0.0 , y: 1.0 }, d: 0.0 }, 1));
    // lines.push((Line { u: Vector { x: 1.0 , y: 0.0 }, d: 1.0 }, 1));
    // lines.push((Line { u: Vector { x: 0.0 , y: 1.0 }, d: 1.0 }, 1));
    // lines.push((Line { u: Vector { x: 1.0 , y: 0.0 }, d: 0.0 }, 1));

    let unit_square: Square = Square {
        a: Line { u: Vector { x: 0.0 , y: 1.0 }, d: 0.0 },
        b: Line { u: Vector { x: 1.0 , y: 0.0 }, d: 1.0 },
        c: Line { u: Vector { x: 0.0 , y: 1.0 }, d: 1.0 },
        d: Line { u: Vector { x: 1.0 , y: 0.0 }, d: 0.0 }
    };

    let mut lines: LineContainer = make_line_container();

    lines.push(&Line { u: Vector { x: 0.0 , y: 1.0 }, d: 0.0 });
    lines.push(&Line { u: Vector { x: 1.0 , y: 0.0 }, d: 1.0 });
    lines.push(&Line { u: Vector { x: 0.0 , y: 1.0 }, d: 1.0 });
    lines.push(&Line { u: Vector { x: 1.0 , y: 0.0 }, d: 0.0 });

	for i in 0_u16..4 {
		compute_round(i, &mut points, &mut lines, &unit_square);
		// println!("done round {} ({} lines {} points)", i, lines.len(), points.len());
        //
        // 	because some lines are being made outside of the square, we need to filter
        // 	out lines based on if they become segments.
	}

	let mut segments: Vec<(Segment, u64)> = Vec::new();
    let flat_lines: Vec<&(Line, u64)> = lines.flatten();
	// for i in 0..lines.len() {
	for i in 0..flat_lines.len() {
		let (success, segment) = unit_square.clip(&flat_lines[i].0);
		if success { segments.push((segment, flat_lines[i].1)) }
	}

    // let marks: Vctor> = points.flatten();
	let marks: Vec<&Vector> = Vec::new();
	
    // println!("{} lines, {} segments, {} points", lines.len(), segments.len(), points.len());
	println!("{} lines, {} segments, {} points", lines.len(), segments.len(), points.len());
	draw(&segments, &marks);
}

