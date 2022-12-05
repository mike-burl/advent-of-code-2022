use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    println!("Day 05");
	let f = File::open("1.txt").expect("1.txt should be accessible");
	let f = BufReader::new(f);
	
	// Puzzle input
	let mut stacks = [
		vec!['X'], // Keep this just to make later parsing easier
		vec!['R', 'S', 'L', 'F', 'Q'], // 1
		vec!['N', 'Z', 'Q', 'G', 'P', 'T'], //2
		vec!['S', 'M', 'Q', 'B'], //3
		vec!['T', 'G', 'Z', 'J', 'H', 'C', 'B', 'Q'], //4
		vec!['P', 'H', 'M', 'B', 'N', 'F', 'S'], //5
		vec!['P', 'C', 'Q', 'N', 'S', 'L', 'V', 'G'], //6
		vec!['W', 'C', 'F'], //7
		vec!['Q', 'H', 'G', 'Z', 'W', 'V', 'P', 'M'], //8
		vec!['G', 'Z', 'D', 'L', 'C', 'N', 'R']
	];
	
	for line in f.lines() {
		let line = line.expect("We should be able to read strings from the file");
		
		// Extract the line's source, target, and # of crates to be moved
		let move_vector: Vec<&str> = line.split(",").collect();
		let move_count : i32 = move_vector[0].parse().unwrap();
		let source : usize = move_vector[1].parse().unwrap();
		let target : usize = move_vector[2].parse().unwrap();
		for count in 0..move_count {
			let crate_char : char = stacks[source].pop().unwrap();
			stacks[target].push(crate_char);
		}
	}
	
	for stack in stacks {
		println!("Source : {:?}", stack);
	}
	
	println!("done");
}
