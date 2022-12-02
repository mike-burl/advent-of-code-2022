use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() {	
    println!("Day 02");
	let f = File::open("1.txt").expect("1.txt should be accessible");
	let f = BufReader::new(f);
	
	let mut scores = HashMap::new();
	let mut total_score = 0;
	
	scores.insert(String::from("A X"), 3); // Lose : RvS 3 + 0
	scores.insert(String::from("A Y"), 4); // Draw : RvR 1 + 3
	scores.insert(String::from("A Z"), 8); // Win : RvP 2 + 6
	scores.insert(String::from("B X"), 1); // Lose : PvR 1 + 0
	scores.insert(String::from("B Y"), 5); // Draw : PvP 2 + 3
	scores.insert(String::from("B Z"), 9); // Win : PvS 3 + 6
	scores.insert(String::from("C X"), 2); // Lose : SvP 2 + 0
	scores.insert(String::from("C Y"), 6); // Draw : SvS 3 + 3
	scores.insert(String::from("C Z"), 7); // Win : SvR 1 + 6
	
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
