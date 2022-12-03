use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
	fn authorize(line1: String, line2: String, line3: String) -> u32 {
		let values = line1.chars();
		let mut p_score = 0;
		for c in values {
			if !line2.find(c).is_none() && !line3.find(c).is_none() {
				p_score = score(c);
			}
		}
		p_score
	}
	
	fn compare(line1: String, line2: String) -> u32 {
		let values = line1.chars();
		let mut p_score = 0;
		for c in values {
			if line2.find(c).is_none() {
				// Do nothing
			} else {
				p_score = score(c);
			}
		}
		p_score
	}
	fn score(c: char) -> u32 {
		if c.is_uppercase() {
			27 + (c as u32 - 'A' as u32)
		} else {
			1 + (c as u32 - 'a' as u32)
		}
	}
	
    println!("Day 02");
	let f = File::open("1.txt").expect("1.txt should be accessible");
	let f = BufReader::new(f);
	
	let mut total_priority_score = 0;
	let mut line_count = 0;
	let mut line1 = "".to_string();
	let mut line2 = "".to_string();
	let mut line3 = "".to_string();
	
	for line in f.lines() {
		let line = line.expect("We should be able to read strings from the file");
		line_count = line_count + 1;
		if line_count % 3 == 1 {
			line1 = line;
		} else if line_count % 3 == 2 {
			line2 = line;
		} else {
			line3 = line;
			total_priority_score = total_priority_score + authorize(line1.to_string(), line2.to_string(), line3.to_string());
		}
	}
	
	println!("{}", total_priority_score);
}
