use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() {	
    println!("Day 02");
	let f = File::open("1.txt").expect("1.txt should be accessible");
	let f = BufReader::new(f);
	
	let mut scores = HashMap::new();
	let mut total_score = 0;
	
	scores.insert(String::from("A X"), 4); // RvR 1 + 3
	scores.insert(String::from("A Y"), 8); // RvP 2 + 6
	scores.insert(String::from("A Z"), 3); // RvS 3 + 0
	scores.insert(String::from("B X"), 1); // PvR 1 + 0
	scores.insert(String::from("B Y"), 5); // PvP 2 + 3
	scores.insert(String::from("B Z"), 9); // PvS 3 + 6
	scores.insert(String::from("C X"), 7); // SvR 1 + 6
	scores.insert(String::from("C Y"), 2); // SvP 2 + 0
	scores.insert(String::from("C Z"), 6); // SvS 3 + 3
	
	for line in f.lines() {
		let line = line.expect("We should be able to read strings from the file");
		if line.trim().is_empty() {
			// Do nothing
		} else {
			total_score = &total_score + scores.get(&line).copied().unwrap_or(0);
		}
	}
	
	println!("{}", total_score);
}
