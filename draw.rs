use std::fs::File;
use std::io::prelude::*;
use Vector;
use Segment;

fn svg_points (points: &Vec<Vector>) -> String {
	let mut strings: Vec<String> = Vec::new();
	for i in 0..points.len() {
		let mut string: String = String::new();
		string.push_str("<circle ");
		string.push_str(&format!("cx=\"{}\" ", points[i].x));
		string.push_str(&format!("cy=\"{}\" ", points[i].y));
		string.push_str(&format!("r=\"{}\" ", 0.001));
		string.push_str("/>\n");
		strings.push(string);
	}
	let mut string: String = String::new();
	for i in 0..strings.len() {
		string.push_str(&strings[i]);
	}
	return string;
}

fn svg_lines (segments: &Vec<Segment>) -> String {
	let mut strings: Vec<String> = Vec::new();
	for i in 0..segments.len() {
		let mut string: String = String::new();
		string.push_str("<line ");
		string.push_str(&format!("x1=\"{}\" ", segments[i].a.x));
		string.push_str(&format!("y1=\"{}\" ", segments[i].a.y));
		string.push_str(&format!("x2=\"{}\" ", segments[i].b.x));
		string.push_str(&format!("y2=\"{}\" ", segments[i].b.y));
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

pub fn draw (segments: &Vec<Segment>, points: &Vec<Vector>) {
	let mut svg: String = String::new();
	svg.push_str("<svg version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"-0.02 -0.02 1.04 1.04\" width=\"600px\" height=\"600px\">\n");
	svg.push_str("<g fill=\"none\" stroke=\"black\" stroke-width=\"0.001\">\n");
	svg.push_str("<rect x=\"0\" y=\"0\" width=\"1\" height=\"1\" />\n");
	svg.push_str(&svg_lines(&segments));
	svg.push_str("</g>\n");
	svg.push_str("<g fill=\"#e53\" stroke=\"none\">\n");
	svg.push_str(&svg_points(&points));
	svg.push_str("</g>\n");
	svg.push_str("</svg>\n");
	let _res = write(&svg);
}
