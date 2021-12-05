extern crate queues;

use std::fs;

fn main() {
    fn_1();
    fn_2();
}

fn fn_1() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let lines = contents.lines();
}

fn fn_2() {
	let contents = fs::read_to_string("./input2.txt").unwrap();
    let lines = contents.lines();

}
