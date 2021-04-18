use rabbit_ear as ear;
use self::ear::Rect;
use self::ear::axioms::axiom1;
use self::ear::axioms::axiom2;
use self::ear::axioms::axiom3;
use self::ear::axioms::axiom4;
use self::ear::axioms::axiom5;
use self::ear::axioms::axiom6;
use self::ear::axioms::axiom7;
use super::GridVec;
use super::make_grid;
// use super::QuadTree;
// use super::make_tree;
use super::LineContainer;

use super::CountPoint;
use super::CountLine;

const DEBUG: bool = true;
const VERBOSE: bool = false;

pub fn make_intersections (
	points: &mut GridVec, // already existing intersection points
	old_lines: &Vec<CountLine>, // all lines from previous rounds
	new_lines: &Vec<CountLine>, // the newest set of lines
	polygon: Rect
) -> GridVec {
	let mut round: GridVec = make_grid();
	// concat new and old lines into one list "all_lines"
	let mut all_lines: Vec<CountLine> = Vec::new();
	for i in 0..new_lines.len() { all_lines.push(new_lines[i]) }
	for i in 0..old_lines.len() { all_lines.push(old_lines[i]) }
	// if this message prints, the for loop j in (i+1) will start beyond all_lines. needs fix
	if old_lines.len() == 0 { println!("ATTN: make.rs, case not considered. need fix"); }
	// get intersections points comparing two arrays: new_lines to old_lines
	// this compares every new line to every new AND old line, but avoids
	// old lines getting compared to themselves again (which already happened)
	if new_lines.len() == 0 { return round }
	// println!("starting loop");
	for i in 0..new_lines.len() {
		if DEBUG && VERBOSE { println!("{}/{} new points", i, new_lines.len()); }
		// if DEBUG { println!("{}/{}: {} new points", i, new_lines.len(), round.len()); }
		for j in (i + 1)..all_lines.len() {
			let (success, point) = new_lines[i].0.intersect(all_lines[j].0);
			if !success { continue }
			if !polygon.contains(point) { continue }
			if points.increment_match(&point) { continue }
			if round.increment_match(&point) { continue }
			// round.push(&point); // automatically makes tuple (point, 1)
			round.push(point); // automatically makes tuple (point, 1)
			// round.push((point, 1)); // automatically makes tuple (point, 1)
		}
	}
	if DEBUG { println!("intersections done. {} new points this round", round.len()); }
	return round;
}

// these parameters are pointers, because all these methods are called
// in sequence, preventing variable moving, allowing reuse
pub fn make_axiom1 (
	points: &Vec<CountPoint>, // the previous round of points (build from this)
	old_lines: &mut LineContainer, // the previous round of lines (build from this)
	new_lines: &mut LineContainer, // the current round (check for duplicates only)
	boundary: Rect
) {
	for i in 0..points.len() - 1 {
		if DEBUG && VERBOSE { println!("{}/{}: {} axiom 1", i, points.len(), new_lines.len()); }
		for j in (i + 1)..points.len() {
			let solutions = axiom1(points[i].0, points[j].0, boundary);
			for k in 0..solutions.len() {
				if old_lines.increment_match(&solutions[k]) { continue }
				if new_lines.increment_match(&solutions[k]) { continue }
				new_lines.push(&solutions[k]);
			}
		}
	}
	if DEBUG { println!("axiom 1 done. {} lines this round", new_lines.len()); }
}

pub fn make_axiom2 (
	points: &Vec<CountPoint>, // the previous round of points (build from this)
	old_lines: &mut LineContainer, // the previous round of lines (build from this)
	new_lines: &mut LineContainer, // the current round (check for duplicates only)
	boundary: Rect
) {
	for i in 0..points.len() - 1 {
		if DEBUG && VERBOSE { println!("{}/{}: {} axiom 2", i, points.len(), new_lines.len()); }
		for j in (i + 1)..points.len() {
			let solutions = axiom2(points[i].0, points[j].0, boundary);
			for k in 0..solutions.len() {
				if old_lines.increment_match(&solutions[k]) { continue }
				if new_lines.increment_match(&solutions[k]) { continue }
				new_lines.push(&solutions[k]);
			}
		}
	}
	if DEBUG { println!("axiom 2 done. {} lines this round", new_lines.len()); }
}

