use std::fs::File;
use std::io::prelude::*;
use Vector;
use Segment;

fn svg_points (points: &Vec<&Vector>) -> String {
	let mut strings: Vec<String> = Vec::new();
	for i in 0..points.len() {
		let mut string: String = String::new();
		string.push_str("<circle ");
		string.push_str(&format!("cx=\"{}\" ", points[i].x));
		string.push_str(&format!("cy=\"{}\" ", points[i].y));
		string.push_str(&format!("r=\"{}\" ", 0.002));
		string.push_str("/>\n");
		strings.push(string);
	}
	let mut string: String = String::new();
	for i in 0..strings.len() {
		string.push_str(&strings[i]);
	}
	return string;
}

fn svg_lines (segments: &Vec<(Segment, u64)>) -> String {
	let mut strings: Vec<String> = Vec::new();
	let mut max_occurrence: f64 = 0.0;
	for i in 0..segments.len() {
		if segments[i].1 as f64 > max_occurrence { max_occurrence = segments[i].1 as f64; }
	}
	for i in 0..segments.len() {
		let mut string: String = String::new();
		let opacity: f64 = ((segments[i].1 as f64) / max_occurrence).powf(0.3);
		// let mut opacity: f64 = ((segments[i].1 as f64) / max_occurrence).powf(0.15);
		// if opacity > 1.0 { opacity = 1.0 }
		// opacity *= 0.1;
		// let opacity: f64 = ((segments[i].1 as f64) / max_occurrence);
		string.push_str(&format!("<line stroke-opacity=\"{}\" ", opacity));
		string.push_str(&format!("x1=\"{}\" ", segments[i].0.a.x));
		string.push_str(&format!("y1=\"{}\" ", segments[i].0.a.y));
		string.push_str(&format!("x2=\"{}\" ", segments[i].0.b.x));
		string.push_str(&format!("y2=\"{}\" ", segments[i].0.b.y));
		string.push_str("/>\n");
		strings.push(string);
	}
	let mut string: String = String::new();
	for i in 0..strings.len() {
		string.push_str(&strings[i]);
	}
	return string;
}

fn write(string: &String) -> std::io::Result<()> {
	let mut file = File::create("image.svg")?;
	file.write_all(string.as_bytes())?;
	Ok(())
}

pub fn draw (segments: &Vec<(Segment, u64)>, points: &Vec<&Vector>) {
	let mut svg: String = String::new();
	svg.push_str("<svg version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"-0.01 -0.01 1.02 1.02\" width=\"2000px\" height=\"2000px\">\n");
	svg.push_str("<g fill=\"none\" stroke=\"white\" stroke-width=\"0.0002\">\n");
	svg.push_str("<rect x=\"-1\" y=\"-1\" width=\"3\" height=\"3\" fill=\"black\" stroke=\"none\" />\n");
	svg.push_str(&svg_lines(&segments));
	// boundary
	svg.push_str("<line stroke-opacity=\"1.0\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\" />\n");
	svg.push_str("<line stroke-opacity=\"1.0\" x1=\"1\" y1=\"0\" x2=\"1\" y2=\"1\" />\n");
	svg.push_str("<line stroke-opacity=\"1.0\" x1=\"1\" y1=\"1\" x2=\"0\" y2=\"1\" />\n");
	svg.push_str("<line stroke-opacity=\"1.0\" x1=\"0\" y1=\"1\" x2=\"0\" y2=\"0\" />\n");

	svg.push_str("</g>\n");
	// svg.push_str("<g fill=\"#e53\" stroke=\"none\">\n");
	// svg.push_str(&svg_points(points));
	// svg.push_str("</g>\n");
	svg.push_str("</svg>\n");
	let _res = write(&svg);
}

