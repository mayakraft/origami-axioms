use std::fs::File;
// use std::fs;
use std::io::prelude::*;
use Vector;
use Segment;

const STROKE_W: f64 = 0.0002;

// const SVG_HEADER: &str= "<svg version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"-0.01 -0.01 1.02 1.02\" width=\"907px\" height=\"907px\">\n";
const SVG_HEADER: &str= "<svg version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"-0.01 -0.01 1.02 1.02\" width=\"600px\" height=\"600px\">\n";

fn unit_square_boundary() -> String {
	format!("<g stroke=\"white\" stroke-width=\"{}\" stroke-opacity=\"1.0\">\n<line x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\" />\n<line x1=\"1\" y1=\"0\" x2=\"1\" y2=\"1\" />\n<line x1=\"1\" y1=\"1\" x2=\"0\" y2=\"1\" />\n<line x1=\"0\" y1=\"1\" x2=\"0\" y2=\"0\" />\n</g>\n", STROKE_W)
}

fn circle_elements (points: &Vec<&(Vector, u64)>) -> String {
	let mut strings: Vec<String> = Vec::new();
	// get the largest repeat value. scale all others in relation to this
	let mut repeat_max_f: u64 = 0;
	for i in 0..points.len() {
		if points[i].1 > repeat_max_f { repeat_max_f = points[i].1; }
	}
	let repeat_max: f64 = repeat_max_f as f64;
	for i in 0..points.len() {
		let opacity: f64 = ((points[i].1 as f64) / repeat_max).powf(0.15);
		let mut string: String = String::new();
		string.push_str("<circle ");
		string.push_str(&format!("cx=\"{}\" ", points[i].0.x));
		string.push_str(&format!("cy=\"{}\" ", points[i].0.y));
		string.push_str(&format!("r=\"{}\" ", 0.0005));
		string.push_str(&format!("opacity=\"{}\" ", opacity));
		string.push_str("/>\n");
		strings.push(string);
	}
	let mut string: String = String::new();
	for i in 0..strings.len() {
		string.push_str(&strings[i]);
	}
	return string;
}


fn line_elements (segments: &Vec<(Segment, u64)>) -> String {
	let mut strings: Vec<String> = Vec::new();
	// get the largest repeat value. scale all others in relation to this
	let mut repeat_max_f: u64 = 0;
	for i in 0..segments.len() {
		if segments[i].1 > repeat_max_f { repeat_max_f = segments[i].1; }
	}
	let repeat_max: f64 = repeat_max_f as f64;
	for i in 0..segments.len() {
		let mut string: String = String::new();
		let pct: f64 = (segments[i].1 as f64) / repeat_max; // (0.0, 1.0]
		// let pct2: f64 = ((segments[i].1 - 1) as f64) / repeat_max;  // [0.0, 1.0)
		// let gray: u8 = (255.0 * pct.powf(0.33)).floor() as u8;
		// let hue: u8 = (pct2 * 300.0) as u8;
		// let opacity: f64 = pct.powf(0.4) * 0.8;
		// let opacity: f64 = pct.powf(0.6) * 0.75;
		// let opacity: f64 = 0.03 + 0.15 * (pct.powf(0.3);
		let opacity: f64 = pct.powf(0.33);
		string.push_str("<line ");
		string.push_str(&format!("x1=\"{}\" ", segments[i].0.a.x));
		string.push_str(&format!("y1=\"{}\" ", segments[i].0.a.y));
		string.push_str(&format!("x2=\"{}\" ", segments[i].0.b.x));
		string.push_str(&format!("y2=\"{}\" ", segments[i].0.b.y));
		string.push_str(&format!("stroke-opacity=\"{}\" ", opacity));
		// string.push_str(&format!("stroke=\"rgb({},{},{})\" ", gray, gray, gray));
		// string.push_str(&format!("stroke=\"hsl({}, 85%, 45%)\" ", hue));
		string.push_str("/>\n");
		strings.push(string);
	}
	let mut string: String = String::new();
	for i in 0..strings.len() {
		string.push_str(&strings[i]);
	}
	return string;
}

pub fn svg_lines(segments: &Vec<(Segment, u64)>) -> String {
	let mut svg: String = String::new();
	svg.push_str(&SVG_HEADER.to_string());
	svg.push_str("<rect x=\"-1\" y=\"-1\" width=\"3\" height=\"3\" fill=\"black\" stroke=\"none\" />\n");
	svg.push_str(&format!("<g fill=\"none\" stroke=\"white\" stroke-width=\"{}\">\n", STROKE_W));
	svg.push_str(&line_elements(&segments));
	svg.push_str("</g>\n");
	svg.push_str(&unit_square_boundary());
	svg.push_str("</svg>\n");
	return svg;
}

pub fn svg_points(points: &Vec<&(Vector, u64)>) -> String {
	let mut svg: String = String::new();
	svg.push_str(&SVG_HEADER.to_string());
	svg.push_str("<rect x=\"-1\" y=\"-1\" width=\"3\" height=\"3\" fill=\"black\" stroke=\"none\" />\n");
	svg.push_str("<g fill=\"white\" stroke=\"none\">\n");
	svg.push_str(&circle_elements(&points));
	svg.push_str("</g>\n");
	// svg.push_str(&unit_square_boundary());
	svg.push_str("</svg>\n");
	return svg;
}

fn write(filename: String, data: &String) -> std::io::Result<()> {
	let mut file = File::create(format!("images/{}", filename))?;
	file.write_all(data.as_bytes())?;
	Ok(())
}

pub fn draw (segments: &Vec<(Segment, u64)>, points: &Vec<&(Vector, u64)>) {
	// fs::create_dir_all("/images")?;
	let _res_p = write("points.svg".to_string(), &svg_points(points));
	let _res_l = write("lines.svg".to_string(), &svg_lines(segments));
}

