use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    println!("Day 04");
	let f = File::open("1.txt").expect("1.txt should be accessible");
	let f = BufReader::new(f);
	
	let mut total_overlaps = 0;
	
	for line in f.lines() {
		let line = line.expect("We should be able to read strings from the file");
		let elf_vector: Vec<&str> = line.split(",").collect();
		let elf1 : Vec<&str> = elf_vector[0].split("-").collect();
		let elf2 : Vec<&str> = elf_vector[1].split("-").collect();
		// Get four numbers associated with both elf ranges
		let elf1_lower : i32 = elf1[0].parse().unwrap();
		let elf1_upper : i32 = elf1[1].parse().unwrap();
		let elf2_lower : i32 = elf2[0].parse().unwrap();
		let elf2_upper : i32 = elf2[1].parse().unwrap();
		// Check if elf1 numbers are outside of elf2's
		if elf1_lower <= elf2_lower && elf1_upper >= elf2_upper {
			total_overlaps = total_overlaps + 1;
		}
		// If miss, check if elf2 numbers are outside of elf1's
		else if elf2_lower <= elf1_lower && elf2_upper >= elf1_upper {
			total_overlaps = total_overlaps + 1;
		}
		else {
			// do nothing
		}
		
	}
	
	println!("{}", total_overlaps);
}
