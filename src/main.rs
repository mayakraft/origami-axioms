extern crate rabbit_ear;
use rabbit_ear as ear;
use ear::Vector;
use ear::Line;
use ear::Segment;
use ear::Rect;
use ear::make_square;

mod origami;
use origami::GridVec;
use origami::make_grid;
// use QuadTree;
// use make_tree;
use origami::LineContainer;
use origami::linecontainer::make_line_container;
use origami::draw::draw;

fn make_round (
	round: usize,
	point_quadtree: &mut GridVec,
	line_container: &mut LineContainer,
	boundary: Rect
) {
	// all axioms will be built from function arguments points and lines
	// from the previous round (make points into Vector from the quadtree)
	let points = point_quadtree.flatten();
	let lines = line_container.flatten();
	println!("round start {} points, {} lines", points.len(), lines.len());
	// let points = point_quadtree.flatten_filter(match round {0=>0, 1=>0, 2=>0, 3=>4, _=>2});
	// let lines = line_container.flatten_filter(match round {0=>0, 1=>0, 2=>0, 3=>0, _=>0});

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
	origami::make_axiom1(&points, line_container, &mut new_line_container, boundary);
	origami::make_axiom2(&points, line_container, &mut new_line_container, boundary);
	origami::make_axiom3(&points, &lines, line_container, &mut new_line_container, boundary);
	origami::make_axiom4(&points, &lines, line_container, &mut new_line_container, boundary);
	origami::make_axiom5(&points, &lines, line_container, &mut new_line_container, boundary);
	origami::make_axiom6(&points, &lines, line_container, &mut new_line_container, boundary);
	origami::make_axiom7(&points, &lines, line_container, &mut new_line_container, boundary);
	// origami::make_axiom5(&pts_ax5, &lns_ax5, line_container, &mut new_line_container, boundary);
	// // origami::shortcut_axiom6(&points, &lines, line_container, &mut new_line_container, boundary);
	// origami::make_axiom6(&pts_ax6, &lns_ax6, line_container, &mut new_line_container, boundary);
	// origami::make_axiom7(&pts_ax7, &lns_ax7, line_container, &mut new_line_container, boundary);
	// todo: list more axioms
	// 2. compute new intersection points
	// let mut new_points: Vec<(Vector, u64)> = if make_pts { origami::make_intersections(
	// 	points, &mut new_lines) } else { Vec::new() };
	// let mut new_points: Vec<(Vector, u64)> = origami::make_intersections(
	// 	points, &mut new_lines);

	let new_lines = new_line_container.flatten();
	let old_lines = line_container.flatten();

	// let mut new_points: GridVec = origami::make_intersections(
	// 	point_quadtree, &old_lines, &new_lines, boundary);
	let mut new_points: GridVec = if round < 3 {
		origami::make_intersections(point_quadtree, &old_lines, &new_lines, boundary)
	} else { make_grid() };

	// point_quadtree, lines, &mut new_lines, boundary);
	// 3. merge points and lines from this new round
	point_quadtree.merge(&mut new_points);
	line_container.merge(&mut new_line_container);
}


fn main () {
	// the boundary, all points and lines will be clipped inside
	let unit_square: Rect = make_square();

	// the initial geometry from which all folds will be made
	let mut points: GridVec = make_grid();
	let mut lines: LineContainer = make_line_container();	
	points.push(Vector { x: 0.0, y: 0.0 });
	points.push(Vector { x: 1.0, y: 0.0 });
	points.push(Vector { x: 1.0, y: 1.0 });
	points.push(Vector { x: 0.0, y: 1.0 });
	unit_square.sides.iter().for_each(|side| lines.push(side));

	for round in 0..2 {
		make_round(round, &mut points, &mut lines, unit_square);
		// println!("done round {} ({} lines {} points)", round + 1, lines.len(), points.len());
		// 	because some lines are being made outside of the square, we need to filter
		// 	out lines based on if they become segments.
	}

	let flat_lines = lines.flatten();
	let flat_points = points.flatten();

	println!("finished, {} lines, {} points", flat_lines.len(), flat_points.len());

	// temporarily put a tuple in a tuple
	// (line, number_of_repeats, (clipping_success, segment))
	let mut segments: Vec<(Segment, u64)> = flat_lines.iter()
		.map(|el: &(Line, u64)| (el.0, el.1, unit_square.clip(el.0)))
		.filter(|el: &(Line, u64, (bool, Segment))| (el.2).0)
		.map(|el| ( (el.2).1, el.1) )
		.collect();
	let mut marks: Vec<(Vector, u64)> = flat_points;
	segments.sort_by_key(|el| el.1);
	marks.sort_by_key(|el| el.1);

	draw(&segments, &marks);

	// for i in 0..segments.len() {
	//     println!("{}: {:?}", i, segments[i]);
	// }

	// println!("finished. {} lines, {} segments, {} points",
	// 	lines.len(), segments.len(), points.len());


	// // testing for quadtree
	// let mut tree = make_grid();
	// println!("tree before {} {}", tree.quadrants.len(), tree.points.len());
	// tree.push(Vector { x: 0.5, y: 0.25 });
	// tree.push(Vector { x: 1.0, y: 0.15 });
	// tree.push(Vector { x: 0.5, y: 0.5 });
	// tree.push(Vector { x: 0.5, y: 0.75 });
	// tree.push(Vector { x: 0.75, y: 0.85 });
	// tree.push(Vector { x: 1.0, y: 0.15 });
	// println!("tree after {} {}", tree.quadrants.len(), tree.points.len());
	// for i in 0..tree.quadrants.len() {
	// 	println!("-- {} inside ({}) {}", i, tree.quadrants[i].quadrants.len(), tree.quadrants[i].points.len());
	// }
	// let points = tree.flatten();
	// println!("all points, {}", points.len());
	// for i in 0..points.len() {
	// 	println!("{}: {:?}", i, points[i]);
	// }

	// println!("tree {:?}", tree);
	// for i in 0..tree.quadrants.len() {
	// 	println!("tree {} {:?}", i, tree.quadrants[i]);
	// }
}
