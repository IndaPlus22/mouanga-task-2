/* 
*	Solution to Inda+22 Task 2
*	By: Anders Mouanga (mouanga@kth.se)
*
*	Kattis I/O functionality partly based on code from: 		https://open.kattis.com/help/rust
														   		https://github.com/IndaPlus22/AssignmentInstructions-BlueNote/blob/main/task-2/minimal_scalar_product/src/main.rs

*/
use std::io;
use std::io::prelude::*;

fn main() {
	let mut amount_of_numbers = String::new();

    println!("Hur många tal?");
	io::stdin().read_line(&mut amount_of_numbers).unwrap();
	println!("Du vill ha: {}", amount_of_numbers);
}