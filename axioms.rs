use Vector;
use Line;

const EPSILON: f64 = 1.0e-8;

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

// const test_axiom1_2 = (params, poly) => [params.points
//   .map(p => math.core.overlap_convex_polygon_point(poly, p, ear.math.include))
//   .reduce((a, b) => a && b, true)];

pub fn axiom3 (a: &Line, b: &Line) -> Vec<Line> {
    // get intersection and a test if they are parallel
    let intersect = a.intersect(&b);
    // if lines are parallel (no intersection exists)
    if !intersect.0 {
        let d = (a.d + b.d * a.u.dot(&b.u)) / 2.0;
        return vec![ Line { u: a.u, d: d } ];
    }
    // 2 solutions
    let u1 = a.u.add(&b.u).normalize();
    let u2 = a.u.subtract(&b.u).normalize();
    let d1 = intersect.1.dot(&u1);
    let d2 = intersect.1.dot(&u2);
    return vec![ Line { u: u1, d: d1 }, Line { u: u2, d: d2 } ];
}

pub fn axiom4 (a: &Vector, b: &Line) -> Line {
    let u = b.u.rotate90();
    let d = a.dot(&u);
    return Line { u, d };
    // let NULL = Line { u: Vector { x: 0, y: 0 }, d: 0 };
    // // test the line before we return it
    // // shortest distance between the input point and the input line
    // let dist = b.d - a.dot(&b.u);
    // // dist as a vector, from the point to the line
    // let vector = u.scale(dist);
    // let point = a.add(&vector);
    // let valid = square.contains(&point);
}

// p1 is the point the line will pass through
// p2 is the point that will fold onto the line
pub fn axiom5 (p1: &Vector, p2: &Vector, l: &Line) -> Vec<Line> {
    let NULL = Line { u: Vector { x: 0.0, y: 0.0 }, d: 0.0 };
    let p1base = p1.dot(&l.u);
    let a = l.d - p1base;  // maybe reverse
    let c = p1.distance_to(&p2);
    // if a == c we have one solution
    if a > c { return vec![] }
    let b = (c * c - a * a).sqrt();
    let a_vec = l.u.scale(a); // maybe reverse
    let base_center = p1.subtract(&a_vec);
    let base_vector = l.u.rotate90().scale(b);
    let p2_a = base_center.add(&base_vector);
    let p2_b = base_center.subtract(&base_vector);
    let u_a = p2.subtract(&p2_a).normalize();
    let u_b = p2.subtract(&p2_b).normalize();
    let d_a = p1.dot(&u_a);
    let d_b = p1.dot(&u_b);
    return vec![ Line { u: u_a, d: d_a }, Line { u: u_b, d: d_b } ];
}

// l1 is the perpendicular to our solution
// l2 is the line we bring the point onto
pub fn axiom7 (p: &Vector, l1: &Line, l2: &Line) -> Line {
    let NULL = Line { u: Vector { x: 0.0, y: 0.0 }, d: 0.0 };
    let u = l1.u.rotate90();
    let u_u = u.dot(&l2.u);
    // if u_u is close to 0, the two input lines are parallel, no solution
    if u_u.abs() < EPSILON { return NULL }
    let a = p.dot(&u);
    let b = p.dot(&l2.u);
    let d = (l2.d + 2.0 * a * u_u - b) / (2.0 * u_u);
    return Line { u, d };
}


