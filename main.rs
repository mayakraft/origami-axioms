mod axioms;
mod primitives;
mod quadtree;
mod linecontainer;
mod tests;
mod draw;
mod random;
mod compute;
use primitives::Vector;
use primitives::Line;
use primitives::Segment;
use primitives::Rect;
use primitives::make_square;
use quadtree::QuadTree;
use quadtree::make_tree;
use linecontainer::LineContainer;
use linecontainer::make_line_container;
use tests::run_tests;
use draw::draw;
use compute::compute_intersections;
use compute::compute_axiom1;
use compute::compute_axiom2;
use compute::compute_axiom3;
use compute::compute_axiom4;
use compute::compute_axiom5;
use compute::compute_axiom6;
use compute::shortcut_axiom6;
use compute::compute_axiom7;

fn make_round (
	round: usize,
	point_quadtree: &mut QuadTree,
	line_container: &mut LineContainer,
	boundary: &Rect
) {
	// all axioms will be built from function arguments points and lines
	// from the previous round (make points into Vector from the quadtree)

	// let points = point_quadtree.flatten_filter(if round > 1 { 3 } else { 0 });
	// let lines = line_container.flatten_filter(if round > 1 { 2 } else { 0 });
	// let points = point_quadtree.flatten_filter(round as u64);
	// let lines = line_container.flatten_filter(round as u64);
	let points = point_quadtree.flatten();
	let lines = line_container.flatten();

	// new lines is all the lines made in THIS round
	// let mut new_lines: Vec<(Line, u64)> = Vec::new();
	let mut new_line_container: LineContainer = make_line_container();
	// 1. compute all axioms for this round
	// compute_axiom1(&points, line_container, &mut new_line_container);
	// compute_axiom2(&points, line_container, &mut new_line_container);
	compute_axiom3(&points, &lines, line_container, &mut new_line_container, boundary);
	// compute_axiom4(&points, &lines, line_container, &mut new_line_container, boundary);
	// compute_axiom5(&points, &lines, line_container, &mut new_line_container, boundary);
	// shortcut_axiom6(&points, &lines, line_container, &mut new_line_container, boundary);
	// if round > 1 { shortcut_axiom6(&points, &lines, line_container, &mut new_line_container, boundary); }
	// else { compute_axiom6(&points, &lines, line_container, &mut new_line_container, boundary); }
	// compute_axiom7(&points, &lines, line_container, &mut new_line_container, boundary);
	// todo: list more axioms
	// 2. compute new intersection points
	// let mut new_points: Vec<(Vector, u64)> = if compute_pts { compute_intersections(
	// 	points, &mut new_lines) } else { Vec::new() };
	// let mut new_points: Vec<(Vector, u64)> = compute_intersections(
	// 	points, &mut new_lines);

	let new_lines = new_line_container.flatten();
	let old_lines = line_container.flatten();

	// let mut new_points: QuadTree = compute_intersections(
	// 	point_quadtree, &old_lines, &new_lines, boundary);
	let mut new_points: QuadTree = if round < 2 {
		compute_intersections(point_quadtree, &old_lines, &new_lines, boundary)
	} else { make_tree() };

	// point_quadtree, lines, &mut new_lines, boundary);
	// let mut new_points: QuadTree = if compute_pts { compute_intersections(points, &mut new_lines) }
	// 	else { make_tree() };
	// 3. merge points and lines from this new round
	// points.append(&mut new_points);
	point_quadtree.merge(&mut new_points);
	// lines.append(&mut new_lines);
	line_container.merge(&mut new_line_container);
}

fn main () {
	// the boundary, all points and lines will be clipped inside
	let unit_square: Rect = make_square();

	// the initial geometry from which all folds will be made
	let mut points: QuadTree = make_tree();
	let mut lines: LineContainer = make_line_container();
	points.push(&Vector { x: 0.0, y: 0.0 });
	points.push(&Vector { x: 1.0, y: 0.0 });
	points.push(&Vector { x: 1.0, y: 1.0 });
	points.push(&Vector { x: 0.0, y: 1.0 });
	unit_square.sides.iter().for_each(|side| lines.push(side));

	for round in 0..3 {
		make_round(round, &mut points, &mut lines, &unit_square);
		println!("done round {} ({} lines {} points)", round + 1, lines.len(), points.len());
		// 	because some lines are being made outside of the square, we need to filter
		// 	out lines based on if they become segments.
	}

	// temporarily put a tuple in a tuple
	// (line, number_of_repeats, (clipping_success, segment))
	let segments: Vec<(Segment, u64)> = lines.flatten().iter()
		.map(|el: &(Line, u64)| (el.0, el.1, unit_square.clip(&el.0)))
		.filter(|el: &(Line, u64, (bool, Segment))| (el.2).0)
		.map(|el| ( (el.2).1, el.1) )
		.collect();

	// let marks: Vctor> = points.flatten();
	let marks: Vec<&Vector> = Vec::new();
	draw(&segments, &marks);
	println!("finished. {} lines, {} segments, {} points", lines.len(), segments.len(), points.len());
	// make rust not complain about unused functions
	run_tests();
}