pub fn make_axiom3 (
	_points: &Vec<CountPoint>, // the previous round of points (build from this)
	lines: &Vec<CountLine>, // the previous round of lines as list (build from this)
	old_lines: &mut LineContainer, // the previous round (check for duplicates only)
	new_lines: &mut LineContainer, // the current round (check for duplicates only)
	boundary: Rect
) {
	for i in 0..lines.len() - 1 {
		if DEBUG && VERBOSE { println!("{}/{}: {} axiom 3", i, lines.len(), new_lines.len()); }
		for j in (i + 1)..lines.len() {
			let solutions = axiom3(lines[i].0, lines[j].0, boundary);
			for k in 0..solutions.len() {
				if old_lines.increment_match(&solutions[k]) { continue }
				if new_lines.increment_match(&solutions[k]) { continue }
				new_lines.push(&solutions[k]);
			}
		}
	}
	if DEBUG { println!("axiom 3 done. {} lines this round", new_lines.len()); }
}

pub fn make_axiom4 (
	points: &Vec<CountPoint>, // the previous round of points (build from this)
	lines: &Vec<CountLine>, // the previous round of lines as list (build from this)
	old_lines: &mut LineContainer, // the previous round (check for duplicates only)
	new_lines: &mut LineContainer, // the current round (check for duplicates only)
	boundary: Rect
) {
	for i in 0..points.len() {
		if DEBUG && VERBOSE { println!("{}/{}: {} axiom 4", i, points.len(), new_lines.len()); }
		for j in 0..lines.len() {
			let solutions = axiom4(points[i].0, lines[j].0, boundary);
			for k in 0..solutions.len() {
				if old_lines.increment_match(&solutions[k]) { continue }
				if new_lines.increment_match(&solutions[k]) { continue }
				new_lines.push(&solutions[k]);
			}
		}
	}
	if DEBUG { println!("axiom 4 done. {} lines this round", new_lines.len()); }
}

pub fn make_axiom5 (
	points: &Vec<CountPoint>, // the previous round of points (build from this)
	lines: &Vec<CountLine>, // the previous round of lines as list (build from this)
	old_lines: &mut LineContainer, // the previous round (check for duplicates only)
	new_lines: &mut LineContainer, // the current round (check for duplicates only)
	boundary: Rect
) {
	for i in 0..points.len() {
		for j in 0..points.len() {
			if i == j { continue }
			if DEBUG && VERBOSE { println!("{}/{}: {} axiom 5", i, points.len(), new_lines.len()); }
			for k in 0..lines.len() {
				let solutions = axiom5(points[i].0, points[j].0, lines[k].0, boundary);
				for l in 0..solutions.len() {
					if old_lines.increment_match(&solutions[l]) { continue }
					if new_lines.increment_match(&solutions[l]) { continue }
					new_lines.push(&solutions[l]);
				}
			}
		}
	}
	if DEBUG { println!("axiom 5 done. {} lines this round", new_lines.len()); }
}

pub fn make_axiom6 (
	points: &Vec<CountPoint>, // the previous round of points (build from this)
	lines: &Vec<CountLine>, // the previous round of lines as list (build from this)
	old_lines: &mut LineContainer, // the previous round (check for duplicates only)
	new_lines: &mut LineContainer, // the current round (check for duplicates only)
	boundary: Rect
) {
	for i in 0..points.len() {
		for j in 0..points.len() {
			if DEBUG && VERBOSE { println!("{}/{}pts: ({}pt): {} axiom 6", i, points.len(), j, new_lines.len()); }
			if i == j { continue }
			for k in 0..lines.len() {
				for l in 0..lines.len() {
					if k == l { continue }
					let solutions = axiom6(points[i].0, points[j].0, lines[k].0, lines[l].0, boundary);
					for m in 0..solutions.len() {
						if old_lines.increment_match(&solutions[m]) { continue }
						if new_lines.increment_match(&solutions[m]) { continue }
						new_lines.push(&solutions[m]);
					}
				}
			}
		}
	}
	if DEBUG { println!("axiom 6 done. {} lines this round", new_lines.len()); }
}

