extern crate queues;

use std::fs;
use queues::*;

fn main() {
	fn_1();
	fn_2();
}

fn fn_1() {
	let contents = fs::read_to_string("./input.txt").expect("something went wrong");

	let lines= contents.lines();
	let mut prev_number = std::i32::MIN;
	let mut counter = -1;
	for ele in lines.into_iter() {
		let new_number = ele.parse::<i32>().unwrap();
		if new_number > prev_number {
			print!("increased");
			counter+=1;
		}
		println!("{ }",&ele);
		prev_number = new_number;
	}
	println!("{}", &counter)
}

fn fn_2() {
	let contents = fs::read_to_string("./input.txt").unwrap();
	let lines= contents.lines();
    let mut q: Queue<i32> = queue![];

	let mut current_sum = 0;
	let mut counter = 0;

	for ele in lines.into_iter() {
		let new_number = ele.parse::<i32>().unwrap();
		if q.size() >= 3 {
			let old_sum = current_sum;

			let first= q.remove().unwrap();
			current_sum = current_sum - first + new_number;

			if current_sum > old_sum{
				counter += 1;
			}

			q.add(new_number).unwrap();

		}else{
			current_sum += new_number;
			q.add(new_number).unwrap();
		}
	}
	eprintln!("counter = {:#?}", counter);
	
}