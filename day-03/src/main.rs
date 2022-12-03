use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
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
	
	for line in f.lines() {
		let line = line.expect("We should be able to read strings from the file");
		if line.trim().is_empty() {
			// Do nothing
		} else {
			let pack_size = line.chars().count();
			let compartment_size = pack_size / 2;
			let line1 = &line[0..compartment_size];
			let line2 = &line[compartment_size..pack_size];
			total_priority_score = total_priority_score + compare(line1.to_string(), line2.to_string());
		}
	}
	
	println!("{}", total_priority_score);
}
