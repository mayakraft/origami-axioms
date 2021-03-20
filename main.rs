mod axioms;
mod primitives;
mod quadtree;
mod linecontainer;
mod tests;
mod draw;
// mod random;
mod make;
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

fn make_round (
	round: usize,
	point_quadtree: &mut QuadTree,
	line_container: &mut LineContainer,
	boundary: &Rect
) {
	// all axioms will be built from function arguments points and lines
	// from the previous round (make points into Vector from the quadtree)

	let points = point_quadtree.flatten();
	let lines = line_container.flatten();
	// let lines = line_container.flatten_filter(match round {0..=1=>0, 2=>0, 3=>3, _=>12});

	// let points = point_quadtree.flatten_filter(match round {0..=1=>0, 2=>2, 3=>16, _=>144});
	// let lines = line_container.flatten_filter(match round {0..=1=>0, 2=>0, 3=>6, _=>12});
	// let pts_ax5 = point_quadtree.flatten_filter(match round {0..=1=>0, 2=>8, 3=>72, _=>144});
	// let lns_ax5 = line_container.flatten_filter(match round {0..=1=>0, 2=>8, 3=>64, _=>144});
	// let pts_ax6 = point_quadtree.flatten_filter(match round {0..=1=>0, 2=>12, 3=>72, _=>144});
	// let lns_ax6 = line_container.flatten_filter(match round {0..=1=>0, 2=>12, 3=>64, _=>144});
	// let pts_ax7 = point_quadtree.flatten_filter(match round {0..=1=>0, 2=>4, 3=>72, _=>144});
	// let lns_ax7 = line_container.flatten_filter(match round {0..=1=>0, 2=>4, 3=>64, _=>144});

	// // this is good for making points. faster than the other. half a million points.
	// let points = point_quadtree.flatten_filter(match round {0..=1=>0, 2=>24, 3=>72, _=>144});
	// let lines = line_container.flatten_filter(match round {0..=1=>0, 2=>16, 3=>64, _=>144});
	// let pts_ax5 = point_quadtree.flatten_filter(match round {0..=1=>0, 2=>36, 3=>72, _=>144});
	// let lns_ax5 = line_container.flatten_filter(match round {0..=1=>0, 2=>36, 3=>64, _=>144});
	// let pts_ax6 = point_quadtree.flatten_filter(match round {0..=1=>0, 2=>52, 3=>72, _=>144});
	// let lns_ax6 = line_container.flatten_filter(match round {0..=1=>0, 2=>52, 3=>64, _=>144});
	// let pts_ax7 = point_quadtree.flatten_filter(match round {0..=1=>0, 2=>36, 3=>72, _=>144});
	// let lns_ax7 = line_container.flatten_filter(match round {0..=1=>0, 2=>36, 3=>64, _=>144});

	// // this is good for making points. still takes a while
	// let points = point_quadtree.flatten_filter(match round {0..=1=>0, 2=>12, 3=>72, _=>144});
	// let lines = line_container.flatten_filter(match round {0..=1=>0, 2=>8, 3=>64, _=>144});
	// let pts_ax5 = point_quadtree.flatten_filter(match round {0..=1=>0, 2=>28, 3=>72, _=>144});
	// let lns_ax5 = line_container.flatten_filter(match round {0..=1=>0, 2=>28, 3=>64, _=>144});
	// let pts_ax6 = point_quadtree.flatten_filter(match round {0..=1=>0, 2=>32, 3=>72, _=>144});
	// let lns_ax6 = line_container.flatten_filter(match round {0..=1=>0, 2=>32, 3=>64, _=>144});
	// let pts_ax7 = point_quadtree.flatten_filter(match round {0..=1=>0, 2=>28, 3=>72, _=>144});
	// let lns_ax7 = line_container.flatten_filter(match round {0..=1=>0, 2=>28, 3=>64, _=>144});

	// new lines is all the lines made in THIS round
	// let mut new_lines: Vec<(Line, u64)> = Vec::new();
	let mut new_line_container: LineContainer = make_line_container();
	// 1. compute all axioms for this round
	// make::make_axiom1(&points, line_container, &mut new_line_container);
	// make::make_axiom2(&points, line_container, &mut new_line_container);
	make::make_axiom3(&points, &lines, line_container, &mut new_line_container, boundary);
	make::make_axiom4(&points, &lines, line_container, &mut new_line_container, boundary);
	// make::make_axiom5(&points, &lines, line_container, &mut new_line_container, boundary);
	make::make_axiom7(&points, &lines, line_container, &mut new_line_container, boundary);
	// make::make_axiom5(&pts_ax5, &lns_ax5, line_container, &mut new_line_container, boundary);
	// // make::shortcut_axiom6(&points, &lines, line_container, &mut new_line_container, boundary);
	// make::make_axiom6(&pts_ax6, &lns_ax6, line_container, &mut new_line_container, boundary);
	// make::make_axiom7(&pts_ax7, &lns_ax7, line_container, &mut new_line_container, boundary);
	// todo: list more axioms
	// 2. compute new intersection points
	// let mut new_points: Vec<(Vector, u64)> = if make_pts { make::make_intersections(
	// 	points, &mut new_lines) } else { Vec::new() };
	// let mut new_points: Vec<(Vector, u64)> = make::make_intersections(
	// 	points, &mut new_lines);

	let new_lines = new_line_container.flatten();
	let old_lines = line_container.flatten();

	// let mut new_points: QuadTree = make::make_intersections(
	// 	point_quadtree, &old_lines, &new_lines, boundary);
	let mut new_points: QuadTree = if round < 2 {
		make::make_intersections(point_quadtree, &old_lines, &new_lines, boundary)
	} else { make_tree() };

	// point_quadtree, lines, &mut new_lines, boundary);
	// 3. merge points and lines from this new round
	point_quadtree.merge(&mut new_points);
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
	let mut segments: Vec<(Segment, u64)> = lines.flatten().iter()
		.map(|el: &(Line, u64)| (el.0, el.1, unit_square.clip(&el.0)))
		.filter(|el: &(Line, u64, (bool, Segment))| (el.2).0)
		.map(|el| ( (el.2).1, el.1) )
		.collect();
	let mut marks: Vec<&(Vector, u64)> = points.flatten();
	segments.sort_by_key(|el| el.1);
	marks.sort_by_key(|el| el.1);
	
	draw(&segments, &marks);

	// for i in 0..segments.len() {
	//     println!("{}: {:?}", i, segments[i]);
	// }
	
	println!("finished. {} lines, {} segments, {} points",
		lines.len(), segments.len(), points.len());
	
	// make rust not complain about unused functions
	run_tests();
}

