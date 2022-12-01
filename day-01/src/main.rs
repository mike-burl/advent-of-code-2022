use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {	
	struct Elf {
		total_calories: i32,
		snacks: Vec<i32>
	}
	
	impl Elf {
		pub fn new() -> Elf {
			Elf { total_calories: 0, snacks: [].to_vec() }
		}
		
		pub fn add_snack(&mut self, snack: i32) -> () {
			self.snacks.push(snack);
			self.total_calories = self.total_calories + snack;
		}
	}
	
    println!("Day 01 Part 1");
	let f = File::open("1.txt").expect("1.txt should be accessible");
	let f = BufReader::new(f);
	
	let mut elf_vector : Vec<Elf> = Vec::new();
	let mut elf = Elf::new();
	
	for line in f.lines() {
		let line = line.expect("We should be able to read strings from the file");
		if line.trim().is_empty() {
			elf_vector.push(elf);
			elf = Elf::new();
		} else {
			let calorie: i32 = line.parse().unwrap();
			elf.add_snack(calorie);
		}
	}
	
	let most_calories = { 
		let mut local_high = 0;
		for elf in elf_vector {
			if elf.total_calories > local_high {
				local_high = elf.total_calories
			}
		}
		local_high
	};
	
	println!("{}", most_calories);
}
