/* 
*	Solution to Inda+22 Task 2
*	By: Anders Mouanga (mouanga@kth.se)
*
*	Kattis I/O functionality partly based on code from: 		https://open.kattis.com/help/rust
														   		https://github.com/IndaPlus22/AssignmentInstructions-BlueNote/blob/main/task-2/minimal_scalar_product/src/main.rs
																https://www.youtube.com/watch?v=JkJxRn1OnWA
*/
use std::io;
use std::io::prelude::*;
use std::io::{BufRead};

fn main() {
	// initialize some useful variables!
	let mut amount_of_numbers = String::new();
	let mut List : Vec<i32> = vec![];
	let mut total_sum = 0;
	let mut counter = 0;

    eprintln!("Hur många tal?"); // remove later for faster runtime
	io::stdin()
	.read_line(&mut amount_of_numbers)
	.expect( ""); // Cause thep rogram crashes if you dont have this!

	let mut amount_of_numbers:u32 = amount_of_numbers
	.trim()
	.parse()
	.expect(""); 
	let mut amount_to_add = (amount_of_numbers + 1)/2; // integer divison baby!

//	eprintln!("Du vill ha: {}", amount_of_numbers); // same as above comment
//	eprintln!("Antalet tal som kommer adderas är: {}", amount_to_add);
	
	let Numbers = io::stdin();
	for line in Numbers.lock().lines().map(|l| l.unwrap()) {
		total_sum = 0;
		counter = 0;
        let mut List: Vec<i32> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
			List.sort_by(|a, b| b.cmp(a))
			;
			for Number in List {
				if(counter) < amount_to_add {
					counter += 1;
					total_sum += Number;
			//		eprintln!("{}", total_sum);
				}
				else {
					println!("{}", total_sum);
					break;
				}
			}
	}		
	


}