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

fn main() {
	let mut amount_of_numbers = String::new();

    eprintln!("Hur många tal?"); // remove later for faster runtime
	io::stdin()
	.read_line(&mut amount_of_numbers)
	.expect( ""); // Cause thep rogram crashes if you dont have this!

	let mut amount_of_numbers:u32 = amount_of_numbers
	.trim()
	.parse()
	.expect(""); 
	let mut amount_to_add = (amount_of_numbers + 1)/2; // integer divison baby!

	eprintln!("Du vill ha: {}", amount_of_numbers); // same as above comment
	eprintln!("Antalet tal som kommer adderas är: {}", amount_to_add);

}