// pub fn shortcut_axiom6 (
// 	points: &Vec<CountPoint>, // the previous round of points (build from this)
// 	lines: &Vec<CountLine>, // the previous round of lines as list (build from this)
// 	old_lines: &mut LineContainer, // the previous round (check for duplicates only)
// 	new_lines: &mut LineContainer, // the current round (check for duplicates only)
// 	boundary: Rect
// ) {
// 	let mut rand = Rand::new(0);
// 	let steps: usize = 15;
// 	let point_step = (points.len() as f64) / (steps as f64);
// 	let line_step = (lines.len() as f64) / (steps as f64);
// 	let point_step_i = ((points.len() as f64) / (steps as f64)) as i32;
// 	let line_step_i = ((lines.len() as f64) / (steps as f64)) as i32;
// 	// println!("len {}p, {}l, steps: {}, {}p, {}l, {}, {}", points.len(), lines.len(), steps, point_step, line_step, steps * point_step, steps * line_step);
// 	for istep in 0..steps { // points
// 		let i = (istep as f64 * point_step).floor() as usize;
// 		for jstep in 0..steps { // points
// 			let j = (jstep as f64 * point_step).floor() as usize;
// 			if DEBUG { println!("{}/{}pts: ({}pt): {} axiom 6", i, points.len(), j, new_lines.len()); }
// 			if i == j { continue }
// 			for kstep in 0..steps { // lines
// 				let k = (kstep as f64 * line_step).floor() as usize;
// 				for lstep in 0..steps { // lines
// 					let l = (lstep as f64 * line_step).floor() as usize;
// 					if k == l { continue }
// 					let mut ii = i + rand.rand_range(0, point_step_i) as usize;
// 					let mut jj = j + rand.rand_range(0, point_step_i) as usize;
// 					let mut kk = k + rand.rand_range(0, line_step_i) as usize;
// 					let mut ll = l + rand.rand_range(0, line_step_i) as usize;
// 					if ii >= points.len() { ii = i }
// 					if jj >= points.len() { jj = j }
// 					if kk >= lines.len() { kk = k }
// 					if ll >= lines.len() { ll = l }
// 					let solutions = axiom6(points[ii].0, &points[jj].0, &lines[kk].0, &lines[ll].0, boundary);
// 					for m in 0..solutions.len() {
// 						if old_lines.increment_match(&solutions[m]) { continue }
// 						if new_lines.increment_match(&solutions[m]) { continue }
// 						new_lines.push(&solutions[m]);
// 					}
// 				}
// 			}
// 		}
// 	}
// }

pub fn make_axiom7 (
	points: &Vec<CountPoint>, // the previous round of points (build from this)
	lines: &Vec<CountLine>, // the previous round of lines as list (build from this)
	old_lines: &mut LineContainer, // the previous round (check for duplicates only)
	new_lines: &mut LineContainer, // the current round (check for duplicates only)
	boundary: Rect
) {
	for i in 0..points.len() {
		for j in 0..lines.len() {
			if DEBUG && VERBOSE { println!("{}/{}: {} axiom 7", i, points.len(), new_lines.len()); }
			for k in 0..lines.len() {
				if j == k { continue }
				let solutions = axiom7(points[i].0, lines[j].0, lines[k].0, boundary);
				for l in 0..solutions.len() {
					if old_lines.increment_match(&solutions[l]) { continue }
					if new_lines.increment_match(&solutions[l]) { continue }
					new_lines.push(&solutions[l]);
				}
			}
		}
	}
	if DEBUG { println!("axiom 7 done. {} lines this round", new_lines.len()); }
}
