use Vector;
use Line;
use Square;

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

pub fn axiom3 (a: &Line, b: &Line, _boundary: &Square) -> Vec<Line> {
    // get intersection and a test if they are parallel
    let intersect = a.intersect(&b);
    // if lines are parallel only one solution exists
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

pub fn axiom4 (a: &Vector, b: &Line, boundary: &Square) -> Vec<Line> {
    let u = b.u.rotate90();
    let d = a.dot(&u);
    // test the line before we return it
    // shortest distance between the input point and the input line
    let dist = b.d - a.dot(&b.u);
    // dist as a vector, from the point to the line
    let vector = u.scale(dist);
    let point = a.add(&vector);
    let valid = boundary.contains(&point);
    return if valid { vec![Line { u, d }] } else { vec![] }
}

// p1 is the point the line will pass through
// p2 is the point that will fold onto the line
pub fn axiom5 (p1: &Vector, p2: &Vector, l: &Line, _boundary: &Square) -> Vec<Line> {
    let p1base = p1.dot(&l.u);
    let a = l.d - p1base;  // maybe reverse
    let c = p1.distance_to(&p2);
    // if a == c we have one solution
    if a > c { return vec![] }
    let b = (c * c - a * a).sqrt();
    let a_vec = l.u.scale(a); // maybe reverse
    let base_center = p1.add(&a_vec);
    let base_vector = l.u.rotate90().scale(b);
    let p2_a = base_center.add(&base_vector);
    let p2_b = base_center.subtract(&base_vector);
    let u_a = p2.subtract(&p2_a).normalize();
    let u_b = p2.subtract(&p2_b).normalize();
    let d_a = p1.dot(&u_a);
    let d_b = p1.dot(&u_b);
    return vec![ Line { u: u_a, d: d_a }, Line { u: u_b, d: d_b } ];
}

fn cubrt (n: f64) -> f64 {
    if n < 0.0 { -(-n).powf(1.0/3.0) } else { n.powf(1.0/3.0) }
}

fn make_point (root: f64, line: &Line, line_vec: &Vector) -> Vector {
    line.u.scale(line.d).add(&line_vec.scale(root))
}

fn polynomial (
    degree: u64, a: f64, b: f64, c: f64, d: f64, line: &Line, line_vec: &Vector
) -> Vec<Vector> {
	// linear
    if degree == 1 {
        return vec![make_point(-d / c, line, line_vec)];
    }
    else if degree == 2 {
		// quadratic
        let discriminant = c.powf(2.0) - (4.0 * b * d);
        // no solution
        if discriminant < -EPSILON { return vec![]; }
        // one solution
        let q1 = -c / (2.0 * b);
        if discriminant < EPSILON {
            return vec![make_point(q1, line, line_vec)];
        }
        // two solutions
        let q2 = discriminant.sqrt() / (2.0 * b);
        return vec![
            make_point(q1 + q2, line, line_vec),
            make_point(q1 - q2, line, line_vec)
        ];
    } else if degree == 3 {
		// cubic
        // Cardano's formula. convert to depressed cubic
        let a2 = b / a;
        let a1 = c / a;
        let a0 = d / a;
        let q = (3.0 * a1 - a2.powf(2.0)) / 9.0;
        let r = (9.0 * a2 * a1 - 27.0 * a0 - 2.0 * a2.powf(3.0)) / 54.0;
        let d0 = q.powf(3.0) + r.powf(2.0);
        let u = -a2 / 3.0;
        // one solution
        if d0 > 0.0 {
            let sqrt_d0 = d0.sqrt();
            let s = cubrt(r + sqrt_d0);
            let t = cubrt(r - sqrt_d0);
            return vec![make_point(u + s + t, line, line_vec)];
        }
        // two solutions
        if d0.abs() < EPSILON {
            let s = r.powf(1.0/3.0);
            // let S = cubrt(R);
            // instead of checking if S is NaN, check if R was negative
            // if (isNaN(S)) { break; }
            if r < 0.0 { return vec![]; }
            return vec![
                make_point(u + 2.0 * s, line, line_vec),
                make_point(u - s, line, line_vec)
            ];
        }
        // three solutions
        let sqrt_d0 = (-d0).sqrt();
        let phi = sqrt_d0.atan2(r) / 3.0;
        let r_s = (r.powf(2.0) - d0).powf(1.0/6.0);
        let s_r = r_s * phi.cos();
        let s_i = r_s * phi.sin();
        return vec![
            make_point(u + 2.0 * s_r, line, line_vec),
            make_point(u - s_r - 3.0_f64.sqrt() * s_i, &line, line_vec),
            make_point(u - s_r + 3.0_f64.sqrt() * s_i, &line, line_vec)
        ];
    }
	return vec![];
}

pub fn axiom6 (
    p1: &Vector,
    p2: &Vector,
    l1: &Line,
    l2: &Line,
    _boundary: &Square
) -> Vec<Line> {
	// at least pointA must not be on lineA
	// for some reason this epsilon is much higher than 1e-6
	if (1.0 - (l1.u.dot(&p1) / l1.d)).abs() < 0.02 { return vec![]; }
	// line vec is the first line's vector, along the line, not the normal
    let line_vec = l1.u.rotate90();
    let vec1 = p1.add(&l1.u.scale(l1.d)).subtract(&p2.scale(2.0));
	let vec2 = l1.u.subtract(&p1).scale(l1.d);
    let c1 = p2.dot(&l2.u) - l2.d;
	let c2 = 2.0 * vec2.dot(&line_vec);
	let c3 = vec2.dot(&vec2);
	let c4 = vec1.add(&vec2).dot(&line_vec);
	let c5 = vec1.dot(&vec2);
	let c6 = line_vec.dot(&l2.u);
	let c7 = vec2.dot(&l2.u);
	let a = c6;
	let b = c1 + c4 * c6 + c7;
	let c = c1 * c2 + c5 * c6 + c4 * c7;
	let d = c1 * c3 + c5 * c7;
	// construct the solution from the root, the solution being the parameter
	// point reflected across the fold line, lying on the parameter line
    let mut polynomial_degree = 0;
	if c.abs() > EPSILON { polynomial_degree = 1; }
	if b.abs() > EPSILON { polynomial_degree = 2; }
	if a.abs() > EPSILON { polynomial_degree = 3; }
    let solutions = polynomial(polynomial_degree, a, b, c, d, &l1, &line_vec);
    return vec![];
	// return solutions
	// 	.map(p => normalize(subtract(p, pointA)))
	// 	.map((u, i) => ({ u, d: dot(u, midpoint(solutions[i], pointA)) }))
	// 	.map(ud_to_vector_origin)
	// 	.map(Constructors.line);
}

// l1 is the perpendicular to our solution
// l2 is the line we bring the point onto
pub fn axiom7 (p: &Vector, l1: &Line, l2: &Line, _boundary: &Square) -> Vec<Line> {
    let u = l1.u.rotate90();
    let u_u = u.dot(&l2.u);
    // if u_u is close to 0, the two input lines are parallel, no solution
    if u_u.abs() < EPSILON { return vec![] }
    let a = p.dot(&u);
    let b = p.dot(&l2.u);
    let d = (l2.d + 2.0 * a * u_u - b) / (2.0 * u_u);
    return vec![Line { u, d }];
}

