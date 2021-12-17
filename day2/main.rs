
use std::fs;

fn main() {
    fn_1();
    fn_2();
}

fn fn_1() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let lines = contents.lines();
    let mut pos_y = 0;
    let mut pos_x = 0;

    for ele in lines.into_iter() {
        let parts: Vec<&str> = ele.split_whitespace().collect();
        let command = parts[0];
        let num = parts[1];
        let new_number = num.parse::<i32>().unwrap();
        eprintln!("command = {:#?}", command);
        eprintln!("num = {:#?}", new_number);
        if command == "down" {
            pos_y += new_number;
        } else if command == "up" {
            pos_y -= new_number;
        } else if command == "forward" {
            pos_x += new_number;
        }
    }

	eprintln!("result = {:#?}", pos_x * pos_y);
}

fn fn_2() {

	let contents = fs::read_to_string("./input2.txt").unwrap();
    let lines = contents.lines();
    let mut pos_y = 0;
    let mut pos_x = 0;
	let mut aim = 0;

    for ele in lines.into_iter() {
        let parts: Vec<&str> = ele.split_whitespace().collect();
        let command = parts[0];
        let num = parts[1];
        let new_number = num.parse::<i32>().unwrap();
        eprintln!("command = {:#?}", command);
        eprintln!("num = {:#?}", new_number);

        if command == "down" {
            aim += new_number;
        } else if command == "up" {
            aim -= new_number;
        } else if command == "forward" {
            pos_x += new_number;
			pos_y += new_number * aim;
        }
    }

	eprintln!("result = {:#?}", pos_x * pos_y);

}
